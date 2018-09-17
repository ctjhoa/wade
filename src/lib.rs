#[macro_use]
extern crate stdweb;
extern crate zip;
extern crate quick_xml;

use std::io::{Read};
use std::io::Cursor;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    Blob,
    FileReader,
    IParentNode,
    set_timeout,
    document,
};
use stdweb::web::{FileReaderReadyState,FileReaderResult};
use stdweb::web::event::ChangeEvent;
use stdweb::web::html_element::InputElement;

// use dotext::*;

use zip::ZipArchive;

use quick_xml::Reader;
use quick_xml::events::Event;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
                $y
        }
    };
}

fn reader_done(reader: FileReader) {
    match reader.ready_state() {
        FileReaderReadyState::Empty => console!(log, "Reader empty"),
        FileReaderReadyState::Loading => console!(log, "Reader still loading"),
        FileReaderReadyState::Done => console!(log, "Reader done"),
    }

    match reader.result() {
        Some(FileReaderResult::ArrayBuffer(buffer)) => {
            let buf: Vec<u8> = buffer.into();
            let mut cursor_reader = Cursor::new(buf);

            let mut archive = ZipArchive::new(cursor_reader).expect("Cannot create zip archive");
            let mut xml_data = String::new();

            let mut c_file = archive.by_name("word/document.xml").unwrap();
            c_file.read_to_string(&mut xml_data);

            let mut xml_reader = Reader::from_str(xml_data.as_ref());
            let mut buf = Vec::new();
            let mut txt = Vec::new();

            if xml_data.len() > 0 {
                let mut to_read = false;
                loop {
                    match xml_reader.read_event(&mut buf){
                        Ok(Event::Start(ref e)) => {
                            match e.name() {
                                b"w:p" => {
                                    to_read = true;
                                    txt.push("\n\n".to_string());
                                },
                                b"w:t" => to_read = true,
                                _ => (),
                            }
                        },
                        Ok(Event::Text(e)) => {
                            if to_read {
                                txt.push(e.unescape_and_decode(&xml_reader).unwrap());
                                to_read = false;
                            }
                        },
                        Ok(Event::Eof) => break, // exits the loop when reaching end of file
                        Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
                        _ => (),
                    }
                }
            }
            console!(log, "CONTENT:");
            console!(log, "%s", txt.join(""));
            console!(log, "ENDCONTENT");
        },
        _ => {
            console!(log, "FAIL TO CAST BUFFER");
        }
    };
}

pub fn render(selector: &str) {
    stdweb::initialize();

    let input: InputElement = document().query_selector(selector).unwrap().unwrap().try_into().unwrap();
    input.add_event_listener( enclose!( (input) move |_: ChangeEvent| {
        let blob: Blob = js!( return @{&input}.files[0]; ).try_into().unwrap();
        let reader = FileReader::new();
        reader.read_as_array_buffer(&blob).expect("read_as_array_buffer failed");

        set_timeout(|| reader_done(reader), 1000);
    }));

    stdweb::event_loop();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
