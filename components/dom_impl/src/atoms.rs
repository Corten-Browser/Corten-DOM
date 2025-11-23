//! Pre-interned string atoms for common DOM strings
//!
//! This module provides an atom interning system for frequently used DOM strings.
//! By using atoms instead of strings, we can achieve:
//!
//! - **O(1) comparisons**: Compare atoms by their numeric ID instead of string content
//! - **Reduced memory**: Common strings are stored once and referenced by ID
//! - **Cache efficiency**: Small numeric IDs are more cache-friendly than string pointers
//!
//! # Usage
//!
//! ```rust
//! use browser_dom_impl::atoms::{Atom, atoms};
//!
//! // Use predefined atoms for common strings
//! let div_atom = atoms::DIV;
//! let span_atom = atoms::SPAN;
//!
//! // Fast comparison
//! assert_ne!(div_atom, span_atom);
//!
//! // Look up atom from string
//! if let Some(atom) = Atom::from_str("div") {
//!     assert_eq!(atom, atoms::DIV);
//! }
//!
//! // Get string representation
//! assert_eq!(atoms::DIV.as_str(), Some("div"));
//! ```

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;

/// An interned atom representing a common DOM string.
///
/// Atoms are small integer IDs that map to frequently used strings in the DOM.
/// They enable fast equality comparisons (single integer compare vs string compare)
/// and reduced memory usage for common strings.
///
/// # Categories
///
/// Atoms are organized into ranges:
/// - **1-99**: HTML element tag names
/// - **100-199**: HTML attributes
/// - **200-299**: Event types
/// - **300-399**: CSS property names (common ones)
/// - **400+**: Reserved for dynamic atoms
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atom(u32);

impl Atom {
    /// Create an atom from a raw ID
    ///
    /// This is primarily for internal use. Prefer using predefined atoms
    /// from the `atoms` module or `Atom::from_str()`.
    #[inline]
    pub const fn from_raw(id: u32) -> Self {
        Atom(id)
    }

    /// Get the raw ID of this atom
    #[inline]
    pub const fn raw(&self) -> u32 {
        self.0
    }

    /// Get atom for a string, returning None if not a known atom
    ///
    /// This performs a hash lookup to find the corresponding atom.
    /// For strings that aren't in the atom table, returns None.
    ///
    /// # Example
    ///
    /// ```rust
    /// use browser_dom_impl::atoms::{Atom, atoms};
    ///
    /// assert_eq!(Atom::from_str("div"), Some(atoms::DIV));
    /// assert_eq!(Atom::from_str("unknown-element"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Atom> {
        ATOM_MAP.get(s).copied()
    }

    /// Get string representation of atom
    ///
    /// Returns None if this is not a known static atom.
    ///
    /// # Example
    ///
    /// ```rust
    /// use browser_dom_impl::atoms::atoms;
    ///
    /// assert_eq!(atoms::DIV.as_str(), Some("div"));
    /// ```
    pub fn as_str(&self) -> Option<&'static str> {
        REVERSE_MAP.get(&self.0).copied()
    }

    /// Check if this atom represents an HTML element tag name
    #[inline]
    pub fn is_element(&self) -> bool {
        self.0 >= 1 && self.0 < 100
    }

    /// Check if this atom represents an HTML attribute name
    #[inline]
    pub fn is_attribute(&self) -> bool {
        self.0 >= 100 && self.0 < 200
    }

    /// Check if this atom represents an event type
    #[inline]
    pub fn is_event(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    /// Check if this atom represents a CSS property name
    #[inline]
    pub fn is_css_property(&self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    /// Check if this is a void element (self-closing)
    pub fn is_void_element(&self) -> bool {
        matches!(
            self.0,
            5   // img
            | 6 // input
            | 22 // br
            | 23 // hr
            | 24 // meta
            | 25 // link
            | 26 // area
            | 27 // base
            | 28 // col
            | 29 // embed
            | 30 // param
            | 31 // source
            | 32 // track
            | 33 // wbr
        )
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "Atom({}: {:?})", self.0, s)
        } else {
            write!(f, "Atom({})", self.0)
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{}", s)
        } else {
            write!(f, "[atom:{}]", self.0)
        }
    }
}

/// Common DOM string atoms
///
/// This module contains predefined atoms for frequently used DOM strings.
/// Using these atoms instead of string comparisons improves performance.
pub mod atoms {
    use super::Atom;

    // ==========================================================================
    // HTML Element Tag Names (1-99)
    // ==========================================================================

