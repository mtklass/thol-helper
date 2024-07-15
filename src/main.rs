// The goal is to use Serde to parse something like this:
/*
id=8675
Skull Hat
containable=1
containSize=2.000000,vertSlotRot=-0.500000
permanent=0,minPickupAge=3
noFlip=0
sideAccess=0
heldInHand=0
blocksWalking=0,leftBlockingRadius=0,rightBlockingRadius=0,drawBehindPlayer=0
mapChance=0.000000#biomes_0
heatValue=0
rValue=0.150000
person=0,noSpawn=0
male=0
deathMarker=0
homeMarker=0
floor=0
floorHugging=0
foodValue=0
speedMult=1.000000
heldOffset=23.000000,29.000000
clothing=h
clothingOffset=3.000000,69.999996
deadlyDistance=0
useDistance=1
sounds=153:0.225000,286:0.101010,-1:0.0,-1:0.0
creationSoundInitialOnly=0
creationSoundForce=0
numSlots=0#timeStretch=1.000000
slotSize=1.000000
slotsLocked=0
numSprites=1
spriteID=101377
pos=-1.000000,-29.000000
rot=-0.005000
hFlip=0
color=1.000000,1.000000,1.000000
ageRange=-1.000000,-1.000000
parent=-1
invisHolding=0,invisWorn=0,behindSlots=0
invisCont=0
headIndex=-1
bodyIndex=-1
backFootIndex=-1
frontFootIndex=-1
numUses=1,1.000000
useVanishIndex=-1
useAppearIndex=-1
pixHeight=4
*/
// Into a struct that allows all of this as optional paramters.
// We will have to decide what things are required. For our purposes, perhaps just an id and a second line with the name
// So our parsing will have to make a special case for the second line, where it will just put that into a variable named "name", like how the other lines have variable names like "id" and "clothing"


