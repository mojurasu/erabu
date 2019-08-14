use colored::Colorize;

use crate::BoxResult;
use crate::collection::Collection;

pub fn add_items(collection_name: String, items: Vec<String>) -> BoxResult<()> {
    println!("{} {}", "Added items:".blue().bold(), items.join(", "));
    Collection::new(&collection_name)?
        .add(items)
        .save()?;
    Ok(())
}
