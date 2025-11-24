//! HTML Sanitization for XSS Prevention
//!
//! Provides configurable sanitization policies to prevent cross-site scripting (XSS) attacks.
//!
//! # Overview
//!
//! Cross-site scripting (XSS) is a security vulnerability that allows attackers to inject
//! malicious scripts into web pages. This module provides tools to sanitize HTML content
//! and prevent XSS attacks through:
//!
//! - **Script tag removal**: Removes `<script>`, `<iframe>`, and other dangerous tags
//! - **Event handler stripping**: Strips `onclick`, `onload`, and other event attributes
//! - **JavaScript URL blocking**: Blocks `javascript:`, `vbscript:`, and `data:text/html` URLs
//!
//! # Quick Start
//!
//! ```rust
//! use browser_dom_impl::sanitization::{SanitizationPolicy, sanitize_html};
//!
//! // Use default policy (recommended)
//! let policy = SanitizationPolicy::default();
//!
//! // Check if content should be filtered
//! assert!(policy.should_remove_tag("script"));
//! assert!(policy.should_strip_attribute("onclick", "alert(1)"));
//!
//! // Sanitize HTML string
//! let dirty_html = "<div onclick=\"alert('xss')\">Hello</div><script>evil()</script>";
//! let clean_html = sanitize_html(dirty_html, &policy);
//! ```
//!
//! # Policy Types
//!
//! | Policy | Description | Use Case |
//! |--------|-------------|----------|
//! | `default()` | Balanced security | General use |
//! | `strict()` | Whitelist mode | User-generated content |
//! | `permissive()` | Minimal filtering | Trusted content |
//!
//! # Security Considerations
//!
//! - Always sanitize user input before rendering
//! - Use `strict()` policy for untrusted content
//! - The `sanitize_html` function is a basic implementation; for production
//!   use, consider a proper HTML parser-based sanitizer

use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Dangerous HTML tags that can execute code or load external resources.
///
/// These tags are blocked by default in the sanitization policy:
/// - `script`: Executes JavaScript
/// - `iframe`: Can load arbitrary content
/// - `object`, `embed`, `applet`: Plugin content
/// - `base`: Changes URL resolution
/// - `link`: Can load external stylesheets/resources
/// - `meta`: Can redirect or set cookies
static DANGEROUS_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("script");
    set.insert("iframe");
    set.insert("object");
    set.insert("embed");
    set.insert("applet");
    set.insert("base");
    set.insert("link"); // Can load external resources
    set.insert("meta"); // Can redirect
    set.insert("svg"); // Can contain inline scripts
    set.insert("math"); // MathML can be exploited
    set
});

/// Event handler attributes (on*) that can execute JavaScript.
///
/// All standard DOM event handlers are included:
/// - Mouse events: onclick, ondblclick, onmouse*
/// - Keyboard events: onkey*
/// - Form events: onfocus, onblur, onchange, etc.
/// - Document events: onload, onerror, etc.
/// - Touch events: ontouch*
static EVENT_HANDLERS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    for handler in &[
        // Mouse events
        "onclick",
        "ondblclick",
        "onmousedown",
        "onmouseup",
        "onmouseover",
        "onmousemove",
        "onmouseout",
        "onmouseenter",
        "onmouseleave",
        // Keyboard events
        "onkeydown",
        "onkeyup",
        "onkeypress",
        // Form events
        "onfocus",
        "onblur",
        "onchange",
        "oninput",
        "onsubmit",
        "onreset",
        "oninvalid",
        "onselect",
        // Document/Window events
        "onload",
        "onerror",
        "onabort",
        "onresize",
        "onscroll",
        "onunload",
        "onbeforeunload",
        "onhashchange",
        "onpopstate",
        // Clipboard events
        "oncopy",
        "oncut",
        "onpaste",
        // Drag events
        "ondrag",
        "ondragstart",
        "ondragend",
        "ondragover",
        "ondragenter",
        "ondragleave",
        "ondrop",
        // Context menu
        "oncontextmenu",
        // Animation events
        "onanimationstart",
        "onanimationend",
        "onanimationiteration",
        "ontransitionend",
        // Touch events
        "ontouchstart",
        "ontouchend",
        "ontouchmove",
        "ontouchcancel",
        // Pointer events
        "onpointerdown",
        "onpointerup",
        "onpointermove",
        "onpointerenter",
        "onpointerleave",
        "onpointercancel",
        // Media events
        "onplay",
        "onpause",
        "onended",
        "onvolumechange",
        "ontimeupdate",
        // Other
        "onwheel",
        "onfocusin",
        "onfocusout",
    ] {
        set.insert(*handler);
    }
    set
});

/// URL schemes that can execute JavaScript or load dangerous content.
const DANGEROUS_URL_SCHEMES: &[&str] = &["javascript:", "data:text/html", "vbscript:"];