    /// `<div>` element
    pub const DIV: Atom = Atom(1);
    /// `<span>` element
    pub const SPAN: Atom = Atom(2);
    /// `<p>` paragraph element
    pub const P: Atom = Atom(3);
    /// `<a>` anchor element
    pub const A: Atom = Atom(4);
    /// `<img>` image element
    pub const IMG: Atom = Atom(5);
    /// `<input>` form input element
    pub const INPUT: Atom = Atom(6);
    /// `<button>` button element
    pub const BUTTON: Atom = Atom(7);
    /// `<form>` form element
    pub const FORM: Atom = Atom(8);
    /// `<table>` table element
    pub const TABLE: Atom = Atom(9);
    /// `<tr>` table row element
    pub const TR: Atom = Atom(10);
    /// `<td>` table data cell element
    pub const TD: Atom = Atom(11);
    /// `<th>` table header cell element
    pub const TH: Atom = Atom(12);
    /// `<ul>` unordered list element
    pub const UL: Atom = Atom(13);
    /// `<ol>` ordered list element
    pub const OL: Atom = Atom(14);
    /// `<li>` list item element
    pub const LI: Atom = Atom(15);
    /// `<h1>` heading level 1 element
    pub const H1: Atom = Atom(16);
    /// `<h2>` heading level 2 element
    pub const H2: Atom = Atom(17);
    /// `<h3>` heading level 3 element
    pub const H3: Atom = Atom(18);
    /// `<body>` document body element
    pub const BODY: Atom = Atom(19);
    /// `<head>` document head element
    pub const HEAD: Atom = Atom(20);
    /// `<html>` root HTML element
    pub const HTML: Atom = Atom(21);
    /// `<br>` line break element
    pub const BR: Atom = Atom(22);
    /// `<hr>` horizontal rule element
    pub const HR: Atom = Atom(23);
    /// `<meta>` metadata element
    pub const META: Atom = Atom(24);
    /// `<link>` link element
    pub const LINK: Atom = Atom(25);
    /// `<area>` image map area element
    pub const AREA: Atom = Atom(26);
    /// `<base>` base URL element
    pub const BASE: Atom = Atom(27);
    /// `<col>` table column element
    pub const COL: Atom = Atom(28);
    /// `<embed>` embed element
    pub const EMBED: Atom = Atom(29);
    /// `<param>` object parameter element
    pub const PARAM: Atom = Atom(30);
    /// `<source>` media source element
    pub const SOURCE: Atom = Atom(31);
    /// `<track>` text track element
    pub const TRACK: Atom = Atom(32);
    /// `<wbr>` word break opportunity element
    pub const WBR: Atom = Atom(33);
    /// `<h4>` heading level 4 element
    pub const H4: Atom = Atom(34);
    /// `<h5>` heading level 5 element
    pub const H5: Atom = Atom(35);
    /// `<h6>` heading level 6 element
    pub const H6: Atom = Atom(36);
    /// `<header>` header element
    pub const HEADER: Atom = Atom(37);
    /// `<footer>` footer element
    pub const FOOTER: Atom = Atom(38);
    /// `<nav>` navigation element
    pub const NAV: Atom = Atom(39);
    /// `<main>` main content element
    pub const MAIN: Atom = Atom(40);
    /// `<article>` article element
    pub const ARTICLE: Atom = Atom(41);
    /// `<section>` section element
    pub const SECTION: Atom = Atom(42);
    /// `<aside>` aside element
    pub const ASIDE: Atom = Atom(43);
    /// `<script>` script element
    pub const SCRIPT: Atom = Atom(44);
    /// `<style>` style element
    pub const STYLE_ELEM: Atom = Atom(45);
    /// `<title>` title element
    pub const TITLE: Atom = Atom(46);
    /// `<textarea>` text area element
    pub const TEXTAREA: Atom = Atom(47);
    /// `<select>` select element
    pub const SELECT: Atom = Atom(48);
    /// `<option>` option element
    pub const OPTION: Atom = Atom(49);
    /// `<label>` label element
    pub const LABEL: Atom = Atom(50);
    /// `<iframe>` inline frame element
    pub const IFRAME: Atom = Atom(51);
    /// `<canvas>` canvas element
    pub const CANVAS: Atom = Atom(52);
    /// `<video>` video element
    pub const VIDEO: Atom = Atom(53);
    /// `<audio>` audio element
    pub const AUDIO: Atom = Atom(54);
    /// `<pre>` preformatted text element
    pub const PRE: Atom = Atom(55);
    /// `<code>` code element
    pub const CODE: Atom = Atom(56);
    /// `<strong>` strong importance element
    pub const STRONG: Atom = Atom(57);
    /// `<em>` emphasis element
    pub const EM: Atom = Atom(58);
    /// `<i>` idiomatic text element
    pub const I: Atom = Atom(59);
    /// `<b>` bring attention element
    pub const B: Atom = Atom(60);
    /// `<u>` unarticulated annotation element
    pub const U: Atom = Atom(61);
    /// `<small>` small print element
    pub const SMALL: Atom = Atom(62);
    /// `<sup>` superscript element
    pub const SUP: Atom = Atom(63);
    /// `<sub>` subscript element
    pub const SUB: Atom = Atom(64);
    /// `<blockquote>` block quotation element
    pub const BLOCKQUOTE: Atom = Atom(65);
    /// `<dl>` description list element
    pub const DL: Atom = Atom(66);
    /// `<dt>` description term element
    pub const DT: Atom = Atom(67);
    /// `<dd>` description details element
    pub const DD: Atom = Atom(68);
    /// `<figure>` figure element
    pub const FIGURE: Atom = Atom(69);
    /// `<figcaption>` figure caption element
    pub const FIGCAPTION: Atom = Atom(70);
    /// `<picture>` picture element
    pub const PICTURE: Atom = Atom(71);
    /// `<svg>` SVG element
    pub const SVG: Atom = Atom(72);
    /// `<math>` MathML element
    pub const MATH: Atom = Atom(73);
    /// `<tbody>` table body element
    pub const TBODY: Atom = Atom(74);
    /// `<thead>` table head element
    pub const THEAD: Atom = Atom(75);
    /// `<tfoot>` table foot element
    pub const TFOOT: Atom = Atom(76);
    /// `<colgroup>` column group element
    pub const COLGROUP: Atom = Atom(77);
    /// `<caption>` table caption element
    pub const CAPTION: Atom = Atom(78);
    /// `<fieldset>` fieldset element
    pub const FIELDSET: Atom = Atom(79);
    /// `<legend>` legend element
    pub const LEGEND: Atom = Atom(80);
    /// `<optgroup>` option group element
    pub const OPTGROUP: Atom = Atom(81);
    /// `<datalist>` datalist element
    pub const DATALIST: Atom = Atom(82);
    /// `<output>` output element
    pub const OUTPUT: Atom = Atom(83);
    /// `<progress>` progress element
    pub const PROGRESS: Atom = Atom(84);
    /// `<meter>` meter element
    pub const METER: Atom = Atom(85);
    /// `<details>` details element
    pub const DETAILS: Atom = Atom(86);
    /// `<summary>` summary element
    pub const SUMMARY: Atom = Atom(87);
    /// `<dialog>` dialog element
    pub const DIALOG: Atom = Atom(88);
    /// `<template>` template element
    pub const TEMPLATE: Atom = Atom(89);
    /// `<slot>` slot element
    pub const SLOT: Atom = Atom(90);

