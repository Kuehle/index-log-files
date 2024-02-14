use clap::{arg, Parser, Subcommand};
use std::io::{Read, Write};

// TODO LSM tree - multiple levels
//      memory -> hd?
//      External Memory Algorithms and Data Structures
// TODO streaming so not entire file needs to be read
// TODO getNext by key - (lazy) iterator (streams)
// TODO write readme
// TODO add tests
// TODO benchmark
// TODO refactor the code a bit
// TODO release? / use in other project
// TODO write defragmentation / cleaning
// TODO add retention / TTL
// TODO allow to get by index even after cleaning up old entries
// TODO delete entry (overwrite key & overwrite content with 0x90)
// TODO use different parsing mode for cli with faster upstart time
// TODO build server version
// TODO build coordinator / load balancer on top
// TODO build a simple msg queue
// TODO build a blog
// TODO Perf https://nnethercote.github.io/perf-book
// TODO consider different allocator?

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
    Persist {
        /// Payload to persist - reads from stdin otherwise
        #[arg(index = 1)]
        content: Option<String>,

        /// Key for future retrieval - creates one at random otherwise
        #[arg(long, short)]
        key: Option<String>,
    },
    Retrieve {
        #[arg(index = 1)]
        key: String,
    },
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    println!("{args:?}");

    match args.command {
        Some(cmd) => {
            let mut storage = fs_objstore::init(&args.file);
            match cmd {
                Commands::Persist {
                    content: Some(c),
                    key,
                } => {
                    let key = storage.persist(key, c.as_bytes())?;
                    std::io::stdout().write_fmt(format_args!("{key}"))?;
                }
                Commands::Persist { content: None, key } => {
                    let mut content = String::new();
                    std::io::stdin().read_to_string(&mut content)?;
                    let key = storage.persist(key, content.as_bytes())?;
                    std::io::stdout().write_fmt(format_args!("{key}"))?;
                }
                Commands::Retrieve { key } => {
                    std::io::stdout().write_all(&storage.retrieve(&key).unwrap())?;
                    // Should crash
                    // so it does not write to STDOUT but STDERR instead for better piping
                }
            }
        }
        None => {
            println!("Needs a command, try --help"); // TODO use Clap
        }
    }

    Ok(())
}
