use colored::Colorize;

use crate::BoxResult;
use crate::collection::Collection;

pub fn add_items(collection_name: String, items: Vec<String>, file: String) -> BoxResult<()> {
    println!("{} {}", "Added items:".blue().bold(), items.join(", "));
    Collection::new(&collection_name, file)?
        .add(items)
        .save()?;
    Ok(())
}