    // ==========================================================================
    // Common Attributes (100-199)
    // ==========================================================================

    /// `id` attribute
    pub const ID: Atom = Atom(100);
    /// `class` attribute
    pub const CLASS: Atom = Atom(101);
    /// `style` attribute
    pub const STYLE: Atom = Atom(102);
    /// `src` attribute
    pub const SRC: Atom = Atom(103);
    /// `href` attribute
    pub const HREF: Atom = Atom(104);
    /// `type` attribute
    pub const TYPE: Atom = Atom(105);
    /// `name` attribute
    pub const NAME: Atom = Atom(106);
    /// `value` attribute
    pub const VALUE: Atom = Atom(107);
    /// `alt` attribute
    pub const ALT: Atom = Atom(108);
    /// `title` attribute
    pub const TITLE_ATTR: Atom = Atom(109);
    /// `placeholder` attribute
    pub const PLACEHOLDER: Atom = Atom(110);
    /// `disabled` attribute
    pub const DISABLED: Atom = Atom(111);
    /// `checked` attribute
    pub const CHECKED: Atom = Atom(112);
    /// `readonly` attribute
    pub const READONLY: Atom = Atom(113);
    /// `required` attribute
    pub const REQUIRED: Atom = Atom(114);
    /// `selected` attribute
    pub const SELECTED: Atom = Atom(115);
    /// `hidden` attribute
    pub const HIDDEN: Atom = Atom(116);
    /// `data-*` attribute prefix
    pub const DATA: Atom = Atom(117);
    /// `width` attribute
    pub const WIDTH: Atom = Atom(118);
    /// `height` attribute
    pub const HEIGHT: Atom = Atom(119);
    /// `target` attribute
    pub const TARGET: Atom = Atom(120);
    /// `rel` attribute
    pub const REL: Atom = Atom(121);
    /// `role` attribute
    pub const ROLE: Atom = Atom(122);
    /// `aria-*` attribute prefix
    pub const ARIA: Atom = Atom(123);
    /// `tabindex` attribute
    pub const TABINDEX: Atom = Atom(124);
    /// `autofocus` attribute
    pub const AUTOFOCUS: Atom = Atom(125);
    /// `autocomplete` attribute
    pub const AUTOCOMPLETE: Atom = Atom(126);
    /// `maxlength` attribute
    pub const MAXLENGTH: Atom = Atom(127);
    /// `minlength` attribute
    pub const MINLENGTH: Atom = Atom(128);
    /// `pattern` attribute
    pub const PATTERN: Atom = Atom(129);
    /// `min` attribute
    pub const MIN: Atom = Atom(130);
    /// `max` attribute
    pub const MAX: Atom = Atom(131);
    /// `step` attribute
    pub const STEP: Atom = Atom(132);
    /// `multiple` attribute
    pub const MULTIPLE: Atom = Atom(133);
    /// `accept` attribute
    pub const ACCEPT: Atom = Atom(134);
    /// `action` attribute
    pub const ACTION: Atom = Atom(135);
    /// `method` attribute
    pub const METHOD: Atom = Atom(136);
    /// `enctype` attribute
    pub const ENCTYPE: Atom = Atom(137);
    /// `for` attribute (label)
    pub const FOR: Atom = Atom(138);
    /// `colspan` attribute
    pub const COLSPAN: Atom = Atom(139);
    /// `rowspan` attribute
    pub const ROWSPAN: Atom = Atom(140);
    /// `scope` attribute
    pub const SCOPE: Atom = Atom(141);
    /// `headers` attribute
    pub const HEADERS: Atom = Atom(142);
    /// `download` attribute
    pub const DOWNLOAD: Atom = Atom(143);
    /// `ping` attribute
    pub const PING: Atom = Atom(144);
    /// `hreflang` attribute
    pub const HREFLANG: Atom = Atom(145);
    /// `media` attribute
    pub const MEDIA: Atom = Atom(146);
    /// `sizes` attribute
    pub const SIZES: Atom = Atom(147);
    /// `srcset` attribute
    pub const SRCSET: Atom = Atom(148);
    /// `crossorigin` attribute
    pub const CROSSORIGIN: Atom = Atom(149);
    /// `integrity` attribute
    pub const INTEGRITY: Atom = Atom(150);
    /// `loading` attribute
    pub const LOADING: Atom = Atom(151);
    /// `decoding` attribute
    pub const DECODING: Atom = Atom(152);
    /// `async` attribute
    pub const ASYNC: Atom = Atom(153);
    /// `defer` attribute
    pub const DEFER: Atom = Atom(154);
    /// `nomodule` attribute
    pub const NOMODULE: Atom = Atom(155);
    /// `nonce` attribute
    pub const NONCE: Atom = Atom(156);
    /// `referrerpolicy` attribute
    pub const REFERRERPOLICY: Atom = Atom(157);
    /// `sandbox` attribute
    pub const SANDBOX: Atom = Atom(158);
    /// `allow` attribute
    pub const ALLOW: Atom = Atom(159);
    /// `allowfullscreen` attribute
    pub const ALLOWFULLSCREEN: Atom = Atom(160);
    /// `contenteditable` attribute
    pub const CONTENTEDITABLE: Atom = Atom(161);
    /// `draggable` attribute
    pub const DRAGGABLE: Atom = Atom(162);
    /// `spellcheck` attribute
    pub const SPELLCHECK: Atom = Atom(163);
    /// `translate` attribute
    pub const TRANSLATE: Atom = Atom(164);
    /// `dir` attribute
    pub const DIR: Atom = Atom(165);
    /// `lang` attribute
    pub const LANG: Atom = Atom(166);
    /// `inputmode` attribute
    pub const INPUTMODE: Atom = Atom(167);
    /// `enterkeyhint` attribute
    pub const ENTERKEYHINT: Atom = Atom(168);
    /// `is` attribute (custom elements)
    pub const IS: Atom = Atom(169);
    /// `part` attribute (shadow DOM)
    pub const PART: Atom = Atom(170);
    /// `slot` attribute (shadow DOM)
    pub const SLOT_ATTR: Atom = Atom(171);

