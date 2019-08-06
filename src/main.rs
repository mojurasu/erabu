use structopt::StructOpt;

mod subcommands;
mod collection;

#[derive(StructOpt, Debug)]
#[structopt(name = "erabu")]
enum Opt {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "collection")]
        collection_name: String,
        #[structopt(name = "items")]
        items: Vec<String>,
    },

    #[structopt(name = "del")]
    Del {
        #[structopt(name = "collection")]
        collection_name: String,
        #[structopt(name = "items")]
        items: Vec<String>,
    },

    #[structopt(name = "pick")]
    Pick {
        #[structopt(name = "collection")]
        collection_name: String,
    },

    #[structopt(name = "list")]
    List {
        #[structopt(name = "collection")]
        collection_name: Option<String>,
    },
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = Opt::from_args();
    match matches {
        Opt::Add { collection_name, items } => subcommands::add::add_items(collection_name, items)?,
        Opt::Del { collection_name, items } => subcommands::del::del_items(collection_name, items)?,
        Opt::Pick { collection_name } => subcommands::pick::pick_item(collection_name)?,
        Opt::List { collection_name } => match collection_name {
            Some(name) => subcommands::list::list_items(name)?,
            None => subcommands::list::list_collections()?
        },
    }
    Ok(())
}
