//!
//! a helper tool to extract glpyhs from Microsoft® Segoe MDL2 icon fonts

//! ## Overview
//!
//! This little helper tool has a single purpose:
//!
//! * find the unicode points and their descriptions
//!   for Microsoft® Segoe MDL2 icon fonts
//! * create a rust source file listing all glyphs as const
//!
//! Because a parsable CSS description is missing, the information
//! is extracted from the Microsoft online documentation page (html source).
//!
//! The documentation is published via the the following URL.
//!
//!  * [Segoe MDL2 icons](https://docs.microsoft.com/en-us/windows/uwp/design/style/segoe-ui-symbol-font#icon-list)
//!
//! Save it `segoe_codepoints.html`
//!
//! ## Examples
//! The Servo project provides the basics.
//! The compunity worked to extracted the developed parser and selectors from
//! this project and created the standlone libary crate `kuchiki`.
//! Documentation is quite rare. Have a look at the source :)

//!

//#![feature(extern_doc)]
//#[doc(include="../README.md")]
