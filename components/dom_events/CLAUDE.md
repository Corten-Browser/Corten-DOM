# Component: dom_events

## Component Identification
- **Name**: dom_events
- **Type**: Feature (Level 2)
- **Version**: 0.1.0
- **Dependencies**: dom-types, dom-core

## Responsibility
Implement complete DOM event system with dispatch, bubbling, capturing, and event listeners.

## Key Spec Sections
- Event System Interface (lines 296-336)
- Event Dispatch Algorithm (lines 566-632)

## Core Components

### 1. Event
```rust
pub struct Event {
    event_type: String,
    target: Option<EventTargetRef>,
    current_target: Option<EventTargetRef>,
    event_phase: EventPhase,
    bubbles: bool,
    cancelable: bool,
    default_prevented: bool,
    stop_propagation_flag: bool,
    stop_immediate_propagation_flag: bool,
}

impl Event {
    pub fn new(event_type: &str, init: EventInit) -> Self;
    pub fn prevent_default(&mut self);
    pub fn stop_propagation(&mut self);
    pub fn stop_immediate_propagation(&mut self);
}
```

### 2. EventTarget Trait
```rust
pub trait EventTarget: Node {
    fn add_event_listener(
        &mut self,
        event_type: &str,
        listener: EventListener,
        options: AddEventListenerOptions
    );
    fn remove_event_listener(
        &mut self,
        event_type: &str,
        listener: EventListener,
    );
    fn dispatch_event(&mut self, event: EventRef) -> Result<bool, DomException>;
}
```

### 3. EventDispatcher (Critical)
```rust
pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(event: EventRef, target: EventTargetRef) -> Result<bool, DomException> {
        // 1. Validate
        // 2. Calculate propagation path
        // 3. CAPTURE phase (from root to target)
        // 4. TARGET phase
        // 5. BUBBLE phase (from target to root)
        // 6. Cleanup
    }
}
```

## Critical Algorithm: Event Dispatch

**Must implement spec algorithm exactly (lines 566-632)**:

```rust
// Phase 1: Capture (ancestors → target)
for ancestor in path.iter().rev() {
    if event.stop_propagation_flag { break; }
    invoke_listeners(ancestor, event, EventPhase::Capturing);
}

// Phase 2: Target
invoke_listeners(target, event, EventPhase::AtTarget);

// Phase 3: Bubble (target → ancestors)
if event.bubbles {
    for ancestor in path.iter() {
        if event.stop_propagation_flag { break; }
        invoke_listeners(ancestor, event, EventPhase::Bubbling);
    }
}
```

## TDD: Event Bubbling Test
```rust
#[test]
fn test_event_bubbling_order() {
    let doc = Document::new();
    let parent = doc.create_element("div").unwrap();
    let child = doc.create_element("button").unwrap();
    parent.append_child(child.clone());

    let mut order = vec![];

    // Capture listener on parent
    parent.add_event_listener("click",
        EventListener::Function(Box::new(|_| order.push("parent_capture"))),
        AddEventListenerOptions { capture: true, ..Default::default() }
    );

    // Bubble listener on parent
    parent.add_event_listener("click",
        EventListener::Function(Box::new(|_| order.push("parent_bubble"))),
        AddEventListenerOptions::default()
    );

    // Target listener on child
    child.add_event_listener("click",
        EventListener::Function(Box::new(|_| order.push("child"))),
        AddEventListenerOptions::default()
    );

    // Dispatch
    let event = Event::new("click", EventInit { bubbles: true, ..Default::default() });
    child.dispatch_event(event);

    // Verify order
    assert_eq!(order, vec!["parent_capture", "child", "parent_bubble"]);
}
```

## Quality Gates
- ✅ Event dispatch algorithm correct
- ✅ Capture/bubble phases work
- ✅ stop_propagation works correctly
- ✅ All tests pass, coverage ≥ 80%

## Success Criteria
1. Complete event dispatch implementation
2. Capture and bubble phases work correctly
3. Event listener management works
4. All tests pass

## Estimated Effort
- **LOC**: ~4,000-5,000
- **Time**: 12-14 hours
