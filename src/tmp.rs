use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Branch {
    chain_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn create_xml_reader() -> Result<Reader<BufReader<File>>, quick_xml::Error> {
    let path = "./data/file.xml";
    let reader = Reader::from_file(path);
    return reader;
}

fn iterate_xml(mut reader: Reader<BufReader<File>>) {
    let mut ctx = HashMap::<String, String>::new();
    let mut buf = Vec::<u8>::new();
    let mut key = "";
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => println!("Error while parsing: {:?}", e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(start)) => {
                let tag_name = start.name();
                match tag_name.as_ref() {
                    b"ChainId" => key = "chain_id".as_ref(),
                    b"SubChainId" => key = "sub_chain_id".as_ref(),
                    b"StoreId" => key = "store_id".as_ref(),
                    b"BikoretNo" => key = "bikoret_no".as_ref(),
                    b"DllVerNo" => key = "dll_ver".as_ref(),
                    b"PriceUpdateDate" => key = "price_update".as_ref(),
                    b"ItemCode" => key = "item_code".as_ref(),
                    b"ItemType" => key = "item_type".as_ref(),
                    b"ItemName" => key = "item_name".as_ref(),
                    _ => key = "",
                }
            },
            Ok(Event::Text(text)) => {
                if key != "" {
                    println!("text {:?} --- key {:?}", text.unescape().unwrap().to_owned(), key);
                }
            },
            Ok(_) => {},
        }
        buf.clear();
    }
}

fn main() {
    let result = create_xml_reader();
    match result {
        Err(e) => println!("{:?}", e),
        Ok(reader) => {
            iterate_xml(reader)
        },
    }
}