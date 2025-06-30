use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum ChampionKind {
    Aatrox,
    Ahri,
    Akali,
    Akshan,
    Alistar,
    Ambessa,
    Amumu,
    Anivia,
    Annie,
    // Aphelios,
    // Ashe,
    AurelionSol,
    Aurora,
    Azir,
    Bard,
    Belveth,
    Blitzcrank,
    Brand,
    Braum,
    Briar,
    Caitlyn,
    // Camille,
    Cassiopeia,
    Chogath,
    Corki,
    Darius,
    Diana,
    DrMundo,
    Draven,
    Ekko,
    Elise,
    Evelynn,
    Ezreal,
    Fiddlesticks,
    Fiora,
    Fizz,
    Galio,
    Gangplank,
    Garen,
    Gnar,
    Gragas,
    Graves,
    Gwen,
    Hecarim,
    // Heimerdinger,
    // Hwei,
    Illaoi,
    Irelia,
    Ivern,
    Janna,
    JarvanIV,
    Jax,
    Jayce,
    Jhin,
    Jinx,
    Kaisa,
    Kalista,
    Karma,
    Karthus,
    Kassadin,
    Katarina,
    Kayle,
    Kayn,
    Kennen,
    Khazix,
    Kindred,
    Kled,
    KogMaw,
    KSante,
    Leblanc,
    LeeSin,
    Leona,
    Lillia,
    Lissandra,
    Lucian,
    Lulu,
    Lux,
    Malphite,
    Malzahar,
    Maokai,
    MasterYi,
    Milio,
    MissFortune,
    Mordekaiser,
    Morgana,
    Naafiri,
    Nami,
    Nasus,
    Nautilus,
    Neeko,
    Nidalee,
    Nilah,
    Nocturne,
    Nunu,
    Olaf,
    Orianna,
    Ornn,
    Pantheon,
    Poppy,
    Pyke,
    Qiyana,
    Quinn,
    Rakan,
    Rammus,
    RekSai,
    Rell,
    Renata,
    Renekton,
    Rengar,
    Riven,
    Rumble,
    Ryze,
    Samira,
    Sejuani,
    Senna,
    Seraphine,
    Sett,
    Shaco,
    Shen,
    Shyvana,
    Singed,
    Sion,
    Sivir,
    Skarner,
    Smolder,
    Sona,
    Soraka,
    Swain,
    Sylas,
    Syndra,
    TahmKench,
    Taliyah,
    Talon,
    Taric,
    Teemo,
    Thresh,
    Tristana,
    Trundle,
    Tryndamere,
    TwistedFate,
    Twitch,
    Udyr,
    Urgot,
    Varus,
    Vayne,
    Veigar,
    Velkoz,
    Vex,
    Vi,
    Viego,
    Viktor,
    Vladimir,
    Volibear,
    Warwick,
    MonkeyKing,
    Xayah,
    Xerath,
    XinZhao,
    Yasuo,
    Yone,
    Yorick,
    Yuumi,
    Zac,
    Zed,
    Zeri,
    Ziggs,
    Zilean,
    Zoe,
    Zyra,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub title: String,
    pub full_name: String,
    pub icon: String,
    pub resource: String,
    pub attack_type: String,
    pub adaptive_type: String,
    pub stats: Stats,
    pub positions: Vec<String>,
    pub roles: Vec<String>,
    pub attribute_ratings: AttributeRatings,
    pub abilities: Abilities,
    pub release_date: String,
    pub release_patch: String,
    pub patch_last_changed: String,
    pub price: Price,
    pub lore: Option<String>,
    pub faction: String,
    pub skins: Vec<Skin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub health: Stat,
    pub health_regen: Stat,
    pub mana: Stat,
    pub mana_regen: Stat,
    pub armor: Stat,
    pub magic_resistance: Stat,
    pub attack_damage: Stat,
    pub movespeed: Stat,
    pub acquisition_radius: Stat,
    pub selection_radius: Stat,
    pub pathing_radius: Stat,
    pub gameplay_radius: Stat,
    pub critical_strike_damage: Stat,
    pub critical_strike_damage_modifier: Stat,
    pub attack_speed: Stat,
    pub attack_speed_ratio: Stat,
    pub attack_cast_time: Stat,
    pub attack_total_time: Stat,
    pub attack_delay_offset: Stat,
    pub attack_range: Stat,
    pub aram_damage_taken: Stat,
    pub aram_damage_dealt: Stat,
    pub aram_healing: Stat,
    pub aram_shielding: Stat,
    pub aram_tenacity: Stat,
    pub aram_ability_haste: Stat,
    pub aram_attack_speed: Stat,
    pub aram_energy_regen: Stat,
    pub urf_damage_taken: Stat,
    pub urf_damage_dealt: Stat,
    pub urf_healing: Stat,
    pub urf_shielding: Stat,
}

