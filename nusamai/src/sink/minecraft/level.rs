use fastnbt::{IntArray, Value};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Level {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "allowCommands")]
    pub allow_commands: i8,

    #[serde(rename = "BorderCenterX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_center_x: Option<f64>,

    #[serde(rename = "BorderCenterZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_center_z: Option<f64>,

    #[serde(rename = "BorderDamagePerBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_damage_per_block: Option<f64>,

    #[serde(rename = "BorderSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size: Option<f64>,

    #[serde(rename = "BorderSafeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_safe_zone: Option<f64>,

    #[serde(rename = "BorderSizeLerpTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size_lerp_target: Option<f64>,

    #[serde(rename = "BorderSizeLerpTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size_lerp_time: Option<i64>,

    #[serde(rename = "BorderWarningBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_warning_blocks: Option<f64>,

    #[serde(rename = "BorderWarningTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_warning_time: Option<f64>,

    #[serde(rename = "clearWeatherTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_weather_time: Option<i32>,

    #[serde(rename = "CustomBossEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_boss_events: Option<CustomBossEvents>,

    #[serde(rename = "DataPacks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_packs: Option<DataPacks>,

    #[serde(rename = "DataVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_version: Option<i32>,

    #[serde(rename = "DayTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_time: Option<i64>,

    #[serde(rename = "Difficulty")]
    pub difficulty: i8,

    #[serde(rename = "DifficultyLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_locked: Option<i8>,

    #[serde(rename = "DimensionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_data: Option<DimensionData>,

    #[serde(rename = "GameRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_rules: Option<GameRules>,

    #[serde(rename = "WorldGenSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_gen_settings: Option<WorldGenSettings>,

    #[serde(rename = "GameType")]
    pub game_type: i32,

    #[serde(rename = "generatorName")]
    pub generator_name: String,

    #[serde(rename = "generatorOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_options: Option<String>,

    #[serde(rename = "generatorVersion")]
    pub generator_version: i32,

    #[serde(rename = "hardcore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardcore: Option<i8>,

    #[serde(rename = "initialized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialized: Option<i8>,

    #[serde(rename = "LastPlayed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_played: Option<i64>,

    #[serde(rename = "LevelName")]
    pub level_name: String,

    #[serde(rename = "MapFeatures")]
    pub map_features: i8,

    #[serde(rename = "Player")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<Player>,

    #[serde(rename = "raining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raining: Option<i8>,

    #[serde(rename = "rainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain_time: Option<i32>,

    #[serde(rename = "RandomSeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,

    #[serde(rename = "SizeOnDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,

    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,

    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,

    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,

    #[serde(rename = "thundering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thundering: Option<i8>,

    #[serde(rename = "thunderTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_time: Option<i32>,

    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    #[serde(rename = "version")]
    pub nbt_version: i32,

    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<Version>,

    #[serde(rename = "WanderingTraderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_id: Option<IntArray>,

    #[serde(rename = "WanderingTraderSpawnChance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_spawn_chance: Option<i32>,

    #[serde(rename = "WanderingTraderSpawnDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_spawn_delay: Option<i32>,

    #[serde(rename = "WasModded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_modded: Option<i8>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            allow_commands: 1, // true
            difficulty: 0,     // Peaceful
            game_type: 1,      // Creative
            generator_name: "flat".to_string(),
            generator_version: 1,
            level_name: String::new(),
            map_features: 0, // No structures in place
            spawn_x: 0,
            spawn_y: 160, // Set higher than average building altitude
            spawn_z: 0,
            nbt_version: 19133,
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
            data_packs: None,
            data_version: None,
            day_time: None,
            difficulty_locked: None,
            dimension_data: None,
            game_rules: None,
            world_gen_settings: None,
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
            version_id: None,
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
pub struct DataPacks {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
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

#[derive(Serialize, Deserialize)]
pub struct WorldGenSettings {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}
