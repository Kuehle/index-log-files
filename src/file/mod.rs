use nom::{
    bytes::complete::{tag, take_till, take_until},
    multi::many0,
    IResult,
};
use nom_locate;
use nom_locate::{position, LocatedSpan};

use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Msg {
    payload: Vec<u8>,
}

type Span<'a> = LocatedSpan<&'a [u8]>;

#[derive(Debug)]
struct Log {
    key: String,
    blob: Vec<u8>,
}

#[derive(Debug)]
struct Key {
    key: String,
    pos: u64,
    len: u64,
}

#[derive(Debug)]
struct File {
    header: String, // just a single line, separated by semicolon
    logs: Vec<Log>,
}

fn nl(s: Span) -> IResult<Span, Span> {
    take_till(|b| b == 0x0A)(s)
}

fn key(s: Span) -> IResult<Span, String> {
    let (s, _) = tag([0xf0, 0x9f, 0x94, 0x91])(s)?;
    let (s, val) = nl(s)?;
    let (s, _) = tag("\n")(s)?;

    Ok((s, String::from_utf8_lossy(*val.fragment()).to_string()))
}

fn blob(s: Span) -> IResult<Span, Vec<u8>> {
    let (s, blob) = take_until("\n\n")(s)?;
    let (s, _) = tag("\n\n")(s)?;
    Ok((s, blob.to_vec()))
}

fn log_no_contents(s: Span) -> IResult<Span, Key> {
    let (s, key) = key(s)?;
    let (s, pos) = position(s)?;
    let (s, _) = blob(s)?;
    let (s, pos_after) = position(s)?;

    let pos: u64 = pos.location_offset().try_into().unwrap();
    let len: u64 = (pos_after.location_offset() as u64 - pos - 2)
        .try_into()
        .unwrap();

    Ok((s, Key { key, pos, len }))
}

fn logs_no_contents(s: Span) -> IResult<Span, Vec<Key>> {
    many0(log_no_contents)(s)
}

fn log(s: Span) -> IResult<Span, Log> {
    let (s, key) = key(s)?;
    let (s, blob) = blob(s)?;

    Ok((s, Log { key, blob }))
}

fn logs(s: Span) -> IResult<Span, Vec<Log>> {
    many0(log)(s)
}

fn header(s: Span) -> IResult<Span, String> {
    let (s, _) = tag([0xf0, 0x9f, 0xaa, 0xb5])(s)?;
    let (s, val) = nl(s)?;
    let (s, _) = tag("\n")(s)?;

    Ok((s, String::from_utf8_lossy(val.fragment()).to_string()))
}

fn file(s: Span) -> IResult<Span, File> {
    let (s, header) = header(s)?;
    let (s, logs) = logs(s)?;

    Ok((s, File { header, logs }))
}

// Swallow error to not mess up lifetimes
fn file_no_contents(s: Span) -> Option<Vec<Key>> {
    if let Ok((s, _)) = header(s) {
        if let Ok((_, logs)) = logs_no_contents(s) {
            return Some(logs);
        }
    }
    None
}

// Hydrate the index
pub fn parse_log(file_name: &str) -> HashMap<String, (u64, u64)> {
    let mut result = HashMap::new();

    if let Ok(content) = fs::read_to_string(file_name) {
        if let Some(keys) = file_no_contents(Span::new(content.as_bytes())) {
            for key in keys.iter() {
                result.insert(key.key.clone(), (key.pos, key.len));
            }
        }
    }

    result
}

// TODO add parser that skips blobs
// TODO add timestamps?

// TODO create test
pub fn bar() {
    // let input = vec![0x90, 0xFF, 0x90, 0xFF, 0x00, 0xFF, 0x90];
    let input = r#"🪵File Header!
🔑asdfasdf
Hello World

🔑qwerqwer
12345678
9

🔑ABCDABCD
This is a test
of a 
multi line
blob

"#;
    println!("{:?}", file_no_contents(Span::new(input.as_bytes())));
    println!("slice: {}", &input[81..(81 + 36)]);
}
