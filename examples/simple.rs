use fs_objstore;

fn main() -> Result<(), std::io::Error> {
    let path = "stream.db";

    println!("[INFO] Writing elements to {path}");

    let mut storage = fs_objstore::init(path);

    let messages = vec!["A", "B", "C\nD\n\n"];

    let mut keys = vec![];
    for m in messages.iter() {
        let k = storage.persist(None, m.as_bytes()).unwrap();
        keys.push(k);
    }

    for key in storage.keys().iter() {
        let el = storage.retreive(key).unwrap();
        println!("[INFO] Element: \n{}", String::from_utf8_lossy(&el));
    }

    Ok(())
}
