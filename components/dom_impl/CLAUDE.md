# Component: dom_impl

## Component Identification
- **Name**: dom_impl (browser-dom-impl)
- **Type**: Application (Level 3)
- **Version**: 0.1.0
- **Dependencies**: ALL DOM components

## Responsibility
Main library that re-exports all DOM APIs and provides message bus integration for browser components.

## Key Spec Sections
- Component Message Bus Integration (lines 415-508)
- Integration Points (lines 1186-1257)

## Core Components

### 1. Public API Re-exports
```rust
// Re-export all APIs from component libraries
pub use dom_types::*;
pub use dom_storage::*;
pub use dom_core::*;
pub use dom_collections::*;
pub use dom_events::*;
pub use dom_selectors::*;
pub use dom_advanced::*;

// Local modules
pub mod config;
pub mod messages;
pub mod component;

pub use config::DomConfig;
pub use messages::{DomComponentMessage, DomComponentResponse};
pub use component::DomComponent;
```

### 2. Message Bus Integration
```rust
pub enum DomComponentMessage {
    // From HTML Parser
    ParsedDocument {
        request_id: u64,
        root: ParsedNode,
        doctype: Option<DocumentType>,
    },

    // From JavaScript Runtime
    ScriptManipulation {
        operation: DomOperation,
        node_id: NodeId,
        params: OperationParams,
    },

    // From CSS Engine
    StyleInvalidation {
        node_id: NodeId,
        reason: InvalidationReason,
    },

    // From Browser Shell
    UserInteraction {
        event_type: String,
        target_id: NodeId,
        event_data: EventData,
    },
}

pub enum DomComponentResponse {
    // To JavaScript
    DomMutated {
        mutations: Vec<MutationRecord>,
        affected_nodes: Vec<NodeId>,
    },

    // To CSS Engine
    DomTreeChanged {
        change_type: TreeChangeType,
        affected_subtree: NodeId,
        needs_style_recalc: bool,
    },

    // To Rendering
    LayoutInvalidated {
        root_id: NodeId,
        invalidation_type: LayoutInvalidationType,
    },

    // Query responses
    QueryResult {
        request_id: u64,
        result: QueryResultType,
    },
}
```

### 3. DomComponent Main Structure
```rust
pub struct DomComponent {
    documents: HashMap<DocumentId, DocumentRef>,
    node_registry: HashMap<NodeId, WeakNodeRef>,
    config: DomConfig,
}

impl DomComponent {
    pub fn new(config: DomConfig) -> Self;

    pub fn handle_message(&mut self, msg: DomComponentMessage) -> DomComponentResponse {
        match msg {
            DomComponentMessage::ParsedDocument { request_id, root, doctype } => {
                let document = self.build_dom_tree(root);
                self.documents.insert(request_id, document);
                DomComponentResponse::DomTreeReady { request_id, document_id: request_id }
            },

            DomComponentMessage::ScriptManipulation { operation, node_id, params } => {
                self.execute_operation(operation, node_id, params)
            },

            // ... handle other message types
        }
    }

    fn build_dom_tree(&mut self, parsed: ParsedNode) -> DocumentRef {
        // Convert parsed HTML into DOM nodes
        let document = Document::new();
        let root = self.convert_parsed_node(&document, parsed);
        document.set_document_element(root);
        document
    }

    fn execute_operation(&mut self, op: DomOperation, node_id: NodeId, params: OperationParams) -> DomComponentResponse {
        match op {
            DomOperation::AppendChild { parent_id, child_id } => {
                // Execute appendChild on nodes
            },
            DomOperation::SetAttribute { element_id, name, value } => {
                // Execute setAttribute
            },
            // ... other operations
        }
    }
}
```

### 4. Configuration
```rust
#[derive(Debug, serde::Deserialize)]
pub struct DomConfig {
    pub max_tree_depth: usize,
    pub max_children: usize,
    pub enable_mutation_observers: bool,
    pub enable_shadow_dom: bool,
    pub gc_threshold: usize,
    pub arena_capacity: usize,
}

impl Default for DomConfig {
    fn default() -> Self {
        DomConfig {
            max_tree_depth: 512,
            max_children: 10000,
            enable_mutation_observers: true,
            enable_shadow_dom: true,
            gc_threshold: 100000,
            arena_capacity: 50000,
        }
    }
}
```

## Implementation Strategy

This is an **integration component**, so focus on:

1. **Re-exporting APIs**: Make sure all public APIs from other components are accessible
2. **Message Handling**: Translate browser messages to DOM operations
3. **Coordination**: Manage multiple documents, node registries
4. **Configuration**: Provide runtime configuration

**DO NOT reimplement logic** - delegate to library components!

## TDD: Integration Tests
```rust
#[test]
fn test_message_handling_script_manipulation() {
    let mut component = DomComponent::new(DomConfig::default());

    // Create document
    let doc_msg = DomComponentMessage::ParsedDocument {
        request_id: 1,
        root: create_test_parsed_tree(),
        doctype: None,
    };
    component.handle_message(doc_msg);

    // Manipulate via script message
    let script_msg = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::SetAttribute {
            element_id: 123,
            name: "class".to_string(),
            value: "active".to_string(),
        },
        node_id: 123,
        params: OperationParams::default(),
    };

    let response = component.handle_message(script_msg);
    // Verify mutation response
}
```

## Quality Gates
- ✅ All APIs re-exported correctly
- ✅ Message handling works
- ✅ Integration tests pass (100%)
- ✅ Cross-component integration works
- ✅ Coverage ≥ 80%

## Success Criteria
1. All component APIs accessible through dom_impl
2. Message bus integration works
3. Can create and manipulate DOM via messages
4. All integration tests pass

## Estimated Effort
- **LOC**: ~2,000-2,500 (thin integration layer)
- **Time**: 6-8 hours

## Critical Notes
- This component is THIN - it delegates to libraries
- Focus on integration, not reimplementation
- Test cross-component communication
- Ensure all public APIs are accessible
