use std::fs;
use std::io::Write;
use std::{path::Path, process::Command};
mod champions_t;
use strum::IntoEnumIterator;

use crate::champions_t::{Ability, Champion, ChampionKind, Leveling};

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

impl Champion {
    fn get_ability(&self, ability_kind: AbilityKind) -> &Ability {
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
enum Stat {
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

#[derive(Default, Debug)]
enum ScalingKind {
    #[default]
    PercentStat,
    Per100Stat,
}

#[derive(Debug, Default)]
struct AbilityScaling {
    stat: Stat,
    ratio: f32,
    kind: ScalingKind,
}

impl AbilityScaling {
    fn from_parts(kind: &str, value: f32) -> Self {
        match kind {
            // AD Scalings
            "% AD" => AbilityScaling {
                stat: Stat::AD,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% bonus AD" => AbilityScaling {
                stat: Stat::BonusAD,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% per 100 AD" => AbilityScaling {
                stat: Stat::AD,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },
            "% per 100 bonus AD" => AbilityScaling {
                stat: Stat::AD,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },

            // AP Scalings
            "% AP" => AbilityScaling {
                stat: Stat::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% per 100 AP" => AbilityScaling {
                stat: Stat::AP,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },
            "% of Ivern's AP" => AbilityScaling {
                stat: Stat::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of Sona's AP" => AbilityScaling {
                stat: Stat::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            // Health Scalings
            "% of her maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of target's maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of Braum's maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of Zac's maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of target's current health" => AbilityScaling {
                stat: Stat::CurrentHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "%  of target's current health" => AbilityScaling {
                stat: Stat::CurrentHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of target's missing health" => AbilityScaling {
                stat: Stat::MissingHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% bonus health" => AbilityScaling {
                stat: Stat::BonusHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% of his bonus health" => AbilityScaling {
                stat: Stat::BonusHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% per 100 Pantheon's bonus health" => AbilityScaling {
                stat: Stat::BonusHealth,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },

            // Armor Scalings
            "% armor" => AbilityScaling {
                stat: Stat::Armor,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% bonus armor" => AbilityScaling {
                stat: Stat::BonusArmor,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            "% per 100 bonus armor" => AbilityScaling {
                stat: Stat::BonusArmor,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },

            // Magic Restistance Scalings
            "% bonus magic resistance" => AbilityScaling {
                stat: Stat::BonusMagicResist,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            // Mana Scaling
            "% bonus mana" => AbilityScaling {
                stat: Stat::BonusMana,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            // Movement Speed Scalings
            "% bonus movement speed" => AbilityScaling {
                stat: Stat::BonusMoveSpeed,
                ratio: value,
                kind: ScalingKind::PercentStat,
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
    kind: DamageKind,
    base: BaseDamage,
    scalings: Vec<AbilityScaling>,
}

#[derive(Default, Copy, Clone, Debug)]
enum DamageKind {
    #[default]
    None,
    True,
    Physcial,
    Magic,
    Mixed,
}

#[derive(Debug)]
struct AbilityDamage {
    damage: Vec<DamageComponent>,
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
    ability: &Ability,
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
                    match kind.as_str() {
                        "PHYSICAL_DAMAGE" => DamageKind::Physcial,
                        "MAGIC_DAMAGE" => DamageKind::Magic,
                        "TRUE_DAMAGE" => DamageKind::True,
                        "MIXED_DAMAGE" => DamageKind::Mixed,
                        "OTHER_DAMAGE" => DamageKind::Magic,
                        _ => {
                            eprintln!("Unknown damage type: {}", kind);
                            DamageKind::None
                        }
                    }
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
                            kind: damage_kind,
                            base: BaseDamage::Flat(*value),
                            scalings: vec![],
                        });
                    } else if is_percent_damage_with_scaling(unit_str) {
                        dbg!(&ability.name);
                        let scaling = parse_percent_damage_with_scaling(unit_str);
                        if is_percent_target_max_health(unit_str) {
                            result.push(DamageComponent {
                                kind: damage_kind,
                                base: BaseDamage::PercentTargetMaxHealth(*value),
                                scalings: vec![scaling],
                            });
                        }
                    } else if is_percent_target_max_health(unit_str) {
                        result.push(DamageComponent {
                            kind: damage_kind,
                            base: BaseDamage::PercentTargetMaxHealth(*value),
                            scalings: vec![],
                        });
                    } else {
                        let scaling = AbilityScaling::from_parts(unit_str, *value);
                        if let Some(last_part) = result.last_mut() {
                            last_part.scalings.push(scaling);
                        } else {
                            result.push(DamageComponent {
                                kind: damage_kind,
                                base: BaseDamage::None,
                                scalings: vec![scaling],
                            });
                        }
                    }
                }
            }
        }
    }

    AbilityDamage { damage: result}
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
            dbg!(scaling_str);
            if let Some(percent_idx) = scaling_str.find("%") {
                scaling_unit_str = &scaling_str[percent_idx..];
                let value_str = &scaling_str[..percent_idx].trim();
                if let Ok(value) = value_str.parse::<f32>() {
                    dbg!(value);
                    scaling_value = value;
                }
            }
        }
    }

    AbilityScaling::from_parts(scaling_unit_str, scaling_value)
}

fn try_get_damage_kind(leveling: &Leveling) -> Option<DamageKind> {
    // Try to determine damage type based on attribute
    if leveling.attribute.to_lowercase().contains("bonus")
        && leveling.attribute.to_lowercase().contains("true")
    {
        Some(DamageKind::True)
    } else if leveling.attribute.to_lowercase().contains("mixed") {
        Some(DamageKind::Mixed)
    } else if leveling.attribute.to_lowercase().contains("magic") {
        Some(DamageKind::Magic)
    } else if leveling.attribute.to_lowercase().contains("physical") {
        Some(DamageKind::Physcial)
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

    if let Some(effect) = filtered_effects.iter().find(|effect| {
        effect.leveling.iter().any(|leveling| {
            max_patterns
                .iter()
                .any(|pat| leveling.attribute.contains(pat))
        })
    }) {
        let max_levelings: Vec<Leveling> = effect
            .leveling
            .iter()
            .filter(|leveling| {
                max_patterns
                    .iter()
                    .any(|pat| leveling.attribute.contains(pat))
            })
            .cloned()
            .collect();

        let is_mixed_damage = max_levelings
            .iter()
            .any(|leveling| leveling.attribute.to_lowercase().contains("mixed"));

        if is_mixed_damage {
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
        } else {
            result.push(ability_damage(&max_levelings, ability, ability_rank));
        }
    } else {
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

    for champion_kind in ChampionKind::iter() {
        let champion = match read_champion_data(&champion_kind) {
            Ok(champ) => champ,
            Err(e) => {
                eprintln!("Failed to read data for {:?}: {}", champion_kind, e);
                continue;
            }
        };
        if let Some(damage) = ability_damage_by_rank(AbilityKind::Q, AbilityRank::One, &champion) {
            let out = format!("{:#?}\n{:#?}\r\n", champion.name, damage);
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("file.txt")
                .unwrap();
            file.write_all(out.as_bytes()).unwrap();
        }
    }
}

fn read_champion_data(champion: &ChampionKind) -> Result<Champion, Box<dyn std::error::Error>> {
    let path = Path::new("champions").join(format!("{:?}.json", champion));
    let data = fs::read_to_string(path)?;
    let champion = serde_json::from_str(&data)?;
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
                Ok(champ) => champ,
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
