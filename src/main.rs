use std::process::Command;
use std::fs;
mod champions_t;
use crate::champions_t::Champion;

enum Ability {
    P,
    Q,
    W,
    E,
    R
}

impl champions_t::Champion {
    fn get_ability(&self, ability: Ability) -> &Vec<champions_t::Ability> {
        match ability {
            Ability::P => &self.abilities.p,
            Ability::Q => &self.abilities.q,
            Ability::W => &self.abilities.w,
            Ability::E => &self.abilities.e,
            Ability::R => &self.abilities.r,
        }
    }
}

fn ability_damage_by_rank(ability: Ability, champion: &Champion) {
    for ability in champion.get_ability(ability).iter() {
        for effect in ability.effects.as_ref().unwrap() {
            for leveling in &effect.leveling {
                println!("{}", leveling.attribute);
                if leveling.attribute.contains("Total") {
                    println!("True");
                }
                for (i, modifier) in leveling.modifiers.iter().enumerate() {
                    println!("Values by rank: {:?} {:?}", modifier.values, modifier.units);
                }
            }
        }
    }
}

fn main() {
    if !fs::exists("champions").expect("Couldn't find champions folder")  {
        let champions_path = "../lolstaticdata/champions";
        let copy_champions = Command::new("cp")
                                            .args(["-r", champions_path, "."])
                                            .output()
                                            .expect("Failed to copy champions folder");
        if !copy_champions.status.success() {
            eprintln!("Champions cp failed");
        }
    }

    let data= fs::read_to_string("champions/Ahri.json").unwrap();
    let aatrox: Champion = serde_json::from_str(&data).unwrap();
    ability_damage_by_rank(Ability::Q, &aatrox);
}