fn first_in_vec<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v: Vec<T> = Vec::deserialize(deserializer)?;
    v.into_iter()
        .next()
        .ok_or_else(|| D::Error::custom("Expected at least one ability"))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "P", deserialize_with = "first_in_vec")]
    pub p: Ability,
    #[serde(rename = "Q", deserialize_with = "first_in_vec")]
    pub q: Ability,
    #[serde(rename = "W", deserialize_with = "first_in_vec")]
    pub w: Ability,
    #[serde(rename = "E", deserialize_with = "first_in_vec")]
    pub e: Ability,
    #[serde(rename = "R", deserialize_with = "first_in_vec")]
    pub r: Ability,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub name: String,
    pub icon: String,
    pub effects: Vec<Effect>,
    pub cost: Value,
    pub cooldown: Option<Cooldown>,
    pub targeting: String,
    pub affects: Option<String>,
    pub spellshieldable: Option<String>,
    pub resource: Value,
    pub damage_type: Option<String>,
    pub spell_effects: Option<String>,
    pub projectile: Value,
    pub on_hit_effects: Value,
    pub occurrence: Value,
    pub notes: Option<String>,
    pub blurb: Option<String>,
    pub missile_speed: Value,
    pub recharge_rate: Value,
    pub collision_radius: Value,
    pub tether_radius: Value,
    pub on_target_cd_static: Value,
    pub inner_radius: Value,
    pub speed: Value,
    pub width: Value,
    pub angle: Value,
    pub cast_time: Value,
    pub effect_radius: Value,
    pub target_range: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeRatings {
    damage: i64,
    toughness: i64,
    control: i64,
    mobility: i64,
    utility: i64,
    ability_reliance: i64,
    difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub description: String,
    pub leveling: Vec<Leveling>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub modifiers: Vec<Modifier>,
    pub affected_by_cdr: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modifier {
    pub values: Vec<f32>,
    pub units: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leveling {
    pub attribute: String,
    pub modifiers: Vec<Modifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub blue_essence: i64,
    pub rp: i64,
    pub sale_rp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub name: String,
    pub id: i64,
    pub is_base: bool,
    pub availability: String,
    pub format_name: String,
    pub loot_eligible: bool,
    pub cost: Value,
    pub sale: i64,
    pub distribution: Option<String>,
    pub rarity: String,
    pub chromas: Vec<Option<Chroma>>,
    pub lore: Option<String>,
    pub release: String,
    pub set: Vec<String>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub load_screen_vintage_path: Option<String>,
    pub new_effects: bool,
    pub new_animations: bool,
    pub new_recall: bool,
    pub new_voice: bool,
    pub new_quotes: bool,
    pub voice_actor: Vec<String>,
    pub splash_artist: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chroma {
    pub name: String,
    pub id: i64,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<Rarity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub description: Option<String>,
    pub region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rarity {
    pub rarity: Option<i64>,
    pub region: Option<String>,
}
