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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_commands: Option<i8>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_locked: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_data: Option<DimensionData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_rules: Option<GameRules>,

    pub world_gen_settings: WorldGenSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_type: Option<i32>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_name: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_z: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<Vec<String>>,

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
        Data {
            allow_commands: Some(1), // Allow execution of the command
            difficulty: Some(0),     // Peaceful
            game_type: Some(1),      // Creative
            generator_name: None,
            generator_version: None,
            level_name: Some(String::new()),
            map_features: None,
            spawn_x: Some(0),
            spawn_y: Some(160), // Set higher than average building altitude
            spawn_z: Some(0),
            nbt_version: 19133,
            enabled_features: None,
            world_gen_settings: WorldGenSettings {
                bonus_chest: Some(0),
                seed: Some(0),
                generate_features: Some(0),
                dimensions: Dimensions {
                    overworld_dimension_settings: FlatDimensionSettings {
                        kind: "minecraft:overworld".to_string(),
                        generator: FlatGenerator {
                            kind: "minecraft:flat".to_string(),
                            settings: FlatGeneratorSettings {
                                features: 1,
                                lakes: Some(0),
                                layers: vec![Layer {
                                    block: "minecraft:air".to_string(),
                                    height: 1,
                                }],
                                structure_overrides: vec![],
                                biome: "minecraft:the_void".to_string(),
                            },
                        },
                    },
                    the_nether_dimension_settings: NoiseDimensionSettings {
                        kind: "minecraft:the_nether".to_string(),
                        generator: NoiseGenerator {
                            kind: "minecraft:noise".to_string(),
                            settings: "minecraft:nether".to_string(),
                            biome_source: BiomeSource {
                                kind: "minecraft:multi_noise".to_string(),
                                preset: Some("minecraft:nether".to_string()),
                            },
                        },
                    },
                    the_end_dimension_settings: NoiseDimensionSettings {
                        kind: "minecraft:the_end".to_string(),
                        generator: NoiseGenerator {
                            kind: "minecraft:noise".to_string(),
                            settings: "minecraft:end".to_string(),
                            biome_source: BiomeSource {
                                kind: "minecraft:the_end".to_string(),
                                preset: None,
                            },
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
                disabled: Some(vec!["bundle".to_string(), "update_1_20".to_string()]),
                enabled: Some(vec!["vanilla".to_string()]),
            }),
            data_version: Some(3337),
            day_time: None,
            difficulty_locked: None,
            dimension_data: None,
            game_rules: None,
            generator_options: None,
            hardcore: Some(0),
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
            version: None,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_features: Option<i8>,
    pub dimensions: Dimensions,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dimensions {
    // Future implementation of the Nether and the End.
    #[serde(rename = "minecraft:overworld")]
    pub overworld_dimension_settings: FlatDimensionSettings,
    #[serde(rename = "minecraft:the_nether")]
    pub the_nether_dimension_settings: NoiseDimensionSettings,
    #[serde(rename = "minecraft:the_end")]
    pub the_end_dimension_settings: NoiseDimensionSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FlatDimensionSettings {
    #[serde(rename = "type")]
    pub kind: String,
    pub generator: FlatGenerator,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FlatGenerator {
    #[serde(rename = "type")]
    pub kind: String,
    pub settings: FlatGeneratorSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FlatGeneratorSettings {
    pub features: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakes: Option<i8>,
    pub layers: Vec<Layer>,
    pub structure_overrides: Vec<String>,
    pub biome: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NoiseDimensionSettings {
    #[serde(rename = "type")]
    pub kind: String,
    pub generator: NoiseGenerator,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NoiseGenerator {
    #[serde(rename = "type")]
    pub kind: String,
    pub settings: String,
    pub biome_source: BiomeSource,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BiomeSource {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Layer {
    pub block: String,
    pub height: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct Version {
    pub id: i32,
    pub name: String,
    pub series: String,
    pub snapshot: Option<i8>,
}
