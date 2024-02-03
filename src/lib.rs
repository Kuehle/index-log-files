use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
};

pub type Key = String;
pub type Offset = u64;
pub type Size = u64;
pub type Position = (Offset, Size);
pub type Index = HashMap<Key, Position>;

#[derive(Debug)]
pub struct Storage {
    index: Index,
    path: String, // for infor :shrug:
    buf_writer: BufWriter<File>,
    buf_reader: BufReader<File>,
}

// TODO https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#access-a-file-randomly-using-a-memory-map
pub fn init(path: &str) -> Storage {
    let index: Index = HashMap::new();
    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(path)
        .unwrap_or_else(|_| File::create(path).unwrap());
    let buf_reader = BufReader::new(file.try_clone().unwrap());
    let buf_writer = BufWriter::new(file);

    Storage {
        index,
        path: path.to_string(),
        buf_reader,
        buf_writer,
    }
}

// don't make it thread save
// allow routing to take place out of naive storage
// maybe not new but rather open -> creates new connection
impl Storage {
    pub fn write_msg(mut self, key: &[u8], content: &[u8]) -> Result<u64, std::io::Error> {
        self.buf_writer.write_all(key)?;
        self.buf_writer.write_all("\n".as_bytes())?;
        self.buf_writer.write_all(content)?;
        write!(self.buf_writer, "\n\n")?;
        self.buf_writer.flush()?;
        self.buf_writer.seek(SeekFrom::Current(0))
    }
}