    // ==========================================================================
    // Event Types (200-299)
    // ==========================================================================

    /// `click` event
    pub const CLICK: Atom = Atom(200);
    /// `load` event
    pub const LOAD: Atom = Atom(201);
    /// `error` event
    pub const ERROR: Atom = Atom(202);
    /// `submit` event
    pub const SUBMIT: Atom = Atom(203);
    /// `input` event
    pub const INPUT_EVENT: Atom = Atom(204);
    /// `change` event
    pub const CHANGE: Atom = Atom(205);
    /// `focus` event
    pub const FOCUS: Atom = Atom(206);
    /// `blur` event
    pub const BLUR: Atom = Atom(207);
    /// `keydown` event
    pub const KEYDOWN: Atom = Atom(208);
    /// `keyup` event
    pub const KEYUP: Atom = Atom(209);
    /// `keypress` event
    pub const KEYPRESS: Atom = Atom(210);
    /// `mousedown` event
    pub const MOUSEDOWN: Atom = Atom(211);
    /// `mouseup` event
    pub const MOUSEUP: Atom = Atom(212);
    /// `mousemove` event
    pub const MOUSEMOVE: Atom = Atom(213);
    /// `mouseenter` event
    pub const MOUSEENTER: Atom = Atom(214);
    /// `mouseleave` event
    pub const MOUSELEAVE: Atom = Atom(215);
    /// `mouseover` event
    pub const MOUSEOVER: Atom = Atom(216);
    /// `mouseout` event
    pub const MOUSEOUT: Atom = Atom(217);
    /// `dblclick` event
    pub const DBLCLICK: Atom = Atom(218);
    /// `contextmenu` event
    pub const CONTEXTMENU: Atom = Atom(219);
    /// `wheel` event
    pub const WHEEL: Atom = Atom(220);
    /// `scroll` event
    pub const SCROLL: Atom = Atom(221);
    /// `resize` event
    pub const RESIZE: Atom = Atom(222);
    /// `touchstart` event
    pub const TOUCHSTART: Atom = Atom(223);
    /// `touchend` event
    pub const TOUCHEND: Atom = Atom(224);
    /// `touchmove` event
    pub const TOUCHMOVE: Atom = Atom(225);
    /// `touchcancel` event
    pub const TOUCHCANCEL: Atom = Atom(226);
    /// `pointerdown` event
    pub const POINTERDOWN: Atom = Atom(227);
    /// `pointerup` event
    pub const POINTERUP: Atom = Atom(228);
    /// `pointermove` event
    pub const POINTERMOVE: Atom = Atom(229);
    /// `pointerenter` event
    pub const POINTERENTER: Atom = Atom(230);
    /// `pointerleave` event
    pub const POINTERLEAVE: Atom = Atom(231);
    /// `pointerover` event
    pub const POINTEROVER: Atom = Atom(232);
    /// `pointerout` event
    pub const POINTEROUT: Atom = Atom(233);
    /// `pointercancel` event
    pub const POINTERCANCEL: Atom = Atom(234);
    /// `dragstart` event
    pub const DRAGSTART: Atom = Atom(235);
    /// `drag` event
    pub const DRAG: Atom = Atom(236);
    /// `dragend` event
    pub const DRAGEND: Atom = Atom(237);
    /// `dragenter` event
    pub const DRAGENTER: Atom = Atom(238);
    /// `dragleave` event
    pub const DRAGLEAVE: Atom = Atom(239);
    /// `dragover` event
    pub const DRAGOVER: Atom = Atom(240);
    /// `drop` event
    pub const DROP: Atom = Atom(241);
    /// `copy` event
    pub const COPY: Atom = Atom(242);
    /// `cut` event
    pub const CUT: Atom = Atom(243);
    /// `paste` event
    pub const PASTE: Atom = Atom(244);
    /// `animationstart` event
    pub const ANIMATIONSTART: Atom = Atom(245);
    /// `animationend` event
    pub const ANIMATIONEND: Atom = Atom(246);
    /// `animationiteration` event
    pub const ANIMATIONITERATION: Atom = Atom(247);
    /// `transitionstart` event
    pub const TRANSITIONSTART: Atom = Atom(248);
    /// `transitionend` event
    pub const TRANSITIONEND: Atom = Atom(249);
    /// `transitionrun` event
    pub const TRANSITIONRUN: Atom = Atom(250);
    /// `transitioncancel` event
    pub const TRANSITIONCANCEL: Atom = Atom(251);
    /// `DOMContentLoaded` event
    pub const DOMCONTENTLOADED: Atom = Atom(252);
    /// `readystatechange` event
    pub const READYSTATECHANGE: Atom = Atom(253);
    /// `beforeunload` event
    pub const BEFOREUNLOAD: Atom = Atom(254);
    /// `unload` event
    pub const UNLOAD: Atom = Atom(255);
    /// `hashchange` event
    pub const HASHCHANGE: Atom = Atom(256);
    /// `popstate` event
    pub const POPSTATE: Atom = Atom(257);
    /// `storage` event
    pub const STORAGE: Atom = Atom(258);
    /// `message` event
    pub const MESSAGE: Atom = Atom(259);
    /// `online` event
    pub const ONLINE: Atom = Atom(260);
    /// `offline` event
    pub const OFFLINE: Atom = Atom(261);
    /// `visibilitychange` event
    pub const VISIBILITYCHANGE: Atom = Atom(262);

