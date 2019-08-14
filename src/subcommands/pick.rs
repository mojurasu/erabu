use colored::Colorize;

use crate::BoxResult;
use crate::collection::Collection;

pub fn pick_item(collection_name: String, no_format: bool) -> BoxResult<()> {
    let collection = Collection::new(&collection_name)?;
    if no_format {
        println!("{}", collection.pick());
    } else {
        println!("{} {}", "Picked item:".blue().bold(), collection.pick());
    };

    Ok(())
}
