use fs_objstore;

fn main() -> Result<(), std::io::Error> {
    let path = "example_stream.db";

    println!("[INFO] Writing elements to {path}");
    println!("[INFO] Reading elements to {path}");

    let mut storage = fs_objstore::init(path);
    let mut read_storage = fs_objstore::read(path);

    let messages = vec!["A", "B", "C\nD\n\n"];

    let mut keys = vec![];
    for m in messages.iter() {
        let k = storage.persist(None, m.as_bytes()).unwrap();
        keys.push(k);
    }

    // needs to parse the new keys
    for key in read_storage.ordered_keys().iter() {
        let el = storage.retrieve(&key.key).unwrap();
        println!("[INFO] Element: \n{}", String::from_utf8_lossy(&el));
    }

    Ok(())
}
