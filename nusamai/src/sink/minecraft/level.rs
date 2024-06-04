use fastnbt::Value;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Level {
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(rename = "allowCommands")]
    allow_commands: bool,
    #[serde(rename = "BorderCenterX")]
    border_center_x: f64,
    #[serde(rename = "BorderCenterZ")]
    border_center_z: f64,
    #[serde(rename = "BorderDamagePerBlock")]
    border_damage_per_block: f64,
    #[serde(rename = "BorderSize")]
    border_size: f64,
    #[serde(rename = "BorderSafeZone")]
    border_safe_zone: f64,
    #[serde(rename = "BorderSizeLerpTarget")]
    border_size_lerp_target: f64,
    #[serde(rename = "BorderSizeLerpTime")]
    border_size_lerp_time: i64,
    #[serde(rename = "BorderWarningBlocks")]
    border_warning_blocks: f64,
    #[serde(rename = "BorderWarningTime")]
    border_warning_time: f64,
    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,
    #[serde(rename = "CustomBossEvents")]
    custom_boss_events: CustomBossEvents,
    #[serde(rename = "DataPacks")]
    data_packs: DataPacks,
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "DayTime")]
    day_time: i64,
    #[serde(rename = "Difficulty")]
    difficulty: i8,
    #[serde(rename = "DifficultyLocked")]
    difficulty_locked: bool,
    #[serde(rename = "DimensionData")]
    dimension_data: DimensionData,
    #[serde(rename = "GameRules")]
    game_rules: GameRules,
    #[serde(rename = "WorldGenSettings")]
    world_gen_settings: WorldGenSettings,
    #[serde(rename = "GameType")]
    game_type: i32,
    #[serde(rename = "generatorName")]
    generator_name: String,
    #[serde(rename = "generatorOptions")]
    generator_options: GeneratorOptions,
    #[serde(rename = "generatorVersion")]
    generator_version: i32,
    #[serde(rename = "hardcore")]
    hardcore: bool,
    #[serde(rename = "initialized")]
    initialized: bool,
    #[serde(rename = "LastPlayed")]
    last_played: i64,
    #[serde(rename = "LevelName")]
    level_name: String,
    #[serde(rename = "MapFeatures")]
    map_features: bool,
    #[serde(rename = "Player")]
    player: Player,
    #[serde(rename = "raining")]
    raining: bool,
    #[serde(rename = "rainTime")]
    rain_time: i32,
    #[serde(rename = "RandomSeed")]
    random_seed: i64,
    #[serde(rename = "SizeOnDisk")]
    size_on_disk: i64,
    #[serde(rename = "SpawnX")]
    spawn_x: i32,
    #[serde(rename = "SpawnY")]
    spawn_y: i32,
    #[serde(rename = "SpawnZ")]
    spawn_z: i32,
    #[serde(rename = "thundering")]
    thundering: bool,
    #[serde(rename = "thunderTime")]
    thunder_time: i32,
    #[serde(rename = "Time")]
    time: i64,
    #[serde(rename = "version")]
    nbt_version: i32,
    #[serde(rename = "Version")]
    version_id: Version,
    #[serde(rename = "WanderingTraderId")]
    wandering_trader_id: Vec<i32>,
    #[serde(rename = "WanderingTraderSpawnChance")]
    wandering_trader_spawn_chance: i32,
    #[serde(rename = "WanderingTraderSpawnDelay")]
    wandering_trader_spawn_delay: i32,
    #[serde(rename = "WasModded")]
    was_modded: bool,
}

#[derive(Serialize, Deserialize)]
struct CustomBossEvents {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct DataPacks {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct DimensionData {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct GameRules {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct GeneratorOptions {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Player {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WorldGenSettings {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Version {
    #[serde(flatten)]
    other: HashMap<String, Value>,
}
