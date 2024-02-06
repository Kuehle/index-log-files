use nanoid::nanoid;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    usize,
};

mod file;

pub type Key = String;
pub type Offset = u64;
pub type Size = u64;
pub type Position = (Offset, Size);
pub type Index = HashMap<Key, Position>;

#[derive(Debug)]
pub struct Storage {
    pub index: Index,
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
        buf_reader,
        buf_writer,
    }
}

impl Storage {
    pub fn keys(&self) -> Vec<String> {
        self.index.keys().map(|s| s.to_string()).collect()
    }

    pub fn persist(
        &mut self,
        key: Option<String>,
        content: &[u8],
    ) -> Result<String, std::io::Error> {
        let key = key.unwrap_or_else(|| nanoid!());

        self.buf_writer.seek(SeekFrom::End(0))?;
        self.buf_writer.write_all("🔑".as_bytes())?;
        self.buf_writer.write_all(key.as_bytes())?;
        self.buf_writer.write_all("\n".as_bytes())?;

        let start = self.buf_writer.seek(SeekFrom::End(0))?;

        self.buf_writer.write_all(content)?;
        write!(self.buf_writer, "\n\n")?;
        self.buf_writer.flush()?;

        self.index
            .insert(key.clone(), (start, content.len() as u64));

        Ok(key)
    }

    pub fn retreive(&mut self, key: &str) -> Result<Vec<u8>, std::io::Error> {
        let (start, length) = self.index.get(key).unwrap();
        let l = *length as usize;
        self.buf_reader.seek(SeekFrom::Start(*start))?;

        let content = (&mut self.buf_reader)
            .bytes()
            .take(l)
            .map(|b| b.unwrap())
            .collect();

        Ok(content)
    }
}
