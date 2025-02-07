//! Tests the aggregate functionality

use eventbus_tiny::{aggregate_enum, EventBus};

/// Event type A
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventA;

/// Event type B
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventB;

/// Event type C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventC(u8);

// Aggregate event type
aggregate_enum!(AggregateEvent(EventA, EventB, EventC));
impl PartialEq for AggregateEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EventA(l0), Self::EventA(r0)) => l0 == r0,
            (Self::EventB(l0), Self::EventB(r0)) => l0 == r0,
            (Self::EventC(l0), Self::EventC(r0)) => l0 == r0,
            _ => false,
        }
    }
}

/// Tests filter subscribtion
#[test]
fn test() {
    // Create bus and event list
    let bus = EventBus::new();
    let expected = [
        AggregateEvent::EventC(EventC(0x04)),
        AggregateEvent::EventA(EventA),
        AggregateEvent::EventC(EventC(0x07)),
        AggregateEvent::EventB(EventB),
    ];

    // Create subscribers
    let aggregate = bus.subscribe_where(16, AggregateEvent::try_from_event);

    // Send events
    for event in expected.iter() {
        match *event {
            AggregateEvent::EventA(event_a) => bus.publish(event_a),
            AggregateEvent::EventB(event_b) => bus.publish(event_b),
            AggregateEvent::EventC(event_c) => bus.publish(event_c),
        };
    }

    // Receive events
    for expected_event in expected {
        let event = aggregate.recv().expect("failed to receive event");
        assert_eq!(event, expected_event);
    }
}
