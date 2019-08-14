use colored::Colorize;

use crate::BoxResult;
use crate::collection::{Collection, get_collections};

pub fn list_items(collection_name: String) -> BoxResult<()> {
    let collection = Collection::new(&collection_name)?;
    println!("{} {}", "Collection:".blue().bold(), collection_name);
    println!("{:>11} {}", "Items:".blue().bold(), collection.items.join(", "));
    Ok(())
}

pub fn list_collections() -> BoxResult<()> {
    let collections = get_collections()?;
    println!("{} {}", "Collections:".blue().bold(), collections.join(", "));
    Ok(())
}
