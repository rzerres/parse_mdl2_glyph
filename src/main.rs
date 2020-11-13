/*
 * parse_segoe_mdl2 - extract glyphs from Microsoft® Segoe MDL2 icon fonts
 *
 * Copyright 2020 Ralf Zerres <ralf.zerres@networkx.de>
 * SPDX-License-Identifier: (0BSD or MIT)
 */

use std::env;
use std::fs::File;
use std::io::{Read, Write};

/// parse Segoe UI html document
pub fn parse_segoe_html_tree(document: &str) -> Result<(), Box<dyn std::error::Error>> {

    //use kuchiki::traits::*;
    use kuchiki::traits::TendrilSink;

    // kuchiki parses the html input, get an instance
    let dom = kuchiki::parse_html().one(document);
    //let dom = kuchiki::parse_html().one(content);
    //let dom = kuchiki::parse_html().one(buffer);

    let paragraph_data = dom
        .select("p")
        .unwrap()
        .collect::<Vec<_>>();

    let paragraphs = paragraph_data.len();
    println!("number of paragraph: {}", &paragraphs);

    for n in 0..paragraphs {
        println!("paragraph child: {:?}", &*paragraph_data[n].text_contents());
    }

    let table_rows_data = dom
        .select("tr")
        .unwrap()
        .collect::<Vec<_>>();
    let table_rows = table_rows_data.len();

    if table_rows > 0 {
        println!("number of table rows: {:?}", table_rows);

        // Create a temporary file.
        let temp_directory = env::temp_dir();
        let const_color = temp_directory.join("segmdl2_assets_font.rs");

        // Open a file in write-only (ignoring errors).
        // This creates the file if it does not exist (and empty the file if it exists).
        let mut file = File::create(&const_color).unwrap();

        // write the file header
        writeln!(&mut file, "/// Segoe MDL2 Assets Icons: definition of supported glyph constants\n").unwrap();
        writeln!(&mut file, "/*").unwrap();
        writeln!(&mut file, " * CSS Definiton:").unwrap();
        writeln!(&mut file, " */").unwrap();
        writeln!(&mut file, "// CSS-url:     https://aka.ms/SegoeFonts").unwrap();
        writeln!(&mut file, "// font-family: SegoeMDL2Assets").unwrap();
        writeln!(&mut file, "// font-weight: normal;").unwrap();
        writeln!(&mut file, "// font-style:  regular;").unwrap();
        writeln!(&mut file, "// font-format: ttf").unwrap();
        writeln!(&mut file, "// font-src:    SegMdl2.ttf").unwrap();
        writeln!(&mut file, "// import-tool: https://github.com/rzerres/parse_segoe_mdl2.git").unwrap();
        writeln!(&mut file, "// cheet-sheet: http://modernicons.io/segoe-mdl2/cheatsheet/").unwrap();
        writeln!(&mut file, "\n").unwrap();

        for r in 0..table_rows {
            // get concatenated string for given table row
            let child_data =  table_rows_data[r].text_contents();

            // use the iterater to truncate the needed values
            let mut child_iter = child_data.split_whitespace();
            let unicode_codepoint = child_iter.nth(0).unwrap_or("e000");
            let color_name = child_iter.next().unwrap_or("NONE");

            if r == 0 {
                println!("table rows child {} [Header]: {}", r, child_data);
            } else {
                // format the rust code color constant
                if unicode_codepoint != "0000" && color_name != "NONE" {
                    writeln!(&mut file, "pub const SEG_{}: &str = \"\\u{{{}}}\";", color_name.to_uppercase(), unicode_codepoint.to_lowercase()).unwrap();
                } else {
                    writeln!(&mut file, "// pub const SEG_{}: &str = \"\\u{{{}}}\";", color_name.to_uppercase(), unicode_codepoint.to_lowercase()).unwrap();
                }
            }
        };
        //file.write(b"Bytes\n").unwrap();
        println!("Wrote {} glyph constants to: {:?}", table_rows, &*const_color);
    }

    Ok(())
}

