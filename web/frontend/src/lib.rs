use leptos::prelude::*;

// List of champion names (you can generate this from ChampionKind in Rust and send from backend)
const CHAMPIONS: &[&str] = &[
    "Aatrox", "Ahri", "Akali", "Akshan", "Alistar", "Amumu", "Anivia", "Annie", "Corki", // etc...
];

const ABILITY_SLOTS: &[&str] = &["P", "Q", "W", "E", "R"];
const ABILITY_LEVELS: &[u8] = &[1, 2, 3, 4, 5];

#[component]
pub fn App() -> impl IntoView {
    let (champion, set_champion) = signal(CHAMPIONS[0]);
    let (ability_slot, set_ability_slot) = signal(ABILITY_SLOTS[1]); // Default to Q
    let (ability_level, set_ability_level) = signal(1u8);
    let (target, set_target) = signal(CHAMPIONS[1]);

    view! {
        <div style="background:#0a1a2f; min-height:100vh; color:#fff; margin:0; padding:2rem;">
            <h1>"League Damage Calculator"</h1>

            <label>
                "Champion: "
                <select on:change=move |ev| set_champion.set(Box::leak(event_target_value(&ev).into_boxed_str()))>
                    {CHAMPIONS.iter().map(|c| view! { <option value={*c}>{*c}</option> }).collect::<Vec<_>>()}
                </select>
            </label>

            <label>
                "Ability: "
                <select on:change=move |ev| set_ability_slot.set(Box::leak(event_target_value(&ev).into_boxed_str()))>
                    {ABILITY_SLOTS.iter().map(|a| view! { <option value={*a}>{*a}</option> }).collect::<Vec<_>>()}
                </select>
            </label>

            <label>
                "Level: "
                <select on:change=move |ev| set_ability_level.set(event_target_value(&ev).parse().unwrap_or(1))>
                    {ABILITY_LEVELS.iter().map(|l| {
                        let l_str = l.to_string();
                        view! { <option value={l_str.clone()}>{l_str.clone()}</option> }
                    }).collect::<Vec<_>>()}
                </select>
            </label>

            <label>
                "Target Champion: "
                <select on:change=move |ev| set_target.set(Box::leak(event_target_value(&ev).into_boxed_str()))>
                    {CHAMPIONS.iter().map(|c| view! { <option value={*c}>{*c}</option> }).collect::<Vec<_>>()}
                </select>
            </label>

            <p>
                "Source: " {champion}
                ", Ability: " {ability_slot}
                ", Level: " {ability_level}
                ", Target: " {target}
            </p>

            // Add more configuration options for target stats here if needed

            // Add a button to calculate damage (call backend API)
            <button>
                "Calculate Damage"
            </button>
        </div>
    }
}