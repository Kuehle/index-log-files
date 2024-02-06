use nanoid::nanoid;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    usize,
};

mod file;

// This is the Unix philosophy: Write programs that do one thing and do it well. Write programs to work together. Write programs to handle text streams, because that is a universal interface.

// Magic numbers at the beginning. Pretty much required in *nix:
// File version number for backwards compatibility.
// Endianness specification.
// Block structure?
// Checksums?
// Version of your software that wrote the file
// Make clear that it is a binary format - 0-255 allowed except for magic numbers
// Allow markers to skip things (forward compatibility)
//   so your current software can skip them
//
// Check out msgpack - interesting but not really helpful
//
// Random or sequential access?
// Read vs Write - how often?
// Write in one go or as data comes in?
//
// Reserve some space for future developments?

// would be cool to have file IDs at well defined intervals (ie 64kB blocks)

// consume magic byte

// spend some time designing the file format and the priorities
//
//
//
// Easy to append
// Easy to parse
// Low overhead
// Stream directly from file into other stream
// Log - but able to quickly extract one entry from it
//
// not making progress because it is to generic
//   do one thing, and do it well?
//
// log with keyd messages
// timestamp allows to do a binary search and truncate based on persistence duration
// 0xMagic_Begin[key] [timestamp]0xMagic_End
//
// create stream.log.index file with keys
//   do I need keys in the file?
//   is magic enough?
//   do I want to align to full multiples of n-bytes or something?
//
//
// allow segmentation into multiple files by default?

pub type Key = String;
pub type Offset = u64;
pub type Size = u64;
pub type Position = (Offset, Size);
pub type Index = HashMap<Key, Position>;

#[derive(Debug)]
pub struct Storage {
    pub index: Index,
    // last_write: u64,
    path: String, // for infor :shrug:
    buf_writer: BufWriter<File>,
    buf_reader: BufReader<File>,
}

fn create_new_log_file(path: &str) -> File {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap();
    f.write_all("🪵 Awesome Log File V0.0.1;\n".as_bytes())
        .unwrap();
    f
}

// TODO allow to insert and retrieve through streams
pub fn init(path: &str) -> Storage {
    let index = file::parse_log(path);
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .unwrap_or_else(|_| create_new_log_file(path));
    let buf_reader = BufReader::new(file.try_clone().unwrap());
    let buf_writer = BufWriter::new(file);

    Storage {
        index,
        // last_write: 0,
        path: path.to_string(),
        buf_reader,
        buf_writer,
    }
}

// Persists content either under given key or random key. Returns key.
// Content can be any binary blob. Double newline needs to be escaped.
impl Storage {
    pub fn keys(&self) -> Vec<&String> {
        self.index.keys().collect()
    }
    // TODO don't propagate io error types to user
    pub fn persist(
        &mut self,
        key: Option<String>,
        content: &[u8],
    ) -> Result<String, std::io::Error> {
        let key = key.unwrap_or_else(|| nanoid!());

        let last_write = self.buf_writer.seek(SeekFrom::Current(0))?;
        // TODO add timestamps?
        self.buf_writer.write_all("🔑".as_bytes())?;
        self.buf_writer.write_all(key.as_bytes())?;
        self.buf_writer.write_all("\n".as_bytes())?;
        self.buf_writer.write_all(content)?;
        write!(self.buf_writer, "\n\n")?;
        self.buf_writer.flush()?;

        self.index
            .insert(key.clone(), (last_write, content.len() as u64));

        // self.last_write = self.buf_writer.seek(SeekFrom::Current(0))?;
        Ok(key)
    }

    // TODO fix indexes
    pub fn retreive(&mut self, key: &str) -> Result<Vec<u8>, std::io::Error> {
        let (start, length) = self.index.get(key).unwrap();
        let l = *length as usize;
        self.buf_reader.seek(SeekFrom::Start(
            (start + key.len() as u64 + 1).try_into().unwrap(),
        ))?; // there is a new line
             // after the key - hence + 1

        let content = (&mut self.buf_reader)
            .bytes()
            .take(l)
            .map(|b| b.unwrap())
            .collect();

        Ok(content)
    }
}
