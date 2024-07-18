mod object;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use clap::Parser;
use glob::glob;
use object::{ClothingType, Object};
use serde_json::Value;

#[derive(Parser, Default)]
#[command(author, about)]
pub struct Args {
    #[arg(short = 'd', long, default_value = ".")]
    data_directory: String,
}

fn main() -> Result<()> {
    // let mut unique_fields = HashSet::new();
    // Read each object txt file in the provided directory, and attempt to parse it.
    // Do if == on the original file data and the FromStr->ToString chain output.
    let args = Args::parse();
    // Log errors, but keep going.
    let mut data_directory = args.data_directory;
    if !data_directory.ends_with("/") {
        data_directory.push('/');
    }
    let object_directory = data_directory.clone() + "objects/";

    let mut objects = Vec::new();

    for entry in glob(&format!("./{object_directory}/*.json")).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file = File::open(&path).expect("Unable to open file");
                println!("Reading {}", path.to_str().unwrap());
                let reader = BufReader::new(file);
                let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");

                let json_string = serde_json::to_string_pretty(&json)?;

                let object_data: Object = serde_json::from_str(&json_string).expect(&format!("JSON:\n{}", json_string));
                objects.push(object_data);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    objects.iter()
        .filter(|obj| obj.craftable.unwrap_or(false)
            && obj.clothing == Some(ClothingType::Top)
        )
        .for_each(|obj| {
            
        });

    Ok(())
}
