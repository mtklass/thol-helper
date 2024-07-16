mod object;

use std::{fs, io::Read, process, str::FromStr};

use anyhow::Result;
use pretty_assertions::assert_eq;

use object::{BlocksWalkingData, ClothingData, ClothingType, ColorData, ContainSizeData, DoublePair, InvisHoldingData, MapChanceData, NumSlotsData, NumUsesData, Object, PermanentData, PersonData, SoundData, SoundDataVec, SoundsData, SpriteData};

fn main() -> Result<()> {
    // Read each object txt file in the provided directory, and attempt to parse it.
    // Do if == on the original file data and the FromStr->ToString chain output.
    // Log errors, but keep going.
    let object_directory = "../../OneLifeData7/objects/";
    let object_dir_contents = fs::read_dir(object_directory)?;
    for entry in object_dir_contents {
        if let Ok(entry) = entry {
            // Check if the entry is a file and matches the pattern
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_name = entry.file_name();
                    let file_name = file_name.to_string_lossy();

                    if let Some(captures) = regex::Regex::new(r"^(\d+)\.txt$").unwrap().captures(&file_name) {
                        // For debugging, only look at file we care about
                        // if captures.get(1).unwrap().as_str() != "14492" {
                        //     continue;
                        // }
                        println!("Parsing file {file_name}");
                        // Read the file into a string
                        let mut file = fs::File::open(entry.path()).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();

                        let object = Object::from_str(&contents)?;
                        let recreated_string = object.to_string();
                        assert_eq!(contents, recreated_string);
                    }
                }
            }
        }
    }
    Ok(())
}

fn original_test() {
    let object_data_str = r#"id=8675
Skull Hat
containable=1
containSize=2.000000,vertSlotRot=-0.500000
permanent=0,minPickupAge=3
noFlip=0
sideAccess=0
heldInHand=0
blocksWalking=0,leftBlockingRadius=0,rightBlockingRadius=0,drawBehindPlayer=0
mapChance=0.000000#biomes_0,1
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
        heldInHand: Some(0),
        blocksWalking: Some(BlocksWalkingData {
            blocksWalking: false,
            leftBlockingRadius: Some(0),
            rightBlockingRadius: Some(0),
            drawBehindPlayer: Some(false),
        }),
        mapChance: Some(MapChanceData {
            mapChance: 0.0,
            biomes: Some(vec![0,1]),
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
        wallLayer: None,
        frontWall: None,
        foodValue: Some(vec![0]),
        speedMult: Some(1.0),
        containOffset: None,
        heldOffset: Some(DoublePair(23.0, 29.0)),
        clothing: Some(ClothingData { clothing: ClothingType::Head }),
        clothingOffset: Some(DoublePair(3.0, 69.999996)),
        deadlyDistance: Some(0),
        useDistance: Some(1),
        sounds: Some(SoundsData {
            creationSound: SoundDataVec { 0: vec![SoundData { id: 153, volume: 0.225 }] },
            usingSound: SoundDataVec { 0: vec![SoundData { id: 286, volume: 0.10101 }] },
            eatingSound: SoundDataVec { 0: vec![SoundData { id: -1, volume: 0.0 }] },
            decaySound: SoundDataVec { 0: vec![SoundData { id: -1, volume: 0.0 }] },
        }),
        creationSoundInitialOnly: Some(false),
        creationSoundForce: Some(false),
        numSlots: Some(NumSlotsData {
            numSlots: 0,
            timeStretch: Some(1.0),
        }),
        slotSize: Some(1.0),
        slotStyle: None,
        slotsLocked: Some(false),
        slotsNoSwap: None,
        slotPosData: None,
        numSprites: Some(1),
        sprites: Some(vec![
            SpriteData {
                spriteID: 101377,
                pos: DoublePair(-1.0, -29.0),
                rot: -0.005,
                hFlip: false,
                color: ColorData { red: 1.0, green: 1.0, blue: 1.0 },
                ageRange: DoublePair(-1.0, -1.0),
                parent: -1,
                invisHolding: InvisHoldingData { invisHolding: false, invisWorn: 0, behindSlots: false },
                invisCont: Some(false),
                spritesDrawnBehind: None,
                spritesAdditiveBlend: None,
                ignoredCont: None,
            }
        ]),
        headIndex: Some(vec![-1]),
        bodyIndex: Some(vec![-1]),
        backFootIndex: Some(vec![-1]),
        frontFootIndex: Some(vec![-1]),
        numUses: Some(NumUsesData {
            numUses: 1,
            useChance: Some(1.0),
        }),
        useVanishIndex: Some(vec![-1]),
        useAppearIndex: Some(vec![-1]),
        pixHeight: Some(4),
    };

    assert_eq!(object_data_struct.to_string(), object_data_str);

}