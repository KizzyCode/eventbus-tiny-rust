//! A tiny pub/sub event bus

use crate::sub::{self, AnySubscriber, Subscription};
use std::any::Any;
use std::sync::{Arc, RwLock};

/// A tiny pub/sub event bus
///
/// # Performance and Locking
/// While the event bus is not entirely lock-free, blocking write-locks are only used on registry updates; i.e.
/// [`Self::subscribe`] and [`Self::shrink_to_fit`]. Event publication uses a non-blocking read-lock, and event
/// listening and receiption is completely lock-free. In normal scenarios with moderate subscriber fluctuation, event
/// passing is largely independent on mutex performance and only affected by the amount of event types and subscribers.
///
/// # Memory Allocation and Publication
/// The event bus itself only (de-)allocates memory on registry updates; i.e. [`Self::subscribe`] and
/// [`Self::shrink_to_fit`].
///
/// **However**, during event publication events are cloned for each subscriber to achieve strong decoupling. If
/// [`Clone`] is not cheap for your event type or you expect a large number of subscribers, consider wrapping the event
/// into an [`Arc`](std::sync::Arc) or similar referencing types to keep the performance and memory impact low.
///
/// # Many Subscribers
/// Due to the type abstraction layer, each new event needs to be checked against every subscriber to see if the
/// subscriber can handle this event. While this check is cheap, it may accumulate if you have a very high
/// event-throughput with lots of subscribers.
#[derive(Debug, Default)]
pub struct EventBus {
    /// A registry for event subscribers
    registry: RwLock<Vec<Box<dyn AnySubscriber>>>,
}
impl EventBus {
    /// Creates a new event bus
    pub const fn new() -> Self {
        let registry = RwLock::new(Vec::new());
        Self { registry }
    }

    /// Publishes an event to all registered subscribers and returns the amount of subscribers addressed
    pub fn publish<T>(&self, event: T) -> usize
    where
        T: Send + Clone + 'static,
    {
        // Broadcast message to subscribers
        let registry = self.registry.read().expect("failed to lock registry");
        registry.iter().fold(0, |count, subscriber| {
            // Try to send message
            #[allow(clippy::arithmetic_side_effects, reason = "Can never overflow")]
            (count + subscriber.send(&event) as usize)
        })
    }

    /// Subscribes to a given event type
    #[must_use]
    pub fn subscribe<T>(&self, backlog: usize) -> Subscription<T>
    where
        T: Send + Clone + 'static,
    {
        /// Identity filter-map function for `T`
        fn filter_map<T>(event: &dyn Any) -> Option<T>
        where
            T: Clone + 'static,
        {
            event.downcast_ref::<T>().cloned()
        }

        // Create subscription
        self.subscribe_where(backlog, filter_map)
    }

    /// Creates an aggregate subscriber for any event type `X` where `aggregate(&X) => Some(T)`
    ///
    /// # Performance
    /// Please note that the mapping function is called for event for each subscriber to see if the event can be
    /// delivered to the subscriber. If the mapping is expensive, it is recommended to add an early-abort check before
    /// the real mapping begins to quickly reject invalid types.
    ///
    /// # See Also
    /// See also [`crate::where_into`] and [`crate::where_try_into`] to create mappers for `Into` and `TryInto`
    /// convertible types.
    #[must_use]
    pub fn subscribe_where<T, F>(&self, backlog: usize, filter_map: F) -> Subscription<T>
    where
        T: Send + Clone + 'static,
        F: Fn(&dyn Any) -> Option<T> + Send + Sync + 'static,
    {
        // Create channel
        let convert = Arc::new(filter_map);
        let (subscriber, subscription) = sub::pair(backlog, convert);
        let subscriber: Box<dyn AnySubscriber> = Box::new(subscriber);

        // Register sender
        let mut registry = self.registry.write().expect("failed to lock registry");
        registry.push(subscriber);

        // Return associated subscriber
        subscription
    }

    /// Shrinks the allocated capacity as much as possible
    pub fn shrink_to_fit(&self) {
        // Remove all dead subscribers
        let mut registry = self.registry.write().expect("failed to lock registry");
        registry.retain(|subscriber| subscriber.is_alive());
        registry.shrink_to_fit();
    }
}