/// Attributes that can contain URLs and need validation.
const URL_ATTRIBUTES: &[&str] = &[
    "href",
    "src",
    "action",
    "formaction",
    "data",
    "poster",
    "background",
    "cite",
    "codebase",
    "dynsrc",
    "lowsrc",
];

/// Sanitization policy configuration.
///
/// Controls which HTML elements and attributes are allowed or blocked
/// during sanitization.
///
/// # Examples
///
/// ```rust
/// use browser_dom_impl::sanitization::SanitizationPolicy;
/// use std::collections::HashSet;
///
/// // Default policy - blocks dangerous tags and event handlers
/// let default_policy = SanitizationPolicy::default();
///
/// // Strict policy - whitelist mode, only safe tags allowed
/// let strict_policy = SanitizationPolicy::strict();
///
/// // Custom policy
/// let mut custom_policy = SanitizationPolicy::default();
/// custom_policy.blocked_tags.insert("marquee".to_string());
/// ```
#[derive(Debug, Clone)]
pub struct SanitizationPolicy {
    /// Remove dangerous tags entirely (script, iframe, etc.)
    pub remove_dangerous_tags: bool,
    /// Strip event handler attributes (onclick, onload, etc.)
    pub strip_event_handlers: bool,
    /// Block javascript: and other dangerous URL schemes
    pub block_javascript_urls: bool,
    /// Custom allowed tags (whitelist mode if Some)
    /// When set, only tags in this set are allowed
    pub allowed_tags: Option<HashSet<String>>,
    /// Custom blocked tags (blacklist mode, additive to dangerous tags)
    pub blocked_tags: HashSet<String>,
    /// Custom allowed attributes (whitelist mode if Some)
    /// When set, only attributes in this set are allowed
    pub allowed_attributes: Option<HashSet<String>>,
    /// Allow data: URLs for images (default: false for security)
    pub allow_data_urls_for_images: bool,
}

impl Default for SanitizationPolicy {
    /// Creates a default sanitization policy with balanced security.
    ///
    /// - Removes dangerous tags (script, iframe, etc.)
    /// - Strips event handlers (onclick, etc.)
    /// - Blocks javascript: URLs
    fn default() -> Self {
        Self {
            remove_dangerous_tags: true,
            strip_event_handlers: true,
            block_javascript_urls: true,
            allowed_tags: None,
            blocked_tags: HashSet::new(),
            allowed_attributes: None,
            allow_data_urls_for_images: false,
        }
    }
}

impl SanitizationPolicy {
    /// Create a strict policy using whitelist mode.
    ///
    /// Only allows a predefined set of safe tags:
    /// - Text formatting: p, br, b, i, u, strong, em
    /// - Structure: div, span, blockquote
    /// - Lists: ul, ol, li
    /// - Headings: h1-h6
    /// - Links and images: a, img
    /// - Tables: table, thead, tbody, tr, td, th
    /// - Code: code, pre
    ///
    /// # Example
    ///
    /// ```rust
    /// use browser_dom_impl::sanitization::SanitizationPolicy;
    ///
    /// let policy = SanitizationPolicy::strict();
    /// assert!(!policy.should_remove_tag("p"));
    /// assert!(!policy.should_remove_tag("div"));
    /// assert!(policy.should_remove_tag("form"));  // Not in whitelist
    /// ```
    pub fn strict() -> Self {
        let mut allowed = HashSet::new();
        for tag in &[
            // Text content
            "p",
            "br",
            "b",
            "i",
            "u",
            "strong",
            "em",
            "s",
            "strike",
            "del",
            "ins",
            "sub",
            "sup",
            "small",
            "mark",
            "abbr",
            // Structure
            "div",
            "span",
            "blockquote",
            "hr",
            // Lists
            "ul",
            "ol",
            "li",
            "dl",
            "dt",
            "dd",
            // Headings
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            // Links and images
            "a",
            "img",
            // Tables
            "table",
            "caption",
            "thead",
            "tbody",
            "tfoot",
            "tr",
            "td",
            "th",
            "colgroup",
            "col",
            // Code
            "code",
            "pre",
            "kbd",
            "samp",
            "var",
            // Semantic
            "article",
            "section",
            "aside",
            "header",
            "footer",
            "nav",
            "main",
            "figure",
            "figcaption",
            "time",
            "address",
            "details",
            "summary",
        ] {
            allowed.insert(tag.to_string());
        }
        Self {
            allowed_tags: Some(allowed),
            ..Default::default()
        }
    }

    /// Create a permissive policy with minimal sanitization.
    ///
    /// Still blocks the most dangerous content:
    /// - Script tags
    /// - Event handlers
    /// - JavaScript URLs
    ///
    /// Use this for content from semi-trusted sources.
    pub fn permissive() -> Self {
        Self {
            remove_dangerous_tags: true,
            strip_event_handlers: true,
            block_javascript_urls: true,
            allowed_tags: None,
            blocked_tags: HashSet::new(),
            allowed_attributes: None,
            allow_data_urls_for_images: false,
        }
    }

