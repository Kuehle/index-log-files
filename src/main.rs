use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Seek, SeekFrom, Write},
};

use nanoid::nanoid;

type Key = String;
type Offset = u64;
type Size = u64;
type Position = (Offset, Size);
type Index = HashMap<Key, Position>;

fn main() -> Result<(), std::io::Error> {
    // whenever encounters a double new line
    // append to file
    // and add the offset to an index (also in a file?)
    // preallocate memory
    // use serializer to ensure no actual (double) line breaks occur in a stream
    // could also be an efficient object storage if combined with gzip and http
    // would need occasinal defragmentation

    // file handle
    let mut index: Index = HashMap::new();
    let path = "stream.db";
    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(path)
        .unwrap_or_else(|_| File::create(path).unwrap());
    let mut buf_writer = BufWriter::new(file);

    let messages = vec!["Hello World", "This is awesome", "What\ncan we do now?"];

    // insert
    // how to get offset?
    // needs std::io::Cursor that has a Seek
    // could pass the index as well and then see what the last pos was..
    //
    //
    // how to create the keys?
    // autogenerate?
    for m in messages {
        let bytes = m.as_bytes();
        let len = bytes.len() as u64;
        let key = nanoid!();
        let end_of_message_pos = write_msg(&mut buf_writer, key.as_bytes(), bytes)?;
        index.insert(key, (end_of_message_pos - len - 2, len));
    }
    // being flushed before file is closed?
    // maybe flush more often?
    //
    println!("[INFO] Index: {index:?}");

    Ok(())
}

fn write_msg<W: Write + Seek>(
    writer: &mut W,
    key: &[u8],
    content: &[u8],
) -> Result<u64, std::io::Error> {
    writer.write_all(key)?;
    writer.write_all("\n".as_bytes())?;
    writer.write_all(content)?;
    write!(writer, "\n\n")?;

    writer.seek(SeekFrom::Current(0))
}
