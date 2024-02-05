use nom::{
    bytes::complete::{is_not, tag, take_till, take_until, take_while1, take_while_m_n},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
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
    input != 144
}

fn msg(input: &[u8]) -> IResult<&[u8], Msg> {
    let (input, _) = consume_magic(input)?;
    let (input, payload) = take_while1(is_not_magic)(input)?;
    let payload = payload.to_vec();

    Ok((input, Msg { payload }))
}

fn msgs(input: &[u8]) -> IResult<&[u8], Vec<Msg>> {
    many0(msg)(input)
}

pub fn bar() {
    let input = vec![0x90, 0x90, 0xFF, 0x00, 0xFF, 0x90];
    println!("{:?}", msgs(&input));
}