    /// Create a policy that allows no HTML at all.
    ///
    /// Useful for plain text contexts.
    pub fn no_html() -> Self {
        Self {
            remove_dangerous_tags: true,
            strip_event_handlers: true,
            block_javascript_urls: true,
            allowed_tags: Some(HashSet::new()), // Empty whitelist = no tags allowed
            blocked_tags: HashSet::new(),
            allowed_attributes: Some(HashSet::new()), // No attributes allowed
            allow_data_urls_for_images: false,
        }
    }

    /// Check if a tag should be removed based on the policy.
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The HTML tag name (case-insensitive)
    ///
    /// # Returns
    ///
    /// `true` if the tag should be removed, `false` if allowed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use browser_dom_impl::sanitization::SanitizationPolicy;
    ///
    /// let policy = SanitizationPolicy::default();
    /// assert!(policy.should_remove_tag("script"));
    /// assert!(policy.should_remove_tag("SCRIPT"));
    /// assert!(!policy.should_remove_tag("div"));
    /// ```
    pub fn should_remove_tag(&self, tag_name: &str) -> bool {
        let tag_lower = tag_name.to_lowercase();

        // Whitelist mode - if set, only tags in whitelist are allowed
        if let Some(ref allowed) = self.allowed_tags {
            return !allowed.contains(&tag_lower);
        }

        // Blacklist mode - check custom blocked tags
        if self.blocked_tags.contains(&tag_lower) {
            return true;
        }

        // Check dangerous tags
        if self.remove_dangerous_tags && DANGEROUS_TAGS.contains(tag_lower.as_str()) {
            return true;
        }

        false
    }

    /// Check if an attribute should be stripped based on the policy.
    ///
    /// # Arguments
    ///
    /// * `attr_name` - The attribute name (case-insensitive)
    /// * `attr_value` - The attribute value (for URL checking)
    ///
    /// # Returns
    ///
    /// `true` if the attribute should be stripped, `false` if allowed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use browser_dom_impl::sanitization::SanitizationPolicy;
    ///
    /// let policy = SanitizationPolicy::default();
    /// assert!(policy.should_strip_attribute("onclick", "alert(1)"));
    /// assert!(policy.should_strip_attribute("href", "javascript:void(0)"));
    /// assert!(!policy.should_strip_attribute("class", "my-class"));
    /// ```
    pub fn should_strip_attribute(&self, attr_name: &str, attr_value: &str) -> bool {
        let attr_lower = attr_name.to_lowercase();

        // Check event handlers
        if self.strip_event_handlers && EVENT_HANDLERS.contains(attr_lower.as_str()) {
            return true;
        }

        // Check for on* attributes that might not be in our list
        if self.strip_event_handlers && attr_lower.starts_with("on") {
            return true;
        }

        // Check javascript URLs in URL attributes
        if self.block_javascript_urls && URL_ATTRIBUTES.contains(&attr_lower.as_str()) {
            if is_dangerous_url(attr_value) {
                // Special case: allow data: URLs for images if configured
                if self.allow_data_urls_for_images
                    && attr_lower == "src"
                    && is_safe_data_url_for_image(attr_value)
                {
                    return false;
                }
                return true;
            }
        }

        // Check attribute whitelist
        if let Some(ref allowed) = self.allowed_attributes {
            return !allowed.contains(&attr_lower);
        }

        false
    }

    /// Check if a tag is explicitly allowed in whitelist mode.
    ///
    /// Returns `None` if not in whitelist mode.
    pub fn is_tag_allowed(&self, tag_name: &str) -> Option<bool> {
        self.allowed_tags
            .as_ref()
            .map(|allowed| allowed.contains(&tag_name.to_lowercase()))
    }

    /// Add a tag to the custom blocked list.
    pub fn block_tag(&mut self, tag: impl Into<String>) {
        self.blocked_tags.insert(tag.into().to_lowercase());
    }

    /// Add multiple tags to the custom blocked list.
    pub fn block_tags<I, S>(&mut self, tags: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for tag in tags {
            self.blocked_tags.insert(tag.into().to_lowercase());
        }
    }
}

/// Check if a URL uses a dangerous scheme.
///
/// Detects:
/// - `javascript:` URLs
/// - `vbscript:` URLs
/// - `data:text/html` URLs (can contain scripts)
///
/// The check is case-insensitive and handles:
/// - Leading/trailing whitespace
/// - Control characters (used to bypass filters)
/// - Mixed case schemes
///
/// # Example
///
/// ```rust
/// use browser_dom_impl::sanitization::is_dangerous_url;
///
/// assert!(is_dangerous_url("javascript:alert(1)"));
/// assert!(is_dangerous_url("JAVASCRIPT:alert(1)"));
/// assert!(is_dangerous_url("  javascript:alert(1)")); // whitespace
/// assert!(is_dangerous_url("data:text/html,<script>"));
/// assert!(!is_dangerous_url("https://example.com"));
/// assert!(!is_dangerous_url("/relative/path"));
/// ```
pub fn is_dangerous_url(url: &str) -> bool {
    // Normalize: trim, lowercase, remove control chars and whitespace
    let normalized = url.trim().to_lowercase();

    // Remove whitespace and control characters that can bypass filters
    // Attackers use these to evade pattern matching: "java script:" or "java\x00script:"
    let cleaned: String = normalized
        .chars()
        .filter(|c| !c.is_control() && !c.is_whitespace())
        .collect();

    for scheme in DANGEROUS_URL_SCHEMES {
        if cleaned.starts_with(scheme) {
            return true;
        }
    }

    // Additional check for encoded javascript
    // Check for &#x6A;avascript: or similar encodings
    if cleaned.contains("&#") && cleaned.contains("script:") {
        return true;
    }

    false
}

