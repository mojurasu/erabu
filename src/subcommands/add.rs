use crate::collection::Collection;

pub fn add_items(collection_name: String, items: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let collection = Collection::new(collection_name);
    println!("{:#?}", collection);
    Ok(())
}
