use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug)]
pub struct BlocksWalkingData {
    pub blocksWalking: bool,
    pub leftBlockingRadius: Option<i32>,
    pub rightBlockingRadius: Option<i32>,
    pub drawBehindPlayer: Option<bool>,
}

impl ToString for BlocksWalkingData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("blocksWalking={}", self.blocksWalking.to_i8()));
        if let Some(leftBlockingRadius) = self.leftBlockingRadius {
            output.push_str(&format!(",leftBlockingRadius={}", leftBlockingRadius));
        }
        if let Some(rightBlockingRadius) = self.rightBlockingRadius {
            output.push_str(&format!(",rightBlockingRadius={}", rightBlockingRadius));
        }
        if let Some(drawBehindPlayer) = self.drawBehindPlayer {
            output.push_str(&format!(",drawBehindPlayer={}", drawBehindPlayer.to_i8()));
        }
        output
    }
}

impl FromStr for BlocksWalkingData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is blocksWalking. Beyond that, we deal with whatever supported values are present
        let blocksWalking = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse::<i8>()?
        .to_bool();
        let mut leftBlockingRadius = None;
        let mut rightBlockingRadius = None;
        let mut drawBehindPlayer = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "leftBlockingRadius" => leftBlockingRadius = Some(variable_data[1].parse()?),
                "rightBlockingRadius" => rightBlockingRadius = Some(variable_data[1].parse()?),
                "drawBehindPlayer" => drawBehindPlayer = Some(variable_data[1].parse::<i8>()?.to_bool()),
                _ => {
                    log::info!("BlocksWalkingData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(BlocksWalkingData {
            blocksWalking,
            leftBlockingRadius,
            rightBlockingRadius,
            drawBehindPlayer,
        })
    }
}

#[derive(Debug)]
pub struct MapChanceData {
    // line looks something like this: mapChance=0.000000#biomes_0,1,2,3
    pub mapChance: f32,
    // Biome IDs, as of data version 426, are as follows:
    // 0 => Grasslands
    // 1 => Swamp
    // 2 => Yellow Prairies
    // 3 => Badlands
    // 4 => Tundra
    // 5 => Desert
    // 6 => Jungle
    // 7 => Deep Water
    // 8 => Flower Fields
    // 9 => Shallow Water
    pub biomes: Option<Vec<u8>>
}

impl ToString for MapChanceData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("mapChance={:.6}", self.mapChance));
        if let Some(biomes) = &self.biomes {
            output.push_str("#biomes_");
            biomes.iter().for_each(|&biome| output.push_str(&format!("{biome},")));
            output.pop();
        }
        output
    }
}

impl FromStr for MapChanceData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split('#').collect::<Vec<_>>();
        // First section is blocksWalking. Beyond that, we deal with whatever supported values are present
        let mapChance = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse()?;
        let biomes = Some(variable_sections[1][7..].split(',').filter_map(|b| b.parse().ok()).collect::<Vec<_>>());
        Ok(MapChanceData {
            mapChance,
            biomes
        })
    }
}

#[derive(Debug)]
pub struct PersonData {
    pub person: bool,
    pub noSpawn: Option<bool>,
}

impl ToString for PersonData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("person={}", self.person.to_i8()));
        if let Some(noSpawn) = self.noSpawn {
            output.push_str(&format!(",noSpawn={}", noSpawn.to_i8()));
        }
        output
    }
}

impl FromStr for PersonData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is person. Beyond that, we deal with whatever supported values are present
        let person = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse::<i8>()?
        .to_bool();
        let mut noSpawn = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "noSpawn" => noSpawn = Some(variable_data[1].parse::<i8>()?.to_bool()),
                _ => {
                    log::info!("PersonData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(PersonData {
            person,
            noSpawn,
        })
    }
}

#[derive(Debug)]
pub struct PermanentData {
    pub permanent: bool,
    pub minPickupAge: Option<i32>,
}

impl ToString for PermanentData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("permanent={}", self.permanent.to_i8()));
        if let Some(minPickupAge) = self.minPickupAge {
            output.push_str(&format!(",minPickupAge={}", minPickupAge));
        }
        output
    }
}

