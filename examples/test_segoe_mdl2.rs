/*
 * test_segoe_mdl2 - extract glyphs from Microsoft® Segoe MDL2 icon fonts
 *
 * Copyright 2020 Ralf Zerres <ralf.zerres@networkx.de>
 * SPDX-License-Identifier: (0BSD or MIT)
 */

use std::fs::File;
use std::io::{self, Read};

//use kuchiki::traits::*;
use kuchiki::{traits::TendrilSink};

fn main() -> io::Result<()> {

    // raw string: the html content
    let html = r"
        <DOCTYPE html>
        <html>
          <head></head>
          <body>
            <h1>Example</h1>
            <p class='foo'>Testing Segeo MDL2 icon fonts!</p>
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

    // hardcoded: import file, open read-only
    let filename = "./segoe_codepoints.html";
    let mut content = String::new();
    match File::open(filename) {
        // The file is open (no error).
        Ok(mut file) => {
            // Read all the file content into a variable (ignoring the result of the operation).
            println!("\nreading html content from '{:?}'\n", filename);
            file.read_to_string(&mut content).unwrap();

            println!("\nstart: file content\n{:?}\nend file content!", content);

            // The file is automatically closed when is goes out of scope.
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

    // kuchiki parses the html input, get an instance
    //let dom = kuchiki::parse_html().one(buffer);
    let dom = kuchiki::parse_html().one(html);

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
    println!("number of table rows: {:?}", table_rows);

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
                println!("pub const SEG_{}: &str = 'u{{{}}}';", color_name.to_uppercase(), unicode_codepoint.to_lowercase());
            } else {
                println!("// pub const SEG_{}: &str = 'u{{{}}}';", color_name.to_uppercase(), unicode_codepoint.to_lowercase());
            }
        };
    }

    Ok(())
}
