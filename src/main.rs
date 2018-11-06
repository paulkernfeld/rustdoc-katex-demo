//! This project demonstrates an approach to including KaTeX in Rust docs. It tries to balance
//! readable source code, attractive rendered output, and ease of use.
//!
//! Docs with KaTeX can be generated locally and [on docs.rs](https://docs.rs/rustdoc-katex-demo).
//!
//! Setup
//! =====
//!
//! You'll only need one file: just grab `katex-header.html` from this project and put it into the
//! root of your project.
//!
//! ## Rendering Locally
//!
//! This project can be documented locally with the following commands. Dependencies are
//! documented separately because you probably don't want your dependencies' docs to use KaTeX, plus
//! it won't build correctly.
//!
//! ```bash
//! cargo doc
//! RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
//! ```
//!
//! ## Rendering on Docs.rs
//!
//! Include the following snippet in your `Cargo.toml`:
//!
//! ```toml
//! [package.metadata.docs.rs]
//! rustdoc-args = [ "--html-in-header", "katex-header.html" ]
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
//! write KaTeX! I think it's worth it, because being able to use a single dollar sign for inline
//! math expressions makes the source code a lot more readable.
//!
//! If you try to write two dollar signs in one paragraph, your text will $look like this!$
//!
//! Dollar signs inside of code blocks are safe: `$\KaTeX$`.
//!
//! It's safe to write a single $ as long as there are no other dollar signs in the same HTML
//! element. This translates to approximately one Markdown paragraph or bullet item.
//!
//! If you're the kind of person who likes to write many dollar signs all in the same paragraph,
//! you can surround each one in a `<span>` element: <span>$</span><span>$</span><span>$</span>.
//!
//! More
//! ====
//!
//! If you want to use your own set of delimiters, you can change that in `katex-header.html`. The
//! order of the delimiters is important, so you may with to consult the KaTeX auto-render
//! [docs](https://katex.org/docs/autorender.html) and
//! [source code](https://github.com/Khan/KaTeX/blob/master/contrib/auto-render/auto-render.js).
//!
//! This project currently depends on KaTeX's official CDN, jsDelivr. In the future, I would like to
//! create a way to bundle the JS and CSS resources into the project.
//!
//! I am greatly indebted to [rust-num PR #226](https://github.com/rust-num/num/pull/226) by hauleth
//! for providing a template for this technique.
fn main() {}