impl FromStr for PermanentData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is permanent. Beyond that, we deal with whatever supported values are present
        let permanent = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse::<i8>()?
        .to_bool();
        let mut minPickupAge = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "minPickupAge" => minPickupAge = Some(variable_data[1].parse()?),
                _ => {
                    log::info!("PermanentData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(PermanentData {
            permanent,
            minPickupAge,
        })
    }
}

#[derive(Debug)]
pub struct ContainSizeData {
    pub containSize: f32,
    pub vertSlotRot: Option<f32>,
}

impl ToString for ContainSizeData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("containSize={:.6}", self.containSize));
        if let Some(vertSlotRot) = self.vertSlotRot {
            output.push_str(&format!(",vertSlotRot={:.6}", vertSlotRot));
        }
        output
    }
}

//use the code from object's fromstr and put it here instead, and then use it in object's fromstr
impl FromStr for ContainSizeData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is containSize, second optional section is vertSlotRotlet containSize_val = main_variable[1].parse::<f32>()?;
        let containSize = variable_sections[0]
            .split('=')
            .collect::<Vec<_>>()
            [1]
            .parse()?;
        let mut vertSlotRot = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "vertSlotRot" => vertSlotRot = Some(variable_data[1].parse()?),
                _ => {
                    log::info!("ContainsSizeData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(ContainSizeData { containSize, vertSlotRot })
    }
}

#[derive(Debug)]
pub struct ClothingData {
    pub clothing: ClothingType,
}

impl ToString for ClothingData {
    fn to_string(&self) -> String {
        format!("clothing={}", self.clothing.to_string())
    }
}

impl FromStr for ClothingData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clothing = s.split('=').collect::<Vec<_>>()[1].parse()?;

        Ok(ClothingData { clothing })
    }
}

#[derive(Debug)]
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
        match s {
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

#[derive(Debug)]
pub struct SoundData {
    pub id: i32,
    pub volume: f64
}

impl ToString for SoundData {
    fn to_string(&self) -> String {
        if self.volume == 0.0 {
            format!("{}:{:.1}", self.id, self.volume)
        } else {
            format!("{}:{:.6}", self.id, self.volume)
        }
    }
}

//use the code from object's fromstr and put it here instead, and then use it in object's fromstr
impl FromStr for SoundData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(':').collect::<Vec<_>>();
        Ok(SoundData {
            id: variable_sections[0].parse()?,
            volume: variable_sections[1].parse()?
        })
    }
}

#[derive(Debug)]
pub struct SoundsData {
    pub creationSound: SoundData,
    pub usingSound: SoundData,
    pub eatingSound: SoundData,
    pub decaySound: SoundData,
}

impl ToString for SoundsData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str("sounds=");
        output.push_str(&self.creationSound.to_string());
        output.push(',');
        output.push_str(&self.usingSound.to_string());
        output.push(',');
        output.push_str(&self.eatingSound.to_string());
        output.push(',');
        output.push_str(&self.decaySound.to_string());
        output
    }
}

impl FromStr for SoundsData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_sections = s.split('=').collect::<Vec<_>>();
        let variable_sections = line_sections[1].split(',').collect::<Vec<_>>();
        // First section is person. Beyond that, we deal with whatever supported values are present
        Ok(SoundsData {
            creationSound: SoundData::from_str(variable_sections[0])?,
            usingSound: SoundData::from_str(variable_sections[1])?,
            eatingSound: SoundData::from_str(variable_sections[2])?,
            decaySound: SoundData::from_str(variable_sections[3])?,
        })
    }
}

#[derive(Debug)]
pub struct NumSlotsData {
    pub numSlots: i32,
    pub timeStretch: Option<f32>,
}

impl ToString for NumSlotsData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("numSlots={}", self.numSlots));
        if let Some(timeStretch) = &self.timeStretch {
            output.push_str(&format!("#timeStretch={:.6}", timeStretch));
        }
        output
    }
}

impl FromStr for NumSlotsData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split('#').collect::<Vec<_>>();
        // First section is numSlots. Beyond that, we deal with whatever supported values are present
        let numSlots = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse()?;
        let mut timeStretch = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "timeStretch" => timeStretch = Some(variable_data[1].parse()?),
                _ => {
                    log::info!("NumSlotsData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(NumSlotsData {
            numSlots,
            timeStretch,
        })
    }
}

