mod object;

use std::ops::Div;
use std::ops::Mul;
use std::fs::{self, File};
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use glob::glob;
use object::{ClothingType, Object};
use serde_json::Value;

#[derive(Parser, Default)]
#[command(author, about)]
pub struct Args {
    #[arg(short = 'd', long, default_value = "../../TwoHoursOneLife/OneLifeData7")]
    one_life_data_directory: String,
    #[arg(short = 'o', long, default_value = "output.txt")]
    output_file: String,
    #[arg(short = 't', long, default_value = "../../TwoHoursOneLife/twotech")]
    two_tech_data_directory: String,
    // Don't worry about this...it's an unfinished option that will possibly be broken out into another program.
    // The idea is to convert an object list into table entries for a wiki page. It's too hard-coded though, and again, should also be broken out.
    #[arg(long, default_value="false")]
    wiki_table_output: bool,

// Filtering options
    #[arg(long)]
    clothing: Option<String>,
    #[arg(long, default_value = "0")]
    min_pickup_age: i32,
    #[arg(long, default_value = "0")]
    min_slot_size: f32,
    #[arg(long, help = "examples: 1, 1000, 0..1, ..2, 4..")]
    num_slots: Option<I32Range>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut clothing_to_match = Vec::new();
    if args.clothing.is_some() {
        clothing_to_match = args.clothing.unwrap_or_default().split(",").map(|c| ClothingType::from_str(c).unwrap()).collect::<Vec<_>>();
    }
    if let Err(onelife_dir_err) = fs::read_dir(&args.one_life_data_directory) {
        println!("OneLifeData7 directory ({}) could not be opened, please provide different path via the -d option.", args.one_life_data_directory);
        return Err(anyhow!(onelife_dir_err));
    }
    if let Err(twotech_dir_err) = fs::read_dir(&args.two_tech_data_directory) {
        println!("TwoTech directory ({}) could not be opened, please provide different path via the -t option.", args.two_tech_data_directory);
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

    let num_slots_filter = args.num_slots
        .clone()
        .unwrap_or(I32Range(RangeInclusive::new(0, i32::MAX)))
        .0;

    let mut objects = objects.iter()
        .filter(|obj| {
            obj.craftable.unwrap_or(false)
            // Specific type of clothing
            && (clothing_to_match.is_empty() ||
                (obj.clothing.is_some()
                    && clothing_to_match.contains(obj.clothing.as_ref().unwrap())
                )
            )
            && obj.minPickupAge.unwrap_or(0) >= args.min_pickup_age
            && num_slots_filter.contains(&obj.numSlots.unwrap_or(0))
            && obj.slotSize.unwrap_or_default() > args.min_slot_size
            && !&obj.name.clone().unwrap_or_default().contains("removed")
        })
        .collect::<Vec<_>>();
    objects.sort_by_key(|k| k.name.clone());

    if args.wiki_table_output {
        let wiki_output_data =
        objects
            .iter()
            .map(|obj| {
                format!("|-
|{{{{Card|{}}}}}
|{:1.}%
|{}",
                    obj.name.clone().unwrap_or("ERROR: No name!".to_string()),
                    obj.insulation.unwrap_or(0.0).mul(100.0).mul(1000000.0).round().div(1000000.0),
                    obj.numSlots.map(|n| n.to_string()).unwrap_or("N/A".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&args.output_file, wiki_output_data)?;
    } else {
        // Serialize the object list to JSON and save to the output file location
        let objects_as_string = serde_json::to_string(&objects)?;
        std::fs::write(&args.output_file, objects_as_string)?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct I32Range(RangeInclusive<i32>);

impl FromStr for I32Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").collect();
        match parts.len() {
            1 => {
                let start: i32 = parts[0].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))?;
                Ok(I32Range(start..=start))
            },
            2 => {
                let start: i32 = if parts[0].is_empty() { 0 } else { parts[0].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))? };
                let end: i32 = if parts[1].is_empty() { i32::MAX } else { parts[1].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))? };
                Ok(I32Range(start..=end))
            },
            _ => Err(anyhow!("Invalid range format")),
        }
    }
}
