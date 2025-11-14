use dom_types::DomException;
use std::error::Error;

#[test]
fn test_hierarchy_request_error_display() {
    let err = DomException::HierarchyRequestError;
    assert_eq!(err.to_string(), "Hierarchy request error");
}

#[test]
fn test_wrong_document_error_display() {
    let err = DomException::WrongDocumentError;
    assert_eq!(err.to_string(), "Wrong document error");
}

#[test]
fn test_invalid_character_error_display() {
    let err = DomException::InvalidCharacterError;
    assert_eq!(err.to_string(), "Invalid character error");
}

#[test]
fn test_no_modification_allowed_error_display() {
    let err = DomException::NoModificationAllowedError;
    assert_eq!(err.to_string(), "No modification allowed error");
}

#[test]
fn test_not_found_error_display() {
    let err = DomException::NotFoundError;
    assert_eq!(err.to_string(), "Not found error");
}

#[test]
fn test_not_supported_error_display() {
    let err = DomException::NotSupportedError;
    assert_eq!(err.to_string(), "Not supported error");
}

#[test]
fn test_invalid_state_error_display() {
    let err = DomException::InvalidStateError;
    assert_eq!(err.to_string(), "Invalid state error");
}

#[test]
fn test_syntax_error_display() {
    let err = DomException::SyntaxError("invalid selector".to_string());
    assert!(err.to_string().contains("Syntax error"));
    assert!(err.to_string().contains("invalid selector"));
}

#[test]
fn test_invalid_modification_error_display() {
    let err = DomException::InvalidModificationError;
    assert_eq!(err.to_string(), "Invalid modification error");
}

#[test]
fn test_namespace_error_display() {
    let err = DomException::NamespaceError;
    assert_eq!(err.to_string(), "Namespace error");
}

#[test]
fn test_security_error_display() {
    let err = DomException::SecurityError;
    assert_eq!(err.to_string(), "Security error");
}

#[test]
fn test_dom_exception_is_error() {
    let err = DomException::NotFoundError;
    let _: &dyn Error = &err; // Should compile if Error trait is implemented
}

#[test]
fn test_dom_exception_debug() {
    let err = DomException::HierarchyRequestError;
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("HierarchyRequestError"));
}

#[test]
fn test_dom_exception_clone() {
    let err = DomException::NotFoundError;
    let cloned = err.clone();
    assert_eq!(err, cloned);
}

#[test]
fn test_dom_exception_equality() {
    assert_eq!(DomException::NotFoundError, DomException::NotFoundError);
    assert_ne!(DomException::NotFoundError, DomException::SecurityError);

    let err1 = DomException::SyntaxError("test".to_string());
    let err2 = DomException::SyntaxError("test".to_string());
    let err3 = DomException::SyntaxError("other".to_string());

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

#[test]
fn test_syntax_error_with_empty_string() {
    let err = DomException::SyntaxError("".to_string());
    assert!(err.to_string().contains("Syntax error"));
}

#[test]
fn test_syntax_error_with_long_string() {
    let long_msg = "a".repeat(1000);
    let err = DomException::SyntaxError(long_msg.clone());
    assert!(err.to_string().contains(&long_msg));
}

#[test]
fn test_all_exception_types_are_unique() {
    let errors = vec![
        DomException::HierarchyRequestError,
        DomException::WrongDocumentError,
        DomException::InvalidCharacterError,
        DomException::NoModificationAllowedError,
        DomException::NotFoundError,
        DomException::NotSupportedError,
        DomException::InvalidStateError,
        DomException::InvalidModificationError,
        DomException::NamespaceError,
        DomException::SecurityError,
    ];

    // Each error should have a unique display string
    let displays: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
    for (i, display) in displays.iter().enumerate() {
        for (j, other) in displays.iter().enumerate() {
            if i != j {
                assert_ne!(
                    display, other,
                    "Errors at index {} and {} have same display",
                    i, j
                );
            }
        }
    }
}

#[test]
fn test_dom_exception_source() {
    let err = DomException::NotFoundError;
    assert!(err.source().is_none()); // DomException doesn't have a source
}
