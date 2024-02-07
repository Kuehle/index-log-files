use clap::{arg, Parser, Subcommand};
use fs_objstore;

// TODO add bin that is just the api wrapped in a cli?
// TODO move stuff to examples
// TODO add tests
// TODO build coordinator / load balancer on top
// TODO benchmark
// TODO refactor the code a bit
// TODO release? / use in other project
// TODO build a simple msg queue
// TODO build a blog
// TODO write readme
// TODO write defragmentation / cleaning
// TODO delete entry (overwrite key & overwrite content with 0x90)

fn example() -> Result<(), std::io::Error> {
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Db file
    #[arg(index = 1)]
    file: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Insert {
        #[arg(index = 1)]
        content: String,
    },
    Retrieve {
        #[arg(index = 1)]
        key: String,
    },
}

// TODO use different parsing mode for cli with faster upstart time
// TODO build server version
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    println!("{args:?}");

    match args.command {
        Some(Commands::Retrieve { key }) => {
            println!("CMD: ret {key:?}");
        }
        Some(Commands::Insert { content }) => {
            println!("CMD: content {content:?}");
        }
        None => {
            println!("CMD: none :(");
        }
    }

    Ok(())
}
