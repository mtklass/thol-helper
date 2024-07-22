mod object;

use std::ops::Div;
use std::ops::Mul;
use std::fs::{self, File};
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use glob::glob;
use object::{ClothingType, Object};
use serde_json::Value;

#[derive(Parser, Default)]
#[command(author, about)]
pub struct Args {
    #[arg(long)]
    clothing: Option<String>,
    #[arg(short = 'd', long, default_value = "../../TwoHoursOneLife/OneLifeData7")]
    one_life_data_directory: String,
    #[arg(short = 'o', long, default_value = "output.txt")]
    output_file: String,
    #[arg(short = 't', long, default_value = "../../TwoHoursOneLife/twotech")]
    two_tech_data_directory: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let clothing_to_match = args.clothing.unwrap_or_default().split(",").map(|c| ClothingType::from_str(c).unwrap()).collect::<Vec<_>>();
    if let Err(onelife_dir_err) = fs::read_dir(&args.one_life_data_directory) {
        println!("OneLifeData7 directory ({}) could not be opened, please provide different path via the -o option.", args.one_life_data_directory);
        return Err(anyhow!(onelife_dir_err));
    }
    if let Err(twotech_dir_err) = fs::read_dir(&args.two_tech_data_directory) {
        println!("TwoTech directory ({}) could not be opened, please provide different path via the -o option.", args.two_tech_data_directory);
        return Err(anyhow!(twotech_dir_err));
    }
    let mut two_tech_data_directory = args.two_tech_data_directory;
    if !two_tech_data_directory.ends_with("/") {
        two_tech_data_directory.push('/');
    }
    let twotech_object_directory = two_tech_data_directory.clone() + "public/static/objects/";

    let mut objects = Vec::new();

    for entry in glob(&format!("./{twotech_object_directory}/*.json")).expect("Failed to read glob pattern") {
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
            Err(e) => println!("entry error: {:?}", e),
        }
    }

    let mut output_string_lines = Vec::new();

    let mut objects = objects.iter()
        .filter(|obj| {
            obj.craftable.unwrap_or(false)
            // Specific type of clothing
            && (clothing_to_match.is_empty() ||
                (obj.clothing.is_some()
                    && clothing_to_match.contains(obj.clothing.as_ref().unwrap())
                )
            )
            // && obj.clothing.as_ref().unwrap_or(&ClothingType::None) != &ClothingType::None
            && !&obj.name.clone().unwrap_or_default().contains("removed")
        })
        .collect::<Vec<_>>();
    objects.sort_by_key(|k| k.name.clone());

    objects.iter().for_each(|obj| {
            output_string_lines.push("|-".to_string());
            output_string_lines.push(format!("|{{{{Card|{}}}}}", obj.name.clone().unwrap_or("ERROR: No name!".to_string())));
            output_string_lines.push(format!("|{:1.}%", obj.insulation.unwrap_or(0.0).mul(100.0).mul(1000000.0).round().div(1000000.0)));
            output_string_lines.push(format!("|{}", obj.numSlots.map(|n| n.to_string()).unwrap_or("N/A".to_string())));
        });
    std::fs::write(&args.output_file, output_string_lines.join("\n"))?;
    Ok(())
}
