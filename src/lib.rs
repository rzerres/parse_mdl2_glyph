//!
//! a helper tool to extract glpyhs from MDL2 icon fonts

//! ## Overview
//!
//! This little helper tool has a single purpose:
//!
//! * find the unicode points and their descriptions
//!   for MDL2 icon fonts
//! * create a rust source file listing all glyphs as const
//!
//! I. Microsoft® Segoe font family
//! The Microsoft® Segoe font family is lagging a parsable CSS description.
//! It's license (Eula) states very clear: "You may use the Segoe MDL2 Assets and Segoe UI fonts
//! or glyphs included solely to design, develop and test your programs that run on a Microsoft Platform"
//! It does not allow to use and redistribute the fonts.
//! Glyph information is extracted from the Microsoft online documentation page (html source).
//!
//! II. Microsoft® Segoe font family
//! An MIT licensed implementation of MDL2 icons, developed by Scott Dorman is published on Github.
//!
//! Documentation is published via the the following URLs.
//!
//!  * [Segoe MDL2 icons](https://docs.microsoft.com/en-us/windows/uwp/design/style/segoe-ui-symbol-font#icon-list)
//!  * [MDL2 icons](https://github.com/scottdorman/mdl2-icons/)
//!
//! ## Examples
//! The Servo project provides the basics to parse html sources.
//! The community worked to extracted the developed parser and selectors from
//! this project and created the standlone libary crate `kuchiki`.
//! Documentation is quite rare. Have a look at the source :)


//#![feature(extern_doc)]
//#[doc(include="../README.md")]
