//! Tests normal pub-sub functionality

use eventbus_tiny::{EventBus, Subscription};

/// Event type A
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventA(u32);

/// Event type B
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventB(u64);

/// Event type C
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventC(u128);

/// Basic functionality test
#[test]
fn test() {
    // Create bus and subscribers
    let bus = EventBus::new();
    let subscriber_a: Subscription<usize> = bus.subscribe(64);
    let subscriber_b: Subscription<usize> = bus.subscribe(64);

    // Publish some events and ensure they get delivered to all two subscribers
    assert_eq!(bus.publish(1usize), 2);
    assert_eq!(bus.publish(4usize), 2);
    assert_eq!(bus.publish(7usize), 2);

    // Receive events
    assert_eq!(subscriber_a.recv(), Ok(1usize));
    assert_eq!(subscriber_a.recv(), Ok(4usize));
    assert_eq!(subscriber_a.recv(), Ok(7usize));

    assert_eq!(subscriber_b.recv(), Ok(1usize));
    assert_eq!(subscriber_b.recv(), Ok(4usize));
    assert_eq!(subscriber_b.recv(), Ok(7usize));

    // Drop bus and assert the subscribers are dead
    drop(bus);
    assert!(subscriber_a.recv().is_err());
    assert!(subscriber_b.recv().is_err());
}

/// Tests pub-sub spamming
#[test]
fn spam() {
    /// Amount of spam messages
    const SPAM_COUNT: u32 = 3 * 1024;

    // Create bus and subscribers
    let bus = EventBus::new();
    let subscribers_a: Vec<_> = (0..SPAM_COUNT).map(|_| bus.subscribe::<EventA>(SPAM_COUNT as usize)).collect();
    let subscribers_b: Vec<_> = (0..SPAM_COUNT).map(|_| bus.subscribe::<EventB>(SPAM_COUNT as usize)).collect();
    let subscribers_c: Vec<_> = (0..SPAM_COUNT).map(|_| bus.subscribe::<EventC>(SPAM_COUNT as usize)).collect();

    // Spam spam spam
    for index in 0..SPAM_COUNT {
        assert_eq!(bus.publish(EventA(index)), SPAM_COUNT as usize);
        assert_eq!(bus.publish(EventB(index as u64)), SPAM_COUNT as usize);
        assert_eq!(bus.publish(EventC(index as u128)), SPAM_COUNT as usize);
    }

    // Collect spam
    for index in 0..SPAM_COUNT {
        for subscriber in &subscribers_a {
            let event = subscriber.recv().expect("failed to receive event");
            assert_eq!(event, EventA(index));
        }
        for subscriber in &subscribers_b {
            let event = subscriber.recv().expect("failed to receive event");
            assert_eq!(event, EventB(index as u64));
        }
        for subscriber in &subscribers_c {
            let event = subscriber.recv().expect("failed to receive event");
            assert_eq!(event, EventC(index as u128));
        }
    }
}
