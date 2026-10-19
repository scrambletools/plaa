//! Packets read from the file, waiting for their stream's decoder.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use ffmpeg_next as ffmpeg;

use crate::edit::Span;

pub enum Item {
    /// A packet of this serial, from this part of the file.
    Packet(u64, Span, ffmpeg::Packet),
    /// The end of the file, for packets of this serial.
    End(u64),
}

impl Item {
    fn bytes(&self) -> usize {
        match self {
            Item::Packet(_, _, packet) => packet.size(),
            Item::End(_) => 0,
        }
    }
}

#[derive(Default)]
struct Inner {
    items: VecDeque<Item>,
    bytes: usize,
    closed: bool,
    /// The media time read up to, in seconds: the end of the latest
    /// packet.
    end: Option<f64>,
}

#[derive(Default)]
pub struct PacketQueue {
    inner: Mutex<Inner>,
    ready: Condvar,
}

/// Packets worth holding per stream: enough to ride out a slow disk or
/// network without holding much of a file in memory.
const ENOUGH_PACKETS: usize = 48;
const ENOUGH_BYTES: usize = 8 * 1024 * 1024;

impl PacketQueue {
    fn inner(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn put(&self, item: Item) {
        let mut inner = self.inner();
        inner.bytes += item.bytes();
        inner.items.push_back(item);
        self.ready.notify_one();
    }

    /// Notes that packets up to `seconds` of media time have been read.
    pub fn reached(&self, seconds: f64) {
        let mut inner = self.inner();
        if inner.end.is_none_or(|end| seconds > end) {
            inner.end = Some(seconds);
        }
    }

    /// The media time read up to since the last seek, if anything was.
    pub fn end(&self) -> Option<f64> {
        self.inner().end
    }

    /// Whether the decoder has enough waiting for now.
    pub fn has_enough(&self) -> bool {
        let inner = self.inner();
        inner.items.len() >= ENOUGH_PACKETS || inner.bytes >= ENOUGH_BYTES
    }

    pub fn bytes(&self) -> usize {
        self.inner().bytes
    }

    /// Drops every packet waiting, as a seek makes them stale.
    pub fn clear(&self) {
        let mut inner = self.inner();
        inner.items.clear();
        inner.bytes = 0;
        inner.end = None;
    }

    /// The next item, waiting up to `timeout`. `None` when there was none,
    /// and `Err` once the queue is closed.
    pub fn get(&self, timeout: Duration) -> Result<Option<Item>, Closed> {
        let mut inner = self.inner();
        loop {
            if inner.closed {
                return Err(Closed);
            }
            if let Some(item) = inner.items.pop_front() {
                inner.bytes -= item.bytes();
                return Ok(Some(item));
            }
            let (next, waited) = self
                .ready
                .wait_timeout(inner, timeout)
                .unwrap_or_else(PoisonError::into_inner);
            inner = next;
            if waited.timed_out() && inner.items.is_empty() {
                return if inner.closed { Err(Closed) } else { Ok(None) };
            }
        }
    }

    pub fn close(&self) {
        let mut inner = self.inner();
        inner.closed = true;
        inner.items.clear();
        inner.bytes = 0;
        self.ready.notify_all();
    }
}

#[derive(Debug)]
pub struct Closed;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hands_out_in_order_until_closed() {
        let queue = PacketQueue::default();
        queue.put(Item::Packet(
            0,
            Span::WHOLE,
            ffmpeg::Packet::copy(&[1, 2, 3]),
        ));
        queue.put(Item::End(0));
        assert_eq!(queue.bytes(), 3);
        assert!(matches!(
            queue.get(Duration::ZERO),
            Ok(Some(Item::Packet(0, _, _)))
        ));
        assert!(matches!(queue.get(Duration::ZERO), Ok(Some(Item::End(0)))));
        assert!(matches!(queue.get(Duration::from_millis(1)), Ok(None)));
        queue.close();
        assert!(queue.get(Duration::ZERO).is_err());
    }

    #[test]
    fn knows_how_far_it_has_read() {
        let queue = PacketQueue::default();
        assert_eq!(queue.end(), None);
        queue.reached(2.0);
        // A packet out of order does not move it back.
        queue.reached(1.5);
        assert_eq!(queue.end(), Some(2.0));
        queue.clear();
        assert_eq!(queue.end(), None);
    }
}
