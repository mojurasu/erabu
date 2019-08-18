use colored::Colorize;
use dialoguer::Confirmation;

use crate::BoxResult;
use crate::collection::Collection;

pub fn del_items(collection_name: String, items: Vec<String>, force: bool, file: String) -> BoxResult<()> {
    if !items.is_empty() {
        println!("{} {}", "Removed items:".red().bold(), items.join(", "));
        Collection::new(&collection_name, file)?
            .remove(items)
            .save()?;
    } else {
         let conf = if !force {
            Confirmation::new()
                .with_text(format!("{} {} ?", "Remove the collection", &collection_name.red().bold()).as_str())
                .default(false)
                .interact()?
        } else {
            force
        };

        if conf {
            Collection::new(&collection_name, file)?
                .delete()?;
            println!("{} {}", "Removed collection:".red().bold(), &collection_name);
        }
    };
    Ok(())
}
