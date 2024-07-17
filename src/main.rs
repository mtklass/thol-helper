mod object;

use std::{fs, io::Read, str::FromStr};

use anyhow::Result;
use clap::Parser;

use object::Object;

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
    let object_directory = data_directory + "objects/";
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
        .iter()
        .filter(|&obj| {
            !obj.name.contains("outdated")
            && !obj.name.contains("#")
        }).collect::<Vec<_>>();
    println!("Have {} game object files after filtering default unwanted entries", game_objects.len());

    // Now we have a list of all "good" game objects. We can filter them how we want to make some more useful subsets.
    // We can also, in the future, add transition file parsing, and then connect the two.
    // This can allow for really powerful queries regarding recipes, but could get complicated quickly.

    let mut game_objects = game_objects.iter().filter(|&obj| {
        !obj.numSlots.is_none() && obj.numSlots.as_ref().unwrap().numSlots > 0
    }).collect::<Vec<_>>();
    println!("Query matched {} game files", game_objects.len());

    game_objects.sort_by_key(|k| k.id);

    game_objects.iter().for_each(|obj| {
        println!("{:6} [{:.1}]: {}", obj.id, obj.containSize.as_ref().map(|s| s.containSize).unwrap_or(-1.0), obj.name);
    });

    
    Ok(())
}
