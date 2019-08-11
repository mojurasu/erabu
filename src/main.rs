use structopt::StructOpt;

mod subcommands;
mod collection;

#[derive(StructOpt, Debug)]
#[structopt(name = "erabu")]
enum Opt {
    #[structopt(name = "add", about = "Add an item to a collection")]
    Add {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(name = "items", help = "A list of items to add")]
        items: Vec<String>,
    },

    #[structopt(name = "del", about = "Delete an item from a collection")]
    Del {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(name = "items", help = "A list of items to remove")]
        items: Vec<String>,
    },

    #[structopt(name = "pick", about = "Pick a random item from a collection")]
    Pick {
        #[structopt(name = "collection", help = "The name of the collection")]
        collection_name: String,
        #[structopt(long = "no-format", help = "Don't format the output")]
        no_format: bool,
    },

    #[structopt(name = "list", about = "List all collections or the items of a collection")]
    List {
        #[structopt(name = "collection", help = "The name of the collection, Optional")]
        collection_name: Option<String>,
    },
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = Opt::from_args();
    match matches {
        Opt::Add { collection_name, items } => subcommands::add::add_items(collection_name, items)?,
        Opt::Del { collection_name, items } => subcommands::del::del_items(collection_name, items)?,
        Opt::Pick { collection_name, no_format } => subcommands::pick::pick_item(collection_name, no_format)?,
        Opt::List { collection_name } => match collection_name {
            Some(name) => subcommands::list::list_items(name)?,
            None => subcommands::list::list_collections()?
        },
    }
    Ok(())
}
