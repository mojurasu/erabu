use std::fs::File;
use std::path::PathBuf;

use dirs::home_dir;
use serde_json::Value;

#[derive(Debug)]
pub struct Collection {
    name: String,
    items: Vec<String>,
}

fn collection_file() -> Result<PathBuf, Box<std::error::Error>> {
    let filename = &format!(".{}_collections", &env!("CARGO_PKG_NAME"));
    let filepath: PathBuf = match home_dir() {
        Some(path) => [path, PathBuf::from(filename)].iter().collect(),
        None => {
            PathBuf::from(filename)
        }
    };
    Ok(filepath)
}

impl Collection {
    pub fn new(name: String) -> Result<Collection, Box<std::error::Error>> {
        let collection_file = collection_file()?;
        let name = name.to_lowercase();
        if !collection_file.exists() {
            Ok(Collection {
                name,
                items: vec![],
            })
        } else {
            let file = File::open(collection_file)?;
            let mut items: Value = serde_json::from_reader(file)?;

            let empty = vec![];
            let items: Vec<Value> = match items.get(&name) {
                Some(v) => match v.as_array() {
                    Some(v) => v.to_vec(),
                    None => empty
                },
                None => empty
            };
            let items = items.iter()
                .map(|item| item.as_str().unwrap().to_string())
                .collect::<Vec<String>>();

            Ok(Collection { name, items})
        }
    }
}
