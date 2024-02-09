use file::Key;
use nanoid::nanoid;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    usize,
};

mod file;

pub type Offset = u64;
pub type Size = u64;
pub type Position = (Offset, Size);
pub type Index = HashMap<String, Position>;

#[derive(Debug)]
pub struct Storage {
    index: Index,
    keys: Vec<Key>,
    buf_writer: BufWriter<File>,
    buf_reader: BufReader<File>,
}

fn create_new_log_file(path: &str) -> File {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap();
    f.write_all(format!("🪵 Awesome Log File V{VERSION};\n").as_bytes())
        .unwrap();
    f
}

pub fn init(path: &str) -> Storage {
    let keys = file::parse_log(path);

    let mut index = HashMap::new();
    for key in keys.iter() {
        index.insert(key.key.clone(), (key.pos, key.len));
    }
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .unwrap_or_else(|_| create_new_log_file(path));
    let buf_reader = BufReader::new(file.try_clone().unwrap());
    let buf_writer = BufWriter::new(file);

    Storage {
        index,
        keys,
        buf_reader,
        buf_writer,
    }
}

impl Storage {
    pub fn ordered_keys(&self) -> Vec<Key> {
        // Maybe not clone but Rc?
        self.keys.clone()
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

        let pos = self.buf_writer.seek(SeekFrom::End(0))?;
        let len = content.len() as u64;

        self.buf_writer.write_all(content)?;
        write!(self.buf_writer, "\n␜\n")?;
        self.buf_writer.flush()?;

        // TODO a lot of clones
        self.keys.push(Key {
            key: key.clone(),
            len,
            pos,
        });
        self.index.insert(key.clone(), (pos, len));

        Ok(key)
    }

    fn retrieve_by_pos_and_len(&mut self, pos: u64, len: u64) -> Option<Vec<u8>> {
        let l = len as usize;
        if self.buf_reader.seek(SeekFrom::Start(pos)).is_ok() {
            let content = (&mut self.buf_reader)
                .bytes()
                .take(l)
                .map(|b| b.unwrap())
                .collect();
            Some(content)
        } else {
            None
        }
    }

    /// Retrieve the latest element with key
    pub fn retrieve(&mut self, key: &str) -> Option<Vec<u8>> {
        let (pos, len) = self.index.get(key).unwrap();
        self.retrieve_by_pos_and_len(*pos, *len)
    }

    /// Returns the nth element in the log, not considering overwritten fields
    /// Starts at 0
    pub fn retrieve_nth(&mut self, i: usize) -> Option<(String, Vec<u8>)> {
        if let Some(Key { key, pos, len }) = self.ordered_keys().get(i) {
            if let Some(content) = self.retrieve_by_pos_and_len(*pos, *len) {
                return Some((key.clone(), content));
            }
        }
        None
    }
}
