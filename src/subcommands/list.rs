use colored::Colorize;

use crate::BoxResult;
use crate::collection::{Collection, get_collections};

pub fn list_items(collection_name: String, file: String) -> BoxResult<()> {
    let collection = Collection::new(&collection_name, file)?;
    println!("{} {}", "Collection:".blue().bold(), collection_name);
    println!("{:>11} {}", "Items:".blue().bold(), collection.items.join(", "));
    Ok(())
}

pub fn list_collections(file: String) -> BoxResult<()> {
    let collections = get_collections(file)?;
    println!("{} {}", "Collections:".blue().bold(), collections.join(", "));
    Ok(())
}
