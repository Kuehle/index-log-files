use nom::{
    bytes::complete::{is_not, tag, take_till, take_until, take_while, take_while_m_n},
    combinator::map_res,
    multi::{many0, many1},
    sequence::tuple,
    AsBytes, AsChar, IResult,
};

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    // Consumes # leaving the hex codes but would throw if it didn't exist
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

// How to consume a magic byte?
// Stream friendly file format

pub fn foo() {
    println!("{:?}", hex_color("#2F12AD"));
}

#[derive(Debug)]
pub struct Msg {
    payload: Vec<u8>,
}

// take all the magics
// then take while no magic
// then return many
// parse a single one
// m....
// build a parser that runs many
// m....m....m....
fn consume_magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag([0x90])(input)
}

fn is_not_magic(input: u8) -> bool {
    input != 0x90
}

fn msg(input: &[u8]) -> IResult<&[u8], Msg> {
    let (input, _) = consume_magic(input)?;
    let (input, payload) = take_while(is_not_magic)(input)?;
    let payload = payload.to_vec();
    Ok((input, Msg { payload }))
}

fn msgs(input: &[u8]) -> IResult<&[u8], Vec<Msg>> {
    many1(msg)(input)
}

// should I move to pure binary?
// readable logs make sense
// you should still be able to put in blobs..
// treat everything as u8
// but still assume that certain portions are plain text
//
// should I create a magic byte?

// key 0x0a messages 0x0a 0x0a
#[derive(Debug)]
struct Log {
    key: String,
    blob: Vec<u8>,
}

#[derive(Debug)]
struct File {
    header: String, // just a single line, separated by semicolon
    logs: Vec<Log>,
}

fn nl(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|b| b == 0x0A)(input)
}

fn key(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = tag([0xf0, 0x9f, 0x94, 0x91])(input)?;
    let (input, val) = nl(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, String::from_utf8_lossy(val).to_string()))
}

fn blob(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (input, blob) = take_until("\n\n")(input)?;
    let (input, _) = tag("\n\n")(input)?;
    Ok((input, blob.to_vec()))
}

fn log_no_contents(input: &[u8]) -> IResult<&[u8], String> {
    let (input, key) = key(input)?;
    let (input, _) = blob(input)?;

    Ok((input, key))
}

fn logs_no_contents(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    many0(log_no_contents)(input)
}

fn log(input: &[u8]) -> IResult<&[u8], Log> {
    let (input, key) = key(input)?;
    let (input, blob) = blob(input)?;

    Ok((input, Log { key, blob }))
}

fn logs(input: &[u8]) -> IResult<&[u8], Vec<Log>> {
    many0(log)(input)
}

fn header(input: &[u8]) -> IResult<&[u8], String> {
    // Magic Bytes
    let (input, _) = tag([0xf0, 0x9f, 0xaa, 0xb5])(input)?;
    let (input, val) = nl(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, String::from_utf8_lossy(val).to_string()))
}

// TODO add character positions during parsing (&[u8], pos) tupel or something
fn file(input: &[u8]) -> IResult<&[u8], File> {
    let (input, header) = header(input)?;
    let (input, logs) = logs(input)?;

    Ok((input, File { header, logs }))
}

// Only returns keys
fn file_no_contents(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let (input, _) = header(input)?;
    let (input, logs) = logs_no_contents(input)?;

    Ok((input, logs))
}

// TODO add parser that skips blobs
// TODO add timestamps?

pub fn bar() {
    // let input = vec![0x90, 0xFF, 0x90, 0xFF, 0x00, 0xFF, 0x90];
    let input = r#"🪵File Header!
🔑asdfasdf
Hello World

🔑ABCDABCD
This is a test
of a 
multi line
blob

"#;
    // println!("{:?}", msg(&input));
    // println!("{:?}", msgs(&input));
    let f = file(&input.as_bytes()).unwrap().1;
    println!(
        "{:?}",
        f.logs
            .iter()
            .map(|l| String::from_utf8(l.blob.clone()).unwrap())
            .collect::<Vec<String>>()
    );
    println!("{:?}", file_no_contents(&input.as_bytes()).unwrap().1);
}
