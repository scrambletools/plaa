//! Packets read from the file, waiting for their stream's decoder.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use ffmpeg_next as ffmpeg;

pub enum Item {
    Packet(u64, ffmpeg::Packet),
    /// The end of the file, for packets of this serial.
    End(u64),
}

impl Item {
    fn bytes(&self) -> usize {
        match self {
            Item::Packet(_, packet) => packet.size(),
            Item::End(_) => 0,
        }
    }
}

#[derive(Default)]
struct Inner {
    items: VecDeque<Item>,
    bytes: usize,
    closed: bool,
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
        queue.put(Item::Packet(0, ffmpeg::Packet::copy(&[1, 2, 3])));
        queue.put(Item::End(0));
        assert_eq!(queue.bytes(), 3);
        assert!(matches!(
            queue.get(Duration::ZERO),
            Ok(Some(Item::Packet(0, _)))
        ));
        assert!(matches!(queue.get(Duration::ZERO), Ok(Some(Item::End(0)))));
        assert!(matches!(queue.get(Duration::from_millis(1)), Ok(None)));
        queue.close();
        assert!(queue.get(Duration::ZERO).is_err());
    }
}
