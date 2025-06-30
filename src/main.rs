use std::error::Error;
use std::path::PathBuf;
use std::{default, fs};
use std::{path::Path, process::Command};
mod champions_t;
use strum::IntoEnumIterator;

use crate::champions_t::{Ability, Champion, ChampionKind, Leveling, Modifier};

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
    BonusHealth,
    MissingHealth,
    Armor,
    MagicResist,
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

            //         "% per 100 bonus AD" => AbilityScaling::PercentPer100BonusAD(value),
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
            //         "% AP per 100 bonus health" => AbilityScaling::PercentAPPer100BonusHealth(value),
            //         "% of Sona's AP" => AbilityScaling::PercentAP {
            //             base: value,
            //             bonus_per_100_ap: None,
            //             bonus_per_100_bonus_health: None,
            //         },
            "% of Ivern's AP" => AbilityScaling {
                stat: Stat::AP,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            //         "% maximum health" => AbilityScaling::PercentMaxHealth {
            //             base: value,
            //             bonus_per_100_ap: None,
            //         },
            //         "% of maximum health" => AbilityScaling::PercentMaxHealth {
            //             base: value,
            //             bonus_per_100_ap: None,
            //         },
            "% of her maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            //         "% bonus health" => AbilityScaling::PercentBonusHealth(value),
            //         "% of his bonus health" => AbilityScaling::PercentBonusHealth(value),
            "% of target's maximum health" => AbilityScaling {
                stat: Stat::MaxHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            //         "%  of target's maximum health" => AbilityScaling::PercentTargetMaxHealth(value),
            //         "% of the target's maximum health" => AbilityScaling::PercentTargetMaxHealth(value),
            //         "%  of the target's maximum health" => AbilityScaling::PercentTargetMaxHealth(value),
            //         "% of target's current health" => AbilityScaling::PercentTargetCurrentHealth(value),
            //         "%  of target's current health" => AbilityScaling::PercentTargetCurrentHealth(value),
            "% of target's missing health" => AbilityScaling {
                stat: Stat::MissingHealth,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            //         "% of Braum's maximum health" => AbilityScaling::PercentMaxHealth {
            //             base: value,
            //             bonus_per_100_ap: None,
            //         },
            //         "% of Zac's maximum health" => AbilityScaling::PercentMaxHealth {
            //             base: value,
            //             bonus_per_100_ap: None,
            //         },
            "% per 100 Pantheon's bonus health" => AbilityScaling {
                stat: Stat::BonusHealth,
                ratio: value,
                kind: ScalingKind::Per100Stat,
            },
            "% armor" => AbilityScaling {
                stat: Stat::Armor,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },
            //         "% total armor" => AbilityScaling::PercentArmor(value),
            //         "% bonus armor" => AbilityScaling::PercentBonusArmor(value),
            //         "% of target's armor" => AbilityScaling::PercentTargetArmor(value),

            //         "% total magic resistance" => AbilityScaling::PercentMR(value),
            //         "% bonus magic resistance" => AbilityScaling::PercentBonusMR(value),
            "% bonus mana" => AbilityScaling {
                stat: Stat::BonusMana,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            "% bonus movement speed" => AbilityScaling {
                stat: Stat::BonusMoveSpeed,
                ratio: value,
                kind: ScalingKind::PercentStat,
            },

            //         "% of Siphoning Strike stacks" => AbilityScaling::PercentStacks(value),
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
}

#[derive(Default, Debug)]
struct DamageComponent {
    base_damage: BaseDamage,
    scalings: Vec<AbilityScaling>,
}

enum DamageKind {
    Physcial,
    Magic,
    True,
    Mixed,
}

struct AbilityDamage {
    kind: DamageKind,
    damage: Vec<DamageComponent>,
}

fn leveling_contains_damage_effect(leveling: &Leveling) -> bool {
    let mut result = false;

    if leveling.attribute.to_lowercase().contains("damage") {
        result = true;
    }

    result
}

fn modifier_is_base_damage(modifier_idx: usize) -> bool {
    modifier_idx == 0
}

fn try_infer_base_damage_breakdown(
    damage_kind_str: &str,
    leveling: &Leveling,
    value: f32,
) -> Option<BaseDamage> {
    let mut result = None;

    let attribute = &leveling.attribute;
    // let damage_kind = match damage_kind_str {
    //     "PHYSICAL_DAMAGE" => DamageKind::Physcial,
    //     "MAGIC_DAMAGE" => DamageKind::Magic,
    //     "TRUE_DAMAGE" => DamageKind::True,
    //     "MIXED_DAMAGE" => DamageKind::Mixed,
    //     // Skarner W is classified as other damage for some reason???
    //     "OTHER_DAMAGE" => DamageKind::Magic,
    //     _ => panic!("Unexpected damage kind"),
    // };

    if attribute.contains("maximum health") {
        result = Some(BaseDamage::PercentTargetMaxHealth(value));
    } else if attribute.contains("missing health") {
        result = Some(BaseDamage::PercentTargetMissingHealth(value));
    } else {
        result = Some(BaseDamage::Flat(value));
    }

    result
}

fn ability_damage_by_rank(
    ability_kind: AbilityKind,
    ability_rank: AbilityRank,
    champion: &Champion,
) -> Option<Vec<DamageComponent>> {
    let ability = champion.get_ability(ability_kind);
    // Check if ability is damaging ability
    ability.damage_type.as_ref()?;

    let mut result = Vec::new();

    let mut push_ability_damage = |levelings: &[Leveling]| {
        for leveling in levelings {
            let modifiers = &leveling.modifiers;

            if !leveling_contains_damage_effect(leveling) {
                continue;
            }

            let mut ability_damage = DamageComponent::default();
            for (i, modifier) in modifiers.iter().enumerate() {
                if let Some(modifier_str) = modifier.units.first() {
                    if modifier_is_base_damage(i) {
                        if let Some(v) = modifier.values.get(ability_rank as usize) {
                            if let Some(damage_type) = &ability.damage_type {
                                if let Some(base_damage) =
                                    try_infer_base_damage_breakdown(damage_type, leveling, *v)
                                {
                                    ability_damage.base_damage = base_damage;
                                }
                            }
                        };
                    } else if let Some(v) = modifier.values.get(ability_rank as usize) {
                        let scaling = AbilityScaling::from_parts(modifier_str, *v);
                        ability_damage.scalings.push(scaling);
                    }
                }
            }
            dbg!(&champion.name);
            dbg!(&leveling.attribute);
            dbg!(&ability_damage);
            result.push(ability_damage);
        }
    };

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
    ];

    let filtered_effects: Vec<_> = ability
        .effects
        .iter()
        .filter_map(|effect| {
            let filtered_levelings: Vec<_> = effect
                .leveling
                .iter()
                .filter(|leveling| {
                    !exclude_patterns
                        .iter()
                        .any(|pat| leveling.attribute.to_lowercase().contains(pat))
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
        push_ability_damage(&max_levelings);
    } else {
        for effect in &filtered_effects {
            push_ability_damage(&effect.leveling);
        }
    }

    Some(result)
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

    for champion_kind in ChampionKind::iter() {
        let champion = match read_champion_data(&champion_kind) {
            Ok(champ) => champ,
            Err(e) => {
                eprintln!("Failed to read data for {:?}: {}", champion_kind, e);
                continue;
            }
        };
        let damage = ability_damage_by_rank(AbilityKind::W, AbilityRank::One, &champion);
        // println!("{:?}: {:?}", champion_kind, damage);
        // println!();
    }
}

fn read_champion_data(champion: &ChampionKind) -> Result<Champion, Box<dyn std::error::Error>> {
    let path = Path::new("champions").join(format!("{:?}.json", champion));
    let data = fs::read_to_string(path)?;
    let champion = serde_json::from_str(&data)?;
    Ok(champion)
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
