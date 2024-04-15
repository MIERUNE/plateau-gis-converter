use std::io::Write;

use ahash::{HashMap, HashMapExt};

pub struct ProjectionRepository {
    wkt_map: HashMap<u16, String>,
}

// Define WKT1_ESRI strings for various CRSs
const WKT1_ESRI: [(u16, &str); 75] = [
    // WGS84 Geographic 2D
    (
        4326,
        r#"GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]]"#,
    ),
    // WGS84 Geographic 3D
    (
        4979,
        r#"GEOGCS["WGS_1984_3D",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433],LINUNIT["Meter",1.0]]"#,
    ),
    // WGS84 Geocentric (Geocentric CRS is not supported in WKT1_ESRI)
    // (4978, r#""#);
    // Web Mercator (WGS84 / Pseudo-Mercator)
    (
        3857,
        r#"PROJCS["WGS_1984_Web_Mercator_Auxiliary_Sphere",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],PARAMETER["Auxiliary_Sphere_Type",0.0],UNIT["Meter",1.0]]"#,
    ),
    // JGD2011 Geographic 2D
    (
        6668,
        r#"GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]]"#,
    ),
    // JGD2011 Geographic 3D
    (
        6697,
        r#"GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    // JGD2011 / Japan Plane Rectangular CS + JGD2011 (vertical) height
    // Note: Only I - XIII are defined (XIV - XIX does not exist)
    // CompositeCRS system is supported by QGIS 3.36 or higher and Windows version only.
    // Therefore, a 2D CRS is used even in the case of a composite coordinate system.
    (
        10162,
        r#"PROJCS["JGD_2011_Japan_Zone_1",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",129.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10163,
        r#"PROJCS["JGD_2011_Japan_Zone_2",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10164,
        r#"PROJCS["JGD_2011_Japan_Zone_3",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",132.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10165,
        r#"PROJCS["JGD_2011_Japan_Zone_4",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",133.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10166,
        r#"PROJCS["JGD_2011_Japan_Zone_5",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",134.333333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10167,
        r#"PROJCS["JGD_2011_Japan_Zone_6",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10168,
        r#"PROJCS["JGD_2011_Japan_Zone_7",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",137.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10169,
        r#"PROJCS["JGD_2011_Japan_Zone_8",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",138.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10170,
        r#"PROJCS["JGD_2011_Japan_Zone_9",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",139.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10171,
        r#"PROJCS["JGD_2011_Japan_Zone_10",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",40.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10172,
        r#"PROJCS["JGD_2011_Japan_Zone_11",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10173,
        r#"PROJCS["JGD_2011_Japan_Zone_12",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    (
        10174,
        r#"PROJCS["JGD_>2011_Japan_Zone_13",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",144.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]],VERTCS["JGD2011_vertical_height",VDATUM["Japanese_Geodetic_Datum_2011_vertical"],PARAMETER["Vertical_Shift",0.0],PARAMETER["Direction",1.0],UNIT["Meter",1.0]]"#,
    ),
    // JGD2011 / Japan Plane Rectangular CS
    (
        6669,
        r#"PROJCS["JGD_2011_Japan_Zone_1",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",129.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6670,
        r#"PROJCS["JGD_2011_Japan_Zone_2",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6671,
        r#"PROJCS["JGD_2011_Japan_Zone_3",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",132.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6672,
        r#"PROJCS["JGD_2011_Japan_Zone_4",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",133.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6673,
        r#"PROJCS["JGD_2011_Japan_Zone_5",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",134.333333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6674,
        r#"PROJCS["JGD_2011_Japan_Zone_6",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6675,
        r#"PROJCS["JGD_2011_Japan_Zone_7",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",137.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6676,
        r#"PROJCS["JGD_2011_Japan_Zone_8",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",138.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6677,
        r#"PROJCS["JGD_2011_Japan_Zone_9",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",139.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6678,
        r#"PROJCS["JGD_2011_Japan_Zone_10",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",40.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6679,
        r#"PROJCS["JGD_2011_Japan_Zone_11",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6680,
        r#"PROJCS["JGD_2011_Japan_Zone_12",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6681,
        r#"PROJCS["JGD_2011_Japan_Zone_13",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",144.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6682,
        r#"PROJCS["JGD_2011_Japan_Zone_14",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6683,
        r#"PROJCS["JGD_2011_Japan_Zone_15",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",127.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6684,
        r#"PROJCS["JGD_2011_Japan_Zone_16",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",124.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6685,
        r#"PROJCS["JGD_2011_Japan_Zone_17",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6686,
        r#"PROJCS["JGD_2011_Japan_Zone_18",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",20.0],UNIT["Meter",1.0]]"#,
    ),
    (
        6687,
        r#"PROJCS["JGD_2011_Japan_Zone_19",GEOGCS["GCS_JGD_2011",DATUM["D_JGD_2011",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",154.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    // JGD2000 / Japan Plane Rectangular CS
    (
        2443,
        r#"PROJCS["JGD_2000_Japan_Zone_1",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",129.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2444,
        r#"PROJCS["JGD_2000_Japan_Zone_2",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2445,
        r#"PROJCS["JGD_2000_Japan_Zone_3",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",132.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2446,
        r#"PROJCS["JGD_2000_Japan_Zone_4",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",133.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2447,
        r#"PROJCS["JGD_2000_Japan_Zone_5",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",134.333333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2448,
        r#"PROJCS["JGD_2000_Japan_Zone_6",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2449,
        r#"PROJCS["JGD_2000_Japan_Zone_7",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",137.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2450,
        r#"PROJCS["JGD_2000_Japan_Zone_8",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",138.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2451,
        r#"PROJCS["JGD_2000_Japan_Zone_9",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",139.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2452,
        r#"PROJCS["JGD_2000_Japan_Zone_10",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",40.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2453,
        r#"PROJCS["JGD_2000_Japan_Zone_11",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2454,
        r#"PROJCS["JGD_2000_Japan_Zone_12",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2455,
        r#"PROJCS["JGD_2000_Japan_Zone_13",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",144.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2456,
        r#"PROJCS["JGD_2000_Japan_Zone_14",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2457,
        r#"PROJCS["JGD_2000_Japan_Zone_15",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",127.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2458,
        r#"PROJCS["JGD_2000_Japan_Zone_16",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",124.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2459,
        r#"PROJCS["JGD_2000_Japan_Zone_17",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2460,
        r#"PROJCS["JGD_2000_Japan_Zone_18",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",20.0],UNIT["Meter",1.0]]"#,
    ),
    (
        2461,
        r#"PROJCS["JGD_2000_Japan_Zone_19",GEOGCS["GCS_JGD_2000",DATUM["D_JGD_2000",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",154.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    // Tokyo / Japan Plane Rectangular CS
    (
        30161,
        r#"PROJCS["Japan_Zone_1",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",129.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30162,
        r#"PROJCS["Japan_Zone_2",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30163,
        r#"PROJCS["Japan_Zone_3",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",132.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30164,
        r#"PROJCS["Japan_Zone_4",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",133.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",33.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30165,
        r#"PROJCS["Japan_Zone_5",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",134.333333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30166,
        r#"PROJCS["Japan_Zone_6",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30167,
        r#"PROJCS["Japan_Zone_7",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",137.166666666667],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30168,
        r#"PROJCS["Japan_Zone_8",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",138.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30169,
        r#"PROJCS["Japan_Zone_9",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",139.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",36.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30170,
        r#"PROJCS["Japan_Zone_10",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.833333333333],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",40.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30171,
        r#"PROJCS["Japan_Zone_11",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",140.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30172,
        r#"PROJCS["Japan_Zone_12",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30173,
        r#"PROJCS["Japan_Zone_13",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",144.25],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",44.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30174,
        r#"PROJCS["Japan_Zone_14",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",142.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30175,
        r#"PROJCS["Japan_Zone_15",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",127.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30176,
        r#"PROJCS["Japan_Zone_16",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",124.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30177,
        r#"PROJCS["Japan_Zone_17",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",131.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30178,
        r#"PROJCS["Japan_Zone_18",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",136.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",20.0],UNIT["Meter",1.0]]"#,
    ),
    (
        30179,
        r#"PROJCS["Japan_Zone_19",GEOGCS["GCS_Tokyo",DATUM["D_Tokyo",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",154.0],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",26.0],UNIT["Meter",1.0]]"#,
    ),
];

impl ProjectionRepository {
    pub fn new() -> ProjectionRepository {
        let mut wkt_map = HashMap::new();
        for &(code, wkt) in WKT1_ESRI.iter() {
            wkt_map.insert(code, wkt.to_string());
        }
        ProjectionRepository { wkt_map }
    }

    pub fn get_wkt(&self, epsg_code: &u16) -> Option<&str> {
        self.wkt_map.get(epsg_code).map(|s| s.as_str())
    }
}

pub fn write_prj(
    mut writer: impl Write,
    repo: &ProjectionRepository,
    epsg: &u16,
) -> Result<(), std::io::Error> {
    let wkt = repo.get_wkt(epsg);
    if wkt.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid EPSG code: {}", epsg),
        ));
    } else {
        writer.write_all(wkt.unwrap().as_bytes())?;
    }

    writer.flush()?;

    Ok(())
}
