//! Event subscribtion types

use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::Arc;

/// A fallible function that can cast `&dyn Any` to `T`
pub type Converter<T> = dyn Fn(&dyn Any) -> Option<T> + Send + Sync;

/// Type-erasure for event [`Subscriber`]s
pub trait AnySubscriber
where
    Self: Debug + Send + Sync,
{
    /// Whether the subscriber is still alive and could receive messages or not (see also [`Subscriber::send`] plus
    /// notes)
    #[must_use]
    fn is_alive(&self) -> bool;

    /// Sends an event to the subscriber
    #[must_use]
    fn send(&self, event: &dyn Any) -> bool;
}
impl<T> AnySubscriber for Subscriber<T>
where
    T: Send + 'static,
{
    fn is_alive(&self) -> bool {
        Subscriber::is_alive(self)
    }

    fn send(&self, event: &dyn Any) -> bool {
        Subscriber::send(self, event)
    }
}

/// An event subscriber handle for an event type `T`
pub struct Subscriber<T> {
    /// The sender queue
    sender: SyncSender<T>,
    /// An is-alive reference counter
    alive: Arc<AtomicBool>,
    /// A conversion to convert
    convert: Arc<Converter<T>>,
}
impl<T> Subscriber<T> {
    /// Tries to send a non-blocking event to the subscriber and returns if the event was sent successfully
    ///
    /// # Important
    /// Please note that if an event has been sent, this only means that the event is now in a state that it can be
    /// received by the subscriber, but it has not been received or processed yet.
    pub fn send(&self, event: &dyn Any) -> bool {
        // Try to convert the event
        let Some(event) = (self.convert)(event) else {
            // Event is incompatible
            return false;
        };

        // Try to send the event
        let result = self.sender.try_send(event);
        if matches!(result, Err(TrySendError::Disconnected(_))) {
            // Mark subscriber as dead
            self.alive.store(false, Ordering::SeqCst);
        }

        // Return send-state
        result.is_ok()
    }

    /// Whether the subscriber is still alive and could receive messages or not (see also [`Self::send`] plus notes)
    #[must_use]
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
}
impl<T> Debug for Subscriber<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("Subscriber").field("sender", &self.sender).field("alive", &self.alive).finish()
    }
}
impl<T> Clone for Subscriber<T> {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone(), alive: self.alive.clone(), convert: self.convert.clone() }
    }
}

/// An event subscription channel
pub struct Subscription<T> {
    /// The receive queue
    receiver: Receiver<T>,
    /// An is-alive flag
    alive: Arc<AtomicBool>,
}
impl<T> Deref for Subscription<T> {
    type Target = Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}
impl<T> Debug for Subscription<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("Subscription").field("receiver", &self.receiver).field("alive", &self.alive).finish()
    }
}
impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        // Mark as dead
        self.alive.store(false, Ordering::SeqCst);
    }
}

/// Creates a new `(subscriber, subscribption)`-pair with the given backlog as capacity limit
pub fn pair<T>(backlog: usize, convert: Arc<Converter<T>>) -> (Subscriber<T>, Subscription<T>)
where
    T: Clone + 'static,
{
    // Create underlying communication types
    let (sender, receiver) = mpsc::sync_channel(backlog);
    let alive = Arc::new(AtomicBool::new(true));

    // Create connected subscriber/subscription pair
    let subscriber = Subscriber { sender, alive: alive.clone(), convert };
    let subscription = Subscription { receiver, alive };
    (subscriber, subscription)
}