#[derive(Debug)]
pub struct ColorData {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl ToString for ColorData {
    fn to_string(&self) -> String {
        format!("color={:.6},{:.6},{:.6}", self.red, self.green, self.blue)
    }
}

impl FromStr for ColorData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_data = s.split('=')
        .collect::<Vec<_>>()
        [1]
        .split(',')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<_>>();
        Ok(ColorData {
            red: color_data[0],
            green: color_data[1],
            blue: color_data[2],
        })
    }
}

#[derive(Debug)]
pub struct SpriteData {
    pub spriteID: i32,
    pub pos: DoublePair,
    pub rot: f64,
    pub hFlip: bool,
    pub color: ColorData,
    pub ageRange: DoublePair,
    pub parent: i32,
    pub invisHolding: InvisHoldingData,
    pub invisCont: Option<bool>,
}

impl ToString for SpriteData {
    fn to_string(&self) -> String {
        let mut output = format!("spriteID={}
pos={:.6},{:.6}
rot={:.6}
hFlip={}
color={:.6},{:.6},{:.6}
ageRange={:.6},{:.6}
parent={}
{}",
        self.spriteID, self.pos.0, self.pos.1, self.rot, self.hFlip.to_i8(), self.color.red, self.color.green, self.color.blue, self.ageRange.0, self.ageRange.1, self.parent, self.invisHolding.to_string());
        if let Some(invisCont) = self.invisCont {
            output.push_str(&format!("\ninvisCont={}", invisCont.to_i8()));
        }
        output
    }
}

impl SpriteData {
    fn is_sprite_data<'a>(variable_name: &str) -> bool {
        match variable_name {
            "pos" => true,
            "rot" => true,
            "hFlip" => true,
            "color" => true,
            "ageRange" => true,
            "parent" => true,
            "invisHolding" => true,
            "invisCont" => true,
            _ => false,
        }
    }
}

impl FromStr for SpriteData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split('\n').collect::<Vec<_>>();
        // First section is spriteID. Beyond that, we deal with whatever supported values are present
        let spriteID = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse()?;
        let mut pos = None;
        let mut rot = None;
        let mut hFlip = None;
        let mut color = None;
        let mut ageRange = None;
        let mut parent = None;
        let mut invisHolding = None;
        let mut invisCont = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "pos" => pos = Some(variable_data[1].parse::<DoublePair>()?),
                "rot" => rot = Some(variable_data[1].parse::<f64>()?),
                "hFlip" => hFlip = Some(variable_data[1].parse::<i8>()?.to_bool()),
                "color" => color = Some(variable_data[1].parse::<ColorData>()?),
                "ageRange" => ageRange = Some(variable_data[1].parse::<DoublePair>()?),
                "parent" => parent = Some(variable_data[1].parse::<i32>()?),
                "invisHolding" => invisHolding = Some(variable_data[1].parse::<InvisHoldingData>()?),
                "invisCont" => invisCont = Some(variable_data[1].parse::<i8>()?.to_bool()),
                _ => {
                    log::info!("SpriteData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        let pos = pos.ok_or_else(|| anyhow!("Missing required value for \"pos\""))?;
        let rot = rot.ok_or_else(|| anyhow!("Missing required value for \"rot\""))?;
        let hFlip = hFlip.ok_or_else(|| anyhow!("Missing required value for \"hFlip\""))?;
        let color = color.ok_or_else(|| anyhow!("Missing required value for \"color\""))?;
        let ageRange = ageRange.ok_or_else(|| anyhow!("Missing required value for \"ageRange\""))?;
        let parent = parent.ok_or_else(|| anyhow!("Missing required value for \"parent\""))?;
        let invisHolding = invisHolding.ok_or_else(|| anyhow!("Missing required value for \"invisHolding\""))?;
        Ok(SpriteData {
            spriteID,
            pos,
            rot,
            hFlip,
            color,
            ageRange,
            parent,
            invisHolding,
            invisCont,
        })
    }
}

#[derive(Debug)]
pub struct DoublePair(pub f64, pub f64);

impl ToString for DoublePair {
    fn to_string(&self) -> String {
        format!("{:.6},{:.6}", self.0, self.1)
    }
}

impl FromStr for DoublePair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').filter_map(|f| f.parse().ok()).collect::<Vec<_>>();
        Ok(DoublePair(parts[0], parts[1]))
    }
}

