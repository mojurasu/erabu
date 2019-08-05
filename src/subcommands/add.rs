use crate::collection::Collection;

pub fn add_items(collection_name: String, items: Vec<String>) -> Result<(), Box<std::error::Error>> {
    Collection::new(collection_name)?
        .add(items)
        .save()?;
    Ok(())
}