    // ==========================================================================
    // Common CSS Properties (300-399)
    // ==========================================================================

    /// `display` CSS property
    pub const DISPLAY: Atom = Atom(300);
    /// `position` CSS property
    pub const POSITION: Atom = Atom(301);
    /// `top` CSS property
    pub const TOP: Atom = Atom(302);
    /// `right` CSS property
    pub const RIGHT: Atom = Atom(303);
    /// `bottom` CSS property
    pub const BOTTOM: Atom = Atom(304);
    /// `left` CSS property
    pub const LEFT: Atom = Atom(305);
    /// `margin` CSS property
    pub const MARGIN: Atom = Atom(306);
    /// `padding` CSS property
    pub const PADDING: Atom = Atom(307);
    /// `width` CSS property
    pub const WIDTH_CSS: Atom = Atom(308);
    /// `height` CSS property
    pub const HEIGHT_CSS: Atom = Atom(309);
    /// `color` CSS property
    pub const COLOR: Atom = Atom(310);
    /// `background` CSS property
    pub const BACKGROUND: Atom = Atom(311);
    /// `font-size` CSS property
    pub const FONT_SIZE: Atom = Atom(312);
    /// `font-family` CSS property
    pub const FONT_FAMILY: Atom = Atom(313);
    /// `font-weight` CSS property
    pub const FONT_WEIGHT: Atom = Atom(314);
    /// `border` CSS property
    pub const BORDER: Atom = Atom(315);
    /// `opacity` CSS property
    pub const OPACITY: Atom = Atom(316);
    /// `z-index` CSS property
    pub const Z_INDEX: Atom = Atom(317);
    /// `overflow` CSS property
    pub const OVERFLOW: Atom = Atom(318);
    /// `visibility` CSS property
    pub const VISIBILITY: Atom = Atom(319);
    /// `transform` CSS property
    pub const TRANSFORM: Atom = Atom(320);
    /// `transition` CSS property
    pub const TRANSITION: Atom = Atom(321);
    /// `animation` CSS property
    pub const ANIMATION: Atom = Atom(322);
    /// `flex` CSS property
    pub const FLEX: Atom = Atom(323);
    /// `grid` CSS property
    pub const GRID: Atom = Atom(324);
    /// `box-shadow` CSS property
    pub const BOX_SHADOW: Atom = Atom(325);
    /// `text-align` CSS property
    pub const TEXT_ALIGN: Atom = Atom(326);
    /// `line-height` CSS property
    pub const LINE_HEIGHT: Atom = Atom(327);
    /// `cursor` CSS property
    pub const CURSOR: Atom = Atom(328);
    /// `pointer-events` CSS property
    pub const POINTER_EVENTS: Atom = Atom(329);
}