/// Check if a data: URL is safe for use in image src.
///
/// Only allows specific image MIME types.
fn is_safe_data_url_for_image(url: &str) -> bool {
    let normalized = url.trim().to_lowercase();
    let safe_prefixes = [
        "data:image/png",
        "data:image/jpeg",
        "data:image/gif",
        "data:image/webp",
        "data:image/svg+xml", // Note: SVG can contain scripts, use with caution
    ];

    for prefix in safe_prefixes {
        if normalized.starts_with(prefix) {
            // Additional check: SVG can contain scripts
            if prefix.contains("svg") && normalized.contains("<script") {
                return false;
            }
            return true;
        }
    }
    false
}

/// Result of HTML sanitization.
#[derive(Debug, Clone)]
pub struct SanitizationResult {
    /// The sanitized HTML string
    pub html: String,
    /// Number of tags removed
    pub tags_removed: usize,
    /// Number of attributes stripped
    pub attributes_stripped: usize,
    /// List of removed tag names (for logging/debugging)
    pub removed_tags: Vec<String>,
}

/// Sanitize an HTML string according to the given policy.
///
/// **Note**: This is a simplified regex-based implementation suitable for
/// basic use cases. For production use with untrusted input, use a proper
/// HTML parser-based sanitizer.
///
/// # Arguments
///
/// * `html` - The HTML string to sanitize
/// * `policy` - The sanitization policy to apply
///
/// # Returns
///
/// The sanitized HTML string.
///
/// # Example
///
/// ```rust
/// use browser_dom_impl::sanitization::{SanitizationPolicy, sanitize_html};
///
/// let policy = SanitizationPolicy::default();
/// let dirty = "<div onclick=\"alert('xss')\">Hello</div><script>evil()</script>";
/// let clean = sanitize_html(dirty, &policy);
/// assert!(!clean.contains("<script"));
/// ```
pub fn sanitize_html(html: &str, policy: &SanitizationPolicy) -> String {
    let mut result = html.to_string();

    // Remove dangerous tags (both opening and closing)
    if policy.remove_dangerous_tags {
        for tag in DANGEROUS_TAGS.iter() {
            result = remove_tag(&result, tag);
        }
    }

    // Remove custom blocked tags
    for tag in &policy.blocked_tags {
        result = remove_tag(&result, tag);
    }

    // If whitelist mode, remove non-whitelisted tags
    if let Some(ref allowed) = policy.allowed_tags {
        result = remove_non_whitelisted_tags(&result, allowed);
    }

    // Strip event handlers and dangerous URLs from remaining tags
    if policy.strip_event_handlers || policy.block_javascript_urls {
        result = strip_dangerous_attributes(&result, policy);
    }

    result
}

/// Sanitize HTML and return detailed results.
///
/// Like `sanitize_html` but also returns statistics about what was removed.
pub fn sanitize_html_with_stats(html: &str, policy: &SanitizationPolicy) -> SanitizationResult {
    let mut result = html.to_string();
    let mut tags_removed = 0;
    let mut attributes_stripped = 0;
    let mut removed_tags = Vec::new();

    // Remove dangerous tags
    if policy.remove_dangerous_tags {
        for tag in DANGEROUS_TAGS.iter() {
            let (new_result, count) = remove_tag_with_count(&result, tag);
            if count > 0 {
                removed_tags.push(tag.to_string());
                tags_removed += count;
            }
            result = new_result;
        }
    }

    // Remove custom blocked tags
    for tag in &policy.blocked_tags {
        let (new_result, count) = remove_tag_with_count(&result, tag);
        if count > 0 {
            removed_tags.push(tag.clone());
            tags_removed += count;
        }
        result = new_result;
    }

    // Strip dangerous attributes
    if policy.strip_event_handlers || policy.block_javascript_urls {
        let (new_result, attr_count) = strip_dangerous_attributes_with_count(&result, policy);
        result = new_result;
        attributes_stripped = attr_count;
    }

    SanitizationResult {
        html: result,
        tags_removed,
        attributes_stripped,
        removed_tags,
    }
}

