// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Types for tag and attribute names, and tree-builder functionality.

use tendril::StrTendril;

pub use web_atoms::{expanded_name, ElemName, ExpandedName, QualName};

pub mod tree_builder;
pub use self::tree_builder::{create_element, AppendNode, AppendText, ElementFlags, NodeOrText};
pub use self::tree_builder::{LimitedQuirks, NoQuirks, Quirks, QuirksMode};
pub use self::tree_builder::{Tracer, TreeSink};

#[must_use]
#[derive(Debug, PartialEq)]
pub enum TokenizerResult<Handle> {
    Done,
    Script(Handle),
}

/// A tag attribute, e.g. `class="test"` in `<div class="test" ...>`.
///
/// The namespace on the attribute name is almost always ns!("").
/// The tokenizer creates all attributes this way, but the tree
/// builder will adjust certain attribute names inside foreign
/// content (MathML, SVG).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Attribute {
    /// The name of the attribute (e.g. the `class` in `<div class="test">`)
    pub name: QualName,
    /// The value of the attribute (e.g. the `"test"` in `<div class="test">`)
    pub value: StrTendril,
}

#[cfg(test)]
mod tests {
    use web_atoms::{ns, Namespace};

    #[test]
    fn ns_macro() {
        assert_eq!(ns!(), Namespace::from(""));

        assert_eq!(ns!(html), Namespace::from("http://www.w3.org/1999/xhtml"));
        assert_eq!(
            ns!(xml),
            Namespace::from("http://www.w3.org/XML/1998/namespace")
        );
        assert_eq!(ns!(xmlns), Namespace::from("http://www.w3.org/2000/xmlns/"));
        assert_eq!(ns!(xlink), Namespace::from("http://www.w3.org/1999/xlink"));
        assert_eq!(ns!(svg), Namespace::from("http://www.w3.org/2000/svg"));
        assert_eq!(
            ns!(mathml),
            Namespace::from("http://www.w3.org/1998/Math/MathML")
        );
    }
}
