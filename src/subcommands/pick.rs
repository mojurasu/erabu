use crate::collection::Collection;

pub fn pick_item(collection_name: String) -> Result<(), Box<std::error::Error>> {
    let collection = Collection::new(collection_name)?;
    println!("{}", collection.pick());
    Ok(())
}
