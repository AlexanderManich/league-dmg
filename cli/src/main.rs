use std::{fs, process::Command};

use lib::*;

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

    let champion_data = read_champion_data(&champions_t::ChampionKind::Zed)?;

    let champion = Champion::from(champion_data);

    let mut item_sets = Vec::new();

    let base_set = vec![items::ItemKind::TrinityForce, items::ItemKind::Muramana];

    let sorc = item_set_from_base(&base_set, &[items::ItemKind::TheCollector]);
    let spellslinger = item_set_from_base(&base_set, &[items::ItemKind::LordDominiksRegards]);

    item_sets.push(sorc);
    item_sets.push(spellslinger);

    let configs = create_item_configs(&champion, item_sets);

    let (mr_values, damage_values) = compute_damage_vs_resistance(
        &configs,
        AbilitySlot::Q,
        AbilityLevel::Five,
        200,
        5.0,
        StatType::Armor,
    );

    plot_damage_vs_resistance(
        &mr_values,
        &damage_values,
        &configs,
        "graphs/damage_vs_mr.png",
    )?;

    print_breakpoints(&mr_values, &damage_values, &configs);
    print_average_percentage_difference(&mr_values, &damage_values, &configs);

    Ok(())
}