#[derive(Debug)]
pub struct InvisHoldingData {
    pub invisHolding: bool,
    pub invisWorn: i32,
    pub behindSlots: bool,
}

impl ToString for InvisHoldingData {
    fn to_string(&self) -> String {
        format!("invisHolding={},invisWorn={},behindSlots={}", self.invisHolding.to_i8(), self.invisWorn, self.behindSlots.to_i8())
    }
}

impl FromStr for InvisHoldingData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is invisHolding. Beyond that, we deal with whatever supported values are present
        let invisHolding = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse::<i8>()?
        .to_bool();
        let mut invisWorn = None;
        let mut behindSlots = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "invisWorn" => invisWorn = Some(variable_data[1].parse::<i32>()?),
                "behindSlots" => behindSlots = Some(variable_data[1].parse::<i8>()?.to_bool()),
                _ => {
                    log::info!("InvisHoldingData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        let invisWorn = invisWorn.ok_or_else(|| anyhow!("Missing required value for \"invisWorn\""))?;
        let behindSlots = behindSlots.ok_or_else(|| anyhow!("Missing required value for \"behindSlots\""))?;
        Ok(InvisHoldingData {
            invisHolding,
            invisWorn,
            behindSlots,
        })
    }
}

#[derive(Debug)]
pub struct NumUsesData {
    pub numUses: i32,
    pub useChance: Option<f32>,
}

impl FromStr for NumUsesData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variable_sections = s.split(',').collect::<Vec<_>>();
        // First section is numUses. Beyond that, we deal with whatever supported values are present
        let numUses = variable_sections[0]
        .split('=')
        .collect::<Vec<_>>()
        [1]
        .parse()?;
        let mut useChance = None;
        for &variable_section in variable_sections.iter().skip(1) {
            let variable_data = variable_section.split('=').collect::<Vec<_>>();
            match variable_data[0] {
                "useChance" => useChance = Some(variable_data[1].parse()?),
                _ => {
                    log::info!("NumUsesData::FromStr: Unexpected variable name {}", variable_data[0]);
                }
            }
        }
        Ok(NumUsesData {
            numUses,
            useChance,
        })
    }
}

impl ToString for NumUsesData {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("numUses={}", self.numUses));
        if let Some(useChance) = &self.useChance {
            output.push_str(&format!(",{:.6}", useChance));
        }
        output
    }
}

#[derive(Debug)]
pub struct Object {
    pub id: i32,
    pub name: String,
    pub containable: Option<bool>,
    pub containSize: Option<ContainSizeData>,
    pub permanent: Option<PermanentData>,
    pub noFlip: Option<bool>,
    pub sideAccess: Option<bool>,
    pub heldInHand: Option<bool>,
    pub blocksWalking: Option<BlocksWalkingData>,
    pub mapChance: Option<MapChanceData>,
    pub heatValue: Option<i32>,
    pub rValue: Option<f32>,
    pub person: Option<PersonData>,
    pub male: Option<bool>,
    pub deathMarker: Option<bool>,
    pub homeMarker: Option<bool>,
    pub floor: Option<bool>,
    pub floorHugging: Option<bool>,
    pub foodValue: Option<i32>,
    pub speedMult: Option<f32>,
    pub heldOffset: Option<DoublePair>,
    pub clothing: Option<ClothingData>,
    pub clothingOffset: Option<DoublePair>,
    pub deadlyDistance: Option<i32>,
    pub useDistance: Option<i32>,
    pub sounds: Option<SoundsData>,
    pub creationSoundInitialOnly: Option<bool>,
    pub creationSoundForce: Option<bool>,
    pub numSlots: Option<NumSlotsData>,
    pub slotSize: Option<f32>,
    pub slotsLocked: Option<bool>,
    pub numSprites: Option<i32>,
    pub sprites: Option<Vec<SpriteData>>,
    pub headIndex: Option<i32>, // This is for human characters, don't worry about it for now
    pub bodyIndex: Option<i32>, // This is for human characters, don't worry about it for now
    pub backFootIndex: Option<i32>, // This is for human characters, don't worry about it for now
    pub frontFootIndex: Option<i32>, // This is for human characters, don't worry about it for now
    pub numUses: Option<NumUsesData>,
    pub useVanishIndex: Option<Vec<i32>>,
    pub useAppearIndex: Option<Vec<i32>>,
    pub pixHeight: Option<i32>,
}

