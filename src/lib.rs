#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[doc(inline)]
pub use heur_core::*;

#[cfg(feature = "bits")]
#[doc(inline)]
pub use heur_bits as bits;

#[cfg(feature = "genetic")]
#[doc(inline)]
pub use heur_genetic as genetic;
