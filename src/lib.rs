use std::io::Cursor;

use docx::document::{BodyContent, ParagraphContent, RunContent};
use docx::DocxFile;

use wasm_bindgen::prelude::*;
// use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn process_docx(buffer: &[u8]) -> String {
    let cursor_reader = Cursor::new(&buffer);

    let docx_file = DocxFile::from_reader(cursor_reader).expect("should read docx from buf");
    let docx = docx_file.parse().expect("should parse docx correctly");

    let render = docx
        .document
        .body
        .content
        .iter()
        .map(|body_content| match body_content {
            BodyContent::Paragraph(paragraph) => {
                let mut html = String::from("<p>");
                let p_content = paragraph
                    .content
                    .iter()
                    .map(|paragraph_content| match paragraph_content {
                        ParagraphContent::Run(run) => {
                            let mut html = String::from("<span>");
                            let r_content = run
                                .content
                                .iter()
                                .map(|run_content| match run_content {
                                    RunContent::Text(text) => Some(text.text.to_string()),
                                    _ => None,
                                })
                                .filter_map(|x| x)
                                .collect::<Vec<_>>()
                                .join("");
                            html.push_str(&r_content);
                            html.push_str("</span>");
                            Some(html)
                        }
                        _ => None,
                    })
                    .filter_map(|x| x)
                    .collect::<Vec<_>>()
                    .join("");

                html.push_str(&p_content);

                html.push_str("</p>");
                Some(html)
            }
            _ => None,
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>()
        .join("");
    render
}
