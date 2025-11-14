# Component: dom_selectors

## Component Identification
- **Name**: dom_selectors
- **Type**: Feature (Level 2)
- **Version**: 0.1.0
- **Dependencies**: dom-types, dom-core, dom-collections

## Responsibility
Implement CSS selector matching (querySelector, querySelectorAll, matches, closest).

## Key Spec Sections
- Selectors API (lines 224-228)

## Core Components

### 1. Selectable Trait
```rust
pub trait Selectable {
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
    fn query_selector_all(&self, selector: &str) -> Result<NodeList, DomException>;
    fn matches(&self, selector: &str) -> Result<bool, DomException>;
    fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
}
```

### 2. Selector Parser
```rust
use cssparser::Parser;
use selectors::parser::SelectorList;

pub struct SelectorQuery;

impl SelectorQuery {
    pub fn parse(selector: &str) -> Result<SelectorList, DomException> {
        // Use cssparser crate to parse CSS selectors
    }

    pub fn matches(element: &Element, selector: &SelectorList) -> bool {
        // Match element against parsed selector
    }
}
```

## Implementation Strategy

Use existing Rust crates:
- **cssparser**: Parse CSS selectors
- **selectors**: Selector matching engine

**Example**:
```rust
use cssparser::{Parser, ParserInput};
use selectors::{matching, Element as SelectorElement};

impl Selectable for Element {
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException> {
        // 1. Parse selector
        let parsed = parse_selector(selector)?;

        // 2. Depth-first search
        self.find_first_matching(parsed)
    }

    fn matches(&self, selector: &str) -> Result<bool, DomException> {
        let parsed = parse_selector(selector)?;
        Ok(matches_selector(self, &parsed))
    }

    fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException> {
        let parsed = parse_selector(selector)?;

        // Walk up parent chain
        let mut current = Some(self.clone());
        while let Some(element) = current {
            if matches_selector(&element, &parsed) {
                return Ok(Some(element));
            }
            current = element.parent_element();
        }
        Ok(None)
    }
}
```

## TDD Tests
```rust
#[test]
fn test_query_selector_simple() {
    let doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let span = doc.create_element("span").unwrap();
    span.set_attribute("id", "test");
    root.append_child(span);

    let result = root.query_selector("#test").unwrap();
    assert!(result.is_some());
}

#[test]
fn test_query_selector_complex() {
    let doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create: <div><ul><li class="item">Text</li></ul></div>
    let ul = doc.create_element("ul").unwrap();
    let li = doc.create_element("li").unwrap();
    li.set_attribute("class", "item");
    ul.append_child(li);
    root.append_child(ul);

    let result = root.query_selector("div > ul > li.item").unwrap();
    assert!(result.is_some());
}

#[test]
fn test_matches() {
    let element = Element::new("button");
    element.set_attribute("class", "btn primary");

    assert!(element.matches("button").unwrap());
    assert!(element.matches(".btn").unwrap());
    assert!(element.matches("button.primary").unwrap());
    assert!(!element.matches("input").unwrap());
}
```

## Quality Gates
- ✅ All selector types supported (class, ID, tag, attribute, pseudo)
- ✅ Complex selectors work (>, +, ~, combinators)
- ✅ Performance: querySelector <2ms for complex selectors
- ✅ Coverage ≥ 80%

## Success Criteria
1. querySelector/querySelectorAll work
2. matches() and closest() work
3. Complex selectors supported
4. All tests pass

## Estimated Effort
- **LOC**: ~2,500-3,000 (leveraging cssparser/selectors crates)
- **Time**: 8-10 hours
