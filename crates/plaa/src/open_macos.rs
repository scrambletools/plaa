//! Files Finder and other apps ask plaa to open: the "open documents"
//! Apple event, as when a file is double-clicked, chosen in Open With or
//! dropped on plaa's icon, at launch as well as later. AppKit answers it
//! through the app delegate, which is winit's and does not take it, so
//! plaa installs its own handler as the app is about to finish launching,
//! where Apple has apps install theirs, before the first event comes.
#![allow(unsafe_code)]

use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject};
use objc2::{AllocAnyThread, class, define_class, msg_send, sel};
use objc2_foundation::NSString;

/// A four character code, as Apple events name their classes and keys.
const fn code(text: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*text)
}

const CORE_EVENT_CLASS: u32 = code(b"aevt");
const OPEN_DOCUMENTS: u32 = code(b"odoc");
const DIRECT_OBJECT: u32 = code(b"----");

type Handler = Box<dyn Fn(Vec<PathBuf>) + Send>;
static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);

define_class!(
    // SAFETY: NSObject has no subclassing requirements, and the class
    // keeps nothing of its own.
    #[unsafe(super(NSObject))]
    #[name = "PlaaOpenDocuments"]
    struct Receiver;

    impl Receiver {
        /// AppKit has installed its own handlers: plaa's replaces its
        /// handler for opening documents.
        #[unsafe(method(willFinishLaunching:))]
        fn will_finish_launching(&self, _notification: &AnyObject) {
            // SAFETY: the manager keeps the receiver, which lives as long
            // as the app, and calls the selector it names, which takes
            // the event and its reply.
            unsafe {
                let manager: Retained<AnyObject> =
                    msg_send![class!(NSAppleEventManager), sharedAppleEventManager];
                let _: () = msg_send![
                    &*manager,
                    setEventHandler: self,
                    andSelector: sel!(openDocuments:withReply:),
                    forEventClass: CORE_EVENT_CLASS,
                    andEventID: OPEN_DOCUMENTS
                ];
            }
        }

        #[unsafe(method(openDocuments:withReply:))]
        fn open_documents(&self, event: &AnyObject, _reply: &AnyObject) {
            let files = files(event);
            if files.is_empty() {
                return;
            }
            if let Some(handler) = &*HANDLER.lock().unwrap_or_else(PoisonError::into_inner) {
                handler(files);
            }
        }
    }
);

/// The files an "open documents" event names, in its direct object: a
/// list of file URLs.
fn files(event: &AnyObject) -> Vec<PathBuf> {
    // SAFETY: each message is NSAppleEventDescriptor's, with the types its
    // header gives, on descriptors the event holds.
    unsafe {
        let Some(list): Option<Retained<AnyObject>> =
            msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT]
        else {
            return Vec::new();
        };
        let count: isize = msg_send![&*list, numberOfItems];
        let items: Vec<Retained<AnyObject>> = if count == 0 {
            vec![list]
        } else {
            (1..=count)
                .filter_map(|index| msg_send![&*list, descriptorAtIndex: index])
                .collect()
        };
        items
            .iter()
            .filter_map(|item| {
                let url: Option<Retained<AnyObject>> = msg_send![&**item, fileURLValue];
                let path: Option<Retained<NSString>> = msg_send![&*url?, path];
                Some(PathBuf::from(path?.to_string()))
            })
            .collect()
    }
}

/// Sends the files Finder and other apps ask plaa to open to `handler`,
/// from the app's launch on: call it before the event loop starts.
pub fn install(handler: impl Fn(Vec<PathBuf>) + Send + 'static) {
    *HANDLER.lock().unwrap_or_else(PoisonError::into_inner) = Some(Box::new(handler));
    let receiver: Retained<Receiver> = {
        let this = Receiver::alloc().set_ivars(());
        // SAFETY: NSObject's init on the receiver just allocated.
        unsafe { msg_send![super(this), init] }
    };
    let name = NSString::from_str("NSApplicationWillFinishLaunchingNotification");
    // SAFETY: the center calls the selector, which takes the
    // notification, on the receiver, kept for the app's life below.
    unsafe {
        let center: Retained<AnyObject> = msg_send![class!(NSNotificationCenter), defaultCenter];
        let _: () = msg_send![
            &*center,
            addObserver: &*receiver,
            selector: sel!(willFinishLaunching:),
            name: &*name,
            object: std::ptr::null::<AnyObject>()
        ];
    }
    // It answers for as long as the app runs.
    std::mem::forget(receiver);
}
