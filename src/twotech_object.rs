#![allow(non_snake_case)]
use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::EnumIter;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TwoTechObject {
    pub id: String,
    pub name: String,
    pub recipe: Option<ObjectRecipe>,
    pub speedMult: Option<f64>,
    pub version:Option<i32>,
    pub blocksWalking: Option<bool>,
    pub deadlyDistance: Option<i32>,
    pub biomes: Option<Vec<Biome>>,
    pub minPickupAge: Option<i32>,
    pub transitionsTimed: Option<Vec<TransitionTimedData>>,
    pub transitionsToward: Option<Vec<TransitionTowardData>>,
    pub craftable: Option<bool>,
    pub clothing: Option<ClothingType>,
    pub heatValue: Option<i32>,
    pub mapChance: Option<f64>,
    #[serde(deserialize_with = "deserialize_move_type", default = "GetNone")]
    pub moveType: Option<MoveType>,
    pub numSlots: Option<i32>,
    pub numUses: Option<i32>,
    pub useDistance: Option<i32>,
    pub depth: Option<i32>,
    pub foodValue: Option<Vec<i32>>,
    pub insulation: Option<f64>,
    pub size: Option<f32>,
    pub sounds: Option<Vec<String>>,
    pub useChance: Option<f64>,
    pub techTree: Option<Vec<TechTreeNode>>,
    #[serde(deserialize_with = "deserialize_move_distance", default = "GetNone")]
    pub moveDistance: Option<i32>,
    pub transitionsAway: Option<Vec<TransitionAwayData>>,
    pub slotSize: Option<f32>,
}

fn GetNone<T>() -> Option<T> {
    None
}

// Custom deserializer for moveType
fn deserialize_move_type<'de, D>(deserializer: D) -> Result<Option<MoveType>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => {
            let move_type = n.as_i64();
            if move_type.is_none() {
                return Err(serde::de::Error::custom(&format!("Invalid value for move_type {}, doesn't fit into i64!", n.to_string())));
            }
            let move_type = match move_type.unwrap() {
                0 => Some(MoveType::None),
                1 => Some(MoveType::Chase),
                2 => Some(MoveType::Flee),
                3 => Some(MoveType::Random),
                4 => Some(MoveType::North),
                5 => Some(MoveType::South),
                6 => Some(MoveType::East),
                7 => Some(MoveType::West),
                8 => Some(MoveType::Find),
                _ => None,
            };
            if move_type.is_none() {
                Err(serde::de::Error::custom(&format!("Invalid value for move_type {}, out of range!", n.to_string())))
            } else {
                Ok(Some(move_type.unwrap()))
            }
        }
        serde_json::Value::String(s) => {
            let move_type = match s.to_lowercase().as_str() {
                "none" => Some(MoveType::None),
                "chase" => Some(MoveType::Chase),
                "flee" => Some(MoveType::Flee),
                "random" => Some(MoveType::Random),
                "north" => Some(MoveType::North),
                "south" => Some(MoveType::South),
                "east" => Some(MoveType::East),
                "west" => Some(MoveType::West),
                "find" => Some(MoveType::Find),
                _ => None
            };
            if move_type.is_none() {
                Err(serde::de::Error::custom(&format!("Invalid value for move_type {}, no movement type match found!", s)))
            } else {
                Ok(Some(move_type.unwrap()))
            }
        }
        serde_json::Value::Null => Ok(Some(MoveType::None)),
        _ => Err(serde::de::Error::custom("Unexpected value deserializing moveType")),
    }
}

// Custom deserializer for moveDistance
fn deserialize_move_distance<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => s.parse::<i32>().map(Some).map_err(serde::de::Error::custom),
        serde_json::Value::Number(n) => Ok(Some(n.as_i64().unwrap() as i32)),
        serde_json::Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("Unexpected value deserializing moveDistance")),
    }
}

