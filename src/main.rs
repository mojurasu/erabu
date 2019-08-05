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
        collection_name: String,
    },
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = Opt::from_args();
    println!("{:#?}", matches);
    match matches {
        Opt::Add { collection_name, items } => subcommands::add::add_items(collection_name, items)?,
        Opt::Del { collection_name, items } => subcommands::del::del_items(collection_name, items)?,
        Opt::Pick { collection_name } => subcommands::pick::pick_item(collection_name)?,
        _ => ()
    }
    Ok(())
}
