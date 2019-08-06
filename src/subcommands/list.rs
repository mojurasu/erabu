use crate::collection::{Collection, get_collections};

pub fn list_items(collection_name: String) -> Result<(), Box<std::error::Error>> {
    let collection = Collection::new(collection_name)?;
    for item in collection.items {
        println!("{}", item);
    }
    Ok(())
}

pub fn list_collections() -> Result<(), Box<std::error::Error>> {
    let collections = get_collections()?;
    for col in collections {
        println!("{}", col);
    }
    Ok(())
}
