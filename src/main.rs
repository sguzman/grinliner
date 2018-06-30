#[macro_use]
extern crate html5ever;
extern crate reqwest;

use std::iter::repeat;
use std::default::Default;
use std::string::String;

use html5ever::parse_document;
use html5ever::rcdom::{NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;

fn walk(indent: usize, handle: Handle) {
    let node = handle;
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document
        => println!("#Document"),

        NodeData::Doctype { ref name, ref public_id, ref system_id }
        => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { ref contents }
        => println!("#text: {}", escape_default(&contents.borrow())),

        NodeData::Comment { ref contents }
        => println!("#Comment: {}", contents),

        NodeData::Element { ref name, ref attrs, .. } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }

    for child in node.children.borrow().iter() {
        walk(indent+4, child.clone());
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}


static URL: &str = "http://example.org/";

fn main() {
    let mut body = reqwest::get(URL).unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut body)
        .unwrap();

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.into_iter() {
            println!("    {}", err);
        }

        return;
    }

    let handle = dom.document;
    let indent = 0;

    walk(indent, handle);
}