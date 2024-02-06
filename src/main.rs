use fs_objstore;

// TODO move stuff to examples
// TODO add bin that is just the api wrapped in a cli?
// TODO build coordinator / load balancer on top
// TODO benchmark
// TODO refactor the code a bit
// TODO release? / use in other project
// TODO build a simple msg queue
// TODO build a blog

fn main() -> Result<(), std::io::Error> {
    let mut storage = fs_objstore::init("stream.db");

    let messages = vec!["Hello World", "This is awesome", "What\ncan we do now?"];

    let mut keys = vec![];
    for m in messages.iter() {
        let k = storage.persist(None, m.as_bytes()).unwrap();
        keys.push(k);
    }

    for key in storage.keys().iter() {
        let el = storage.retreive(key).unwrap();
        println!("[INFO] Element: {}", String::from_utf8_lossy(&el));
    }

    Ok(())
}
