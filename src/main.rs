mod object;

use std::ops::Div;
use std::{collections::HashSet, ops::Mul};
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
                // println!("Reading {}", path.to_str().unwrap());
                let reader = BufReader::new(file);
                let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");

                let json_string = serde_json::to_string(&json)?;

                let object_data: Object = serde_json::from_str(&json_string).expect(&format!("JSON:\n{}", serde_json::to_string_pretty(&json)?));
                objects.push(object_data);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    let mut output_string_lines = Vec::new();

    let mut objects = objects.iter()
        .filter(|obj| 
            obj.craftable.unwrap_or(false)
            && obj.clothing == Some(ClothingType::Shoe)
            && !&obj.name.clone().unwrap_or_default().contains("removed")
        )
        .collect::<Vec<_>>();
    objects.sort_by_key(|k| k.name.clone());

    objects.iter().for_each(|obj| {
            output_string_lines.push("|-".to_string());
            output_string_lines.push(format!("|{{{{Card|{}}}}}", obj.name.clone().unwrap_or("ERROR: No name!".to_string())));
            output_string_lines.push(format!("|{:1.}%", obj.insulation.unwrap_or(0.0).mul(100.0).mul(1000000.0).round().div(1000000.0)));
        });
    std::fs::write("output-Shoe.txt", output_string_lines.join("\n"))?;
    Ok(())
}