// Static atom lookup tables
static ATOM_MAP: Lazy<HashMap<&'static str, Atom>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // HTML Elements
    map.insert("div", atoms::DIV);
    map.insert("span", atoms::SPAN);
    map.insert("p", atoms::P);
    map.insert("a", atoms::A);
    map.insert("img", atoms::IMG);
    map.insert("input", atoms::INPUT);
    map.insert("button", atoms::BUTTON);
    map.insert("form", atoms::FORM);
    map.insert("table", atoms::TABLE);
    map.insert("tr", atoms::TR);
    map.insert("td", atoms::TD);
    map.insert("th", atoms::TH);
    map.insert("ul", atoms::UL);
    map.insert("ol", atoms::OL);
    map.insert("li", atoms::LI);
    map.insert("h1", atoms::H1);
    map.insert("h2", atoms::H2);
    map.insert("h3", atoms::H3);
    map.insert("body", atoms::BODY);
    map.insert("head", atoms::HEAD);
    map.insert("html", atoms::HTML);
    map.insert("br", atoms::BR);
    map.insert("hr", atoms::HR);
    map.insert("meta", atoms::META);
    map.insert("link", atoms::LINK);
    map.insert("area", atoms::AREA);
    map.insert("base", atoms::BASE);
    map.insert("col", atoms::COL);
    map.insert("embed", atoms::EMBED);
    map.insert("param", atoms::PARAM);
    map.insert("source", atoms::SOURCE);
    map.insert("track", atoms::TRACK);
    map.insert("wbr", atoms::WBR);
    map.insert("h4", atoms::H4);
    map.insert("h5", atoms::H5);
    map.insert("h6", atoms::H6);
    map.insert("header", atoms::HEADER);
    map.insert("footer", atoms::FOOTER);
    map.insert("nav", atoms::NAV);
    map.insert("main", atoms::MAIN);
    map.insert("article", atoms::ARTICLE);
    map.insert("section", atoms::SECTION);
    map.insert("aside", atoms::ASIDE);
    map.insert("script", atoms::SCRIPT);
    map.insert("style", atoms::STYLE_ELEM);
    map.insert("title", atoms::TITLE);
    map.insert("textarea", atoms::TEXTAREA);
    map.insert("select", atoms::SELECT);
    map.insert("option", atoms::OPTION);
    map.insert("label", atoms::LABEL);
    map.insert("iframe", atoms::IFRAME);
    map.insert("canvas", atoms::CANVAS);
    map.insert("video", atoms::VIDEO);
    map.insert("audio", atoms::AUDIO);
    map.insert("pre", atoms::PRE);
    map.insert("code", atoms::CODE);
    map.insert("strong", atoms::STRONG);
    map.insert("em", atoms::EM);
    map.insert("i", atoms::I);
    map.insert("b", atoms::B);
    map.insert("u", atoms::U);
    map.insert("small", atoms::SMALL);
    map.insert("sup", atoms::SUP);
    map.insert("sub", atoms::SUB);
    map.insert("blockquote", atoms::BLOCKQUOTE);
    map.insert("dl", atoms::DL);
    map.insert("dt", atoms::DT);
    map.insert("dd", atoms::DD);
    map.insert("figure", atoms::FIGURE);
    map.insert("figcaption", atoms::FIGCAPTION);
    map.insert("picture", atoms::PICTURE);
    map.insert("svg", atoms::SVG);
    map.insert("math", atoms::MATH);
    map.insert("tbody", atoms::TBODY);
    map.insert("thead", atoms::THEAD);
    map.insert("tfoot", atoms::TFOOT);
    map.insert("colgroup", atoms::COLGROUP);
    map.insert("caption", atoms::CAPTION);
    map.insert("fieldset", atoms::FIELDSET);
    map.insert("legend", atoms::LEGEND);
    map.insert("optgroup", atoms::OPTGROUP);
    map.insert("datalist", atoms::DATALIST);
    map.insert("output", atoms::OUTPUT);
    map.insert("progress", atoms::PROGRESS);
    map.insert("meter", atoms::METER);
    map.insert("details", atoms::DETAILS);
    map.insert("summary", atoms::SUMMARY);
    map.insert("dialog", atoms::DIALOG);
    map.insert("template", atoms::TEMPLATE);
    map.insert("slot", atoms::SLOT);

    // Attributes
    map.insert("id", atoms::ID);
    map.insert("class", atoms::CLASS);
    map.insert("src", atoms::SRC);
    map.insert("href", atoms::HREF);
    map.insert("type", atoms::TYPE);
    map.insert("name", atoms::NAME);
    map.insert("value", atoms::VALUE);
    map.insert("alt", atoms::ALT);
    map.insert("placeholder", atoms::PLACEHOLDER);
    map.insert("disabled", atoms::DISABLED);
    map.insert("checked", atoms::CHECKED);
    map.insert("readonly", atoms::READONLY);
    map.insert("required", atoms::REQUIRED);
    map.insert("selected", atoms::SELECTED);
    map.insert("hidden", atoms::HIDDEN);
    map.insert("data", atoms::DATA);
    map.insert("width", atoms::WIDTH);
    map.insert("height", atoms::HEIGHT);
    map.insert("target", atoms::TARGET);
    map.insert("rel", atoms::REL);
    map.insert("role", atoms::ROLE);
    map.insert("aria", atoms::ARIA);
    map.insert("tabindex", atoms::TABINDEX);
    map.insert("autofocus", atoms::AUTOFOCUS);
    map.insert("autocomplete", atoms::AUTOCOMPLETE);
    map.insert("maxlength", atoms::MAXLENGTH);
    map.insert("minlength", atoms::MINLENGTH);
    map.insert("pattern", atoms::PATTERN);
    map.insert("min", atoms::MIN);
    map.insert("max", atoms::MAX);
    map.insert("step", atoms::STEP);
    map.insert("multiple", atoms::MULTIPLE);
    map.insert("accept", atoms::ACCEPT);
    map.insert("action", atoms::ACTION);
    map.insert("method", atoms::METHOD);
    map.insert("enctype", atoms::ENCTYPE);
    map.insert("for", atoms::FOR);
    map.insert("colspan", atoms::COLSPAN);
    map.insert("rowspan", atoms::ROWSPAN);
    map.insert("scope", atoms::SCOPE);
    map.insert("headers", atoms::HEADERS);
    map.insert("download", atoms::DOWNLOAD);
    map.insert("ping", atoms::PING);
    map.insert("hreflang", atoms::HREFLANG);
    map.insert("media", atoms::MEDIA);
    map.insert("sizes", atoms::SIZES);
    map.insert("srcset", atoms::SRCSET);
    map.insert("crossorigin", atoms::CROSSORIGIN);
    map.insert("integrity", atoms::INTEGRITY);
    map.insert("loading", atoms::LOADING);
    map.insert("decoding", atoms::DECODING);
    map.insert("async", atoms::ASYNC);
    map.insert("defer", atoms::DEFER);
    map.insert("nomodule", atoms::NOMODULE);
    map.insert("nonce", atoms::NONCE);
    map.insert("referrerpolicy", atoms::REFERRERPOLICY);
    map.insert("sandbox", atoms::SANDBOX);
    map.insert("allow", atoms::ALLOW);
    map.insert("allowfullscreen", atoms::ALLOWFULLSCREEN);
    map.insert("contenteditable", atoms::CONTENTEDITABLE);
    map.insert("draggable", atoms::DRAGGABLE);
    map.insert("spellcheck", atoms::SPELLCHECK);
    map.insert("translate", atoms::TRANSLATE);
    map.insert("dir", atoms::DIR);
    map.insert("lang", atoms::LANG);
    map.insert("inputmode", atoms::INPUTMODE);
    map.insert("enterkeyhint", atoms::ENTERKEYHINT);
    map.insert("is", atoms::IS);
    map.insert("part", atoms::PART);

    // Event types
    map.insert("click", atoms::CLICK);
    map.insert("load", atoms::LOAD);
    map.insert("error", atoms::ERROR);
    map.insert("submit", atoms::SUBMIT);
    map.insert("change", atoms::CHANGE);
    map.insert("focus", atoms::FOCUS);
    map.insert("blur", atoms::BLUR);
    map.insert("keydown", atoms::KEYDOWN);
    map.insert("keyup", atoms::KEYUP);
    map.insert("keypress", atoms::KEYPRESS);
    map.insert("mousedown", atoms::MOUSEDOWN);
    map.insert("mouseup", atoms::MOUSEUP);
    map.insert("mousemove", atoms::MOUSEMOVE);
    map.insert("mouseenter", atoms::MOUSEENTER);
    map.insert("mouseleave", atoms::MOUSELEAVE);
    map.insert("mouseover", atoms::MOUSEOVER);
    map.insert("mouseout", atoms::MOUSEOUT);
    map.insert("dblclick", atoms::DBLCLICK);
    map.insert("contextmenu", atoms::CONTEXTMENU);
    map.insert("wheel", atoms::WHEEL);
    map.insert("scroll", atoms::SCROLL);
    map.insert("resize", atoms::RESIZE);
    map.insert("touchstart", atoms::TOUCHSTART);
    map.insert("touchend", atoms::TOUCHEND);
    map.insert("touchmove", atoms::TOUCHMOVE);
    map.insert("touchcancel", atoms::TOUCHCANCEL);
    map.insert("pointerdown", atoms::POINTERDOWN);
    map.insert("pointerup", atoms::POINTERUP);
    map.insert("pointermove", atoms::POINTERMOVE);
    map.insert("pointerenter", atoms::POINTERENTER);
    map.insert("pointerleave", atoms::POINTERLEAVE);
    map.insert("pointerover", atoms::POINTEROVER);
    map.insert("pointerout", atoms::POINTEROUT);
    map.insert("pointercancel", atoms::POINTERCANCEL);
    map.insert("dragstart", atoms::DRAGSTART);
    map.insert("drag", atoms::DRAG);
    map.insert("dragend", atoms::DRAGEND);
    map.insert("dragenter", atoms::DRAGENTER);
    map.insert("dragleave", atoms::DRAGLEAVE);
    map.insert("dragover", atoms::DRAGOVER);
    map.insert("drop", atoms::DROP);
    map.insert("copy", atoms::COPY);
    map.insert("cut", atoms::CUT);
    map.insert("paste", atoms::PASTE);
    map.insert("animationstart", atoms::ANIMATIONSTART);
    map.insert("animationend", atoms::ANIMATIONEND);
    map.insert("animationiteration", atoms::ANIMATIONITERATION);
    map.insert("transitionstart", atoms::TRANSITIONSTART);
    map.insert("transitionend", atoms::TRANSITIONEND);
    map.insert("transitionrun", atoms::TRANSITIONRUN);
    map.insert("transitioncancel", atoms::TRANSITIONCANCEL);
    map.insert("DOMContentLoaded", atoms::DOMCONTENTLOADED);
    map.insert("readystatechange", atoms::READYSTATECHANGE);
    map.insert("beforeunload", atoms::BEFOREUNLOAD);
    map.insert("unload", atoms::UNLOAD);
    map.insert("hashchange", atoms::HASHCHANGE);
    map.insert("popstate", atoms::POPSTATE);
    map.insert("storage", atoms::STORAGE);
    map.insert("message", atoms::MESSAGE);
    map.insert("online", atoms::ONLINE);
    map.insert("offline", atoms::OFFLINE);
    map.insert("visibilitychange", atoms::VISIBILITYCHANGE);

    // CSS properties
    map.insert("display", atoms::DISPLAY);
    map.insert("position", atoms::POSITION);
    map.insert("top", atoms::TOP);
    map.insert("right", atoms::RIGHT);
    map.insert("bottom", atoms::BOTTOM);
    map.insert("left", atoms::LEFT);
    map.insert("margin", atoms::MARGIN);
    map.insert("padding", atoms::PADDING);
    map.insert("color", atoms::COLOR);
    map.insert("background", atoms::BACKGROUND);
    map.insert("font-size", atoms::FONT_SIZE);
    map.insert("font-family", atoms::FONT_FAMILY);
    map.insert("font-weight", atoms::FONT_WEIGHT);
    map.insert("border", atoms::BORDER);
    map.insert("opacity", atoms::OPACITY);
    map.insert("z-index", atoms::Z_INDEX);
    map.insert("overflow", atoms::OVERFLOW);
    map.insert("visibility", atoms::VISIBILITY);
    map.insert("transform", atoms::TRANSFORM);
    map.insert("transition", atoms::TRANSITION);
    map.insert("animation", atoms::ANIMATION);
    map.insert("flex", atoms::FLEX);
    map.insert("grid", atoms::GRID);
    map.insert("box-shadow", atoms::BOX_SHADOW);
    map.insert("text-align", atoms::TEXT_ALIGN);
    map.insert("line-height", atoms::LINE_HEIGHT);
    map.insert("cursor", atoms::CURSOR);
    map.insert("pointer-events", atoms::POINTER_EVENTS);

    map
});

