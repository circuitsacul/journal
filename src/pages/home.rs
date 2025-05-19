use std::collections::HashSet;

use dioxus::prelude::*;
use rand::Rng;

use crate::components::BiScrollFeed;

#[derive(Clone, PartialEq)]
struct EntryData {
    pub id: u64,
    pub title: String,
    pub height: u64,
}

async fn get_ids_around(id: u64) -> HashSet<u64> {
    const START_BUFFER: u64 = 50;
    const END_BUFFER: u64 = 50;
    const LAST_ENTRY: u64 = 100_000_000;
    let start = id.saturating_sub(START_BUFFER);
    let end = id.saturating_add(END_BUFFER).min(LAST_ENTRY);

    (start..=end).collect()
}

async fn get_entry(id: u64) -> EntryData {
    EntryData {
        id,
        title: format!("Entry {id}"),
        height: rand::rng().random_range(1..10),
    }
}

fn render_entry(entry: EntryData) -> Element {
    rsx! {
        div {
            class: "px-4 pt-4 w-full flex flex-col items-center",

            div {
                class: "rounded-lg bg-ctp-mantle p-4 w-full max-w-3xl",

                h1 { {entry.title} }
                for i in 0..entry.height {
                    p { "line {i}" }
                }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        BiScrollFeed {
            initial: 1_000,
            update_dist: 5_000.0,
            get_ids_around: move |id| Box::pin(get_ids_around(id)),
            get_entry: move |id| Box::pin(get_entry(id)),
            render_entry: render_entry,
        }
    }
}