/// Remove a specific tag (opening and closing) from HTML.
fn remove_tag(html: &str, tag: &str) -> String {
    let mut result = html.to_string();

    // Remove opening tags: <script>, <script attr="value">, <script/>, etc.
    let open_pattern = format!("<{}", tag);
    while let Some(start) = result.to_lowercase().find(&open_pattern) {
        // Make sure it's actually a tag start (not just text containing the pattern)
        if start > 0 {
            let prev_char = result.chars().nth(start - 1);
            if prev_char.map(|c| c.is_alphanumeric()).unwrap_or(false) {
                // Part of another word, skip
                break;
            }
        }

        // Find the end of the opening tag
        if let Some(end_offset) = result[start..].find('>') {
            let end = start + end_offset + 1;

            // Check if this is a self-closing tag
            let tag_content = &result[start..end];
            let is_self_closing = tag_content.ends_with("/>");

            if is_self_closing {
                // Just remove the self-closing tag
                result = format!("{}{}", &result[..start], &result[end..]);
            } else {
                // Look for closing tag
                let close_pattern = format!("</{}>", tag);
                if let Some(close_start) = result.to_lowercase()[end..].find(&close_pattern) {
                    let close_start = end + close_start;
                    let close_end = close_start + close_pattern.len();
                    // Remove everything from start to close_end
                    result = format!("{}{}", &result[..start], &result[close_end..]);
                } else {
                    // No closing tag, just remove opening tag
                    result = format!("{}{}", &result[..start], &result[end..]);
                }
            }
        } else {
            break;
        }
    }

    // Remove any orphaned closing tags
    let close_pattern = format!("</{}>", tag);
    result = result.replace(&close_pattern, "");

    result
}

/// Remove a tag and return the count of removals.
fn remove_tag_with_count(html: &str, tag: &str) -> (String, usize) {
    let original_len = html.len();
    let result = remove_tag(html, tag);
    let removed_chars = original_len.saturating_sub(result.len());
    // Rough estimate: each tag removal removes at least the tag name + brackets
    let estimated_count = if removed_chars > 0 {
        removed_chars / (tag.len() + 3).max(1)
    } else {
        0
    };
    (result, estimated_count.max(if removed_chars > 0 { 1 } else { 0 }))
}

/// Remove tags not in the whitelist (simplified implementation).
fn remove_non_whitelisted_tags(html: &str, allowed: &HashSet<String>) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    let mut in_tag = false;
    let mut current_tag = String::new();

    while let Some(c) = chars.next() {
        if c == '<' {
            in_tag = true;
            current_tag.clear();
            current_tag.push(c);
        } else if in_tag {
            current_tag.push(c);
            if c == '>' {
                in_tag = false;
                // Extract tag name
                let tag_name = extract_tag_name(&current_tag);
                if tag_name.is_empty() || allowed.contains(&tag_name.to_lowercase()) {
                    result.push_str(&current_tag);
                }
                // else: skip this tag (don't add to result)
            }
        } else {
            result.push(c);
        }
    }

    // If we ended mid-tag, decide what to do
    if in_tag {
        // Don't add incomplete tag
    }

    result
}

/// Extract tag name from a tag string like "<div>" or "<div class='x'>".
fn extract_tag_name(tag: &str) -> String {
    let tag = tag.trim_start_matches('<');
    let tag = tag.trim_start_matches('/');
    let tag = tag.trim_end_matches('>');
    let tag = tag.trim_end_matches('/');

    // Get the tag name (first word)
    tag.split(|c: char| c.is_whitespace() || c == '/' || c == '>')
        .next()
        .unwrap_or("")
        .to_string()
}

/// Strip dangerous attributes from HTML tags.
fn strip_dangerous_attributes(html: &str, policy: &SanitizationPolicy) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    let mut in_tag = false;
    let mut current_tag = String::new();

    while let Some(c) = chars.next() {
        if c == '<' && !in_tag {
            in_tag = true;
            current_tag.clear();
            current_tag.push(c);
        } else if in_tag {
            current_tag.push(c);
            if c == '>' {
                in_tag = false;
                // Process the tag to remove dangerous attributes
                let cleaned_tag = clean_tag_attributes(&current_tag, policy);
                result.push_str(&cleaned_tag);
            }
        } else {
            result.push(c);
        }
    }

    // If we ended mid-tag, include it as-is
    if in_tag {
        result.push_str(&current_tag);
    }

    result
}

/// Strip dangerous attributes and return count.
fn strip_dangerous_attributes_with_count(html: &str, policy: &SanitizationPolicy) -> (String, usize) {
    let original_len = html.len();
    let result = strip_dangerous_attributes(html, policy);
    let diff = original_len.saturating_sub(result.len());
    // Rough estimate of attributes removed
    let count = if diff > 10 { diff / 15 } else { 0 };
    (result, count.max(if diff > 0 { 1 } else { 0 }))
}

