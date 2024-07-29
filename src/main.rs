mod one_life_data_object;
mod twotech_object;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::format;
use std::io::Read;
use std::io::Write;
use std::ops::Div;
use std::ops::Mul;
use std::fs::{self, File};
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::process;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use glob::glob;
use one_life_data_object::OneLifeDataObject;
use one_life_data_object::SlotStyle;
use serde::Deserialize;
use serde::Serialize;
use twotech_object::{ClothingType, TwoTechObject};
use serde_json::Value;

const DEFAULT_OUTOUT_FILENAME: &str = "output.json";

#[derive(Parser, Default)]
#[command(
    author,
    about,
    about = r#"Filter twotech's object data for objects that interest you."#
)]
pub struct Args {
    #[arg(
        short = 'o',
        long,
        default_value = DEFAULT_OUTOUT_FILENAME,
        help = "Output file to write to. "
    )]
    output_file: String,

    #[arg(
        short = 'r',
        long,
        default_value = "false",
        help = "Refresh cached OneLifeData7 and twotech data",
    )]
    regenerate_data: bool,

    #[arg(short = 'd', long, default_value = "../../TwoHoursOneLife/OneLifeData7")]
    one_life_data_directory: String,
    #[arg(short = 't', long, default_value = "../TwoTech-ProcessOutput")]
    twotech_data_directory: String,
    // Don't worry about this...it's an unfinished option that will possibly be broken out into another program.
    // The idea is to convert an object list into table entries for a wiki page. It's too hard-coded though, and again, should also be broken out.
    #[arg(long, default_value="false")]
    wiki_table_output: bool,

// Filtering options
    #[arg(long)]
    clothing: Option<String>,
    #[arg(long, default_value = "0")]
    min_pickup_age: i32,
    #[arg(long, help = "examples: 1, 0.1..8, 0..1.5, 1..2, 4..")]
    slot_size: Option<F32Range>,
    #[arg(long, help = "examples: 1, 1000, 0..1, ..2, 4..")]
    num_slots: Option<I32Range>,
    #[arg(long)]
    // If is_food is None, no filter. If Some(), either filter for food (true), or non-food (false)
    is_food: Option<bool>,
    #[arg(long)]
    immediate_food_value: Option<I32Range>,
    #[arg(long)]
    bonus_food_value: Option<I32Range>,
    #[arg(long)]
    total_food_value: Option<I32Range>,
    #[arg(
        long,
        help = "Filter for specific ingredient(s) being present in object's recursive recipe trees (comma-separated, can use object name or ID).
