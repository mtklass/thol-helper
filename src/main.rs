use std::str::FromStr;

use object::{BlocksWalkingData, ClothingData, ClothingType, ColorData, ContainSizeData, DoublePair, InvisHoldingData, MapChanceData, NumSlotsData, NumUsesData, Object, PermanentData, PersonData, SoundData, SoundsData, SpriteData};

mod object;

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
        heldInHand: Some(false),
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
        foodValue: Some(0),
        speedMult: Some(1.0),
        heldOffset: Some(DoublePair(23.0, 29.0)),
        clothing: Some(ClothingData { clothing: ClothingType::Head }),
        clothingOffset: Some(DoublePair(3.0, 69.999996)),
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
                pos: DoublePair(-1.0, -29.0),
                rot: -0.005,
                hFlip: false,
                color: ColorData { red: 1.0, green: 1.0, blue: 1.0 },
                ageRange: DoublePair(-1.0, -1.0),
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

    // println!("\n{object_data_str}\n\n{}\n", object_data_struct.to_string());

    assert_eq!(object_data_struct.to_string(), object_data_str);

    println!("from_str struct:\n{:#?}", Object::from_str(object_data_str));

}
