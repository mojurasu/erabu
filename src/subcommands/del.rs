use colored::Colorize;

use crate::collection::Collection;

pub fn del_items(collection_name: String, items: Vec<String>) -> Result<(), Box<std::error::Error>> {
    println!("{} {}", "Removed items:".red().bold(), items.join(", "));
    Collection::new(&collection_name)?
        .remove(items)
        .save()?;
    Ok(())
}
