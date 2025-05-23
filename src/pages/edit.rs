use crate::{Route, components::Editor};
use dioxus::prelude::*;

#[component]
pub fn Edit() -> Element {
    let id = "test".to_string();
    rsx! {
        div {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Edit {}, "Edit" }
        }

        Editor { id: id.as_str() }
    }
}
