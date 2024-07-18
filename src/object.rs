#![allow(non_snake_case)]
use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Object {
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
    pub id: Option<String>,
    pub clothing: Option<ClothingType>,
    pub heatValue: Option<i32>,
    pub mapChance: Option<f64>,
    pub moveType: Option<i32>,
    pub numSlots: Option<i32>,
    pub numUses: Option<i32>,
    pub useDistance: Option<i32>,
    pub depth: Option<i32>,
    pub foodValue: Option<Vec<i32>>,
    pub insulation: Option<f64>,
    pub size: Option<f32>,
    pub sounds: Option<Vec<String>>,
    pub useChance: Option<f64>,
    pub name: Option<String>,
    pub techTree: Option<Vec<TechTreeNode>>,
    #[serde(deserialize_with = "deserialize_move_distance", default = "GetNone")]
    pub moveDistance: Option<i32>,
    pub transitionsAway: Option<Vec<TransitionAwayData>>,
    pub slotSize: Option<f32>,
}

fn GetNone() -> Option<i32> {
    None
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
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ObjectRecipe {
    pub steps: Option<Vec<Vec<RecipeStep>>>,
    pub ingredients: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RecipeStep {
    id: Option<String>,
    mainBranch: Option<bool>,
    depth: Option<i32>,
    actorID: Option<String>,
    hand: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Biome {
    id: Option<String>,
    spawnChance: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TransitionTimedData {
    targetID: Option<String>,
    newTargetID: Option<String>,
    decay: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TransitionTowardData {
    actorID: Option<String>,
    targetID: Option<String>,
    newActorID: Option<String>,
    newTargetID: Option<String>,
    hand: Option<bool>,
    decay: Option<String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TransitionAwayData {
    actorID: Option<String>,
    targetID: Option<String>,
    newActorID: Option<String>,
    newTargetID: Option<String>,
    newActorUses: Option<String>,
    newActorWeight: Option<f32>,
    targetRemains: Option<bool>,
    hand: Option<bool>,
    tool: Option<bool>,
    decay: Option<String>,
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
        match s.trim() {
            "b" => Ok(ClothingType::Bottom),
            "h" => Ok(ClothingType::Head),
            "p" => Ok(ClothingType::Pack),
            "p0" => Ok(ClothingType::Shield),
            "s" => Ok(ClothingType::Shoe),
            "t" => Ok(ClothingType::Top),
            "n" => Ok(ClothingType::None),
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TechTreeNode {
    id: Option<String>,
    nodes: Option<Vec<TechTreeNode>>,
}
