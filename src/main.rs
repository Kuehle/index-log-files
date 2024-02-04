use fs_objstore;

// TODO write index into a new file
// TODO combine with stream package
// TODO enable dead letter queue by just saving the indecies of unprocessed messages
// TODO learn about memory mapped file mode
// TODO benchmark against opening file handles and reading
// TODO learn how to build databases -> https://cstack.github.io/db_tutorial/
// TODO move stuff to examples
// TODO add bin that is just the api?
// TODO build coordinator / load balancer on top

fn main() -> Result<(), std::io::Error> {
    let mut storage = fs_objstore::init("stream.db");

    let messages = vec!["Hello World", "This is awesome", "What\ncan we do now?"];

    let mut keys = vec![];
    for m in messages.iter() {
        let k = storage.persist(None, m.as_bytes()).unwrap();
        keys.push(k);
    }

    // TODO get all the keys
    //      this should involve building the index from scratch?
    for k in keys.iter() {
        let c = storage.retreive(k).unwrap();
        println!("{k}: {}", String::from_utf8(c).unwrap());
    }

    Ok(())
}
