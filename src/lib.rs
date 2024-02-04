use nanoid::nanoid;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

pub type Key = String;
pub type Offset = u64;
pub type Size = u64;
pub type Position = (Offset, Size);
pub type Index = HashMap<Key, Position>;

#[derive(Debug)]
pub struct Storage {
    index: Index,
    last_write: u64,
    path: String, // for infor :shrug:
    buf_writer: BufWriter<File>,
    buf_reader: BufReader<File>,
}

// TODO allow to insert and retrieve through streams
// TODO https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#access-a-file-randomly-using-a-memory-map
pub fn init(path: &str) -> Storage {
    let index: Index = HashMap::new();
    // TODO should first build the index
    //      or read it from the file if possible
    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .read(true)
        .open(path)
        .unwrap_or_else(|_| File::create(path).unwrap());
    let buf_reader = BufReader::new(file.try_clone().unwrap());
    let buf_writer = BufWriter::new(file);

    Storage {
        index,
        last_write: 0,
        path: path.to_string(),
        buf_reader,
        buf_writer,
    }
}

// Persists content either under given key or random key. Returns key.
// Content can be any binary blob. Double newline needs to be escaped.
impl Storage {
    // TODO don't propagate io error types to user
    pub fn persist(
        &mut self,
        key: Option<String>,
        content: &[u8],
    ) -> Result<String, std::io::Error> {
        let key = key.unwrap_or_else(|| nanoid!());

        // TODO check if key exists
        //      and return error that user can deal with
        self.buf_writer.write_all(key.as_bytes())?;
        self.buf_writer.write_all("\n".as_bytes())?;
        self.buf_writer.write_all(content)?;
        write!(self.buf_writer, "\n\n")?;
        self.buf_writer.flush()?;

        self.index
            .insert(key.clone(), (self.last_write, content.len() as u64));

        // TODO update index
        self.last_write = self.buf_writer.seek(SeekFrom::Current(0))?;
        Ok(key)
    }

    pub fn retreive(&mut self, key: &str) -> Result<Vec<u8>, std::io::Error> {
        let (start, length) = self.index.get(key).unwrap();
        let l = *length as usize;
        // is it better to seek from current position if this is shorter?
        self.buf_reader
            .seek(SeekFrom::Start(start + key.len() as u64 + 1))?; // there is a new line
                                                                   // after the key - hence + 1

        // create a new buffered reader?
        let content = (&mut self.buf_reader)
            .bytes()
            .take(l)
            .map(|b| b.unwrap())
            .collect();

        // index starts right before key
        Ok(content)
    }
}
