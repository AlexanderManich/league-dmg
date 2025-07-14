use std::fs::{self};
use std::io::Write;
use std::str::FromStr;
use std::{path::Path, process::Command};
mod champions_t;
mod items;
use strum_macros::EnumIter;

use crate::champions_t::{
    AbilitiesRaw, AbilityRaw, ChampionKind, ChampionRaw, Description, Effect, Leveling, StatsRaw,
};
use crate::items::{read_item_data, ItemKind};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
enum AbilityLevel {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
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

#[derive(Default, Debug, Copy, Clone, EnumIter)]
enum StatType {
    #[default]
    AD,
    BonusAD,
    FlatArmorPen,
    PercentArmorPen,
    // ArmorPenetration,
    AP,
    FlatMagicPen,
    PercentMagicPen,
    // MagicPenetration,
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

#[derive(Debug, Default, Clone)]
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

#[derive(Default, Debug, Clone)]
enum BaseDamage {
    #[default]
    None,
    Flat(f32),
    PercentTargetMaxHealth(f32),
    PercentTargetMissingHealth(f32),
    PercentResource(f32),
}

#[derive(Default, Debug, Clone)]
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
                target_champion.stat_at_level(StatType::MaxHealth) * (value / 100.0)
            }
            BaseDamage::PercentTargetMissingHealth(_) => todo!(),
            BaseDamage::PercentResource(_) => todo!(),
            BaseDamage::None => todo!(),
        }
    }

    fn total_scaling_bonus(&self, source_champion: &Champion) -> f32 {
        self.scalings
            .iter()
            .map(|scaling| scaling.scaled_value(source_champion.stat_at_level(scaling.stat_type)))
            .sum()
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
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

fn damage_components_from_levelings(
    levelings: &[Leveling],
    ability_level: AbilityLevel,
    default_damage_type: Option<DamageType>,
) -> Vec<DamageComponent> {
    let mut result = Vec::new();

    // TODO: There is more work to be done on determining damage types (E.g., see Gwen)
    let mut damage_type = default_damage_type.unwrap_or(DamageType::None);

    for leveling in levelings {
        if !leveling_contains_damage_effect(leveling) {
            continue;
        }

        damage_type = match try_get_damage_type(leveling) {
            Some(new) => new,
            None => damage_type,
        };

        for modifier in &leveling.modifiers {
            if let Some(unit_str) = modifier.units.first() {
                if let Some(value) = modifier.values.get(ability_level as usize) {
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

    result
}

#[derive(Debug, Clone)]
struct Ability {
    components: Vec<DamageComponent>,
    variants: Option<Vec<Ability>>,
    description: String,
}

impl Ability {
    fn new(
        ability_slot: AbilitySlot,
        ability_level: AbilityLevel,
        champion: &Champion,
    ) -> Option<Ability> {
        let mut variants = Vec::new();

        let ability_raw = champion.get_ability_raw(ability_slot);

        // Check if ability is damaging ability
        ability_raw.damage_type.as_ref()?;

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

        let filtered_effects = filter_effects(&ability_raw.effects, &exclude_patterns);

        let max_patterns = ["Maximum", "Total"];

        let mut max_levelings = Vec::new();
        // Collect all levelings that contain "max_patterns" in their attribute
        for effect in &filtered_effects {
            for leveling in &effect.leveling {
                if max_patterns
                    .iter()
                    .any(|pat| leveling.attribute.contains(pat))
                {
                    max_levelings.push((leveling.clone(), effect.description.clone()));
                }
            }
        }

        let default_damage_type = ability_raw
            .damage_type
            .as_ref()
            .map(|s| DamageType::from_str(s));

        let is_mixed_damage = max_levelings
            .iter()
            .any(|(leveling, _)| leveling.attribute.to_lowercase().contains("mixed")) || default_damage_type == Some(DamageType::Mixed);

        if !max_levelings.is_empty() && is_mixed_damage {
            // If we have levelings that contain "max_patterns", we need to check if the damage type is mixed.
            // If so we exclude the "max_pattern" leveling, and construct the ability damage from the individual levelings
            // that make up the maximum. We do this because we need to know the actual damage type of every damage component,
            // such that we can apply damage mitigation later.

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

                variants.push(Ability {
                    components: damage_components_from_levelings(
                        &damage_components,
                        ability_level,
                        default_damage_type,
                    ),
                    variants: None,
                    description: effect.description.clone(),
                });
            }
        } else if !max_levelings.is_empty() {
            // Here we pass levelings individually, instead of passing the max_levelings vec, because we might have
            // abilities with multiple effects containing a "max_pattern" such as Aatrox Q that has different variants
            // (Sweetspot / Non-Sweetspot) where we want an individual AbilityDamage for each variant.
            // NOTE: We assume that an ability variant only ever has one "max_pattern" leveling.
            for (leveling, description) in max_levelings {
                variants.push(Ability {
                    components: damage_components_from_levelings(
                        &[leveling],
                        ability_level,
                        default_damage_type,
                    ),
                    variants: None,
                    description,
                });
            }
        } else {
            // No leveling had a "max_pattern" so we just pass all levelings in all effects
            dbg!(&filtered_effects.len());
            for effect in &filtered_effects {
                variants.push(Ability {
                    components: damage_components_from_levelings(
                        &effect.leveling,
                        ability_level,
                        default_damage_type,
                    ),
                    variants: None,
                    description: effect.description.clone(),
                });
            }
        }

        match variants.len() {
            0 => None,
            1 => Some(variants.remove(0)),
            _ => {
                let mut all_variants = variants;
                let base = all_variants.remove(0);
                let base_components = base.components.clone();
                Some(Ability {
                    components: base_components,
                    variants: Some(all_variants),
                    description: base.description,
                })
            }
        }
    }
}

fn filter_effects(effects: &[Effect], exclude_patterns: &[&str]) -> Vec<Effect> {
    effects
        .iter()
        .filter_map(|effect| {
            let filtered_levelings: Vec<Leveling> = effect
                .leveling
                .iter()
                .filter(|leveling| !should_exclude_attribute(&leveling.attribute, exclude_patterns))
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
        .collect()
}

fn leveling_contains_damage_effect(leveling: &Leveling) -> bool {
    let mut result = false;

    if leveling.attribute.to_lowercase().contains("damage") {
        result = true;
    }

    result
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
    dbg!(leveling);
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
            ChampionKind::Syndra => patch_syndra(champion),
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

fn patch_syndra(champion: &mut ChampionRaw) {
    let w = &mut champion.abilities.w;
    if let Some(effect) = w.effects.get_mut(3) {
        if let Some(leveling) = effect.leveling.first_mut() {
            leveling.attribute = String::from("True Damage");
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatBlock {
    flat: f32,
    percent: f32,
    per_level: f32,
    percent_per_level: f32,
}

impl StatBlock {
    fn total(&self, champion_level: i32) -> f32 {
        let mut result = 0.0;
        result += self.flat;

        // TODO: I dont know if this applies to all per level scalings!!! (I only considered health when writing it)
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

    fn with_flat(value: f32) -> Self {
        Self {
            flat: value,
            ..Default::default()
        }
    }
}

impl std::ops::AddAssign<StatBlock> for StatBlock {
    fn add_assign(&mut self, rhs: StatBlock) {
        self.flat += rhs.flat;
        self.per_level += rhs.per_level;
        self.percent += rhs.percent;
        self.percent_per_level += rhs.percent_per_level;
    }
}

#[derive(Default, Clone, Debug)]
struct Stat {
    base: StatBlock,
    bonus: StatBlock,
}

impl Stat {
    fn with_total(value: f32) -> Self {
        Self {
            base: Default::default(),
            bonus: StatBlock::with_flat(value),
        }
    }
}

impl From<StatBlock> for Stat {
    fn from(value: StatBlock) -> Self {
        Self {
            base: value,
            bonus: Default::default(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Stats {
    health: Stat,
    health_regen: Stat,
    mana: Stat,
    mana_regen: Stat,
    armor: Stat,
    magic_resistance: Stat,
    attack_damage: Stat,
    armor_penetration: Stat,
    ability_power: Stat,
    magic_penetration: Stat,
    movespeed: Stat,
    ability_haste: Stat,
    cooldown_reduction: Stat,
    critical_strike_chance: Stat,
    critical_strike_damage: Stat,
    critical_strike_damage_modifier: Stat,
    attack_speed: Stat,
    attack_speed_ratio: Stat,
    attack_cast_time: Stat,
    attack_total_time: Stat,
    gold_per10: Stat,
    heal_and_shield_power: Stat,
    lifesteal: Stat,
    omnivamp: Stat,
    tenacity: Stat,
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
            armor_penetration: Stat::default(),
            ability_power: Stat::default(),
            magic_penetration: Stat::default(),
            movespeed: raw.movespeed.into(),
            critical_strike_damage: raw.critical_strike_damage.into(),
            critical_strike_damage_modifier: raw.critical_strike_damage_modifier.into(),
            attack_speed: raw.attack_speed.into(),
            attack_speed_ratio: raw.attack_speed_ratio.into(),
            attack_cast_time: raw.attack_cast_time.into(),
            attack_total_time: raw.attack_total_time.into(),
            ability_haste: Stat::default(),
            cooldown_reduction: Stat::default(),
            critical_strike_chance: Stat::default(),
            gold_per10: Stat::default(),
            heal_and_shield_power: Stat::default(),
            lifesteal: Stat::default(),
            omnivamp: Stat::default(),
            tenacity: Stat::default(),
        }
    }
}

#[derive(Debug, Default)]
struct Champion {
    kind: ChampionKind,
    stats: Stats,
    abilities: AbilitiesRaw,
    level: i32,
}

impl Champion {
    fn get_ability_raw(&self, ability_kind: AbilitySlot) -> &AbilityRaw {
        match ability_kind {
            AbilitySlot::P => &self.abilities.p,
            AbilitySlot::Q => &self.abilities.q,
            AbilitySlot::W => &self.abilities.w,
            AbilitySlot::E => &self.abilities.e,
            AbilitySlot::R => &self.abilities.r,
        }
    }

    fn custom_with_stats(stats: Stats) -> Self {
        Self {
            kind: ChampionKind::Custom,
            stats,
            abilities: AbilitiesRaw::default(),
            level: 1,
        }
    }

    fn stat_at_level(&self, stat_type: StatType) -> f32 {
        match stat_type {
            StatType::AD => {
                self.stats.attack_damage.base.total(self.level)
                    + self.stats.attack_damage.bonus.total(self.level)
            }
            StatType::AP => {
                self.stats.ability_power.base.total(self.level)
                    + self.stats.ability_power.bonus.total(self.level)
            }
            // TODO: Should we even differentiate between flat and percent, or should we just use armor pen, and use the statblock
            // TODO: Also as it is now we dont consider per level stuff for penetration
            StatType::FlatArmorPen => {
                self.stats.armor_penetration.base.flat + self.stats.armor_penetration.bonus.flat
            }
            StatType::PercentArmorPen => {
                let base = self.stats.armor_penetration.base.percent;
                let bonus = self.stats.armor_penetration.bonus.percent;
                multiplicative_percent_pen(base, bonus)
            }
            StatType::FlatMagicPen => {
                self.stats.magic_penetration.base.flat + self.stats.magic_penetration.bonus.flat
            }
            StatType::PercentMagicPen => {
                let base = self.stats.magic_penetration.base.percent;
                let bonus = self.stats.magic_penetration.bonus.percent;
                multiplicative_percent_pen(base, bonus)
            }
            StatType::BonusAD => self.stats.attack_damage.bonus.total(self.level),
            StatType::MaxHealth => {
                self.stats.health.base.total(self.level) + self.stats.health.bonus.total(self.level)
            }
            StatType::CurrentHealth => self.stats.health.base.total(self.level), // TODO: fix
            StatType::BonusHealth => self.stats.health.bonus.total(self.level),
            StatType::MissingHealth => {
                // TODO: fix
                0.0
            }
            StatType::Armor => {
                self.stats.armor.base.total(self.level) + self.stats.armor.bonus.total(self.level)
            }
            StatType::BonusArmor => self.stats.armor.bonus.total(self.level),
            StatType::MagicResist => {
                self.stats.magic_resistance.base.total(self.level)
                    + self.stats.magic_resistance.bonus.total(self.level)
            }
            StatType::BonusMagicResist => self.stats.magic_resistance.bonus.total(self.level),
            StatType::BonusMana => self.stats.mana.bonus.total(self.level),
            StatType::BonusMoveSpeed => self.stats.movespeed.bonus.total(self.level),
        }
    }

    fn apply_items_stats(&mut self, item_kinds: Vec<ItemKind>) {
        assert!(item_kinds.len() <= 6, "You can't have more than 6 items");

        let mut items = Vec::new();

        for item_kind in item_kinds {
            let item = read_item_data(item_kind).unwrap();
            items.push(item);
        }

        // TODO: Percentage stats should be multiplicative!!!
        for item in items {
            self.stats.health.bonus += item.stats.health;
            self.stats.health_regen.bonus += item.stats.health_regen;
            self.stats.ability_power.bonus += item.stats.ability_power;
            self.stats.armor.bonus += item.stats.armor;
            self.stats.attack_damage.bonus += item.stats.attack_damage;
            self.stats.attack_speed.bonus += item.stats.attack_speed;
            self.stats.critical_strike_chance.bonus += item.stats.critical_strike_chance;
            self.stats.critical_strike_damage.bonus += item.stats.critical_strike_damage;
            // League of legends is dumb and has a seperate stat Lethality which is literally just flat armor pen
            self.stats.armor_penetration.bonus.flat += item.stats.lethality.flat;
            self.stats.armor_penetration.bonus.percent += item.stats.armor_penetration.percent;
            self.stats.magic_penetration.bonus += item.stats.magic_penetration;
            self.stats.magic_resistance.bonus += item.stats.magic_resistance;
            self.stats.mana.bonus += item.stats.mana;
            self.stats.mana_regen.bonus += item.stats.mana_regen;
            self.stats.movespeed.bonus += item.stats.movespeed;
            self.stats.cooldown_reduction.bonus += item.stats.cooldown_reduction;
            self.stats.ability_haste.bonus += item.stats.ability_haste;
            self.stats.gold_per10.bonus += item.stats.gold_per10;
            self.stats.heal_and_shield_power.bonus += item.stats.heal_and_shield_power;
            self.stats.lifesteal.bonus += item.stats.lifesteal;
            self.stats.omnivamp.bonus += item.stats.omnivamp;
            self.stats.tenacity.bonus += item.stats.tenacity;
        }
    }
}

fn multiplicative_percent_pen(base: f32, bonus: f32) -> f32 {
    match (base != 0.0, bonus != 0.0) {
        (true, true) => ((100.0 - base) * (100.0 - bonus)) / 100.0,
        (true, false) => base,
        (false, true) => bonus,
        (false, false) => 0.0,
    }
}

impl From<ChampionRaw> for Champion {
    fn from(value: ChampionRaw) -> Self {
        Self {
            kind: ChampionKind::from_str(&value.key).expect("yikes"),
            stats: value.stats.into(),
            abilities: value.abilities,
            level: 1,
        }
    }
}

fn calculate_damage(dmg: &Ability, source_champion: &Champion, target_champion: &Champion) -> f32 {
    dmg.components
        .iter()
        .map(|comp| {
            let base_damage = comp.base_damage(source_champion, target_champion);
            let scaling_damage: f32 = comp.total_scaling_bonus(source_champion);

            let pre_mitigation_damage = base_damage + scaling_damage;

            let effective_res =
                effective_resistance(comp.damage_type, source_champion, target_champion);

            apply_damage_mitigation(pre_mitigation_damage, effective_res)
        })
        .sum()
}

fn effective_resistance(
    damage_type: DamageType,
    source_champion: &Champion,
    target_champion: &Champion,
) -> f32 {
    // TODO: Also apply resistance reduction
    let (raw_res, flat_pen, percent_pen) = match damage_type {
        DamageType::Magic => (
            target_champion.stat_at_level(StatType::MagicResist),
            source_champion.stat_at_level(StatType::FlatMagicPen),
            source_champion.stat_at_level(StatType::PercentMagicPen),
        ),
        DamageType::Physical => (
            target_champion.stat_at_level(StatType::Armor),
            source_champion.stat_at_level(StatType::FlatArmorPen),
            source_champion.stat_at_level(StatType::PercentArmorPen),
        ),
        DamageType::True => (0.0, 0.0, 0.0),
        DamageType::Mixed => {
            panic!("We should never get DamageType::Mixed, when calculating effective resistance")
        }
        DamageType::None => {
            panic!("We should never get DamageType::None, when calculating effective resistance")
        }
    };

    let mut result = raw_res;
    // result -= resistance_flat_reduction;
    // result *= 1.0 - (resistance_percent_reduction / 100.0);
    result *= 1.0 - (percent_pen / 100.0);
    result -= flat_pen;
    result
}

fn apply_damage_mitigation(damage_value: f32, resistance_value: f32) -> f32 {
    let modifier = if damage_value >= 0.0 {
        100.0 / (100.0 + resistance_value)
    } else {
        2.0 - 100.0 / (100.0 + resistance_value)
    };

    damage_value * modifier
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    if !fs::exists("items").expect("Couldn't find items folder") {
        let items_path = "../lolstaticdata/items";
        let copy_items = Command::new("cp")
            .args(["-r", items_path, "."])
            .output()
            .expect("Failed to copy items folder");
        if !copy_items.status.success() {
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
        if let Some(damage) = Ability::new(AbilitySlot::Q, AbilityLevel::One, champion) {
            let out = format!("{:#?}\n{:#?}\r\n", champion.kind, damage);
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("file.txt")
                .unwrap();
            file.write_all(out.as_bytes()).unwrap();
        }
    };

    let champion_data = read_champion_data(&ChampionKind::Syndra)?;

    // write_ability_damage(&champion);

    let mut champion = Champion::from(champion_data);

    let items = vec![
        // ItemKind::RabadonsDeathcap,
        // ItemKind::LudensCompanion,
        // ItemKind::VoidStaff,
    ];

    champion.apply_items_stats(items);

    let q_ability = Ability::new(AbilitySlot::Q, AbilityLevel::One, &champion).unwrap();

    let before = &q_ability;
    println!("{before:#?}");

    let mut targets = Vec::new();
    for i in 0..10 {
        targets.push(Champion::custom_with_stats(Stats {
            magic_resistance: Stat::with_total(i as f32 * 25.0),
            ..Default::default()
        }));
    }
    for target in &targets {
        let q_damage = calculate_damage(&q_ability, &champion, target);

        println!("MR: {:?}", target.stat_at_level(StatType::MagicResist),);
        dbg!(q_damage);
    }

    Ok(())
}

fn read_champion_data(champion: &ChampionKind) -> Result<ChampionRaw, Box<dyn std::error::Error>> {
    let path = Path::new("champions").join(format!("{champion:?}.json"));
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
            if champion_kind == ChampionKind::Custom {
                continue;
            };
            let champion = match read_champion_data(&champion_kind) {
                Ok(champ) => Champion::from(champ),
                Err(e) => panic!(
                    "Failed to read data for {champion_kind:?}: {e} (file: champions/{champion_kind:?}.json)"
                ),
            };
            println!("Testing Champion: {:?}", &champion_kind);
            let ability = champion.get_ability_raw(AbilitySlot::Q);
            if ability.damage_type.is_none() {
                println!("Skipping {champion_kind:?} because Q is not a damage ability");
                continue;
            }
            let rank1_damage = Ability::new(AbilitySlot::Q, AbilityLevel::One, &champion);
            println!("{:?}", &rank1_damage);
            if let Some(ref damage) = rank1_damage {
                for comp in &damage.components {
                    assert!(comp.damage_type != DamageType::Mixed);
                    assert!(comp.damage_type != DamageType::None);
                }
            }
            assert!(rank1_damage.is_some(), "No damage for {champion_kind:?}");
        }
    }
}
