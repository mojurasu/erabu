use structopt::StructOpt;

mod subcommands;
mod collection;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "erabu")]
enum Opt {
    #[structopt(name = "add", about = "Add an item to a collection")]
    Add {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(name = "items", help = "A list of items to add")]
        items: Vec<String>,
        #[structopt(long = "file", default_value = "~/.erabu_collections", help = "The file where the collections are stored")]
        file: String,
    },

    #[structopt(name = "del", about = "Delete an item from a collection")]
    Del {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(name = "items", help = "A list of items to remove")]
        items: Vec<String>,
        #[structopt(short = "f", long = "force", help = "Don't ask for confirmation")]
        force: bool,
        #[structopt(long = "file", default_value = "~/.erabu_collections", help = "The file where the collections are stored")]
        file: String,
    },

    #[structopt(name = "pick", about = "Pick a random item from a collection")]
    Pick {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(long = "no-format", help = "Don't format the output")]
        no_format: bool,
        #[structopt(long = "file", default_value = "~/.erabu_collections", help = "The file where the collections are stored")]
        file: String,
    },

    #[structopt(name = "list", about = "List all collections or the items of a collection")]
    List {
        #[structopt(name = "collection", help = "The name of the collection, Optional")]
        collection_name: Option<String>,
        #[structopt(long = "file", default_value = "~/.erabu_collections", help = "The file where the collections are stored")]
        file: String,
    },
}

fn main() -> BoxResult<()> {
    let matches: Opt = Opt::from_args();
    match matches {
        Opt::Add { collection_name, items, file } => subcommands::add::add_items(collection_name, items, file)?,
        Opt::Del { collection_name, items, force, file } => subcommands::del::del_items(collection_name, items, force, file)?,
        Opt::Pick { collection_name, no_format, file } => subcommands::pick::pick_item(collection_name, no_format, file)?,
        Opt::List { collection_name, file } => match collection_name {
            Some(name) => subcommands::list::list_items(name, file)?,
            None => subcommands::list::list_collections(file)?
        },
    }
    Ok(())
}