/// Clean a single tag's attributes according to policy.
fn clean_tag_attributes(tag: &str, policy: &SanitizationPolicy) -> String {
    // Quick check: if no attributes, return as-is
    if !tag.contains(' ') && !tag.contains('=') {
        return tag.to_string();
    }

    // Check if it's a closing tag (no attributes to clean)
    if tag.starts_with("</") {
        return tag.to_string();
    }

    let mut result = String::new();
    let tag = tag.trim_start_matches('<').trim_end_matches('>');
    let is_self_closing = tag.ends_with('/');
    let tag = tag.trim_end_matches('/');

    // Split into tag name and attributes
    let mut parts = tag.splitn(2, |c: char| c.is_whitespace());
    let tag_name = parts.next().unwrap_or("");
    let attrs_str = parts.next().unwrap_or("");

    result.push('<');
    result.push_str(tag_name);

    // Parse and filter attributes
    if !attrs_str.is_empty() {
        let clean_attrs = filter_attributes(attrs_str, policy);
        if !clean_attrs.is_empty() {
            result.push(' ');
            result.push_str(&clean_attrs);
        }
    }

    if is_self_closing {
        result.push_str(" /");
    }
    result.push('>');

    result
}

/// Filter attributes according to policy.
fn filter_attributes(attrs_str: &str, policy: &SanitizationPolicy) -> String {
    let mut result = Vec::new();
    let mut remaining = attrs_str.trim();

    while !remaining.is_empty() {
        // Try to parse an attribute
        if let Some((attr_name, attr_value, rest)) = parse_next_attribute(remaining) {
            if !policy.should_strip_attribute(&attr_name, &attr_value) {
                // Keep this attribute
                if attr_value.is_empty() {
                    result.push(attr_name);
                } else {
                    let quote = if attr_value.contains('"') { '\'' } else { '"' };
                    result.push(format!("{}={}{}{}", attr_name, quote, attr_value, quote));
                }
            }
            remaining = rest.trim();
        } else {
            // Can't parse, skip the rest
            break;
        }
    }

    result.join(" ")
}

/// Parse the next attribute from a string, returning (name, value, rest).
fn parse_next_attribute(s: &str) -> Option<(String, String, &str)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Find attribute name (up to = or whitespace)
    let mut name_end = 0;
    for (i, c) in s.char_indices() {
        if c == '=' || c.is_whitespace() {
            name_end = i;
            break;
        }
        name_end = i + c.len_utf8();
    }

    if name_end == 0 {
        return None;
    }

    let name = s[..name_end].to_string();
    let rest = s[name_end..].trim_start();

    // Check for value
    if rest.starts_with('=') {
        let value_part = rest[1..].trim_start();

        // Check for quoted value
        if value_part.starts_with('"') {
            if let Some(end) = value_part[1..].find('"') {
                let value = value_part[1..=end].to_string();
                let remaining = &value_part[end + 2..];
                return Some((name, value, remaining));
            }
        } else if value_part.starts_with('\'') {
            if let Some(end) = value_part[1..].find('\'') {
                let value = value_part[1..=end].to_string();
                let remaining = &value_part[end + 2..];
                return Some((name, value, remaining));
            }
        } else {
            // Unquoted value (up to whitespace)
            let end = value_part
                .find(char::is_whitespace)
                .unwrap_or(value_part.len());
            let value = value_part[..end].to_string();
            let remaining = &value_part[end..];
            return Some((name, value, remaining));
        }
    }

    // Boolean attribute (no value)
    Some((name, String::new(), rest))
}

/// Get the list of dangerous tags.
pub fn dangerous_tags() -> impl Iterator<Item = &'static str> {
    DANGEROUS_TAGS.iter().copied()
}

/// Get the list of event handler attributes.
pub fn event_handlers() -> impl Iterator<Item = &'static str> {
    EVENT_HANDLERS.iter().copied()
}

/// Check if a string looks like it might contain HTML.
pub fn might_contain_html(s: &str) -> bool {
    s.contains('<') && s.contains('>')
}

