use std::fs::{self};
use std::path::Path;
use std::str::FromStr;
pub mod champions_t;
pub mod items;
use strum_macros::EnumIter;

use crate::champions_t::{AbilityRaw, ChampionKind, ChampionRaw, Effect, Leveling, StatsRaw};
use crate::items::{read_item_data, ItemKind};

use plotters::prelude::*;
use plotters::style::PaletteColor;
use serde::{Deserialize, Serialize}; // Add this at the top

#[derive(Debug, Copy, Clone)]
pub enum AbilityLevel {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
}

#[derive(Copy, Clone, Debug)]
pub enum AbilitySlot {
    P,
    Q,
    W,
    E,
    R,
}

#[derive(Default, Debug, Copy, Clone, EnumIter)]
pub enum StatType {
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
    MaxMana,
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
            "% maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of her maximum health" => AbilityScaling {
                stat_type: StatType::MaxHealth,
                ratio: value,
                kind: ScalingType::PercentStat,
                per_stat: None,
            },
            "% of his maximum health" => AbilityScaling {
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
            "%  of target's missing health" => AbilityScaling {
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
            "% of primary target's bonus health" => AbilityScaling {
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
            "% maximum mana" => AbilityScaling {
                stat_type: StatType::MaxMana,
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

            // Special Scalings
            // "% of Siphoning Strike stacks" => AbilityScaling {
            // }
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
    default_damage_type: Option<DamageType>,
) -> Vec<Vec<DamageComponent>> {
    let mut result = Vec::new();

    // collect only levelings that actually contain damage
    let relevant_levelings: Vec<&Leveling> = levelings
        .iter()
        .filter(|l| leveling_contains_damage_effect(l))
        .collect();

    if relevant_levelings.is_empty() {
        return result;
    }

    // TODO: Still more work to be done on determining damage types.
    let mut damage_types_per_leveling: Vec<DamageType> =
        Vec::with_capacity(relevant_levelings.len());
    let mut current_dt = default_damage_type.unwrap_or(DamageType::None);
    for leveling in &relevant_levelings {
        if let Some(new_dt) = try_get_damage_type(leveling) {
            current_dt = new_dt;
        }
        damage_types_per_leveling.push(current_dt);
    }

    let max_values = relevant_levelings
        .iter()
        .flat_map(|l| l.modifiers.iter().map(|m| m.values.len()))
        .max()
        .unwrap_or(0);

    if max_values == 0 {
        return result;
    }

    for idx in 0..max_values {
        let mut components_for_value_index: Vec<DamageComponent> = Vec::new();

        for (lev_idx, leveling) in relevant_levelings.iter().enumerate() {
            let local_dt = damage_types_per_leveling[lev_idx];

            for modifier in &leveling.modifiers {
                if let Some(unit_str) = modifier.units.first() {
                    if let Some(value) = modifier.values.get(idx) {
                        if is_flat_damage(unit_str) {
                            components_for_value_index.push(DamageComponent {
                                damage_type: local_dt,
                                base: BaseDamage::Flat(*value),
                                scalings: vec![],
                            });
                        } else if is_percent_damage_with_scaling(unit_str) {
                            let scaling = parse_percent_damage_with_scaling(unit_str);
                            if is_percent_target_max_health(unit_str) {
                                components_for_value_index.push(DamageComponent {
                                    damage_type: local_dt,
                                    base: BaseDamage::PercentTargetMaxHealth(*value),
                                    scalings: vec![scaling],
                                });
                            }
                        } else if is_percent_target_max_health(unit_str) {
                            components_for_value_index.push(DamageComponent {
                                damage_type: local_dt,
                                base: BaseDamage::PercentTargetMaxHealth(*value),
                                scalings: vec![],
                            });
                        } else {
                            let scaling = AbilityScaling::from_parts(unit_str, *value);
                            if components_for_value_index.is_empty() {
                                components_for_value_index.push(DamageComponent {
                                    damage_type: local_dt,
                                    base: BaseDamage::None,
                                    scalings: vec![scaling],
                                });
                            } else {
                                for component in components_for_value_index.iter_mut() {
                                    component.scalings.push(scaling.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        result.push(components_for_value_index);
    }

    result
}

#[derive(Default, Debug, Clone)]
struct Abilities {
    p: Vec<Ability>,
    q: Vec<Ability>,
    w: Vec<Ability>,
    e: Vec<Ability>,
    r: Vec<Ability>,
}

#[derive(Debug, Clone)]
struct Ability {
    components: Vec<DamageComponent>,
    variants: Option<Vec<Ability>>,
    description: String,
}

impl Ability {
    fn ability_at_all_levels(ability_raw: AbilityRaw) -> Option<Vec<Ability>> {
        let mut variants: Vec<Vec<Ability>> = Vec::new();

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
            .any(|(leveling, _)| leveling.attribute.to_lowercase().contains("mixed"))
            || default_damage_type == Some(DamageType::Mixed);

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

                let components_all_levels =
                    damage_components_from_levelings(&damage_components, default_damage_type);

                let mut variant_all_levels = Vec::new();

                for components in components_all_levels {
                    variant_all_levels.push(Ability {
                        components,
                        variants: None,
                        description: effect.description.clone(),
                    });
                }

                variants.push(variant_all_levels);
            }
        } else if !max_levelings.is_empty() {
            // Here we pass levelings individually, instead of passing the max_levelings vec, because we might have
            // abilities with multiple effects containing a "max_pattern" such as Aatrox Q that has different variants
            // (Sweetspot / Non-Sweetspot) where we want an individual AbilityDamage for each variant.
            // NOTE: We assume that an ability variant only ever has one "max_pattern" leveling.
            for (leveling, description) in max_levelings {
                let components_all_levels =
                    damage_components_from_levelings(&[leveling], default_damage_type);

                let mut variant_all_levels = Vec::new();

                for components in components_all_levels {
                    variant_all_levels.push(Ability {
                        components,
                        variants: None,
                        description: description.clone(),
                    });
                }

                variants.push(variant_all_levels);
            }
        } else {
            // No leveling had a "max_pattern" so we just pass all levelings in all effects
            for effect in &filtered_effects {
                let components_all_levels =
                    damage_components_from_levelings(&effect.leveling, default_damage_type);

                let mut variant_all_levels = Vec::new();

                for components in components_all_levels {
                    variant_all_levels.push(Ability {
                        components: components.to_vec(),
                        variants: None,
                        description: effect.description.clone(),
                    });
                }

                variants.push(variant_all_levels);
            }
        }

        let mut result = Vec::new();

        match variants.len() {
            0 => None,
            1 => Some(variants.remove(0)),
            _ => {
                let mut all_variants = variants;
                let base_variant_levels = all_variants.remove(0);

                for (level_idx, base_ability) in base_variant_levels.iter().enumerate() {
                    let other_variant_abilities: Vec<Ability> = all_variants
                        .iter()
                        .filter_map(|variant_levels| variant_levels.get(level_idx).cloned())
                        .collect();
                    result.push(Ability {
                        components: base_ability.components.clone(),
                        variants: if other_variant_abilities.is_empty() {
                            None
                        } else {
                            Some(other_variant_abilities)
                        },
                        description: base_ability.description.clone(),
                    });
                }
                Some(result)
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

#[derive(Default, Clone, Debug, Serialize)]
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

#[derive(Default, Clone, Debug, Serialize)]
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

impl Stats {
    fn from_parts(value: f32, stat_type: StatType) -> Self {
        let mut result = Stats::default();
        match stat_type {
            StatType::MagicResist => result.magic_resistance = Stat::with_total(value),
            StatType::Armor => result.armor = Stat::with_total(value),
            _ => todo!(),
        }
        result
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

#[derive(Debug, Clone, Default)]
pub struct Champion {
    // kind: ChampionKind,
    pub stats: Stats,
    abilities: Abilities,
    level: i32,
}

impl Champion {
    fn get_ability(&self, ability_kind: AbilitySlot) -> &Vec<Ability> {
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
            stats,
            abilities: Abilities::default(),
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
            StatType::MaxMana => {
                self.stats.mana.base.total(self.level) + self.stats.health.bonus.total(self.level)
            }
            StatType::BonusMana => self.stats.mana.bonus.total(self.level),
            StatType::BonusMoveSpeed => self.stats.movespeed.bonus.total(self.level),
        }
    }

    fn apply_items_stats(&self, item_kinds: Vec<ItemKind>) -> Champion {
        assert!(item_kinds.len() <= 6, "You can't have more than 6 items");

        let mut items = Vec::new();

        for item_kind in item_kinds {
            let item = read_item_data(item_kind).unwrap();
            items.push(item);
        }

        let mut result = self.clone();

        // TODO: Percentage stats should be multiplicative!!!
        for item in items {
            result.stats.health.bonus += item.stats.health;
            result.stats.health_regen.bonus += item.stats.health_regen;
            result.stats.ability_power.bonus += item.stats.ability_power;
            result.stats.armor.bonus += item.stats.armor;
            result.stats.attack_damage.bonus += item.stats.attack_damage;
            result.stats.attack_speed.bonus += item.stats.attack_speed;
            result.stats.critical_strike_chance.bonus += item.stats.critical_strike_chance;
            result.stats.critical_strike_damage.bonus += item.stats.critical_strike_damage;
            // League of legends is dumb and has a seperate stat Lethality which is literally just flat armor pen
            result.stats.armor_penetration.bonus.flat += item.stats.lethality.flat;
            result.stats.armor_penetration.bonus.percent += item.stats.armor_penetration.percent;
            result.stats.magic_penetration.bonus += item.stats.magic_penetration;
            result.stats.magic_resistance.bonus += item.stats.magic_resistance;
            result.stats.mana.bonus += item.stats.mana;
            result.stats.mana_regen.bonus += item.stats.mana_regen;
            result.stats.movespeed.bonus += item.stats.movespeed;
            result.stats.cooldown_reduction.bonus += item.stats.cooldown_reduction;
            result.stats.ability_haste.bonus += item.stats.ability_haste;
            result.stats.gold_per10.bonus += item.stats.gold_per10;
            result.stats.heal_and_shield_power.bonus += item.stats.heal_and_shield_power;
            result.stats.lifesteal.bonus += item.stats.lifesteal;
            result.stats.omnivamp.bonus += item.stats.omnivamp;
            result.stats.tenacity.bonus += item.stats.tenacity;
        }

        result
    }
}

pub struct Config {
    name: String,
    champion: Champion,
    item_set: Vec<ItemKind>,
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
            // kind: ChampionKind::from_str(&value.key).expect("yikes"),
            stats: value.stats.into(),
            abilities: Abilities {
                p: Ability::ability_at_all_levels(value.abilities.p).unwrap_or_default(),
                q: match Ability::ability_at_all_levels(value.abilities.q) {
                    Some(q) => q,
                    None => {
                        eprintln!("Champion {} has no Q ability!", value.name);
                        Vec::new()
                    }
                },
                w: match Ability::ability_at_all_levels(value.abilities.w) {
                    Some(w) => w,
                    None => {
                        eprintln!("Champion {} has no W ability!", value.name);
                        Vec::new()
                    }
                },
                e: match Ability::ability_at_all_levels(value.abilities.e) {
                    Some(e) => e,
                    None => {
                        eprintln!("Champion {} has no E ability!", value.name);
                        Vec::new()
                    }
                },
                r: match Ability::ability_at_all_levels(value.abilities.r) {
                    Some(r) => r,
                    None => {
                        eprintln!("Champion {} has no R ability!", value.name);
                        Vec::new()
                    }
                },
            },
            level: 1,
        }
    }
}

fn calculate_ability_damage(
    ability_slot: AbilitySlot,
    ability_level: AbilityLevel,
    source_champion: &Champion,
    target_champion: &Champion,
) -> f32 {
    source_champion.get_ability(ability_slot)[ability_level as usize]
        .components
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

pub fn plot_damage_vs_resistance(
    resistance_values: &[f32],
    damage_values: &[Vec<f32>],
    config_names: &[Config], // <-- Add this parameter
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_resistance = *resistance_values.last().unwrap_or(&200.0);
    let max_damage = damage_values
        .iter()
        .flat_map(|series| series.iter())
        .cloned()
        .fold(f32::MIN, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Damage vs Resistance", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f32..max_resistance, 0f32..max_damage)?;

    chart
        .configure_mesh()
        .x_desc("Resistance")
        .y_desc("Damage")
        .draw()?;

    for (i, series) in damage_values.iter().enumerate() {
        let color = PaletteColor::<Palette99>::pick(i).mix(0.8);
        chart
            .draw_series(LineSeries::new(
                resistance_values
                    .iter()
                    .zip(series.iter())
                    .map(|(&x, &y)| (x, y)),
                &color,
            ))?
            .label(&config_names[i].name)
            .legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    println!("Graph saved to damage_vs_mr.png");

    Ok(())
}

pub fn create_item_configs(champion: &Champion, item_sets: Vec<Vec<ItemKind>>) -> Vec<Config> {
    let common = common_items(&item_sets);
    let mut result = Vec::new();
    for item_set in &item_sets {
        let config_name = diff_item_set_name(&common, item_set);
        result.push(Config {
            name: config_name,
            champion: champion.clone(),
            item_set: item_set.clone(),
        });
    }
    result
}

fn diff_item_set_name(common: &[ItemKind], set: &[ItemKind]) -> String {
    if common.is_empty() {
        let names: Vec<String> = set.iter().map(|item| format!("{:?}", item)).collect();
        return names.join(" + ");
    }
    let diff: Vec<_> = set.iter().filter(|item| !common.contains(item)).collect();
    if diff.is_empty() {
        "Common".to_string()
    } else {
        let names: Vec<String> = diff.iter().map(|item| format!("{:?}", item)).collect();
        format!("Common + {}", names.join(" + "))
    }
}

pub fn print_breakpoints(
    resistance_values: &[f32],
    damage_values: &[Vec<f32>],
    config_names: &[Config],
) {
    let num_configs = damage_values.len();
    let num_points = resistance_values.len();

    for i in 0..num_configs {
        for j in 0..num_points {
            let this = damage_values[i][j];
            let is_best = damage_values.iter().all(|series| this >= series[j]);
            if is_best {
                let damages: Vec<String> = damage_values
                    .iter()
                    .map(|series| format!("{:.2}", series[j]))
                    .collect();
                println!(
                    "Breakpoint for {} at resistance = {:.1}:",
                    &config_names[i].name, resistance_values[j],
                );
                println!(
                    "{}",
                    damages
                        .iter()
                        .enumerate()
                        .map(|(idx, dmg)| format!("{} = {}", &config_names[idx].name, dmg))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                break;
            }
        }
    }
}

pub fn print_average_percentage_difference(
    mr_values: &[f32],
    damage_values: &[Vec<f32>],
    config_names: &[Config],
) {
    let num_configs = damage_values.len();
    let num_points = mr_values.len();

    let mut max_avg_diff = 0.0;
    let mut max_pair = (0, 0);

    for i in 0..num_configs {
        for j in (i + 1)..num_configs {
            let mut total_percent_diff = 0.0;
            for k in 0..num_points {
                let a = damage_values[i][k];
                let b = damage_values[j][k];
                if b != 0.0 {
                    total_percent_diff += ((a - b).abs() / b.abs()) * 100.0;
                }
            }
            let avg_percent_diff = total_percent_diff / num_points as f32;
            if avg_percent_diff > max_avg_diff {
                max_avg_diff = avg_percent_diff;
                max_pair = (i, j);
            }
        }
    }

    println!(
        "Highest average percentage difference is between {} and {}: {:.2}%",
        &config_names[max_pair.0].name, &config_names[max_pair.1].name, max_avg_diff
    );
}

pub fn compute_damage_vs_resistance(
    configs: &[Config],
    ability_slot: AbilitySlot,
    ability_level: AbilityLevel,
    max_resistance: usize,
    resistance_step: f32,
    resistance_type: StatType,
) -> (Vec<f32>, Vec<Vec<f32>>) {
    let mut targets = Vec::new();
    let upper = (max_resistance as f32 / resistance_step).ceil();
    for i in 0..=upper as usize {
        let stats = Stats::from_parts(i as f32 * resistance_step, resistance_type);
        targets.push(Champion::custom_with_stats(stats));
    }

    let mr_values: Vec<f32> = targets
        .iter()
        .map(|target| target.stat_at_level(resistance_type))
        .collect();

    let mut damage_values = Vec::new();

    for config in configs {
        let items_applied = config.champion.apply_items_stats(config.item_set.clone());
        let config_damage_values: Vec<f32> = targets
            .iter()
            .map(|target| {
                calculate_ability_damage(ability_slot, ability_level, &items_applied, target)
            })
            .collect();
        damage_values.push(config_damage_values);
    }

    (mr_values, damage_values)
}

pub fn item_set_from_base(base_set: &[ItemKind], new_items: &[ItemKind]) -> Vec<ItemKind> {
    let mut new_set = base_set.to_vec();
    new_set.extend_from_slice(new_items);
    new_set
}

pub fn read_champion_data(
    champion: &ChampionKind,
) -> Result<ChampionRaw, Box<dyn std::error::Error>> {
    let path = Path::new("lib/champions").join(format!("{champion:?}.json"));
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

fn common_items(item_sets: &[Vec<ItemKind>]) -> Vec<ItemKind> {
    if item_sets.is_empty() {
        return vec![];
    }
    let mut common = item_sets[0].clone();
    for set in item_sets.iter().skip(1) {
        common.retain(|item| set.contains(item));
    }
    common
}