Specify multiple times for logical OR across specified lists",
        value_parser = clap::value_parser!(IngredientSet),
    )]
    with_ingredients: Option<Vec<IngredientSet>>,
    #[arg(
        long,
        help = "Filter for specific ingredient(s) being present in object's recursive recipe trees (comma-separated, can use object name or ID).
    Specify multiple times for logical OR across specified lists",
        value_parser = clap::value_parser!(IngredientSet),
    )]
    without_ingredients: Option<Vec<IngredientSet>>,
    #[arg(
        long,
        help = "0 => Box, 1 => Table, 2 => Ground",
    )]
    container_slot_type: Option<Vec<SlotStyle>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameObject {
    pub one_life_game_data: Option<OneLifeDataObject>,
    pub twotech_data: Option<TwoTechObject>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SharedGameObject {
    pub one_life_game_data: OneLifeDataObject,
    pub twotech_data: TwoTechObject,
}

#[derive(Debug, Clone)]
pub struct IngredientSet(Vec<String>);

impl FromStr for IngredientSet {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IngredientSet(s.split(',').map(|s| s.to_string()).collect::<Vec<String>>()))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    // If the user specified the wiki output option, but didn't specify an output file, the defaul output.json will be misleading.
    // Warn the user and ask them to say yes to continue.
    let mut wiki_output_file_check = true;
    if args.wiki_table_output && args.output_file.as_str() == DEFAULT_OUTOUT_FILENAME {
        let err_msg = format!("Wiki output selected, but default {DEFAULT_OUTOUT_FILENAME} file still being used. To avoid confusion, perhaps specify a different value with the -o option?");
        wiki_output_file_check = pause(Some(err_msg));
    }
    if !wiki_output_file_check {
        return Ok(());
    }
    let mut clothing_to_match = Vec::new();
    if args.clothing.is_some() {
        clothing_to_match = args.clothing.unwrap_or_default().split(",").map(|c| ClothingType::from_str(c).unwrap()).collect::<Vec<_>>();
    }
    const INTERMEDIATE_FILES_DIR: &str = "intermediate-files";
    if fs::read_dir(INTERMEDIATE_FILES_DIR).is_err() {
        fs::create_dir(INTERMEDIATE_FILES_DIR)?;
    }
    const ONELIFEDATA7_OBJECT_DATA_FILE: &str = "intermediate-files/OneLifeData7_Objects.json";
    const TWOTECH_OBJECT_DATA_FILE: &str = "intermediate-files/twotech_Objects.json";
    // Force regeneration of intermediate-files
    if args.regenerate_data {
        println!("Generated data refresh triggered.");
        println!(" > Removing intermediate-files for OneLifeData7 and twotech object data.");
        fs::remove_file(ONELIFEDATA7_OBJECT_DATA_FILE).ok();
        fs::remove_file(TWOTECH_OBJECT_DATA_FILE).ok();
    }
    // Try to load intermediate-files data into OneLifeData7 object data BTreeMap
    // If the file exists but doesn't make sense, delete it
    // If it didn't make sense or it didn't exist, recreate data and save to intermediate-files/OneLifeData7_Objects.json
    // If it parsed, great!
    let initial_one_life_game_objects = if let Ok(one_life_file_data) = fs::read_to_string(ONELIFEDATA7_OBJECT_DATA_FILE) {
        serde_json::from_str::<BTreeMap<String, OneLifeDataObject>>(one_life_file_data.as_str())?
    } else {
        println!("Intermediate file for OneLifeData7 object data is not present, we must regenerate it from OneLifeData7 data.");
        if let Err(onelife_dir_err) = fs::read_dir(&args.one_life_data_directory) {
            println!("OneLifeData7 directory ({}) could not be opened, please provide different path via the -d option.", args.one_life_data_directory);
            return Err(anyhow!(onelife_dir_err));
        }
        let mut one_life_data_directory = args.one_life_data_directory;
        if !one_life_data_directory.ends_with("/") {
            one_life_data_directory.push('/');
        }
        let one_life_object_directory = one_life_data_directory + "objects/";
        let one_life_object_dir_contents = fs::read_dir(one_life_object_directory)?;
        let mut one_life_game_objects = BTreeMap::new();
        for one_life_data_entry in one_life_object_dir_contents {
            if let Ok(one_life_data_entry) = one_life_data_entry {
                // Check if the entry is a file and matches the pattern
                if let Ok(metadata) = one_life_data_entry.metadata() {
                    if metadata.is_file() {
                        let file_name = one_life_data_entry.file_name();
                        let file_name = file_name.to_string_lossy();
                        if let Some(captures) = regex::Regex::new(r"^(\d+)\.txt$").unwrap().captures(&file_name) {
                            // For debugging, only look at file we care about
                            // if captures.get(1).unwrap().as_str() != "14492" {
                            //     continue;
                            // }
                            // println!("Parsing file {file_name}");
                            // Read the file into a string
                            let object_id = match captures.get(1) {
                                Some(id) => id.as_str(),
                                None => continue,
                            };
                            let mut file = fs::File::open(one_life_data_entry.path()).unwrap();
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();
                            if let Ok(object) = OneLifeDataObject::from_str(&contents) {
                                one_life_game_objects.insert(object_id.to_string(), object);
                            } else {
                                println!("Error converting file contents to object: {}", one_life_data_entry.path().to_string_lossy());
                            }
                        }
                    }
                }
            }
        }
        println!("Parsed {} OneLifeData7 objects", one_life_game_objects.len());
        fs::write(ONELIFEDATA7_OBJECT_DATA_FILE, serde_json::to_string(&one_life_game_objects)?)?;
        one_life_game_objects
    };

    let initial_twotech_objects = if let Ok(twotech_file_data) = fs::read_to_string(TWOTECH_OBJECT_DATA_FILE) {
        serde_json::from_str::<BTreeMap<String, TwoTechObject>>(twotech_file_data.as_str())?
    } else {
        println!("Intermediate file for twotech object data is not present, we must regenerate it from twotech data.");
        if let Err(twotech_dir_err) = fs::read_dir(&args.twotech_data_directory) {
            println!("TwoTech directory ({}) could not be opened, please provide different path via the -t option.", args.twotech_data_directory);
            return Err(anyhow!(twotech_dir_err));
        }
        let mut twotech_data_directory = args.twotech_data_directory;
        if !twotech_data_directory.ends_with("/") {
            twotech_data_directory.push('/');
        }
        let twotech_object_directory = twotech_data_directory.clone() + "public/static/objects/";

        let mut twotech_objects = BTreeMap::new();
        for entry in glob(&format!("./{twotech_object_directory}/*.json")).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let file = File::open(&path).expect("Unable to open file");
                    let reader = BufReader::new(file);
                    let json: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");
                    let json_string = serde_json::to_string(&json)?;
                    let object_data: TwoTechObject = serde_json::from_str(&json_string).expect(&format!("JSON:\n{}", serde_json::to_string_pretty(&json)?));
                    if let Some (id) = object_data.id.clone() {
                        twotech_objects.insert(id, object_data);
                    }
                }
                Err(e) => println!("entry error: {:?}", e),
            }
        }
        println!("Parsed {} twotech objects", twotech_objects.len());
        fs::write(TWOTECH_OBJECT_DATA_FILE, serde_json::to_string(&twotech_objects)?)?;
        twotech_objects
    };

    let mut initial_shared_objects = BTreeMap::new();
    for (key, onelifedata_obj) in &initial_one_life_game_objects {
        if let Some(twotech_obj) = initial_twotech_objects.get(key) {
            initial_shared_objects.insert(key.to_owned(), SharedGameObject {
                one_life_game_data: onelifedata_obj.to_owned(),
                twotech_data: twotech_obj.to_owned(),
            });
        } else {
            // println!("OneLifeData7 contained ID {}, name {}, but twotech did not!", onelifedata_obj.id, onelifedata_obj.name);
        }
    }
    for (key, twotech_obj) in &initial_twotech_objects {
        if !initial_shared_objects.contains_key(key) {
            if let Some(onelifedata_obj) = initial_one_life_game_objects.get(key) {
                initial_shared_objects.insert(key.to_owned(), SharedGameObject {
                    one_life_game_data: onelifedata_obj.to_owned(),
                    twotech_data: twotech_obj.to_owned(),
                });
            } else {
                // println!("OneLifeData7 contained ID {}, name {}, but twotech did not!", twotech_obj.id.as_ref().unwrap_or(&"ERR: NO ID".to_string()), twotech_obj.name.as_ref().unwrap_or(&"ERR: NO NAME".to_string()));
            }
        }
    }

    let mut shared_objects = initial_shared_objects.clone();

    // Prepare ingredient sets to exclude based on user input
    let ingredient_sets_to_exclude = args.without_ingredients
        .map(|ingredient_sets| {
            ingredient_sets.into_iter()
            .map(|ingredient_set| {
                // Convert ingredient set into an option that either has the converted ingredient name into an ID, the ID as-provided, or None
                ingredient_set.0
                .into_iter()
                .filter_map(|ingredient| {
                    if ingredient.parse::<i32>().is_ok() {
                        Some(ingredient)
                    } else {
                        initial_shared_objects
                        .iter()
                        .map(|(_, obj)| &obj.twotech_data)
                        .find(|o| o.name.as_ref().is_some_and(|n| n == &ingredient))
                        .map(|o| o.id.clone())
                        .flatten()
                    }
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        });

    // Prepare ingredient sets to find based on user input
    let ingredient_sets_to_find = args.with_ingredients
        // Act on args.needs_ingredients if it is present
        .map(|ingredient_sets| {
            // Iterate through all the sets of ingredients the user provided (via separately defined option calls)
            ingredient_sets.into_iter()
            .map(|ingredient_set| {
                // Convert ingredient set into an option that either has the converted ingredient name into an ID, the ID as-provided, or None
                ingredient_set.0
                .into_iter()
                .filter_map(|ingredient| {
                    if ingredient.parse::<i32>().is_ok() {
                        Some(ingredient)
                    } else {
                        initial_shared_objects
                        .iter()
                        .map(|(_, obj)| &obj.twotech_data)
                        .find(|o| o.name.as_ref().is_some_and(|n| n == &ingredient))
                        .map(|o| o.id.clone())
                        .flatten()
                    }
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        });

    // numSlots filter. Default is all values > 0
    let num_slots_filter = args.num_slots
        .clone()
        .unwrap_or(I32Range(RangeInclusive::new(0, i32::MAX)))
        .0;

    // slotSize filter. Default is all values
    let slot_size_filter = args.slot_size
        .clone()
        .unwrap_or(F32Range(RangeInclusive::new(f32::MIN, f32::MAX)))
        .0;

    shared_objects = shared_objects.into_iter()
        .filter(|(_, shared_obj)| {
            let onelifedata_obj = &shared_obj.one_life_game_data;
            let twotech_obj = &shared_obj.twotech_data;
            twotech_obj.craftable.unwrap_or(false)
            // Specific type of clothing
            && (clothing_to_match.is_empty() ||
                (twotech_obj.clothing.is_some()
                    && clothing_to_match.contains(twotech_obj.clothing.as_ref().unwrap())
                )
            )
            // Is over minimum pickup age filter (0 if not specified)
            && twotech_obj.minPickupAge.unwrap_or(0) >= args.min_pickup_age
            // Number of slots for item falls within specified range (default is all positive values)
            && num_slots_filter.contains(&twotech_obj.numSlots.unwrap_or(0))
            // slotSize is for item falls within specified range (default is all values allowed)
            && slot_size_filter.contains(&twotech_obj.slotSize.unwrap_or(f32::MIN))
            // User either wants to filter for items being food or not food, or args.is_food will be None
            && (
                args.is_food.is_none() ||
                twotech_obj.foodValue.as_ref().is_some_and(|f| f.len() > 0) == args.is_food.unwrap()
            )
            // Total food supplied by the item, including immediate food and bonus
            && (
                args.total_food_value.is_none() ||
                twotech_obj.foodValue.as_ref().is_some_and(|f| {
                    let food_value_filter = args.total_food_value
                        .clone()
                        .unwrap() // Okay to do because we're in the else if an is_none()
                        .0;
                    food_value_filter.contains(&f.into_iter().sum())
                })
            )
            // Immediate food supplied by the item
            && (
                args.immediate_food_value.is_none() ||
                twotech_obj.foodValue.as_ref().is_some_and(|f| {
                    let food_value_filter = args.immediate_food_value
                        .clone()
                        .unwrap()
                        .0;
                    food_value_filter.contains(&f.into_iter().sum())
                })
            )
            // Bonus food supplied by the item
            && (
                args.bonus_food_value.is_none() ||
                twotech_obj.foodValue.as_ref().is_some_and(|f| {
                    let food_value_filter = args.bonus_food_value
                        .clone()
                        .unwrap()
                        .0;
                    food_value_filter.contains(&f.into_iter().sum())
                })
            )
            && (
                args.container_slot_type.is_none() ||
                onelifedata_obj.slotStyle.as_ref().is_some_and(|ss| args.container_slot_type.as_ref().unwrap().contains(&ss))
            )
            // object isn't marked as removed
            && !&twotech_obj.name.clone().unwrap_or_default().contains("removed")
            // Don't actually do this filter, but it shows using OneLifeData7 data to filter
            // && onelifedata_obj.containable.is_some_and(|v| v)
        })
        .collect::<BTreeMap<_,_>>();

    // Filter for objects that contain any set of other object IDs in its recipe (recursively)
    if let Some(ingredient_sets_to_find) = ingredient_sets_to_find {
        shared_objects = shared_objects.into_iter()
            .filter(|(_, obj)| {
                // Instead of just looking for the one target ID, we need to look for the all the values in each set.
                // If any set has all its values matched, we have a match!
                let mut found_match = false;
                for ingredient_set in &ingredient_sets_to_find {
                    // All ingredients must be present in recipe (actual check is "no ingredients may not be present")
                    let ingredient_set_matches = !ingredient_set
                        .iter()
                        // Take each ID string and map it to a bool saying whether the object has this ID has an ingredient
                        .map(|i| find_target_ingredient(obj, i.as_str(), &initial_shared_objects))
                        .collect::<Vec<_>>()
                        .contains(&None);

                    if ingredient_set_matches {
                        // Immediately exit if we've found a match
                        found_match = true;
                        break;
                    }
                }
                found_match
            })
            .collect::<BTreeMap<_,_>>();
    }

    // Filter for objects that DO NOT contain any set of other object IDs in its recipe (recursively)
    // Item must not include ANY of these sets of ingredients in its recipe tree
    if let Some(ingredient_sets_to_exclude) = ingredient_sets_to_exclude {
        shared_objects = shared_objects.into_iter()
            .filter(|(_, obj)| {
                let mut an_ingredient_set_matches = false;
                // If any ingredient set is present, we've found a match
                for ingredient_set in &ingredient_sets_to_exclude {
                    // All ingredients must be present for ingredient set to be a match
                    let missing_an_ingredient = ingredient_set
                        .iter()
                        // Take each ID string and map it to a bool saying whether the object has this ID has an ingredient
                        .map(|i| find_target_ingredient(obj, i.as_str(), &initial_shared_objects))
                        .collect::<Vec<_>>()
                        .contains(&None);
                    if !missing_an_ingredient {
                        an_ingredient_set_matches = true;
                        break;
                    }
                }
                // We only want to keep objects that don't contain any of the ingredient sets in the query
                !an_ingredient_set_matches
            })
            .collect::<BTreeMap<_,_>>();
    }

    // Finally, sort the objects by their name, since it's the most human-friendly ordering
    shared_objects = shared_objects
        .into_values()
        .filter(|v| { v.twotech_data.name.is_some() })
        .map(|v| (v.twotech_data.name.clone().unwrap(), v))
        .collect::<BTreeMap<_,_>>();

    if args.wiki_table_output {
        let wiki_output_data =
        shared_objects
            .iter()
            .map(|(_, obj)| {
                _wiki_format_line_food(obj)
            })
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&args.output_file, wiki_output_data)?;
    } else {
        // Serialize the object list to JSON and save to the output file location
        let objects_as_string = serde_json::to_string(&shared_objects)?;
        std::fs::write(&args.output_file, objects_as_string)?;
    }
    println!("Wrote {} matching objects' data to output file at {}", shared_objects.len(), args.output_file);
    Ok(())
}

fn _wiki_format_card_template_object_id(obj: &SharedGameObject) -> String {
    let mut output = Vec::new();
    let id = obj.twotech_data.id.as_ref().unwrap();
    let name = obj.twotech_data.name.as_ref().unwrap();
    // if name.contains(" - ") {
    //     let name_portion = name.split(" - ").collect::<Vec<_>>()[0];
    //     output.push(format!("| {name_portion} = https://twotech.twohoursonelife.com/{id}"))
    // }
    output.push(format!("| {name} = https://twotech.twohoursonelife.com/{id}"));
    output.join("\n")
}

fn _wiki_format_line_slot_item(obj: &SharedGameObject) -> String {
    format!("|-
|{{{{Card|{}}}}}
|{}
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        obj.twotech_data.numSlots.map(|n| n.to_string()).unwrap_or("0".to_string()),
        obj.twotech_data.slotSize.map(|n| n.to_string()).unwrap_or("0".to_string()),
    )
}

fn _wiki_format_line_food(obj: &SharedGameObject) -> String {
    let food_value = obj.twotech_data.foodValue.clone().unwrap_or(vec![0,0]);
    format!("|-
|{{{{Card|{}}}}}
|{}
|{}
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        food_value[0].to_string(),
        food_value[1].to_string(),
        food_value.iter().sum::<i32>()
    )
}

fn _wiki_format_line_clothing_with_slots(obj: &SharedGameObject) -> String {
    format!("|-
|{{{{Card|{}}}}}
|{:1.}%
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        obj.twotech_data.insulation.unwrap_or(0.0).mul(100.0).mul(1000000.0).round().div(1000000.0),
        obj.twotech_data.numSlots.map(|n| n.to_string()).unwrap_or("0".to_string())
    )
}

fn find_target_ingredient<'a>(root_obj: &'a SharedGameObject, target_id: &str, object_database: &'a BTreeMap<String, SharedGameObject>) -> Option<&'a SharedGameObject> {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push(root_obj);
    // println!("Searching for ({:>5}, {}) in ({:>5}, {})",
    //     target_id,
    //     object_database.get(target_id).map_or("", |_, o| o.name.as_ref().map(|s| s.as_str()).unwrap_or("")),
    //     root_obj.id.as_ref().map_or("", |s| s.as_str()),
    //     root_obj.name.as_ref().map_or("", |s| s.as_str()),
    // );
    while let Some(obj) = stack.pop() {
        // If object has no ID or has already been visited, skip it
        let obj_id = obj.twotech_data.id.clone().unwrap_or_default();
        if obj_id.is_empty() || visited.contains(&obj_id) {
            continue;
        }
        // If current object is the ID we're looking for, return true!
        if obj_id.as_str() == target_id {
            return Some(obj);
        }
        // println!("New Total: {} after adding object ID to visited: {obj_id}", visited.len());
        visited.insert(obj_id);

        let obj_recipe = match &obj.twotech_data.recipe {
            Some(recipe) => recipe,
            None => continue,
        };

        // Check each ingredient for being the target_id, and if we haven't yet visited the ingredient, push it to the list
        if let Some(ingredients) = obj_recipe.ingredients.as_ref().map(|ivec| HashSet::<&String>::from_iter(ivec.iter())) {
            for ingredient in ingredients {
                if ingredient.as_str() == target_id {
                    return Some(obj);
                }
                if !visited.contains(ingredient) {
                    // if let Some(ingredient_object) = get_object_by_id(&ingredient, object_database) {
                    if let Some(ingredient_object) = object_database.get(ingredient) {
                        stack.push(ingredient_object);
                    }
                }
            }
        }

        // Push onto our stack all the unique values in the object recipe that we haven't yet visited
        obj_recipe.steps
        .as_ref()
        .unwrap_or(&Vec::default())
        .into_iter()
        .flatten()
        .flat_map(|rs| [rs.actorID.clone().unwrap_or_default(), rs.targetID.clone().unwrap_or_default()])
        .filter(|ingredient| !visited.contains(ingredient))
        .filter_map(|ingredient| object_database.get(&ingredient))
        .for_each(|recipe_ingredient_object| stack.push(recipe_ingredient_object));

        if obj.twotech_data.recipe.as_ref().is_some_and(|r|
            r.ingredients.as_ref()
            .map(|ingredients| HashSet::<&String>::from_iter(ingredients.iter()))
            .is_some_and(|ingredients| {
                // Now we have a list of all the unique items in the ingredients list.
                // We need to check if any of them are the target_id
                // We then need to check if we've already visited each ingredient, and if not, push it to the stack
                for ingredient in ingredients {
                    // println!("Checking ingredient {ingredient}");
                    if ingredient.as_str() == target_id {
                        println!("Ingredient {ingredient} matched!");
                        return true;
                    }
                    if !visited.contains(ingredient) {
                        // if let Some(ingredient_object) = get_object_by_id(&ingredient, object_database) {
                        if let Some(ingredient_object) = object_database.get(ingredient) {
                            stack.push(ingredient_object);
                        } else {
                            println!("Oh crud");
                            process::exit(-10);
                        }
                    }
                }
                // If none of the ingredients contained the target_id, return false
                return false;
            })
        ) {
            return Some(obj);
        }
    }
    return None;
}

fn pause(message: Option<String>) -> bool {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    // If the caller defined an initial message to send to the user, print it out.
    if let Some(message) = message {
        println!("{}", message);
    }

    // Always print out the default message, explaining how to continue or exit.
    let default_message = "Type y or yes and ENTER to continue, anything else to exit: ";
    stdout.write(default_message.as_bytes()).unwrap();
    stdout.flush().unwrap();

    // We want to save the string entered by the user.
    let mut stdin_data = String::new();

    // Look at what the user typed and only return true if they types "y" or "yes", where any/all letters can be uppercase or lowercase
    let _bytes_read = stdin.read_line(&mut stdin_data).unwrap();
    let trimmed_stdin_data = stdin_data.trim();
    if trimmed_stdin_data.len() > 0 {
        let stdin_data = trimmed_stdin_data.to_lowercase();
        let stdin_data_str = stdin_data.as_str();
        if stdin_data_str == "y" || stdin_data_str == "yes" {
            return true;
        }
    }
    return false;
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

#[derive(Clone, Debug)]
pub struct F32Range(RangeInclusive<f32>);

impl FromStr for F32Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").collect();
        match parts.len() {
            1 => {
                let start: f32 = parts[0].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))?;
                Ok(F32Range(start..=start))
            },
            2 => {
                let start: f32 = if parts[0].is_empty() { 0.0 } else { parts[0].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))? };
                let end: f32 = if parts[1].is_empty() { f32::MAX } else { parts[1].parse().map_err(|_| "Invalid number").map_err(|e| anyhow!(e))? };
                Ok(F32Range(start..=end))
            },
            _ => Err(anyhow!("Invalid range format")),
        }
    }
}
