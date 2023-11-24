pub mod bridge;
pub mod building;
pub mod cityfurniture;
pub mod cityobjectgroup;
pub mod generics;
pub mod iur;
pub mod landuse;
pub mod other_construction;
pub mod relief;
pub mod transportation;
pub mod tunnel;
pub mod vegetation;
pub mod waterbody;

pub use bridge::Bridge;
pub use building::Building;
pub use cityfurniture::CityFurniture;
pub use cityobjectgroup::CityObjectGroup;
pub use generics::GenericCityObject;
pub use iur::landslide::SedimentDisasterProneArea;
pub use iur::underground_building::UndergroundBuilding;
pub use landuse::LandUse;
pub use other_construction::OtherConstruction;
pub use relief::ReliefFeature;
pub use transportation::{Railway, Road, Square, Track, Waterway};
pub use tunnel::Tunnel;
pub use vegetation::{PlantCover, SolitaryVegetationObject};
pub use waterbody::WaterBody;

use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub enum CityObject {
    #[default]
    Unknown,
    //
    // CityGML standard city objects
    //
    #[citygml(path = b"bldg:Building")]
    Building(Building),
    #[citygml(path = b"tran:Road")]
    Road(Road),
    #[citygml(path = b"tran:Railway")]
    Railway(Railway),
    #[citygml(path = b"tran:Track")]
    Track(Track),
    #[citygml(path = b"tran:Square")]
    Square(Square),
    #[citygml(path = b"uro:Waterway")]
    Waterway(Waterway),
    #[citygml(path = b"brid:Bridge")]
    Bridge(Bridge),
    #[citygml(path = b"frn:CityFurniture")]
    CityFurniture(CityFurniture),
    #[citygml(path = b"veg:SolitaryVegetationObject")]
    SolitaryVegetationObject(SolitaryVegetationObject),
    #[citygml(path = b"veg:PlantCover")]
    PlantCover(PlantCover),
    #[citygml(path = b"veg:LandUse")]
    LandUse(LandUse),
    #[citygml(path = b"tun:Tunnel")]
    Tunnel(Tunnel),
    #[citygml(path = b"dem:ReliefFeature")]
    ReliefFeature(ReliefFeature),
    #[citygml(path = b"wtr:WaterBody")]
    WaterBody(WaterBody),
    #[citygml(path = b"gen:GenericCityObject")]
    GenericCityObject(GenericCityObject),
    #[citygml(path = b"grp:CityObjectGroup")]
    CityObjectGroup(CityObjectGroup),
    //
    // i-UR urban objects
    //
    #[citygml(path = b"uro:OtherConstruction")]
    OtherConstruction(OtherConstruction),
    #[citygml(path = b"uro:UndergroundBuilding")]
    UndergroundBuilding(UndergroundBuilding),
    //
    // i-UR urban functions
    //
    #[citygml(path = b"urf:SedimentDisasterProneArea")]
    SedimentDisasterProneArea(SedimentDisasterProneArea),
    //
    // and more ...
}
