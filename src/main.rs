use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::{path::Path, process::Command};
mod champions_t;
use strum::IntoEnumIterator;

use crate::champions_t::{Champion, ChampionKind, Leveling};

enum AbilityKind {
    P,
    Q,
    W,
    E,
    R,
}

#[derive(Copy, Clone)]
enum AbilityRank {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl champions_t::Champion {
    fn get_ability(&self, ability: AbilityKind) -> &champions_t::Ability {
        match ability {
            AbilityKind::P => &self.abilities.p,
            AbilityKind::Q => &self.abilities.q,
            AbilityKind::W => &self.abilities.w,
            AbilityKind::E => &self.abilities.e,
            AbilityKind::R => &self.abilities.r,
        }
    }
}

fn ability_damage_by_rank(
    ability_kind: AbilityKind,
    ability_rank: AbilityRank,
    champion: &Champion,
) -> Option<Vec<(f64, f64)>> {
    let ability = champion.get_ability(ability_kind);

    // Check if ability is damaging ability
    ability.damage_type.as_ref()?;

    let mut result = Vec::new();

    let mut push_ability_damage = |levelings: &[Leveling]| {
        for leveling in levelings {
            let modifiers = &leveling.modifiers;
            println!("{}", leveling.attribute);
            let ability_damage = modifiers
                .first()
                .and_then(|m| m.values.get(ability_rank as usize))
                .copied()
                .unwrap_or(0.0);
            let ability_ratio = modifiers
                .get(1)
                .and_then(|m| m.values.get(ability_rank as usize))
                .copied()
                .unwrap_or(0.0);
            result.push((ability_damage, ability_ratio));
        }
    };

    // TODO: Resistances Reduction is a valid thing, that i should take into account
    // TODO: Somehow handle abilties that buff auto attacks
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
        "heal"
    ];

    let filtered_effects: Vec<_> = ability
        .effects
        .iter()
        .filter_map(|effect| {
            // Filter the levelings inside each effect
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
            // Only keep the effect if it has any relevant levelings left
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
        let damage = ability_damage_by_rank(AbilityKind::Q, AbilityRank::One, &champion);
        println!("{:?}: {:?}", champion_kind, damage);
        println!();
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
            println!("Testing Champion: {:?}", champion_kind);
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
