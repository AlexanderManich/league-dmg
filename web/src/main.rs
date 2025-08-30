use std::time::Instant;
use std::{collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;
use sysinfo::System;

use axum::{extract::Query, response::Json, routing::get, Router};
use lib::{champions_t::ChampionKind, read_champion_data, Champion};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Measure start time and memory
    let start = Instant::now();
    let mut sys = System::new();
    sys.refresh_all();
    let mem_before = sys.used_memory();

    // Load all champions
    let mut champions = Vec::new();
    for kind in ChampionKind::iter() {
        if let Ok(data) = read_champion_data(&kind) {
            champions.push(Champion::from(data));
        }
    }

    // Measure end time and memory
    sys.refresh_all();
    let mem_after = sys.used_memory();
    let duration = start.elapsed();

    println!(
        "Loaded {} champions in {:.2?} seconds. Memory used: {} KB",
        champions.len(),
        duration,
        mem_after.saturating_sub(mem_before)
    );

    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/champions", get(get_champion));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_champion(Query(params): Query<HashMap<String, String>>) -> Json<serde_json::Value> {
    let names: Vec<_> = params
        .iter()
        .filter_map(|(k, v)| if k == "name" { Some(v) } else { None })
        .collect();

    let mut champions_json = Vec::new();

    for name in names {
        match ChampionKind::from_str(name) {
            Ok(kind) => match read_champion_data(&kind) {
                Ok(champion_data) => {
                    let champion = Champion::from(champion_data);
                    champions_json.push(json!({
                        "name": name,
                        "stats": champion.stats,
                    }));
                }
                Err(e) => {
                    println!("Failed to read champion data for '{}': {}", name, e);
                }
            },
            Err(e) => {
                println!("Failed to parse ChampionKind for '{}': {}", name, e);
            }
        }
    }

    Json(json!({ "champion": champions_json }))
}