pub trait ToI8 {
    fn to_i8(self) -> i8;
}

impl ToI8 for bool {
    fn to_i8(self) -> i8 {
        if self {
            1
        } else {
            0
        }
    }
}

pub trait ToBool {
    fn to_bool(self) -> bool;
}

impl ToBool for i8 {
    fn to_bool(self) -> bool {
        return self != 0;
    }
}

impl ToString for Object {

    fn to_string(&self) -> String {
        let mut output = Vec::new();
        output.push(format!("id={}", self.id));
        output.push(self.name.clone());
        if let Some(containable) = self.containable {
            output.push(format!("containable={}", containable.to_i8()));
        }
        if let Some(containSize) = &self.containSize {
            output.push(containSize.to_string());
        }
        if let Some(permanent) = &self.permanent {
            output.push(permanent.to_string());
        }
        if let Some(noFlip) = self.noFlip {
            output.push(format!("noFlip={}", noFlip.to_i8()));
        }
        if let Some(sideAccess) = self.sideAccess {
            output.push(format!("sideAccess={}", sideAccess.to_i8()));
        }
        if let Some(heldInHand) = self.heldInHand {
            output.push(format!("heldInHand={}", heldInHand.to_i8()));
        }
        if let Some(blocksWalking) = &self.blocksWalking {
            output.push(blocksWalking.to_string());
        }
        if let Some(mapChance) = &self.mapChance {
            output.push(mapChance.to_string());
        }
        if let Some(heatValue) = self.heatValue {
            output.push(format!("heatValue={}", heatValue));
        }
        if let Some(rValue) = self.rValue {
            output.push(format!("rValue={:.6}", rValue));
        }
        if let Some(person) = &self.person {
            output.push(person.to_string());
        }
        if let Some(male) = self.male {
            output.push(format!("male={}", male.to_i8()));
        }
        if let Some(deathMarker) = self.deathMarker {
            output.push(format!("deathMarker={}", deathMarker.to_i8()));
        }
        if let Some(homeMarker) = self.homeMarker {
            output.push(format!("homeMarker={}", homeMarker.to_i8()));
        }
        if let Some(floor) = self.floor {
            output.push(format!("floor={}", floor.to_i8()));
        }
        if let Some(floorHugging) = self.floorHugging {
            output.push(format!("floorHugging={}", floorHugging.to_i8()));
        }
        if let Some(foodValue) = self.foodValue {
            output.push(format!("foodValue={}", foodValue));
        }
        if let Some(speedMult) = self.speedMult {
            output.push(format!("speedMult={:.6}", speedMult));
        }
        if let Some(heldOffset) = &self.heldOffset {
            output.push(format!("heldOffset={:.6},{:.6}", heldOffset.0, heldOffset.1));
        }
        if let Some(clothing) = &self.clothing {
            output.push(clothing.to_string());
        }
        if let Some(clothingOffset) = &self.clothingOffset {
            output.push(format!("clothingOffset={:.6},{:.6}", clothingOffset.0, clothingOffset.1));
        }
        if let Some(deadlyDistance) = self.deadlyDistance {
            output.push(format!("deadlyDistance={}", deadlyDistance));
        }
        if let Some(useDistance) = self.useDistance {
            output.push(format!("useDistance={}", useDistance));
        }
        if let Some(sounds) = &self.sounds {
            output.push(sounds.to_string());
        }
        if let Some(creationSoundInitialOnly) = self.creationSoundInitialOnly {
            output.push(format!("creationSoundInitialOnly={}", creationSoundInitialOnly.to_i8()));
        }
        if let Some(creationSoundForce) = self.creationSoundForce {
            output.push(format!("creationSoundForce={}", creationSoundForce.to_i8()));
        }
        if let Some(numSlots) = &self.numSlots {
            output.push(numSlots.to_string());
        }
        if let Some(slotSize) = self.slotSize {
            output.push(format!("slotSize={:.6}", slotSize));
        }
        if let Some(slotsLocked) = self.slotsLocked {
            output.push(format!("slotsLocked={}", slotsLocked.to_i8()));
        }
        if let Some(numSprites) = self.numSprites {
            output.push(format!("numSprites={}", numSprites));
        }
        if let Some(sprites) = &self.sprites {
            for sprite in sprites {
                output.push(sprite.to_string());
            }
        }
        if let Some(headIndex) = self.headIndex {
            output.push(format!("headIndex={}", headIndex));
        }
        if let Some(bodyIndex) = self.bodyIndex {
            output.push(format!("bodyIndex={}", bodyIndex));
        }
        if let Some(backFootIndex) = self.backFootIndex {
            output.push(format!("backFootIndex={}", backFootIndex));
        }
        if let Some(frontFootIndex) = self.frontFootIndex {
            output.push(format!("frontFootIndex={}", frontFootIndex));
        }
        if let Some(numUses) = &self.numUses {
            output.push(numUses.to_string());
        }
        if let Some(useVanishIndex) = &self.useVanishIndex {
            let mut line = "useVanishIndex=".to_string();
            for vanishIndex in useVanishIndex {
                line.push_str(&vanishIndex.to_string());
                line.push(',');
            }
            line.pop();
            output.push(line);
        }
        if let Some(useAppearIndex) = &self.useAppearIndex {
            let mut line = "useAppearIndex=".to_string();
            for appearIndex in useAppearIndex {
                line.push_str(&appearIndex.to_string());
                line.push(',');
            }
            line.pop();
            output.push(line);
        }
        if let Some(pixHeight) = self.pixHeight {
            output.push(format!("pixHeight={}", pixHeight));
        }
        output.join("\n")
    }
}

