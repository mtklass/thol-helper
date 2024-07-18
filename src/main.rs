mod object;

use std::{clone, fs, io::Read, str::FromStr};

use anyhow::Result;
use clap::Parser;

use object::{ClothingType, Object};

#[derive(Parser, Default)]
#[command(author, about)]
pub struct Args {
    #[arg(short = 'd', long, default_value = ".")]
    data_directory: String,
}

fn main() -> Result<()> {
    // Read each object txt file in the provided directory, and attempt to parse it.
    // Do if == on the original file data and the FromStr->ToString chain output.
    let args = Args::parse();
    // Log errors, but keep going.
    let mut data_directory = args.data_directory;
    if !data_directory.ends_with("/") {
        data_directory.push('/');
    }
    let object_directory = data_directory.clone() + "objects/";
    let object_dir_contents = fs::read_dir(object_directory)?;
    let mut game_objects = Vec::new();

    for entry in object_dir_contents {
        if let Ok(entry) = entry {
            // Check if the entry is a file and matches the pattern
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_name = entry.file_name();
                    let file_name = file_name.to_string_lossy();

                    if let Some(_captures) = regex::Regex::new(r"^(\d+)\.txt$").unwrap().captures(&file_name) {
                        // For debugging, only look at file we care about
                        // if captures.get(1).unwrap().as_str() != "14492" {
                        //     continue;
                        // }
                        // println!("Parsing file {file_name}");
                        // Read the file into a string
                        let mut file = fs::File::open(entry.path()).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();
                        let object = Object::from_str(&contents)?;
                        let recreated_string = object.to_string();
                        let contents = contents
                            .trim()
                            .chars()
                            .filter(|c| c.is_ascii_graphic() || *c == '\n' || *c == ' ').collect::<String>();
                        let recreated_string = recreated_string
                            .trim()
                            .chars()
                            .filter(|c| c.is_ascii_graphic() || *c == '\n' || *c == ' ').collect::<String>();
                        if contents != recreated_string {
                            println!("For file {}, original and recreated file contents differ!", file_name);
                            let recreated_object = Object::from_str(&recreated_string)?;
                            if object == recreated_object {
                                println!("However, the objects created but each string are identical!");
                                for diff in diff::lines(&contents, &recreated_string) {
                                    match diff {
                                        diff::Result::Left(l)    => println!("-{}", l),
                                        diff::Result::Both(l, _) => println!(" {}", l),
                                        diff::Result::Right(r)   => println!("+{}", r)
                                    }
                                }
                                println!("");
                            }
                        }

                        game_objects.push(object);
                    }
                }
            }
        }
    }
    println!("Parsed {} game object files", game_objects.len());

    // Filter out all known items we don't want.
    // TODO: How do we figure out which things are "UNCRAFTABLE"?
    // Answer to the above: We probably have to do the transitions parsing first
    let game_objects = game_objects
        .into_iter()
        .filter(|obj| {
            !obj.name.contains("outdated")
            && !obj.name.contains("@")
            && !obj.name.contains("#")
        }).collect::<Vec<_>>();
    println!("Have {} game object files after filtering default unwanted entries", game_objects.len());

    // Now we have a list of all "good" game objects. We can filter them how we want to make some more useful subsets.
    // We can also, in the future, add transition file parsing, and then connect the two.
    // This can allow for really powerful queries regarding recipes, but could get complicated quickly.
    let transitions_directory = data_directory.clone() + "transitions/";
    let transition_files = fs::read_dir(&transitions_directory)?
        .filter_map(|f| {
            let entry_type = f.as_ref().map(|d| d.file_type());
            if entry_type.is_ok() && entry_type.unwrap().unwrap().is_file() {
                f.unwrap().file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();




    // Filter out what we want. This is an example where we are looking for Head-type clothing items
    let game_objects = game_objects.into_iter().filter(|obj| {
        // obj.clothing.as_ref().map(|c| c.clothing == ClothingType::Head).unwrap_or(false)
        obj.clothing.as_ref().map(|c| c.clothing == ClothingType::Top).unwrap_or(false)
    }).collect::<Vec<_>>();




    // Go through all the transitions and make sure at least one has the object ID as an output of the transition
    // The other objects are uncraftable, so we should filter them out.
    // let game_objects = game_objects.into_iter().filter(|obj| {
    //     // TODO: Need to compare the numbers in the file name with the numbers in the file.
    //     // If the object in question is in both the name and contents of the file, that's no bueno for crafting.
    //     for transition_file in &transition_files {
    //         let transition_filename_no_ext = &transition_file[0..transition_file.find(".txt").unwrap_or(transition_file.len())];
    //         let transition_filename_parts = transition_filename_no_ext.split('_').collect::<Vec<_>>();
    //         if transition_filename_parts.len() < 2 {
    //             println!("Transition filename \"{transition_file}\" does not contain enough data for us to use. Skipping...");
    //             continue;
    //         }
    //         let transition_input_1 = transition_filename_parts[0].parse::<i32>().unwrap_or(0);
    //         let transition_input_2 = transition_filename_parts[1].parse::<i32>().unwrap_or(0);
    //         let transition_file_contents = fs::read_to_string(transitions_directory.clone() + transition_file).unwrap_or(String::new());
    //         if transition_file_contents.is_empty() { 
    //             println!("transition file \"{transition_file}\" was empty or unreadable. Skipping...");
    //             continue;
    //         }
    //         let transition_file_content_parts = transition_file_contents.split(" ").collect::<Vec<_>>();
    //         if transition_file_content_parts.len() < 2 {
    //             println!("Transition file \"{transition_file}\" does not contain enough data for us to use. Skipping...");
    //             continue;
    //         }
    //         let transition_output_1 = transition_file_content_parts[0].parse::<i32>().unwrap_or(0);
    //         let transition_output_2 = transition_file_content_parts[1].parse::<i32>().unwrap_or(0);
    //         if transition_output_1 == obj.id || transition_output_2 == obj.id {
    //             if transition_input_1 == obj.id || transition_input_2 == obj.id {
    //                 return false;
    //             } else {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }).collect::<Vec<_>>();

    // For now, we'll just ask TwoTech whether this is uncraftable or not. This is ugly, and we'll want to fix our detection to work properly.
    let game_objects = game_objects.into_iter().filter(|obj| {
        let mut resp_buf = String::new();
        reqwest::blocking::get(&format!("https://twotech.twohoursonelife.com/{}", obj.id))
            .map_or(false, |mut resp| {
                resp.read_to_string(&mut resp_buf).ok();
                if !resp_buf.is_empty() {
                    println!("resp_buf = {resp_buf}");
                }
                if resp_buf.contains("UNCRAFTABLE") {
                    println!("Item \"{}\" is UNCRAFTABLE according to twotech!", obj.name);
                }
                !resp_buf.contains("UNCRAFTABLE")
            })
    }).collect::<Vec<_>>();

    // Go through all remaining game objects and filter out those that have a corresponding category file
    let categories_directory = data_directory.clone() + "categories/";
    let categories_files = fs::read_dir(&categories_directory)?
        .filter_map(|f| {
            let entry_type = f.as_ref().map(|d| d.file_type());
            if entry_type.is_ok() && entry_type.unwrap().unwrap().is_file() {
                f.unwrap().file_name().into_string().map(|s| s.trim().to_string()).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut game_objects = game_objects.into_iter().filter(|obj| {
        if categories_files.contains(&format!("{}.txt", obj.id)) {
            println!("Found obj.id {} in categories", obj.id);
        }
        !categories_files.contains(&format!("{}.txt", obj.id))
    }).collect::<Vec<_>>();

    println!("Query matched {} craftable game objects", game_objects.len());
    
    // Now we sort the objects by name (could do id)
    // TODO: Make sorting method a CLI option
    game_objects.sort_by_key(|k| k.name.clone());

    let mut output_string_lines = Vec::new();

    game_objects.iter().for_each(|obj| {
        output_string_lines.push("|-".to_string());
        output_string_lines.push(format!("|{{{{Card|{}}}}}", obj.name));
        output_string_lines.push(format!("|{:.6}", obj.rValue.unwrap_or(0.0)));
        // println!("{:6} [{:.1}]: {}", obj.id, obj.containSize.as_ref().map(|s| s.containSize).unwrap_or(-1.0), obj.name);
    });

    std::fs::write("output.txt", output_string_lines.join("\n"))?;
    
    Ok(())
}