/// Escape HTML special characters.
///
/// Converts:
/// - `&` to `&amp;`
/// - `<` to `&lt;`
/// - `>` to `&gt;`
/// - `"` to `&quot;`
/// - `'` to `&#x27;`
pub fn escape_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#x27;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dangerous_tag_detection() {
        let policy = SanitizationPolicy::default();
        assert!(policy.should_remove_tag("script"));
        assert!(policy.should_remove_tag("SCRIPT"));
        assert!(policy.should_remove_tag("Script"));
        assert!(policy.should_remove_tag("iframe"));
        assert!(policy.should_remove_tag("IFRAME"));
        assert!(policy.should_remove_tag("object"));
        assert!(policy.should_remove_tag("embed"));
        assert!(policy.should_remove_tag("applet"));
        assert!(policy.should_remove_tag("base"));
        assert!(policy.should_remove_tag("link"));
        assert!(policy.should_remove_tag("meta"));
        assert!(!policy.should_remove_tag("div"));
        assert!(!policy.should_remove_tag("span"));
        assert!(!policy.should_remove_tag("p"));
    }

    #[test]
    fn test_event_handler_stripping() {
        let policy = SanitizationPolicy::default();
        assert!(policy.should_strip_attribute("onclick", "alert(1)"));
        assert!(policy.should_strip_attribute("ONCLICK", "alert(1)"));
        assert!(policy.should_strip_attribute("onClick", "alert(1)"));
        assert!(policy.should_strip_attribute("onload", ""));
        assert!(policy.should_strip_attribute("onerror", "malicious()"));
        assert!(policy.should_strip_attribute("onmouseover", "doSomething()"));
        assert!(policy.should_strip_attribute("onfocus", "focus()"));
        assert!(!policy.should_strip_attribute("class", "foo"));
        assert!(!policy.should_strip_attribute("id", "bar"));
        assert!(!policy.should_strip_attribute("style", "color: red"));
    }

    #[test]
    fn test_custom_event_handlers_starting_with_on() {
        let policy = SanitizationPolicy::default();
        // Even unknown on* handlers should be stripped
        assert!(policy.should_strip_attribute("oncustomevent", "handler()"));
        assert!(policy.should_strip_attribute("onwhatever", "handler()"));
    }

    #[test]
    fn test_javascript_url_blocking() {
        assert!(is_dangerous_url("javascript:alert(1)"));
        assert!(is_dangerous_url("JAVASCRIPT:alert(1)"));
        assert!(is_dangerous_url("JavaScript:alert(1)"));
        assert!(is_dangerous_url("  javascript:alert(1)")); // leading whitespace
        assert!(is_dangerous_url("javascript:alert(1)  ")); // trailing whitespace
        assert!(is_dangerous_url("\tjavascript:alert(1)")); // tab
        assert!(is_dangerous_url("data:text/html,<script>alert(1)</script>"));
        assert!(is_dangerous_url("vbscript:msgbox('xss')"));
        assert!(!is_dangerous_url("https://example.com"));
        assert!(!is_dangerous_url("http://example.com"));
        assert!(!is_dangerous_url("/path/to/page"));
        assert!(!is_dangerous_url("./relative/path"));
        assert!(!is_dangerous_url("#anchor"));
    }

    #[test]
    fn test_javascript_url_with_control_chars() {
        // Attackers use control characters to bypass filters
        assert!(is_dangerous_url("java\x00script:alert(1)"));
        assert!(is_dangerous_url("java\nscript:alert(1)"));
        assert!(is_dangerous_url("java\tscript:alert(1)"));
    }

    #[test]
    fn test_strict_policy_whitelist() {
        let policy = SanitizationPolicy::strict();
        assert!(!policy.should_remove_tag("p"));
        assert!(!policy.should_remove_tag("div"));
        assert!(!policy.should_remove_tag("span"));
        assert!(!policy.should_remove_tag("a"));
        assert!(!policy.should_remove_tag("img"));
        assert!(!policy.should_remove_tag("table"));
        assert!(policy.should_remove_tag("form"));   // Not in whitelist
        assert!(policy.should_remove_tag("input"));  // Not in whitelist
        assert!(policy.should_remove_tag("button")); // Not in whitelist
        assert!(policy.should_remove_tag("select")); // Not in whitelist
    }

    #[test]
    fn test_permissive_policy() {
        let policy = SanitizationPolicy::permissive();
        // Still blocks dangerous things
        assert!(policy.should_remove_tag("script"));
        assert!(policy.should_strip_attribute("onclick", "alert(1)"));
        // Allows most other tags
        assert!(!policy.should_remove_tag("form"));
        assert!(!policy.should_remove_tag("input"));
    }

    #[test]
    fn test_no_html_policy() {
        let policy = SanitizationPolicy::no_html();
        assert!(policy.should_remove_tag("div"));
        assert!(policy.should_remove_tag("span"));
        assert!(policy.should_remove_tag("p"));
        assert!(policy.should_strip_attribute("class", "foo"));
    }

    #[test]
    fn test_custom_blocked_tags() {
        let mut policy = SanitizationPolicy::default();
        policy.block_tag("marquee");
        policy.block_tag("blink");
        assert!(policy.should_remove_tag("marquee"));
        assert!(policy.should_remove_tag("blink"));
        assert!(!policy.should_remove_tag("div"));
    }

    #[test]
    fn test_url_attribute_checking() {
        let policy = SanitizationPolicy::default();

        // href with javascript
        assert!(policy.should_strip_attribute("href", "javascript:alert(1)"));

        // src with javascript
        assert!(policy.should_strip_attribute("src", "javascript:alert(1)"));

        // action with javascript
        assert!(policy.should_strip_attribute("action", "javascript:alert(1)"));

        // Safe URLs
        assert!(!policy.should_strip_attribute("href", "https://example.com"));
        assert!(!policy.should_strip_attribute("src", "/images/logo.png"));
    }

    #[test]
    fn test_sanitize_html_removes_script() {
        let policy = SanitizationPolicy::default();
        let html = "<div>Hello</div><script>alert('xss')</script><p>World</p>";
        let sanitized = sanitize_html(html, &policy);
        assert!(!sanitized.to_lowercase().contains("<script"));
        assert!(!sanitized.to_lowercase().contains("</script>"));
        assert!(sanitized.contains("<div>Hello</div>"));
        assert!(sanitized.contains("<p>World</p>"));
    }

    #[test]
    fn test_sanitize_html_removes_iframe() {
        let policy = SanitizationPolicy::default();
        let html = "<iframe src=\"https://evil.com\"></iframe><p>Safe content</p>";
        let sanitized = sanitize_html(html, &policy);
        assert!(!sanitized.to_lowercase().contains("<iframe"));
        assert!(sanitized.contains("<p>Safe content</p>"));
    }

    #[test]
    fn test_sanitize_html_strips_event_handlers() {
        let policy = SanitizationPolicy::default();
        let html = "<div onclick=\"alert('xss')\" class=\"container\">Content</div>";
        let sanitized = sanitize_html(html, &policy);
        assert!(!sanitized.to_lowercase().contains("onclick"));
        assert!(sanitized.contains("class"));
        assert!(sanitized.contains("Content"));
    }

    #[test]
    fn test_sanitize_html_with_stats() {
        let policy = SanitizationPolicy::default();
        let html = "<script>evil()</script><div onclick=\"bad()\">Safe</div>";
        let result = sanitize_html_with_stats(html, &policy);
        assert!(!result.html.to_lowercase().contains("<script"));
        assert!(result.tags_removed > 0);
        assert!(result.removed_tags.contains(&"script".to_string()));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<div>"), "&lt;div&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(escape_html("it's"), "it&#x27;s");
        assert_eq!(escape_html("<script>alert('xss')</script>"),
                   "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
    }

    #[test]
    fn test_might_contain_html() {
        assert!(might_contain_html("<div>"));
        assert!(might_contain_html("<script>"));
        assert!(might_contain_html("Some <b>bold</b> text"));
        assert!(!might_contain_html("Plain text"));
        assert!(!might_contain_html("3 < 5"));
        assert!(!might_contain_html("x > y"));
    }

    #[test]
    fn test_extract_tag_name() {
        assert_eq!(extract_tag_name("<div>"), "div");
        assert_eq!(extract_tag_name("<div class='x'>"), "div");
        assert_eq!(extract_tag_name("</div>"), "div");
        assert_eq!(extract_tag_name("<br/>"), "br");
        assert_eq!(extract_tag_name("<br />"), "br");
        assert_eq!(extract_tag_name("<img src='x' />"), "img");
    }

    #[test]
    fn test_is_tag_allowed_whitelist_mode() {
        let policy = SanitizationPolicy::strict();
        assert_eq!(policy.is_tag_allowed("p"), Some(true));
        assert_eq!(policy.is_tag_allowed("form"), Some(false));

        let default_policy = SanitizationPolicy::default();
        assert_eq!(default_policy.is_tag_allowed("anything"), None);
    }

    #[test]
    fn test_dangerous_tags_iterator() {
        let tags: Vec<_> = dangerous_tags().collect();
        assert!(tags.contains(&"script"));
        assert!(tags.contains(&"iframe"));
    }

    #[test]
    fn test_event_handlers_iterator() {
        let handlers: Vec<_> = event_handlers().collect();
        assert!(handlers.contains(&"onclick"));
        assert!(handlers.contains(&"onload"));
    }

    #[test]
    fn test_complex_xss_patterns() {
        let policy = SanitizationPolicy::default();

        // SVG-based XSS
        assert!(policy.should_remove_tag("svg"));

        // Image with error handler
        let html = "<img src='x' onerror='alert(1)'>";
        let sanitized = sanitize_html(html, &policy);
        assert!(!sanitized.contains("onerror"));

        // Link with javascript
        let html2 = "<a href='javascript:alert(1)'>Click</a>";
        let sanitized2 = sanitize_html(html2, &policy);
        assert!(!sanitized2.contains("javascript:"));
    }

    #[test]
    fn test_safe_data_url_for_image() {
        assert!(is_safe_data_url_for_image("data:image/png;base64,ABC123"));
        assert!(is_safe_data_url_for_image("data:image/jpeg;base64,XYZ"));
        assert!(is_safe_data_url_for_image("data:image/gif;base64,ABC"));
        assert!(!is_safe_data_url_for_image("data:text/html,<script>"));
        assert!(!is_safe_data_url_for_image("data:application/javascript,alert(1)"));
    }

    #[test]
    fn test_parse_attributes() {
        let (name, value, rest) = parse_next_attribute("class=\"foo\" id=\"bar\"").unwrap();
        assert_eq!(name, "class");
        assert_eq!(value, "foo");
        assert!(rest.contains("id"));

        let (name2, value2, _) = parse_next_attribute("disabled").unwrap();
        assert_eq!(name2, "disabled");
        assert_eq!(value2, "");
    }
}
