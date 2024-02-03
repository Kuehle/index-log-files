use nanoid::nanoid;

use fs_objstore;

// TODO write index into a new file
// TODO combine with stream package
// TODO enable dead letter queue by just saving the indecies of unprocessed messages
// TODO learn about memory mapped file mode
// TODO benchmark against opening file handles and reading
// TODO learn how to build databases -> https://cstack.github.io/db_tutorial/

fn main() -> Result<(), std::io::Error> {
    let storage = fs_objstore::init("stream.db");

    let messages = vec!["Hello World", "This is awesome", "What\ncan we do now?"];

    for m in messages {
        let bytes = m.as_bytes();
        let len = bytes.len() as u64;
        let key = nanoid!();
        let end_of_message_pos = storage.write_msg(key.as_bytes(), bytes)?;
    }

    Ok(())
}
