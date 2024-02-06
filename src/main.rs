use fs_objstore::{self, Storage};

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

    println!("INDEX HYDRATED: {:?}", storage.index);

    let messages = vec!["Hello World", "This is awesome", "What\ncan we do now?"];

    let mut keys = vec![];
    for m in messages.iter() {
        let k = storage.persist(None, m.as_bytes()).unwrap();
        keys.push(k);
    }

    println!("INDEX AFTER INSERTION: {:?}", storage.index);

    // for k in keys.iter() {
    //     let c = storage.retreive(k).unwrap();
    //     // this threw because the indices are off
    //     println!("{k}: {}", String::from_utf8_lossy(&c));
    // }
    // let keys = storage.keys();
    // println!("{keys:?}");
    //
    // INDEX AFTER INSERTION: {"gMCSzUcOiljdG3FNoNN2L": (0, 11), "Q5v1eDaBNx7QpMeNXOYu8": (69, 15), "tlnBU0frVhqMAJBFTuFbf": (112, 19)}
    // INDEX HYDRATED: {"Q5v1eDaBNx7QpMeNXOYu8": (95, 15), "tlnBU0frVhqMAJBFTuFbf": (138, 19), "gMCSzUcOiljdG3FNoNN2L": (56, 11)}
    //
    //
    //
    //
INDEX AFTER INSERTION: {"-y1GwJZp0mTBTzZ0GPnpV": (112, 19), "W4OuJnbxgYQASZDPyrWmp": (0, 11), "_1miNONeMTZFbBDSdg2Cn": 
(69, 15)}
INDEX HYDRATED: {"_1miNONeMTZFbBDSdg2Cn": (95, 15), "-y1GwJZp0mTBTzZ0GPnpV": (138, 19), "W4OuJnbxgYQASZDPyrWmp": (56, 1
1)}



// TODO something messed up with the file index (first line file header)


INDEX AFTER INSERTION: {"lG20ucmfSipDM6EEtfpeG": (30, 11), "pVpZsoUgtnTpxgQkSICZD": (112, 19), "jtqfQKOjHhomo2DzdPj0j":
 (69, 15)}
INDEX HYDRATED: {"lG20ucmfSipDM6EEtfpeG": (56, 11), "pVpZsoUgtnTpxgQkSICZD": (138, 19), "jtqfQKOjHhomo2DzdPj0j": (95, 1
5)}

    Ok(())
}
