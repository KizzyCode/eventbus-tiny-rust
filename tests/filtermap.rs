//! Tests the aggregate functionality

use eventbus_tiny::{enum_from, where_into, EventBus};
use std::collections::BTreeSet;

/// Event type A
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventA;

/// Event type B
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventB;

/// Event type C
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventC(u8);

/// Aggregate event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum AggregateEvent {
    A(EventA),
    B(EventB),
    C(EventC),
}
enum_from! {
    EventA => AggregateEvent::A,
    EventB => AggregateEvent::B,
    EventC => AggregateEvent::C
}

/// Tests filter subscribtion
#[test]
fn test() {
    // Create bus and event list
    let bus = EventBus::new();
    let expected = BTreeSet::from_iter([
        AggregateEvent::C(EventC(0x04)),
        AggregateEvent::A(EventA),
        AggregateEvent::C(EventC(0x07)),
        AggregateEvent::B(EventB),
    ]);

    // Create subscribers
    let aggregate = bus.subscribe_where(16, where_into!(EventA, EventB, EventC => AggregateEvent));

    // Send events
    for event in expected.iter() {
        match *event {
            AggregateEvent::A(event_a) => bus.publish(event_a),
            AggregateEvent::B(event_b) => bus.publish(event_b),
            AggregateEvent::C(event_c) => bus.publish(event_c),
        };
    }

    // Collect events
    let mut events = BTreeSet::new();
    while events.len() < expected.len() {
        // Receive next event
        let event = aggregate.recv().expect("failed to receive event");
        events.insert(event);
    }

    // Validate received events
    assert_eq!(events, expected);
}
