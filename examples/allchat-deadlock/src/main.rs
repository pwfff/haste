use haste::{
    parser::{self, Context, Parser, Visitor},
    protos::{self, prost::Message},
};
use std::{fs::File, io::BufReader};

struct MyVisitor;

impl Visitor for MyVisitor {
    fn on_packet(&mut self, _ctx: &Context, packet_type: u32, data: &[u8]) -> parser::Result<()> {
        if packet_type == protos::ECitadelClientMessages::CitadelCmChatMsg as u32 {
            let msg = protos::CCitadelClientMsgChatMsg::decode(data)?;
            println!("{:?}", msg);
        }
        Ok(())
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1);
    if filepath.is_none() {
        eprintln!("usage: allchat <filepath>");
        std::process::exit(42);
    }

    let file = File::open(filepath.unwrap())?;
    let buf_reader = BufReader::new(file);
    let mut parser = Parser::from_reader_with_visitor(buf_reader, MyVisitor)?;
    parser.run_to_end()
}