pub type RecipeStepRow = Vec<RecipeStep>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectRecipe {
    pub steps: Option<Vec<RecipeStepRow>>,
    pub ingredients: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RecipeStep {
    pub id: Option<String>,
    pub mainBranch: Option<bool>,
    pub depth: Option<i32>,
    pub actorID: Option<String>,
    pub actorUses: Option<String>,
    pub hand: Option<bool>,
    pub uses: Option<String>,
    pub targetID: Option<String>,
    pub targetUses: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Biome {
    pub id: Option<String>,
    pub spawnChance: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
pub enum MoveType {
    None,
    Chase,
    Flee,
    Random,
    North,
    South,
    East,
    West,
    Find,
}

impl MoveType {
    pub fn to_i32(&self) -> i32 {
        match self {
            MoveType::None => 0,
            MoveType::Chase => 1,
            MoveType::Flee => 2,
            MoveType::Random => 3,
            MoveType::North => 4,
            MoveType::South => 5,
            MoveType::East => 6,
            MoveType::West => 7,
            MoveType::Find => 8,
        }
    }
}

impl ToString for MoveType {
    fn to_string(&self) -> String {
        match self {
            MoveType::None => "None",
            MoveType::Chase => "Chase",
            MoveType::Flee => "Flee",
            MoveType::Random => "Random",
            MoveType::North => "North",
            MoveType::South => "South",
            MoveType::East => "East",
            MoveType::West => "West",
            MoveType::Find => "Find",
        }.to_string()
    }
}

impl FromStr for MoveType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().replace("_", "").replace(" ", "").as_str() {
            "0" | "none" => Ok(MoveType::None),
            "1" | "chase" => Ok(MoveType::Chase),
            "2" | "flee" => Ok(MoveType::Flee),
            "3" | "random" => Ok(MoveType::Random),
            "4" | "north" => Ok(MoveType::North),
            "5" | "south" => Ok(MoveType::South),
            "6" | "east" => Ok(MoveType::East),
            "7" | "west" => Ok(MoveType::West),
            "8" | "find" => Ok(MoveType::Find),
            _ => Err(anyhow!("Unknown moveType value {s}"))
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransitionTimedData {
    pub targetID: Option<String>,
    pub newTargetID: Option<String>,
    pub decay: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransitionTowardData {
    pub actorID: Option<String>,
    pub targetID: Option<String>,
    pub newActorID: Option<String>,
    pub newTargetID: Option<String>,
    pub hand: Option<bool>,
    pub decay: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransitionAwayData {
    pub actorID: Option<String>,
    pub targetID: Option<String>,
    pub newActorID: Option<String>,
    pub newTargetID: Option<String>,
    pub newActorUses: Option<String>,
    pub newActorWeight: Option<f32>,
    pub targetRemains: Option<bool>,
    pub hand: Option<bool>,
    pub tool: Option<bool>,
    pub decay: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClothingType {
    Bottom,
    Head,
    Pack,
    Shield,
    Shoe,
    Top,
    None,
}

impl ToString for ClothingType {
    fn to_string(&self) -> String {
        match self {
            ClothingType::Bottom => "b".to_string(),
            ClothingType::Head => "h".to_string(),
            ClothingType::Pack => "p".to_string(),
            ClothingType::Shield => "p0".to_string(),
            ClothingType::Shoe => "s".to_string(),
            ClothingType::Top => "t".to_string(),
            ClothingType::None => "n".to_string(),
            
        }
    }
}

impl FromStr for ClothingType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "b" | "bottom" => Ok(ClothingType::Bottom),
            "h" | "head" => Ok(ClothingType::Head),
            "p" | "pack" => Ok(ClothingType::Pack),
            "p0" | "shield" => Ok(ClothingType::Shield),
            "s" | "shoe" => Ok(ClothingType::Shoe),
            "t" | "top" => Ok(ClothingType::Top),
            "n" | "none" => Ok(ClothingType::None),
            _ => {
                Err(anyhow!("Unsupported clothing type value \"{s}\""))
            }
        }
    }
}

impl Serialize for ClothingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            ClothingType::Bottom => "b",
            ClothingType::Head => "h",
            ClothingType::Pack => "p",
            ClothingType::Shield => "p0",
            ClothingType::Shoe => "s",
            ClothingType::Top => "t",
            ClothingType::None => "n",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ClothingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "b" => Ok(ClothingType::Bottom),
            "h" => Ok(ClothingType::Head),
            "p" => Ok(ClothingType::Pack),
            "p0" => Ok(ClothingType::Shield),
            "s" => Ok(ClothingType::Shoe),
            "t" => Ok(ClothingType::Top),
            "n" => Ok(ClothingType::None),
            _ => Err(serde::de::Error::custom("Unexpected value")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TechTreeNode {
    pub id: Option<String>,
    pub nodes: Option<Vec<TechTreeNode>>,
}
