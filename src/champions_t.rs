use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub lore: String,
    pub faction: String,
    pub skins: Vec<Skin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub health: Health,
    pub health_regen: HealthRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
    pub armor: Armor,
    pub magic_resistance: MagicResistance,
    pub attack_damage: AttackDamage,
    pub movespeed: Movespeed,
    pub acquisition_radius: AcquisitionRadius,
    pub selection_radius: SelectionRadius,
    pub pathing_radius: PathingRadius,
    pub gameplay_radius: GameplayRadius,
    pub critical_strike_damage: CriticalStrikeDamage,
    pub critical_strike_damage_modifier: CriticalStrikeDamageModifier,
    pub attack_speed: AttackSpeed,
    pub attack_speed_ratio: AttackSpeedRatio,
    pub attack_cast_time: AttackCastTime,
    pub attack_total_time: AttackTotalTime,
    pub attack_delay_offset: AttackDelayOffset,
    pub attack_range: AttackRange,
    pub aram_damage_taken: AramDamageTaken,
    pub aram_damage_dealt: AramDamageDealt,
    pub aram_healing: AramHealing,
    pub aram_shielding: AramShielding,
    pub aram_tenacity: AramTenacity,
    pub aram_ability_haste: AramAbilityHaste,
    pub aram_attack_speed: AramAttackSpeed,
    pub aram_energy_regen: AramEnergyRegen,
    pub urf_damage_taken: UrfDamageTaken,
    pub urf_damage_dealt: UrfDamageDealt,
    pub urf_healing: UrfHealing,
    pub urf_shielding: UrfShielding,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthRegen {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mana {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManaRegen {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicResistance {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackDamage {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movespeed {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcquisitionRadius {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRadius {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathingRadius {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameplayRadius {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriticalStrikeDamage {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriticalStrikeDamageModifier {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackSpeed {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackSpeedRatio {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackCastTime {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackTotalTime {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackDelayOffset {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackRange {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramDamageTaken {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramDamageDealt {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramHealing {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramShielding {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramTenacity {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramAbilityHaste {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramAttackSpeed {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AramEnergyRegen {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrfDamageTaken {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrfDamageDealt {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrfHealing {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrfShielding {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_per_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeRatings {
    pub damage: i64,
    pub toughness: i64,
    pub control: i64,
    pub mobility: i64,
    pub utility: i64,
    pub ability_reliance: i64,
    pub difficulty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "P")]
    pub p: Vec<Ability>,
    #[serde(rename = "Q")]
    pub q: Vec<Ability>,
    #[serde(rename = "W")]
    pub w: Vec<Ability>,
    #[serde(rename = "E")]
    pub e: Vec<Ability>,
    #[serde(rename = "R")]
    pub r: Vec<Ability>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub effects: Option<Vec<Effect>>,
    pub cost: Option<Value>,
    pub cooldown: Option<Cooldown>,
    pub targeting: Option<String>,
    pub affects: Option<String>,
    pub spellshieldable: Option<String>,
    pub resource: Option<Value>,
    pub damage_type: Option<String>,
    pub spell_effects: Option<String>,
    pub projectile: Option<Value>,
    pub on_hit_effects: Option<Value>,
    pub occurrence: Option<Value>,
    pub notes: Option<String>,
    pub blurb: Option<String>,
    pub missile_speed: Option<Value>,
    pub recharge_rate: Option<Value>,
    pub collision_radius: Option<Value>,
    pub tether_radius: Option<Value>,
    pub on_target_cd_static: Option<Value>,
    pub inner_radius: Option<Value>,
    pub speed: Option<Value>,
    pub width: Option<Value>,
    pub angle: Option<Value>,
    pub cast_time: Option<Value>,
    pub effect_radius: Option<Value>,
    pub target_range: Option<Value>,
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
    pub values: Vec<f64>,
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
    pub chromas: Vec<Chroma>,
    pub lore: String,
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
