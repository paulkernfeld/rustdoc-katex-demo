//! This project demonstrates an approach to including KaTeX in Rust docs. It tries to balance
//! readable source code, attractive rendered output, and ease of use.
//!
//! This project can be rendered locally with the following commands. Dependencies are
//! documented separately because you probably don't want your dependencies' docs to use KaTeX.
//!
//! ```bash
//! cargo doc
//! RUSTDOCFLAGS="--html-in-header header.html" cargo doc --no-deps
//! ```
//!
//! How to Write KaTeX
//! ==================
//!
//! Here is some inline $\KaTeX$.
//!
//! And now for a fancy math expression:
//!
//! $$
//! f(x) = \int_{-\infty}^\infty
//!   \hat f(\xi)\,e^{2 \pi i \xi x}
//!   \,d\xi
//! $$
//!
//! How to Not Write KaTeX
//! ======================
//!
//! Somehow the section on how to *not* write KaTeX is longer and more complex than that on how to
//! write KaTeX!
//!
//! If you try to write two dollar signs in one element, your text will $look like this!$
//!
//! Dollar signs inside of code blocks are safe: `$\KaTeX$`.
//!
//! It's safe to write a single $ as long as there are no other dollar signs in the same HTML
//! element. This translates to approximately one Markdown paragraph or bullet item.
//!
//! If you're the kind of person who likes to write many dollar signs all in the same paragraph,
//! you can surround each one in a `<span>` element: <span>$</span><span>$</span><span>$</span>.
//!
//! Customization
//! =============
//!
//! I like 
fn main() {}
