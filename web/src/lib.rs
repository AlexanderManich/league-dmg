use leptos::logging::log;
use leptos::{prelude::*};
use leptos::task::spawn_local;
use lib::champions_t::*;
use strum::IntoEnumIterator;
use leptos::web_sys::{window, Response, Request, RequestInit, RequestMode};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[component]
fn ChampionSelector(
    champions: ReadSignal<Vec<ChampionRaw>>,
    selected: ReadSignal<Option<String>>,
    set_selected: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (search, set_search) = signal(String::new());

    view! {
        <div>
            <input
                type="text"
                placeholder="Search champion..."
                on:input=move |ev| set_search.set(event_target_value(&ev))
                style="margin-bottom: 1rem;"
            />
            <div style="display: flex; flex-wrap: wrap; gap: 1rem;">
                {move || {
                    let filter = search.get().to_lowercase();
                    if !filter.is_empty() {
                        champions.get().iter()
                        .filter(|champ| champ.name.to_lowercase().contains(&filter))
                        .map(|champ| {
                            let name = champ.name.clone();
                            let icon = champ.icon.clone();
                            log!("{name}");
                            view! {
                                <button
                                style="background: none; border: solid; cursor: pointer; display: flex; align-items: center; padding: 0.5rem 1rem; border-radius: 8px;"
                                on:click=move |_| set_selected.set(Some(name.clone()))
                                >
                                <img src={icon} alt={name.clone()} width="32" height="32" style="margin-right: 0.5rem;" />
                                <span>{name.clone()}</span>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                    else {
                        Vec::new()
                    }
                }}
            </div>
            <p>
                "Selected champion: " { move ||selected.get().unwrap_or_else(|| "None".to_string())}
            </p>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (champions, set_champions) = signal(Vec::<ChampionRaw>::new());
    let (selected, set_selected) = signal(None::<String>);

    spawn_local(async move {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let mut loaded = Vec::new();
        for champion in ChampionKind::iter() {
            let name = champion.to_string();
            let url = format!("/champions/{name}.json");

            let request = Request::new_with_str_and_init(&url, &opts).unwrap();
            let window = window().unwrap();
            let response_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

            let response: Response = response_value.dyn_into().unwrap();
            let json = JsFuture::from(response.json().unwrap()).await.unwrap();

            loaded.push(json);
        }

        let mut champions = Vec::new();
        for champion_json in loaded {
            let champion: ChampionRaw = from_value(champion_json).unwrap();
            champions.push(champion);
        }

        set_champions.set(champions);
    });

    view! {
        <div>
            <h1>"League Damage Calc"</h1>
            <ChampionSelector
                champions={champions}
                selected={selected}
                set_selected={set_selected}
            />
        </div>
    }
}