/// parse MDL2 html document
pub fn parse_mdl2_html_tree(document: &str) -> Result<(), Box<dyn std::error::Error>> {

    //use kuchiki::traits::*;
    use kuchiki::traits::TendrilSink;

    // kuchiki parses the html input, get an instance
    // ref: https://stackoverflow.com/questions/56329121/how-to-get-only-text-node-with-kuchiki
    let dom = kuchiki::parse_html().one(document);

    let selector = "tr";
    let selector_rows_data = dom
        .select(selector)
        .unwrap()
        .collect::<Vec<_>>();
    let selector_rows = selector_rows_data.len();

    // process all table rows
    if selector_rows > 0 {
        println!("\nnumber of table rows: {:?}", selector_rows);

        // Create a temporary file.
        let temp_directory = env::temp_dir();
        let target_file = temp_directory.join("mdl2_assets_font.rs");

        // Open a file in write-only (ignoring errors).
        // This creates the file if it does not exist (and empty the file if it exists).
        let mut file = File::create(&target_file).unwrap();

        // write the file header
        writeln!(&mut file, "/// MDL2 Assets Icons: definition of supported glyph constants\n").unwrap();
        writeln!(&mut file, "/*").unwrap();
        writeln!(&mut file, " * CSS Definiton:").unwrap();
        writeln!(&mut file, " */").unwrap();
        writeln!(&mut file, "// CSS-url:     https://github.com/scottdorman/mdl2-icons/tree/master/css/").unwrap();
        writeln!(&mut file, "// font-family: 'MDL2'").unwrap();
        writeln!(&mut file, "// font-weight: normal").unwrap();
        writeln!(&mut file, "// font-style:  normal").unwrap();
        writeln!(&mut file, "// font-format: ttf").unwrap();
        writeln!(&mut file, "// font-src:    mdl2.ttf").unwrap();
        writeln!(&mut file, "// cheet-sheet: https://github.com/scottdorman/mdl2-icons/icons.html").unwrap();
        writeln!(&mut file, "// import-tool: https://github.com/rzerres/parse_segoe_mdl2.git").unwrap();
        writeln!(&mut file, "\n").unwrap();

        for r in 0..selector_rows {
            // get child data for given table row
            let child_data = selector_rows_data[r]
                .as_node()
                .children()
                .collect::<Vec<_>>();
            //let childs = child_data.len();
            //println!("row {:?}: has {} childs", &r, &childs);

            let child_unicode = &*child_data[0].text_contents();
            //println!("unicode: {:?}", &child_unicode);
            let child_name = &*child_data[2].text_contents();
            //println!("unicode: {:?}", &child_name);
            let child_description = &*child_data[3].text_contents();
            //println!("unicode: {:?}", &child_description);

            // for t in 0..childs {
            //    println!("texts[{}]: {:?}", &t, &*child_data[t].text_contents());
            // }

            if r == 0 {
                println!("table rows child {} [Header]: {}, {}, {}",
                         r,
                         &child_unicode,
                         &child_name,
                         &child_description
                );
            } else {
                // format the rust code color constant
                if child_unicode != "0000" && child_description != "NONE" {
                    writeln!(&mut file, "pub const MDL2_{}: &str = \"\\u{{{}}}\";",
                             child_description.to_uppercase(),
                             child_unicode.to_lowercase()).unwrap();
                } else {
                    writeln!(&mut file, "// pub const MDL2_{}: &str = \"\\u{{{}}}\";",
                             child_description.to_uppercase(),
                             child_unicode.to_lowercase()).unwrap();
                }
            }
        };
        println!("Wrote {} glyph constants to: {:?}", selector_rows, &*target_file);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // hardcoded: import file, open read-only
    let filename = "./mdl2_codepoints.html";
    let mut content = String::new();
    match File::open(filename) {
        // The file is open (no error).
        Ok(mut file) => {
            // Read all the file content into a variable (ignoring the result of the operation).
            println!("reading html content from {:?}", filename);
            file.read_to_string(&mut content).unwrap();
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        },
    }

    // import to a string `buffer` that gets the content from stdin
    //let mut buffer = String::new();
    //let mut stdin = io::stdin(); // We get `Stdin` here.
    //stdin.read_to_string(&mut buffer)?;

    //parse_segoe_html_tree(&content)?;
    parse_mdl2_html_tree(&content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segoe_html() {
        // raw string: the html content
        let html = r"
        <DOCTYPE html>
        <html>
          <head></head>
          <body>
            <h1>Test for parse_segoe_mdl2</h1>
            <p class='foo'>We need this file to test parsing success.</p>
            <h2 id='using-the-icons' class='heading-anchor'><a class='anchor-link docon docon-link' href='#using-the-icons' aria-labelledby='using-the-icons'></a>Using the icons</h2>
            <p>If you are developing an app in C#/VB/C++ and XAML, you can use specified glyphs from Segoe MDL2 Assets with the <a href='https://docs.microsoft.com/en-us/uwp/api/windows.ui.xaml.controls.symbol' data-linktype='absolute-path'>Symbol enumeration</a>.</p>
            <div class='table-scroll-wrapper'><table class='table'><caption class='visually-hidden'>Icon list</caption>
              <tbody>
                <tr>
                  <td>Symbol</td>
                  <td>Unicode point</td>
                  <td>Description</td>
                </tr>
                <tr>
                  <td aria-label='No value'><img src='segoe_codepoints-Dateien/e700.png' alt='GlobalNavigationButton' data-linktype='relative-path' width='32' height='32'></td>
                  <td>E700</td>
                  <td>GlobalNavigationButton</td>
                </tr>
                <tr>
                  <td aria-label='No value'><img src='segoe_codepoints-Dateien/e701.png' alt='Wifi' data-linktype='relative-path' width='32' height='32'></td>
                  <td>E701</td>
                  <td>Wifi</td>
                </tr>
                <tr>
                  <td aria-label='No value'><img src='segoe_codepoints-Dateien/e702.png' alt='Bluetooth' data-linktype='relative-path' width='32' height='32'></td>
                  <td>E702</td>
                  <td>Bluetooth</td>
                </tr>
              </tbody></table></div>
            </body>
        </html>
        ";

        let _ = parse_segoe_html_tree(&html);
        //assert_eq!(inlined, expected)
    }
}
