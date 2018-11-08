//! This crate demonstrates an approach to including KaTeX in Rust docs. It tries to balance
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
//! documented separately because you probably don't want your dependencies' docs to use KaTeX.
//! Also, dependencies would not build correctly because of relative paths.
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
//!   \hat f(\xi)e^{2 \pi i \xi x}
//!   d\xi
//! $$
//!
//! How to Not Write KaTeX
//! ======================
//!
//! Somehow the section on how to *not* write KaTeX is longer and more complex than that on how to
//! write KaTeX! I think it's worth it, because being able to use a single dollar sign for inline
//! math expressions makes the source code a lot more readable.
//!
//! If you try to write two dollar signs in one paragraph, your text will render as KaTeX! An
//! example: $1.00 + $2.00 = $3.00
//!
//! Dollar signs inside of code blocks are safe: `$1.00 + $2.00 = $3.00`.
//!
//! It's safe to write a single $ as long as there are no other dollar signs in the same HTML
//! element. This translates to approximately one Markdown paragraph or bullet item.
//!
//! If you're the kind of person who likes to write many dollar signs all in the same paragraph,
//! you can surround each one in a `<span>` element: <span>$1.00</span> + <span>$2.00</span> =
//! <span>$3.00</span>.
//!
//! More
//! ====
//!
//! If you want to use your own set of delimiters, you can change that in `katex-header.html`. The
//! order of the delimiters is important, so you may with to consult the KaTeX auto-render
//! [docs](https://katex.org/docs/autorender.html) and
//! [source code](https://github.com/Khan/KaTeX/blob/master/contrib/auto-render/auto-render.js).
//!
//! This project currently depends on KaTeX's official CDN, jsDelivr. I would also like to describe
//! a way to bundle the JS and CSS resources into the project.
//!
//! Resources
//! =========
//!
//! - [xss-probe on docs.rs](https://docs.rs/xss-probe): a Rust crate that injects JS and CSS into
//!   rendered docs.rs documentation by using `build.rs` to rewrite some files on the docs.rs build
//!   machine.
//! - The curve25519-dalek crate includes
//!   [lovely rendered KaTeX](https://doc-internal.dalek.rs/curve25519_dalek/curve_models/index.html)
//!   on their self-hosted docs site.
//! - [pwnies on docs.rs](https://docs.rs/pwnies): a Rust crate that injects JS and CSS into
//!   rendered docs.rs documentation using the `--html-in-header` argument.
//! - [GitHub issue for pwnies](https://github.com/rust-lang-nursery/docs.rs/issues/167)
//! - [rust-num PR #226](https://github.com/rust-num/num/pull/226): a nice example of KaTeX
//!   injection with `--html-in-header`. In that thread, cuviper highlights a few problems with
//!   KaTeX in rustdoc:
//!   * I would prefer all resources stay local, especially for offline use. KaTeX advertises that
//!     it is self-contained to allow bundling, so that should be possible.
//!   * The injection is a bit fragile, leaning on an environment variable, and RUSTDOCFLAGS isn't
//!     even supported in stable cargo. This also injects into all crates at the time, not just
//!     num's.
//!   * This won't work (as-is) for the sub-crates when generated independently. That's important
//!     for docs.rs where everything is independent.
//!   * Even from the main crate, it won't work if generated as a dependency in someone else's docs.