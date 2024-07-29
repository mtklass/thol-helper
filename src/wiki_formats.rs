use std::ops::{Div, Mul};

use crate::SharedGameObject;
use crate::twotech_object::MoveType;

pub fn _wiki_format_card_template_object_id(obj: &SharedGameObject) -> String {
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

pub fn _wiki_format_line_slot_item(obj: &SharedGameObject) -> String {
    format!("|-
|{{{{Card|{}}}}}
|{}
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        obj.twotech_data.numSlots.map(|n| n.to_string()).unwrap_or("0".to_string()),
        obj.twotech_data.slotSize.map(|n| n.to_string()).unwrap_or("0".to_string()),
    )
}

pub fn _wiki_format_line_food(obj: &SharedGameObject) -> String {
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

pub fn _wiki_format_line_clothing_with_slots(obj: &SharedGameObject) -> String {
    format!("|-
|{{{{Card|{}}}}}
|{:1.}%
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        obj.twotech_data.insulation.unwrap_or(0.0).mul(100.0).mul(1000000.0).round().div(1000000.0),
        obj.twotech_data.numSlots.map(|n| n.to_string()).unwrap_or("0".to_string())
    )
}

pub fn _wiki_format_line_movers(obj: &SharedGameObject) -> String {
    format!("|-
|{{{{Card|{}}}}}
|{}",
        obj.twotech_data.name.clone().unwrap_or("ERROR: No name!".to_string()),
        obj.twotech_data.moveType.as_ref().unwrap_or(&MoveType::None).to_string(),
    )
}
