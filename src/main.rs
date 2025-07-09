use std::fs;
use std::io::Write;
use std::str::FromStr;
use std::{path::Path, process::Command};
mod champions_t;
use strum::IntoEnumIterator;

use crate::champions_t::{
    AbilitiesRaw, AbilityRaw, ChampionKind, ChampionRaw, Leveling, StatRaw, StatsRaw,
};

#[derive(Copy, Clone)]
enum AbilityLevel {
    One,
    Two,
    Three,
    Four,
    Five,
}

enum AbilitySlot {
    P,
    Q,
    W,
    E,
    R,
}

impl ChampionRaw {
    fn get_ability(&self, ability_kind: AbilitySlot) -> &AbilityRaw {
        match ability_kind {
            AbilitySlot::P => &self.abilities.p,
            AbilitySlot::Q => &self.abilities.q,
            AbilitySlot::W => &self.abilities.w,
            AbilitySlot::E => &self.abilities.e,
            AbilitySlot::R => &self.abilities.r,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
enum StatType {
    #[default]
    AD,
    BonusAD,
    FlatArmorPen,
    PercentArmorPen,
    AP,
    FlatMagicPen,
    PercentMagicPen,
    MaxHealth,
    CurrentHealth,
    BonusHealth,
    MissingHealth,
    Armor,
    BonusArmor,
    MagicResist,
    BonusMagicResist,
    BonusMana,
    BonusMoveSpeed,
}

#[derive(Default, Debug, Copy, Clone)]
enum ScalingType {
    #[default]
    PercentStat,
    Per100Stat,
}

#[derive(Debug, Default)]
struct AbilityScaling {
    stat_type: StatType,
    ratio: f32,
    kind: ScalingType,
    per_stat: Option<StatType>,
}

impl AbilityScaling {
    fn scaled_value(&self, stat_value: f32) -> f32 {
        match self.kind {
            ScalingType::PercentStat => (self.ratio / 100.0) * stat_value,
            ScalingType::Per100Stat => (self.ratio / 100.0) * (stat_value / 100.0),
        }
    }

    fn from_parts(kind: &str, value: f32) -> Self {
        match kind {
            // AD Scalings
            "% AD" => AbilityScaling {
                stat_type: StatType::AD,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% bonus AD" => AbilityScaling {
                stat_type: StatType::BonusAD,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% per 100 AD" => AbilityScaling {
                stat_type: StatType::AD,
                ratio: value,
                kind: ScalingType::Per100Stat,
                per_stat: None,
            },
            "% per 100 bonus AD" => AbilityScaling {
                stat_type: StatType::BonusAD,
                ratio: value,
                kind: ScalingType::Per100Stat,
                per_stat: None,
            },

            // AP Scalings
            "% AP" => AbilityScaling {
                stat_type: StatType::AP,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% per 100 AP" => AbilityScaling {
                stat_type: StatType::AP,
                ratio: value,
                kind: ScalingType::Per100Stat,
                per_stat: None,
            },
            "% of Ivern's AP" => AbilityScaling {
                stat_type: StatType::AP,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of Sona's AP" => AbilityScaling {
                stat_type: StatType::AP,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% AP per 100 bonus health" => AbilityScaling {
                stat_type: StatType::AP,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: Some(StatType::BonusHealth),
            },

            // Health Scalings
            "% of her maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of target's maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of Braum's maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of Zac's maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of target's current health" => AbilityScaling {
                stat_type: StatType::CurrentHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "%  of target's current health" => AbilityScaling {
                stat_type: StatType::CurrentHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of target's missing health" => AbilityScaling {
                stat_type: StatType::MissingHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% bonus health" => AbilityScaling {
                stat_type: StatType::BonusHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of his bonus health" => AbilityScaling {
                stat_type: StatType::BonusHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% per 100 Pantheon's bonus health" => AbilityScaling {
                stat_type: StatType::BonusHealth,
                ratio: value,
                kind: ScalingType::Per100Stat,
                per_stat: None,
            },

            // Armor Scalings
            "% armor" => AbilityScaling {
                stat_type: StatType::Armor,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% bonus armor" => AbilityScaling {
                stat_type: StatType::BonusArmor,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% per 100 bonus armor" => AbilityScaling {
                stat_type: StatType::BonusArmor,
                ratio: value,
                kind: ScalingType::Per100Stat,
                per_stat: None,
            },

            // Magic Restistance Scalings
            "% bonus magic resistance" => AbilityScaling {
                stat_type: StatType::BonusMagicResist,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },

            // Mana Scaling
            "% bonus mana" => AbilityScaling {
                stat_type: StatType::BonusMana,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },

            // Movement Speed Scalings
            "% bonus movement speed" => AbilityScaling {
                stat_type: StatType::BonusMoveSpeed,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },

            _ => {
                eprintln!("No ability scaling for kind: {kind}");
                AbilityScaling::default()
            }
        }
    }
}

#[derive(Default, Debug)]
enum BaseDamage {
    #[default]
    None,
    Flat(f32),
    PercentTargetMaxHealth(f32),
    PercentTargetMissingHealth(f32),
    PercentResource(f32),
}

#[derive(Default, Debug)]
struct DamageComponent {
    damage_type: DamageType,
    base: BaseDamage,
    scalings: Vec<AbilityScaling>,
}

impl DamageComponent {
    fn base_damage(&self, source_champion: &Champion, target_champion: &Champion) -> f32 {
        match self.base {
            BaseDamage::Flat(value) => value,
            BaseDamage::PercentTargetMaxHealth(value) => {
                target_champion
                    .stats
                    .stat_at_level(StatType::MaxHealth, target_champion.level)
                    * (value / 100.0)
            }
            BaseDamage::PercentTargetMissingHealth(_) => todo!(),
            BaseDamage::PercentResource(_) => todo!(),
            BaseDamage::None => todo!(),
        }
    }

    fn total_scaled_bonus(&self, source_champion: &Champion) -> f32 {
        self.scalings
            .iter()
            .map(|scaling| {
                scaling.scaled_value(
                    source_champion
                        .stats
                        .stat_at_level(scaling.stat_type, source_champion.level),
                )
            })
            .sum()
    }
}

#[derive(Default, Copy, Clone, Debug)]
enum DamageType {
    #[default]
    None,
    True,
    Physical,
    Magic,
    Mixed,
}

impl DamageType {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "PHYSICAL_DAMAGE" => DamageType::Physical,
            "MAGIC_DAMAGE" => DamageType::Magic,
            "TRUE_DAMAGE" => DamageType::True,
            "MIXED_DAMAGE" => DamageType::Mixed,
            // Skarner W deals magic damage but is classified as OTHER for some reason...
            "OTHER_DAMAGE" => DamageType::Magic,
            _ => DamageType::None,
        }
    }
}

#[derive(Debug)]
struct AbilityDamage {
    components: Vec<DamageComponent>,
}

fn leveling_contains_damage_effect(leveling: &Leveling) -> bool {
    let mut result = false;

    if leveling.attribute.to_lowercase().contains("damage") {
        result = true;
    }

    result
}

fn ability_damage(
    levelings: &[Leveling],
    ability: &AbilityRaw,
    ability_rank: AbilityLevel,
) -> AbilityDamage {
    let mut result = Vec::new();

    for leveling in levelings {
        if !leveling_contains_damage_effect(leveling) {
            continue;
        }

        let damage_type = match try_get_damage_type(leveling) {
            None => {
                if let Some(kind) = &ability.damage_type {
                    DamageType::from_str(kind)
                } else {
                    DamageType::None
                }
            }
            Some(kind) => kind,
        };

        for modifier in &leveling.modifiers {
            if let Some(unit_str) = modifier.units.first() {
                if let Some(value) = modifier.values.get(ability_rank as usize) {
                    if is_flat_damage(unit_str) {
                        result.push(DamageComponent {
                            damage_type,
                            base: BaseDamage::Flat(*value),
                            scalings: vec![],
                        });
                    } else if is_percent_damage_with_scaling(unit_str) {
                        let scaling = parse_percent_damage_with_scaling(unit_str);
                        if is_percent_target_max_health(unit_str) {
                            result.push(DamageComponent {
                                damage_type,
                                base: BaseDamage::PercentTargetMaxHealth(*value),
                                scalings: vec![scaling],
                            });
                        }
                    } else if is_percent_target_max_health(unit_str) {
                        result.push(DamageComponent {
                            damage_type,
                            base: BaseDamage::PercentTargetMaxHealth(*value),
                            scalings: vec![],
                        });
                    } else {
                        let scaling = AbilityScaling::from_parts(unit_str, *value);
                        if let Some(last_part) = result.last_mut() {
                            last_part.scalings.push(scaling);
                        } else {
                            result.push(DamageComponent {
                                damage_type,
                                base: BaseDamage::None,
                                scalings: vec![scaling],
                            });
                        }
                    }
                }
            }
        }
    }

    AbilityDamage { components: result }
}

fn is_percent_damage_with_scaling(unit_str: &str) -> bool {
    unit_str.contains("(+") && unit_str.contains("per 100")
}

fn parse_percent_damage_with_scaling(unit_str: &str) -> AbilityScaling {
    let mut scaling_value = 0.0;
    let mut scaling_unit_str = "";

    if let Some(start) = unit_str.find("(+") {
        if let Some(end) = unit_str[start..].find(")") {
            let scaling_str = &unit_str[start + 3..start + end];
            if let Some(percent_idx) = scaling_str.find("%") {
                scaling_unit_str = &scaling_str[percent_idx..];
                let value_str = &scaling_str[..percent_idx].trim();
                if let Ok(value) = value_str.parse::<f32>() {
                    scaling_value = value;
                }
            }
        }
    }

    AbilityScaling::from_parts(scaling_unit_str, scaling_value)
}

fn try_get_damage_type(leveling: &Leveling) -> Option<DamageType> {
    // Try to determine damage type based on attribute
    if leveling.attribute.to_lowercase().contains("true") {
        Some(DamageType::True)
    } else if leveling.attribute.to_lowercase().contains("mixed") {
        Some(DamageType::Mixed)
    } else if leveling.attribute.to_lowercase().contains("magic") {
        Some(DamageType::Magic)
    } else if leveling.attribute.to_lowercase().contains("physical") {
        Some(DamageType::Physical)
    } else {
        None
    }
}

fn ability_damage_by_level(
    ability_slot: AbilitySlot,
    ability_level: AbilityLevel,
    champion: &Champion,
) -> Option<Vec<AbilityDamage>> {
    let ability = champion.get_ability(ability_slot);
    // Check if ability is damaging ability
    ability.damage_type.as_ref()?;

    // TODO: Resistances Reduction is a valid thing, that i should take into account
    // TODO: Somehow handle abilties that buff auto attacks
    // TODO: We cannot exclude entire levelings based on this, since other modifiers might hold damage
    let exclude_patterns = [
        "monster",
        "duration",
        "minion",
        "movement",
        "slow",
        "range",
        "refunded",
        "healing",
        "restored",
        "heal",
        "damage reduction",
        "structure",
        "non-champion",
    ];

    let filtered_effects: Vec<_> = ability
        .effects
        .iter()
        .filter_map(|effect| {
            let filtered_levelings: Vec<_> = effect
                .leveling
                .iter()
                .filter(|leveling| {
                    !should_exclude_attribute(&leveling.attribute, &exclude_patterns)
                })
                .cloned()
                .collect();
            if !filtered_levelings.is_empty() {
                let mut effect = effect.clone();
                effect.leveling = filtered_levelings;
                Some(effect)
            } else {
                None
            }
        })
        .collect();

    let max_patterns = ["Maximum", "Total"];

    let mut result = Vec::new();

    let mut max_levelings = Vec::new();

    // Collect all levelings that contain "max_patterns" in their attribute
    for effect in &filtered_effects {
        for leveling in &effect.leveling {
            if max_patterns
                .iter()
                .any(|pat| leveling.attribute.contains(pat))
            {
                max_levelings.push(leveling.clone());
            }
        }
    }

    if !max_levelings.is_empty() {
        // If we have levelings that contain "max_patterns", we need to check if the damage type is mixed.
        // If so we exclude the "max_pattern" leveling, and construct the ability damage from the individual levelings
        // that make up the maximum. We do this because we need to know the actual damage type of every damage component,
        // such that we can apply damage mitigation later.
        let is_mixed_damage = max_levelings
            .iter()
            .any(|leveling| leveling.attribute.to_lowercase().contains("mixed"));

        if is_mixed_damage {
            for effect in &filtered_effects {
                let damage_components: Vec<Leveling> = effect
                    .leveling
                    .iter()
                    .filter(|leveling| {
                        !max_patterns
                            .iter()
                            .any(|pat| leveling.attribute.contains(pat))
                    })
                    .cloned()
                    .collect();

                result.push(ability_damage(&damage_components, ability, ability_level));
            }
        } else {
            // Here we pass levelings individually, instead of passing the max_levelings vec, because we might have
            // abilities with multiple effects containing a "max_pattern" such as Aatrox Q that has different variants
            // (Sweetspot / Non-Sweetspot) where we want an individual AbilityDamage for each variant.
            // NOTE: We assume/assert that an ability variant only ever has one "max_pattern" leveling.
            for leveling in max_levelings {
                result.push(ability_damage(&[leveling], ability, ability_level));
            }
        }
    } else {
        // No leveling had a "max_pattern" so we just pass all levelings in all effects
        for effect in &filtered_effects {
            result.push(ability_damage(&effect.leveling, ability, ability_level));
        }
    }

    Some(result)
}

fn is_percent_target_max_health(unit_str: &str) -> bool {
    let mut result = false;

    if unit_str.contains("target's maximum health") {
        result = true;
    }

    result
}

fn is_flat_damage(unit_str: &str) -> bool {
    let mut result = false;
    if unit_str.is_empty() {
        result = true;
    }

    result
}

fn patch_champion(champion: &mut ChampionRaw) {
    if let Ok(champion_kind) = ChampionKind::from_str(&champion.name) {
        match champion_kind {
            ChampionKind::Aatrox => patch_aatrox(champion),
            ChampionKind::Ahri => patch_ahri(champion),
            _ => {}
        }
    }
}

fn patch_aatrox(champion: &mut ChampionRaw) {
    let q = &mut champion.abilities.q;

    let mut new_effects = Vec::new();
    for effect in &q.effects {
        for leveling in &effect.leveling {
            let attr = leveling.attribute.to_lowercase();
            if attr.contains("cast damage") || attr.contains("sweetspot damage") {
                let mut new_effect = effect.clone();
                new_effect.leveling = vec![leveling.clone()];
                new_effects.push(new_effect);
            }
        }
    }
    if !new_effects.is_empty() {
        q.effects = new_effects
    }
}

fn patch_ahri(champion: &mut ChampionRaw) {
    let q = &mut champion.abilities.q;

    if let Some(effect) = q.effects.first_mut() {
        if let Some(leveling) = effect.leveling.first_mut() {
            leveling.attribute = String::from("Magic Damage");
            let mut new_leveling = leveling.clone();
            new_leveling.attribute = String::from("True Damage");
            effect.leveling.insert(1, new_leveling);
        }
    }
}

#[derive(Default, Clone, Debug)]
struct BaseStat {
    flat: f32,
    percent: f32,
    per_level: f32,
    percent_per_level: f32,
}

impl BaseStat {
    fn total(&self, champion_level: i32) -> f32 {
        let mut result = 0.0;
        result += self.flat;

        if self.per_level != 0.0 {
            let level = champion_level - 1;
            if level > 0 {
                result += self.per_level * (level as f32 * (0.7025 + 0.0175 * level as f32));
            }
        }

        if self.percent != 0.0 {
            result += self.flat * self.percent / 100.0;
        }

        if self.percent_per_level != 0.0 {
            result += self.flat * self.percent_per_level / 100.0 * champion_level as f32;
        }

        result
    }
}

impl From<StatRaw> for BaseStat {
    fn from(value: StatRaw) -> Self {
        Self {
            flat: value.flat,
            percent: value.percent,
            per_level: value.per_level,
            percent_per_level: value.percent_per_level,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Stat {
    base: BaseStat,
    bonus: f32,
}

impl Stat {
    fn total(&self, champion_level: i32) -> f32 {
        let mut result = 0.0;

        result += self.base.total(champion_level);
        result += self.bonus;

        result
    }
}

impl From<StatRaw> for Stat {
    fn from(value: StatRaw) -> Self {
        Self {
            base: value.into(),
            bonus: 0.0,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Stats {
    health: Stat,
    health_regen: Stat,
    mana: Stat,
    mana_regen: Stat,
    armor: Stat,
    magic_resistance: Stat,
    attack_damage: Stat,
    flat_armor_pen: Stat,
    percent_armor_pen: Stat,
    ability_power: Stat,
    flat_magic_pen: Stat,
    percent_magic_pen: Stat,
    movespeed: Stat,
    critical_strike_damage: Stat,
    critical_strike_damage_modifier: Stat,
    attack_speed: Stat,
    attack_speed_ratio: Stat,
    attack_cast_time: Stat,
    attack_total_time: Stat,
}

impl Stats {
    fn stat_at_level(&self, stat_type: StatType, level: i32) -> f32 {
        match stat_type {
            StatType::AD => self.attack_damage.total(level),
            StatType::AP => self.ability_power.total(level),
            StatType::FlatArmorPen => self.flat_armor_pen.total(level),
            StatType::PercentArmorPen => self.percent_armor_pen.total(level),
            StatType::FlatMagicPen => self.flat_magic_pen.total(level),
            StatType::PercentMagicPen => self.percent_magic_pen.total(level),
            StatType::BonusAD => self.attack_damage.bonus,
            StatType::MaxHealth => self.health.total(level),
            StatType::CurrentHealth => self.health.total(level), // Placeholder: needs actual current health tracking
            StatType::BonusHealth => self.health.bonus,
            StatType::MissingHealth => {
                // Placeholder: assumes missing health is zero (full health)
                0.0
            }
            StatType::Armor => self.armor.total(level),
            StatType::BonusArmor => self.armor.bonus,
            StatType::MagicResist => self.magic_resistance.total(level),
            StatType::BonusMagicResist => self.magic_resistance.bonus,
            StatType::BonusMana => self.mana.bonus,
            StatType::BonusMoveSpeed => self.movespeed.bonus,
        }
    }
}

impl From<StatsRaw> for Stats {
    fn from(raw: StatsRaw) -> Self {
        Self {
            health: raw.health.into(),
            health_regen: raw.health_regen.into(),
            mana: raw.mana.into(),
            mana_regen: raw.mana_regen.into(),
            armor: raw.armor.into(),
            magic_resistance: raw.magic_resistance.into(),
            attack_damage: raw.attack_damage.into(),
            flat_armor_pen: Stat::default(),
            percent_armor_pen: Stat::default(),
            ability_power: Stat::default(),
            flat_magic_pen: Stat::default(),
            percent_magic_pen: Stat::default(),
            movespeed: raw.movespeed.into(),
            critical_strike_damage: raw.critical_strike_damage.into(),
            critical_strike_damage_modifier: raw.critical_strike_damage_modifier.into(),
            attack_speed: raw.attack_speed.into(),
            attack_speed_ratio: raw.attack_speed_ratio.into(),
            attack_cast_time: raw.attack_cast_time.into(),
            attack_total_time: raw.attack_total_time.into(),
        }
    }
}

struct Champion {
    kind: ChampionKind,
    stats: Stats,
    abilities: AbilitiesRaw,
    level: i32,
}

impl Champion {
    fn get_ability(&self, ability_kind: AbilitySlot) -> &AbilityRaw {
        match ability_kind {
            AbilitySlot::P => &self.abilities.p,
            AbilitySlot::Q => &self.abilities.q,
            AbilitySlot::W => &self.abilities.w,
            AbilitySlot::E => &self.abilities.e,
            AbilitySlot::R => &self.abilities.r,
        }
    }
}

impl From<ChampionRaw> for Champion {
    fn from(value: ChampionRaw) -> Self {
        Self {
            kind: ChampionKind::from_str(&value.name).expect("yikes"),
            stats: value.stats.into(),
            abilities: value.abilities,
            level: 1,
        }
    }
}

fn calculate_damage(
    dmg: &AbilityDamage,
    source_champion: &Champion,
    target_champion: &Champion,
) -> f32 {
    dmg.components
        .iter()
        .map(|comp| {
            let base_damage = comp.base_damage(source_champion, target_champion);
            let scaling_damage: f32 = comp.total_scaled_bonus(source_champion);

            let pre_mitigation_damage = base_damage + scaling_damage;

            match comp.damage_type {
                DamageType::Magic => {
                    let magic_resistance = apply_penetration(
                        target_champion
                            .stats
                            .magic_resistance
                            .total(target_champion.level),
                        0.0,
                        0.0,
                        source_champion
                            .stats
                            .flat_magic_pen
                            .total(source_champion.level),
                        source_champion
                            .stats
                            .percent_magic_pen
                            .total(source_champion.level),
                    );
                    apply_mitigation(pre_mitigation_damage, magic_resistance)
                }
                DamageType::Physical => apply_mitigation(
                    pre_mitigation_damage,
                    target_champion.stats.armor.total(target_champion.level),
                ),
                DamageType::True => pre_mitigation_damage,
                _ => -1.0,
            }
        })
        .sum()
}

fn apply_penetration(
    raw_resistance: f32,
    resistance_flat_reduction: f32,
    resistance_percent_reduction: f32,
    flat_pen: f32,
    percent_pen: f32,
) -> f32 {
    let mut result = raw_resistance;

    result -= resistance_flat_reduction;
    result *= 1.0 - (resistance_percent_reduction / 100.0);
    result *= 1.0 - (percent_pen / 100.0);
    result -= flat_pen;
    result
}

fn apply_mitigation(value: f32, resistance_value: f32) -> f32 {
    let modifier = if value >= 0.0 {
        100.0 / (100.0 + resistance_value)
    } else {
        2.0 - 100.0 / (100.0 + resistance_value)
    };

    value * modifier
}

fn main() {
    if !fs::exists("champions").expect("Couldn't find champions folder") {
        let champions_path = "../lolstaticdata/champions";
        let copy_champions = Command::new("cp")
            .args(["-r", champions_path, "."])
            .output()
            .expect("Failed to copy champions folder");
        if !copy_champions.status.success() {
            eprintln!("Champions cp failed");
        }
    }

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("file.txt")
        .expect("Failed to truncate file");

    let write_ability_damage = |champion: &Champion| {
        if let Some(damage) = ability_damage_by_level(AbilitySlot::Q, AbilityLevel::One, champion) {
            let out = format!("{:#?}\n{:#?}\r\n", champion.kind, damage);
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("file.txt")
                .unwrap();
            file.write_all(out.as_bytes()).unwrap();
        }
    };

    if let Ok(champion) = read_champion_data(&ChampionKind::Syndra) {
        // write_ability_damage(&champion);
        let mut champion = Champion::from(champion);
        champion.stats.ability_power = Stat {
            base: BaseStat::default(),
            bonus: 100.0,
        };
        champion.stats.flat_magic_pen = Stat {
            base: BaseStat::default(),
            bonus: 10.0,
        };
        if let Some(q_damage) =
            ability_damage_by_level(AbilitySlot::Q, AbilityLevel::One, &champion)
        {
            let before = &q_damage;
            let q_damage: Vec<f32> = q_damage
                .iter()
                .map(|dmg| calculate_damage(dmg, &champion, &champion))
                .collect();

            for dmg in q_damage {
                println!("{:#?}", before);
                dbg!(dmg);
            }
        }
    }

    // for champion_kind in ChampionType::iter() {
    //     let champion = match read_champion_data(&champion_kind) {
    //         Ok(champ) => Champion::from(champ),
    //         Err(e) => {
    //             eprintln!("Failed to read data for {:?}: {}", champion_kind, e);
    //             continue;
    //         }
    //     };
    // write_ability_damage(&champion);
    // if champion_kind == ChampionType::Aatrox {
    //     let champion_level = 2;
    //     dbg!(&champion.stats.health);
    //     dbg!(champion.stats.clone().health.total(champion_level));

    //     for champion_level in 1..=18 {
    //         dbg!(champion_level);
    // let linear_scale = champion.stats.health.flat + champion.stats.health.per_level * (champion_level - 1) as f32;
    //             let prev= champion.stats.clone().health.total(champion_level-1);
    //             let current = champion.stats.clone().health.total(champion_level);
    //             let diff = current - prev;
    //             let ratio = diff / champion.stats.health.base.per_level;
    //             dbg!(ratio);
    //         }
    //     }
    // }
}

fn read_champion_data(champion: &ChampionKind) -> Result<ChampionRaw, Box<dyn std::error::Error>> {
    let path = Path::new("champions").join(format!("{:?}.json", champion));
    let data = fs::read_to_string(path)?;
    let mut champion = serde_json::from_str(&data)?;
    patch_champion(&mut champion);
    Ok(champion)
}

fn should_exclude_attribute(attr: &str, exclude_patterns: &[&str]) -> bool {
    let attr = attr.to_lowercase();
    for pat in exclude_patterns {
        if *pat == "minion" {
            // Exclude only if "minion" is present and "non-minion" is NOT present
            if attr.contains("minion") && !attr.contains("non-minion") {
                return true;
            }
        } else if attr.contains(pat) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn all_champions_q_damage_by_level() {
        for champion_kind in ChampionKind::iter() {
            let champion = match read_champion_data(&champion_kind) {
                Ok(champ) => Champion::from(champ),
                Err(e) => panic!(
                    "Failed to read data for {:?}: {} (file: champions/{:?}.json)",
                    champion_kind, e, champion_kind
                ),
            };
            dbg!("Testing Champion: {:?}", &champion_kind);
            let ability = champion.get_ability(AbilitySlot::Q);
            if ability.damage_type.is_none() {
                println!(
                    "Skipping {:?} because Q is not a damage ability",
                    champion_kind
                );
                continue;
            }
            let rank1_damage =
                ability_damage_by_level(AbilitySlot::Q, AbilityLevel::One, &champion);
            dbg!(&rank1_damage);
            assert!(rank1_damage.is_some(), "No damage for {:?}", champion_kind);
            assert!(
                !rank1_damage.unwrap().is_empty(),
                "Empty damage for {:?}",
                champion_kind
            );
        }
    }
}