impl FromStr for Object {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        let id = lines[0].split('=').nth(1).unwrap().parse()?;
        let name = lines[1].to_string();
        let mut containable = None;
        let mut containSize = None;
        let mut mapChance = None;
        let mut permanent = None;
        let mut noFlip = None;
        let mut sideAccess = None;
        let mut heldInHand = None;
        let mut blocksWalking = None;
        let mut heatValue = None;
        let mut rValue = None;
        let mut person = None;
        let mut male = None;
        let mut deathMarker = None;
        let mut homeMarker = None;
        let mut floor = None;
        let mut floorHugging = None;
        let mut foodValue = None;
        let mut speedMult = None;
        let mut heldOffset = None;
        let mut clothing = None;
        let mut clothingOffset = None;
        let mut deadlyDistance = None;
        let mut useDistance = None;
        let mut sounds = None;
        let mut creationSoundInitialOnly = None;
        let mut creationSoundForce = None;
        let mut numSlots = None;
        let mut slotSize = None;
        let mut slotsLocked = None;
        let mut numSprites = None;
        let mut sprites = None;
        let mut headIndex = None;
        let mut bodyIndex = None;
        let mut backFootIndex = None;
        let mut frontFootIndex = None;
        let mut numUses = None;
        let mut useVanishIndex = None;
        let mut useAppearIndex = None;
        let mut pixHeight = None;

        let mut lines_iter = lines
            .iter()
            .peekable();
        lines_iter.next();
        lines_iter.next();

