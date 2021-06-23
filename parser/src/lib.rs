#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(str_split_whitespace_as_str)]

pub mod cssom;
pub mod dom;
pub mod prelude;

pub use cssom::prelude::StyleSheetParser;
pub use dom::prelude::DocumentObjectParser;