use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
struct BlocksWalkingData {
    blocksWalking: bool,
    leftBlockingRadius: Option<i32>,
    rightBlockingRadius: Option<i32>,
    drawBehindPlayer: Option<bool>,
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

#[derive(Debug)]
struct MapChanceData {
    // line looks something like this: mapChance=0.000000#biomes_0,1,2,3
    mapChance: f32,
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
    biomes: Option<Vec<u8>>
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

#[derive(Debug)]
struct PersonData {
    person: bool,
    noSpawn: Option<bool>,
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

#[derive(Debug)]
struct PermanentData {
    permanent: bool,
    minPickupAge: Option<i32>,
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

#[derive(Debug)]
struct ContainSizeData {
    containSize: f32,
    vertSlotRot: Option<f32>,
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

#[derive(Debug)]
enum ClothingType {
    Bottom,
    Head,
    Pack,
    Shield,
    Shoe,
    Top,
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
        }
    }
}

#[derive(Debug)]
struct SoundData {
    id: i32,
    volume: f64
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

#[derive(Debug)]
struct SoundsData {
    creationSound: SoundData,
    usingSound: SoundData,
    eatingSound: SoundData,
    decaySound: SoundData,
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

#[derive(Debug)]
struct NumSlotsData {
    numSlots: i32,
    timeStretch: Option<f32>,
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

#[derive(Debug)]
struct ColorData {
    red: f32,
    green: f32,
    blue: f32,
}

impl ToString for ColorData {
    fn to_string(&self) -> String {
        format!("color={:.6},{:.6},{:.6}", self.red, self.green, self.blue)
    }
}

#[derive(Debug)]
struct SpriteData {
    spriteID: i32,
    pos: (f64, f64),
    rot: f64,
    hFlip: bool,
    color: ColorData,
    ageRange: (f64, f64),
    parent: i32,
    invisHolding: InvisHoldingData,
    invisCont: Option<bool>,
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

#[derive(Debug)]
struct InvisHoldingData {
    invisHolding: bool,
    invisWorn: i32,
    behindSlots: bool,
}

impl ToString for InvisHoldingData {
    fn to_string(&self) -> String {
        format!("invisHolding={},invisWorn={},behindSlots={}", self.invisHolding.to_i8(), self.invisWorn, self.behindSlots.to_i8())
    }
}

#[derive(Debug)]
struct NumUsesData {
    numUses: i32,
    useChance: Option<f32>,
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
struct Object {
    id: i32,
    name: String,
    containable: Option<bool>,
    containSize: Option<ContainSizeData>,
    permanent: Option<PermanentData>,
    noFlip: Option<bool>,
    sideAccess: Option<bool>,
    heldInHand: Option<bool>,
    blocksWalking: Option<BlocksWalkingData>,
    mapChance: Option<MapChanceData>,
    heatValue: Option<i32>,
    rValue: Option<f32>,
    person: Option<PersonData>,
    male: Option<bool>,
    deathMarker: Option<bool>,
    homeMarker: Option<bool>,
    floor: Option<bool>,
    floorHugging: Option<bool>,
    foodValue: Option<i32>,
    speedMult: Option<f32>,
    heldOffset: Option<(f64, f64)>,
    clothing: Option<ClothingType>,
    clothingOffset: Option<(f64, f64)>,
    deadlyDistance: Option<i32>,
    useDistance: Option<i32>,
    sounds: Option<SoundsData>,
    creationSoundInitialOnly: Option<bool>,
    creationSoundForce: Option<bool>,
    numSlots: Option<NumSlotsData>,
    slotSize: Option<f32>,
    slotsLocked: Option<bool>,
    numSprites: Option<i32>,
    sprites: Option<Vec<SpriteData>>,
    headIndex: Option<i32>, // This is for human characters, don't worry about it for now
    bodyIndex: Option<i32>, // This is for human characters, don't worry about it for now
    backFootIndex: Option<i32>, // This is for human characters, don't worry about it for now
    frontFootIndex: Option<i32>, // This is for human characters, don't worry about it for now
    numUses: Option<NumUsesData>,
    useVanishIndex: Option<Vec<i32>>,
    useAppearIndex: Option<Vec<i32>>,
    pixHeight: Option<i32>,
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
        if let Some(heldOffset) = self.heldOffset {
            output.push(format!("heldOffset={:.6},{:.6}", heldOffset.0, heldOffset.1));
        }
        if let Some(clothing) = &self.clothing {
            output.push(format!("clothing={}", clothing.to_string()));
        }
        if let Some(clothingOffset) = self.clothingOffset {
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

fn main() {
    let object_data_str = r#"id=8675
Skull Hat
containable=1
containSize=2.000000,vertSlotRot=-0.500000
permanent=0,minPickupAge=3
noFlip=0
sideAccess=0
heldInHand=0
blocksWalking=0,leftBlockingRadius=0,rightBlockingRadius=0,drawBehindPlayer=0
mapChance=0.000000#biomes_0
heatValue=0
rValue=0.150000
person=0,noSpawn=0
male=0
deathMarker=0
homeMarker=0
floor=0
floorHugging=0
foodValue=0
speedMult=1.000000
heldOffset=23.000000,29.000000
clothing=h
clothingOffset=3.000000,69.999996
deadlyDistance=0
useDistance=1
sounds=153:0.225000,286:0.101010,-1:0.0,-1:0.0
creationSoundInitialOnly=0
creationSoundForce=0
numSlots=0#timeStretch=1.000000
slotSize=1.000000
slotsLocked=0
numSprites=1
spriteID=101377
pos=-1.000000,-29.000000
rot=-0.005000
hFlip=0
color=1.000000,1.000000,1.000000
ageRange=-1.000000,-1.000000
parent=-1
invisHolding=0,invisWorn=0,behindSlots=0
invisCont=0
headIndex=-1
bodyIndex=-1
backFootIndex=-1
frontFootIndex=-1
numUses=1,1.000000
useVanishIndex=-1
useAppearIndex=-1
pixHeight=4"#;

    let object_data_struct = Object {
        id: 8675,
        name: "Skull Hat".to_string(),
        containable: Some(true),
        containSize: Some(ContainSizeData {
            containSize: 2.0,
            vertSlotRot: Some(-0.5),
        }),
        permanent: Some(PermanentData {
            permanent: false,
            minPickupAge: Some(3),
        }),
        noFlip: Some(false),
        sideAccess: Some(false),
        heldInHand: Some(false),
        blocksWalking: Some(BlocksWalkingData {
            blocksWalking: false,
            leftBlockingRadius: Some(0),
            rightBlockingRadius: Some(0),
            drawBehindPlayer: Some(false),
        }),
        mapChance: Some(MapChanceData {
            mapChance: 0.0,
            biomes: Some(vec![0]),
        }),
        heatValue: Some(0),
        rValue: Some(0.15),
        person: Some(PersonData {
            person: false,
            noSpawn: Some(false),
        }),
        male: Some(false),
        deathMarker: Some(false),
        homeMarker: Some(false),
        floor: Some(false),
        floorHugging: Some(false),
        foodValue: Some(0),
        speedMult: Some(1.0),
        heldOffset: Some((23.0, 29.0)),
        clothing: Some(ClothingType::Head),
        clothingOffset: Some((3.0, 69.999996)),
        deadlyDistance: Some(0),
        useDistance: Some(1),
        sounds: Some(SoundsData {
            creationSound: SoundData { id: 153, volume: 0.225  },
            usingSound: SoundData { id: 286, volume: 0.10101  },
            eatingSound: SoundData { id: -1, volume: 0.0  },
            decaySound: SoundData { id: -1, volume: 0.0  },
        }),
        creationSoundInitialOnly: Some(false),
        creationSoundForce: Some(false),
        numSlots: Some(NumSlotsData {
            numSlots: 0,
            timeStretch: Some(1.0),
        }),
        slotSize: Some(1.0),
        slotsLocked: Some(false),
        numSprites: Some(1),
        sprites: Some(vec![
            SpriteData {
                spriteID: 101377,
                pos: (-1.0, -29.0),
                rot: -0.005,
                hFlip: false,
                color: ColorData { red: 1.0, green: 1.0, blue: 1.0 },
                ageRange: (-1.0, -1.0),
                parent: -1,
                invisHolding: InvisHoldingData { invisHolding: false, invisWorn: 0, behindSlots: false },
                invisCont: Some(false)
            }
        ]),
        headIndex: Some(-1),
        bodyIndex: Some(-1),
        backFootIndex: Some(-1),
        frontFootIndex: Some(-1),
        numUses: Some(NumUsesData {
            numUses: 1,
            useChance: Some(1.0),
        }),
        useVanishIndex: Some(vec![-1]),
        useAppearIndex: Some(vec![-1]),
        pixHeight: Some(4),
    };

    println!("\n{object_data_str}\n\n{}\n", object_data_struct.to_string());

    assert_eq!(object_data_struct.to_string(), object_data_str);

}
