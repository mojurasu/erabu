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
    #[structopt(name = "pick")]
    Pick {
        #[structopt(name = "collection")]
        collection: String,
    },
    #[structopt(name = "list")]
    List {
        #[structopt(name = "collection")]
        collection: String,
    },
    #[structopt(name = "del")]
    Del {
        #[structopt(name = "collection")]
        collection: String,
        #[structopt(name = "item")]
        item: String,
    },
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = Opt::from_args();
    println!("{:#?}", matches);
    match matches {
        Opt::Add { collection_name, items } => subcommands::add::add_items(collection_name, items)?,
        _ => ()
    }
    Ok(())
}
