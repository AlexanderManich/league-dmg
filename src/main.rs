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
enum AbilityRank {
    One,
    Two,
    Three,
    Four,
    Five,
}

enum AbilityKind {
    P,
    Q,
    W,
    E,
    R,
}

impl ChampionRaw {
    fn get_ability(&self, ability_kind: AbilityKind) -> &AbilityRaw {
        match ability_kind {
            AbilityKind::P => &self.abilities.p,
            AbilityKind::Q => &self.abilities.q,
            AbilityKind::W => &self.abilities.w,
            AbilityKind::E => &self.abilities.e,
            AbilityKind::R => &self.abilities.r,
        }
    }
}

#[derive(Default, Debug)]
enum StatKind {
    #[default]
    None,
    AD,
    BonusAD,
    AP,
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
enum ScalingKind {
    #[default]
    PercentStat,
    Per100Stat,
}

impl ScalingKind {
    fn apply(self, ratio: f32, value: f32) -> f32 {
        match self {
            ScalingKind::PercentStat => (ratio / 100.0) * value,
            ScalingKind::Per100Stat => (ratio / 100.0) * (value / 100.0),
        }
    }
}

#[derive(Debug, Default)]
struct AbilityScaling {
    stat: StatKind,
    ratio: f32,
    kind: ScalingKind,
    per_stat: Option<StatKind>,
}

impl AbilityScaling {
    fn from_parts(kind: &str, value: f32) -> Self {
        match kind {
            // AD Scalings
            "% AD" => AbilityScaling {
                stat: StatKind::AD,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% bonus AD" => AbilityScaling {
                stat: StatKind::BonusAD,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% per 100 AD" => AbilityScaling {
                stat: StatKind::AD,
                ratio: value,
                kind: ScalingKind::Per100Stat,
                per_stat: None,
            },
            "% per 100 bonus AD" => AbilityScaling {
                stat: StatKind::BonusAD,
                ratio: value,
                kind: ScalingKind::Per100Stat,
                per_stat: None,
            },

            // AP Scalings
            "% AP" => AbilityScaling {
                stat: StatKind::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% per 100 AP" => AbilityScaling {
                stat: StatKind::AP,
                ratio: value,
                kind: ScalingKind::Per100Stat,
                per_stat: None,
            },
            "% of Ivern's AP" => AbilityScaling {
                stat: StatKind::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of Sona's AP" => AbilityScaling {
                stat: StatKind::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% AP per 100 bonus health" => AbilityScaling {
                stat: StatKind::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: Some(StatKind::BonusHealth),
            },

            // Health Scalings
            "% of her maximum health" => AbilityScaling {
                stat: StatKind::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of target's maximum health" => AbilityScaling {
                stat: StatKind::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of Braum's maximum health" => AbilityScaling {
                stat: StatKind::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of Zac's maximum health" => AbilityScaling {
                stat: StatKind::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of target's current health" => AbilityScaling {
                stat: StatKind::CurrentHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "%  of target's current health" => AbilityScaling {
                stat: StatKind::CurrentHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of target's missing health" => AbilityScaling {
                stat: StatKind::MissingHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% bonus health" => AbilityScaling {
                stat: StatKind::BonusHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% of his bonus health" => AbilityScaling {
                stat: StatKind::BonusHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% per 100 Pantheon's bonus health" => AbilityScaling {
                stat: StatKind::BonusHealth,
                ratio: value,
                kind: ScalingKind::Per100Stat,
                per_stat: None,
            },

            // Armor Scalings
            "% armor" => AbilityScaling {
                stat: StatKind::Armor,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% bonus armor" => AbilityScaling {
                stat: StatKind::BonusArmor,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },
            "% per 100 bonus armor" => AbilityScaling {
                stat: StatKind::BonusArmor,
                ratio: value,
                kind: ScalingKind::Per100Stat,
                per_stat: None,
            },

            // Magic Restistance Scalings
            "% bonus magic resistance" => AbilityScaling {
                stat: StatKind::BonusMagicResist,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },

            // Mana Scaling
            "% bonus mana" => AbilityScaling {
                stat: StatKind::BonusMana,
                ratio: value,
                kind: ScalingKind::PercentStat,
                per_stat: None,
            },

            // Movement Speed Scalings
            "% bonus movement speed" => AbilityScaling {
                stat: StatKind::BonusMoveSpeed,
                ratio: value,
                kind: ScalingKind::PercentStat,
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
    damage_kind: DamageKind,
    base: BaseDamage,
    scalings: Vec<AbilityScaling>,
}

#[derive(Default, Copy, Clone, Debug)]
enum DamageKind {
    #[default]
    None,
    True,
    Physical,
    Magic,
    Mixed,
}

impl DamageKind {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "PHYSICAL_DAMAGE" => DamageKind::Physical,
            "MAGIC_DAMAGE" => DamageKind::Magic,
            "TRUE_DAMAGE" => DamageKind::True,
            "MIXED_DAMAGE" => DamageKind::Mixed,
            // Skarner W deals magic damage but is classified as OTHER for some reason...
            "OTHER_DAMAGE" => DamageKind::Magic,
            _ => DamageKind::None,
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
    ability_rank: AbilityRank,
) -> AbilityDamage {
    let mut result = Vec::new();

    for leveling in levelings {
        if !leveling_contains_damage_effect(leveling) {
            continue;
        }

        let damage_kind = match try_get_damage_kind(leveling) {
            None => {
                if let Some(kind) = &ability.damage_type {
                    DamageKind::from_str(kind)
                } else {
                    DamageKind::None
                }
            }
            Some(kind) => kind,
        };

        for modifier in &leveling.modifiers {
            if let Some(unit_str) = modifier.units.first() {
                if let Some(value) = modifier.values.get(ability_rank as usize) {
                    if is_flat_damage(unit_str) {
                        result.push(DamageComponent {
                            damage_kind,
                            base: BaseDamage::Flat(*value),
                            scalings: vec![],
                        });
                    } else if is_percent_damage_with_scaling(unit_str) {
                        let scaling = parse_percent_damage_with_scaling(unit_str);
                        if is_percent_target_max_health(unit_str) {
                            result.push(DamageComponent {
                                damage_kind,
                                base: BaseDamage::PercentTargetMaxHealth(*value),
                                scalings: vec![scaling],
                            });
                        }
                    } else if is_percent_target_max_health(unit_str) {
                        result.push(DamageComponent {
                            damage_kind,
                            base: BaseDamage::PercentTargetMaxHealth(*value),
                            scalings: vec![],
                        });
                    } else {
                        let scaling = AbilityScaling::from_parts(unit_str, *value);
                        if let Some(last_part) = result.last_mut() {
                            last_part.scalings.push(scaling);
                        } else {
                            result.push(DamageComponent {
                                damage_kind,
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

fn try_get_damage_kind(leveling: &Leveling) -> Option<DamageKind> {
    // Try to determine damage type based on attribute
    if leveling.attribute.to_lowercase().contains("true") {
        Some(DamageKind::True)
    } else if leveling.attribute.to_lowercase().contains("mixed") {
        Some(DamageKind::Mixed)
    } else if leveling.attribute.to_lowercase().contains("magic") {
        Some(DamageKind::Magic)
    } else if leveling.attribute.to_lowercase().contains("physical") {
        Some(DamageKind::Physical)
    } else {
        None
    }
}

fn ability_damage_by_rank(
    ability_kind: AbilityKind,
    ability_rank: AbilityRank,
    champion: &Champion,
) -> Option<Vec<AbilityDamage>> {
    let ability = champion.get_ability(ability_kind);
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

                result.push(ability_damage(&damage_components, ability, ability_rank));
            }
        } else {
            // Here we pass levelings individually, instead of passing the max_levelings vec, because we might have
            // abilities with multiple effects containing a "max_pattern" such as Aatrox Q that has different variants
            // (Sweetspot / Non-Sweetspot) where we want an individual AbilityDamage for each variant.
            // NOTE: We assume/assert that an ability variant only ever has one "max_pattern" leveling.
            for leveling in max_levelings {
                result.push(ability_damage(&[leveling], ability, ability_rank));
            }
        }
    } else {
        // No leveling had a "max_pattern" so we just pass all levelings in all effects
        for effect in &filtered_effects {
            result.push(ability_damage(&effect.leveling, ability, ability_rank));
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
    flat_armor_pen: f32,
    percent_armor_pen: f32,
    ability_power: f32,
    flat_magic_pen: f32,
    percent_magic_pen: f32,
    movespeed: Stat,
    critical_strike_damage: Stat,
    critical_strike_damage_modifier: Stat,
    attack_speed: Stat,
    attack_speed_ratio: Stat,
    attack_cast_time: Stat,
    attack_total_time: Stat,
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
            flat_armor_pen: 0.0,
            percent_armor_pen: 0.0,
            ability_power: 0.0,
            flat_magic_pen: 0.0,
            percent_magic_pen: 0.0,
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
    fn get_ability(&self, ability_kind: AbilityKind) -> &AbilityRaw {
        match ability_kind {
            AbilityKind::P => &self.abilities.p,
            AbilityKind::Q => &self.abilities.q,
            AbilityKind::W => &self.abilities.w,
            AbilityKind::E => &self.abilities.e,
            AbilityKind::R => &self.abilities.r,
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
            let base_damage = match comp.base {
                BaseDamage::Flat(value) => value,
                BaseDamage::PercentTargetMaxHealth(value) => {
                    target_champion.stats.health.total(target_champion.level) * (value / 100.0)
                }
                _ => 0.0,
            };

            let scaling_damage: f32 = comp
                .scalings
                .iter()
                .map(|scaling| match &scaling.stat {
                    StatKind::AP => scaling
                        .kind
                        .apply(scaling.ratio, source_champion.stats.ability_power),
                    _ => 0.0,
                })
                .sum();

            let pre_mitigation_damage = base_damage + scaling_damage;

            match comp.damage_kind {
                DamageKind::Magic => {
                    let magic_resistance = apply_penetration(
                        target_champion
                            .stats
                            .magic_resistance
                            .total(target_champion.level),
                        0.0,
                        0.0,
                        source_champion.stats.flat_magic_pen,
                        source_champion.stats.percent_magic_pen,
                    );
                    apply_mitigation(
                        pre_mitigation_damage,
                        magic_resistance
                    )
                }
                DamageKind::Physical => apply_mitigation(
                    pre_mitigation_damage,
                    target_champion.stats.armor.total(target_champion.level),
                ),
                DamageKind::True => pre_mitigation_damage,
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
        if let Some(damage) = ability_damage_by_rank(AbilityKind::Q, AbilityRank::One, champion) {
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
        champion.stats.ability_power = 100.0;
        champion.stats.flat_magic_pen = 10.0;
        if let Some(q_damage) = ability_damage_by_rank(AbilityKind::Q, AbilityRank::One, &champion)
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

    // for champion_kind in ChampionKind::iter() {
    //     let champion = match read_champion_data(&champion_kind) {
    //         Ok(champ) => Champion::from(champ),
    //         Err(e) => {
    //             eprintln!("Failed to read data for {:?}: {}", champion_kind, e);
    //             continue;
    //         }
    //     };
    // write_ability_damage(&champion);
    // if champion_kind == ChampionKind::Aatrox {
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
    fn all_champions_q_damage_by_rank() {
        for champion_kind in ChampionKind::iter() {
            let champion = match read_champion_data(&champion_kind) {
                Ok(champ) => Champion::from(champ),
                Err(e) => panic!(
                    "Failed to read data for {:?}: {} (file: champions/{:?}.json)",
                    champion_kind, e, champion_kind
                ),
            };
            dbg!("Testing Champion: {:?}", &champion_kind);
            let ability = champion.get_ability(AbilityKind::Q);
            if ability.damage_type.is_none() {
                println!(
                    "Skipping {:?} because Q is not a damage ability",
                    champion_kind
                );
                continue;
            }
            let rank1_damage = ability_damage_by_rank(AbilityKind::Q, AbilityRank::One, &champion);
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
