use fastnbt::{IntArray, Value};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Level {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    #[serde(rename = "allowCommands")]
    pub allow_commands: i8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_center_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_center_z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_damage_per_block: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_safe_zone: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size_lerp_target: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size_lerp_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_warning_blocks: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_warning_time: Option<f64>,

    #[serde(rename = "clearWeatherTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_weather_time: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_boss_events: Option<CustomBossEvents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_packs: Option<DataPacks>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_version: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_time: Option<i64>,

    pub difficulty: i8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_locked: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_data: Option<DimensionData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_rules: Option<GameRules>,

    pub world_gen_settings: WorldGenSettings,

    pub game_type: i32,

    #[serde(rename = "generatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_name: Option<String>,

    #[serde(rename = "generatorOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_options: Option<String>,

    #[serde(rename = "generatorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_version: Option<i32>,

    #[serde(rename = "hardcore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardcore: Option<i8>,

    #[serde(rename = "initialized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialized: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_played: Option<i64>,

    pub level_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_features: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<Player>,

    #[serde(rename = "raining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raining: Option<i8>,

    #[serde(rename = "rainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain_time: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,

    pub spawn_x: i32,

    pub spawn_y: i32,

    pub spawn_z: i32,

    #[serde(rename = "thundering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thundering: Option<i8>,

    #[serde(rename = "thunderTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_time: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    #[serde(rename = "version")]
    pub nbt_version: i32,

    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_id: Option<IntArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_spawn_chance: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_spawn_delay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_modded: Option<i8>,
}

impl Default for Data {
    fn default() -> Self {
        // let mut rng = rand::thread_rng();
        // let seed: u64 = rng.gen();

        Data {
            allow_commands: 1, // true
            difficulty: 0,     // Peaceful
            game_type: 1,      // Creative
            // generator_name: "flat".to_string(),
            generator_name: None,
            generator_version: None,
            level_name: String::new(),
            map_features: None,
            spawn_x: 0,
            spawn_y: 160, // Set higher than average building altitude
            spawn_z: 0,
            nbt_version: 19133,
            world_gen_settings: WorldGenSettings {
                bonus_chest: None,
                seed: Some(1),
                generate_features: 0,
                dimensions: Dimensions {
                    overworld_dimension_settings: DimensionSettings {
                        kind: "minecraft:overworld".to_string(),
                        generator: Generator {
                            kind: "minecraft:flat".to_string(),
                            settings: GeneratorSettings {
                                features: 0,
                                lakes: None,
                                layers: vec![],
                                structure_overrides: vec![
                                    "minecraft:strongholds".to_string(),
                                    "minecraft:villages".to_string(),
                                ],
                                biome: "minecraft:plains".to_string(),
                            },
                            biome_source: None,
                        },
                    },
                },
            },
            border_center_x: None,
            border_center_z: None,
            border_damage_per_block: None,
            border_size: None,
            border_safe_zone: None,
            border_size_lerp_target: None,
            border_size_lerp_time: None,
            border_warning_blocks: None,
            border_warning_time: None,
            clear_weather_time: None,
            custom_boss_events: None,
            data_packs: Some(DataPacks {
                disabled: vec!["bundle".to_string(), "update_1_20".to_string()],
                enabled: vec!["vanilla".to_string()],
            }),
            data_version: Some(3337),
            // data_version: None,
            day_time: None,
            difficulty_locked: None,
            dimension_data: None,
            game_rules: None,
            generator_options: None,
            hardcore: None,
            initialized: None,
            last_played: None,
            player: None,
            raining: None,
            rain_time: None,
            random_seed: None,
            size_on_disk: None,
            thundering: None,
            thunder_time: None,
            time: None,
            version: Some(Version {
                id: 3337,
                name: "1.19.4".to_string(),
                series: "main".to_string(),
                snapshot: Some(1),
            }),
            wandering_trader_id: None,
            wandering_trader_spawn_chance: None,
            wandering_trader_spawn_delay: None,
            was_modded: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomBossEvents {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct DataPacks {
    pub disabled: Vec<String>,
    pub enabled: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionData {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct GameRules {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorldGenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_chest: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    pub generate_features: i8,
    pub dimensions: Dimensions,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dimensions {
    // Future implementation of the Nether and the End.
    #[serde(rename = "minecraft:overworld")]
    pub overworld_dimension_settings: DimensionSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DimensionSettings {
    #[serde(rename = "type")]
    pub kind: String,
    pub generator: Generator,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Generator {
    #[serde(rename = "type")]
    pub kind: String,
    pub settings: GeneratorSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biome_source: Option<BiomeSource>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratorSettings {
    pub features: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakes: Option<i8>,
    pub layers: Vec<Layer>,
    pub structure_overrides: Vec<String>,
    pub biome: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BiomeSource {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Layer {
    pub block: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct Version {
    pub id: i32,
    pub name: String,
    pub series: String,
    pub snapshot: Option<i8>,
}
