/// mdBook generator for MathHook KB
///
/// This module generates markdown files suitable for mdBook, a static site generator
/// for technical documentation. Output is reference-style with deep technical detail.
mod generator;

pub use generator::MdBookGenerator;