static REVERSE_MAP: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    ATOM_MAP
        .iter()
        .map(|(&s, &atom)| (atom.0, s))
        .collect()
});

/// Get the total number of predefined atoms
pub fn atom_count() -> usize {
    ATOM_MAP.len()
}

/// Iterator over all predefined atoms
pub fn all_atoms() -> impl Iterator<Item = (&'static str, Atom)> {
    ATOM_MAP.iter().map(|(&s, &a)| (s, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_equality() {
        assert_eq!(atoms::DIV, atoms::DIV);
        assert_ne!(atoms::DIV, atoms::SPAN);
    }

    #[test]
    fn test_atom_from_str() {
        assert_eq!(Atom::from_str("div"), Some(atoms::DIV));
        assert_eq!(Atom::from_str("span"), Some(atoms::SPAN));
        assert_eq!(Atom::from_str("p"), Some(atoms::P));
        assert_eq!(Atom::from_str("unknown"), None);
    }

    #[test]
    fn test_atom_as_str() {
        assert_eq!(atoms::DIV.as_str(), Some("div"));
        assert_eq!(atoms::SPAN.as_str(), Some("span"));
        assert_eq!(atoms::CLICK.as_str(), Some("click"));
    }

    #[test]
    fn test_atom_categories() {
        // Elements
        assert!(atoms::DIV.is_element());
        assert!(atoms::SPAN.is_element());
        assert!(!atoms::DIV.is_attribute());
        assert!(!atoms::DIV.is_event());

        // Attributes
        assert!(atoms::ID.is_attribute());
        assert!(atoms::CLASS.is_attribute());
        assert!(!atoms::ID.is_element());
        assert!(!atoms::ID.is_event());

        // Events
        assert!(atoms::CLICK.is_event());
        assert!(atoms::LOAD.is_event());
        assert!(!atoms::CLICK.is_element());
        assert!(!atoms::CLICK.is_attribute());

        // CSS Properties
        assert!(atoms::DISPLAY.is_css_property());
        assert!(atoms::POSITION.is_css_property());
        assert!(!atoms::DISPLAY.is_element());
    }

    #[test]
    fn test_void_elements() {
        assert!(atoms::IMG.is_void_element());
        assert!(atoms::INPUT.is_void_element());
        assert!(atoms::BR.is_void_element());
        assert!(atoms::HR.is_void_element());
        assert!(atoms::META.is_void_element());
        assert!(atoms::LINK.is_void_element());

        assert!(!atoms::DIV.is_void_element());
        assert!(!atoms::SPAN.is_void_element());
        assert!(!atoms::P.is_void_element());
    }

    #[test]
    fn test_atom_debug_format() {
        let debug = format!("{:?}", atoms::DIV);
        assert!(debug.contains("div"));
        assert!(debug.contains("1"));
    }

    #[test]
    fn test_atom_display_format() {
        assert_eq!(format!("{}", atoms::DIV), "div");
        assert_eq!(format!("{}", atoms::CLICK), "click");
    }

    #[test]
    fn test_atom_raw_id() {
        assert_eq!(atoms::DIV.raw(), 1);
        assert_eq!(atoms::SPAN.raw(), 2);
        assert_eq!(atoms::ID.raw(), 100);
        assert_eq!(atoms::CLICK.raw(), 200);
    }

    #[test]
    fn test_atom_from_raw() {
        let atom = Atom::from_raw(1);
        assert_eq!(atom, atoms::DIV);
    }

    #[test]
    fn test_atom_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(atoms::DIV);
        set.insert(atoms::SPAN);
        set.insert(atoms::DIV); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&atoms::DIV));
        assert!(set.contains(&atoms::SPAN));
    }

    #[test]
    fn test_all_elements_have_reverse_lookup() {
        for (s, atom) in all_atoms() {
            assert_eq!(atom.as_str(), Some(s));
        }
    }

    #[test]
    fn test_atom_count() {
        // Should have a reasonable number of atoms
        assert!(atom_count() > 100);
    }

    #[test]
    fn test_common_elements() {
        // Verify common HTML elements are present
        let common_elements = [
            "div", "span", "p", "a", "img", "input", "button", "form", "table", "ul", "ol", "li",
            "h1", "h2", "h3", "body", "head", "html",
        ];

        for elem in common_elements.iter() {
            assert!(
                Atom::from_str(elem).is_some(),
                "Missing common element: {}",
                elem
            );
        }
    }

    #[test]
    fn test_common_attributes() {
        // Verify common attributes are present
        let common_attrs = [
            "id", "class", "src", "href", "type", "name", "value", "alt", "disabled", "checked",
        ];

        for attr in common_attrs.iter() {
            assert!(
                Atom::from_str(attr).is_some(),
                "Missing common attribute: {}",
                attr
            );
        }
    }

    #[test]
    fn test_common_events() {
        // Verify common events are present
        let common_events = [
            "click", "load", "error", "submit", "change", "focus", "blur", "keydown", "mousedown",
        ];

        for event in common_events.iter() {
            assert!(
                Atom::from_str(event).is_some(),
                "Missing common event: {}",
                event
            );
        }
    }

    #[test]
    fn test_atom_copy() {
        let atom1 = atoms::DIV;
        let atom2 = atom1; // Copy
        assert_eq!(atom1, atom2);
    }

    #[test]
    fn test_atom_clone() {
        let atom1 = atoms::DIV;
        #[allow(clippy::clone_on_copy)]
        let atom2 = atom1.clone();
        assert_eq!(atom1, atom2);
    }
}