        while let Some(&line) = lines_iter.next() {

            let line_sections = line.split('=').collect::<Vec<_>>();
            let main_variable_name = line_sections[0];
            let main_variable_value = line_sections[1];

            // let variable_sections = line_sections[1].split(',').collect::<Vec<_>>();
            // let mut sprite_vec = Vec::new();

            let variable_sections = line.split(',').collect::<Vec<_>>();
            let main_variable = variable_sections[0].split('=').collect::<Vec<_>>();
            let mut sprite_vec = Vec::new();
            match main_variable_name {
                "containable" => containable = Some(main_variable_value != "0"),
                "containSize" => containSize = Some(line.parse()?),
                "mapChance" => mapChance = Some(line.parse()?),
                "permanent" => permanent = Some(line.parse()?),
                "noFlip" => noFlip = Some(main_variable[1] != "0"),
                "sideAccess" => sideAccess = Some(main_variable_value != "0"),
                "heldInHand" => heldInHand = Some(main_variable_value != "0"),
                "blocksWalking" => blocksWalking = Some(line.parse()?),
                "heatValue" => heatValue = Some(main_variable_value.parse()?),
                "rValue" => rValue = Some(main_variable_value.parse()?),
                "person" => person = Some(line.parse()?),
                "male" => male = Some(main_variable_value != "0"),
                "deathMarker" => deathMarker = Some(main_variable_value != "0"),
                "homeMarker" => homeMarker = Some(main_variable_value != "0"),
                "floor" => floor = Some(main_variable_value != "0"),
                "floorHugging" => floorHugging = Some(main_variable_value != "0"),
                "foodValue" => foodValue = Some(main_variable_value.parse()?),
                "speedMult" => speedMult = Some(main_variable_value.parse()?),
                "heldOffset" => heldOffset = Some(main_variable_value.parse()?),
                "clothing" => clothing = Some(line.parse()?),
                "clothingOffset" => clothingOffset = Some(main_variable_value.parse()?),
                "deadlyDistance" => deadlyDistance = Some(main_variable_value.parse()?),
                "useDistance" => useDistance = Some(main_variable_value.parse()?),
                "sounds" => sounds = Some(line.parse()?),
                "creationSoundInitialOnly" => creationSoundInitialOnly = Some(main_variable_value != "0"),
                "creationSoundForce" => creationSoundForce = Some(main_variable_value != "0"),
                "numSlots" => numSlots = Some(line.parse()?),
                "slotSize" => slotSize = Some(main_variable_value.parse()?),
                "slotsLocked" => slotsLocked = Some(main_variable_value != "0"),
                "numSprites" => numSprites = Some(main_variable_value.parse()?),
                "spriteId" => {
                    // We will assume numSprites has come before any sprites
                    // But we will make sure numSprites is something and non-zero later
                    // We must determine what to pass into SpriteData::from_str()
                    // We have to know where to end the multi-line string we provide.
                    // If we are not on the last sprite, there will be a future "spriteID" line
                    // If we are on the last sprite, we must look for a non-sprite variable.
                    // So, if we see either of those things, we have hit the end of our current sprite
                    
                    let mut lines_for_sprite = Vec::new();
                    while let Some(&sprite_line) = lines_iter.peek() {
                        let sprite_variable = sprite_line.split('=').collect::<Vec<_>>()[0];
                        if sprite_variable == "spriteID" {
                            break;
                        }
                        if !SpriteData::is_sprite_data(sprite_variable) {
                            break;
                        }
                        // If we passed the gates, we have a good sprite line still.
                        // Add it to lines_for_sprite and continue.
                        lines_for_sprite.push(*lines_iter.next().unwrap());
                    }
                    sprite_vec.push(SpriteData::from_str(&lines_for_sprite.join("\n"))?);
                },
                "headIndex" => headIndex = Some(main_variable_value.parse()?),
                "bodyIndex" => bodyIndex = Some(main_variable_value.parse()?),
                "backFootIndex" => backFootIndex = Some(main_variable_value.parse()?),
                "frontFootIndex" => frontFootIndex = Some(main_variable_value.parse()?),
                "numUses" => numUses = Some(line.parse()?),
                "useVanishIndex" => useVanishIndex = Some(main_variable_value.split(",").filter_map(|v| v.parse().ok()).collect::<Vec<_>>()),
                "useAppearIndex" => useAppearIndex = Some(main_variable_value.split(",").filter_map(|v| v.parse().ok()).collect::<Vec<_>>()),
                "pixHeight" => pixHeight = Some(main_variable_value.parse()?),
                _ => {
                    log::warn!("Unknown variable name {}", main_variable[0]);
                }
            }
            // We will make sure numSprites is something and non-zero
            if numSprites.is_none() || numSprites.unwrap() == 0 {
                sprites = None;
            } else {
                sprites = Some(sprite_vec);
            }
        }

        Ok(Object { id, name, containable, containSize, mapChance, permanent, noFlip, sideAccess, heldInHand, blocksWalking, heatValue, rValue, person, male, deathMarker, homeMarker, floor, floorHugging, foodValue, speedMult, heldOffset, clothing, clothingOffset, deadlyDistance, useDistance, sounds, creationSoundInitialOnly, creationSoundForce, numSlots, slotSize, slotsLocked, numSprites, sprites, headIndex, bodyIndex, backFootIndex, frontFootIndex, numUses, useVanishIndex, useAppearIndex, pixHeight })
    }
}
