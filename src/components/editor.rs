use dioxus::prelude::*;

use crate::mdast_to_html;

fn js_get_text(id: &str) -> String {
    format!(r#"return document.getElementById("{id}").innerText"#)
}

fn js_update_editor(id: &str, text: &str) -> String {
    let text = serde_json::to_string(text).unwrap();
    dbg!(format!("{text}"));
    format!(r#"return window.updateEditor("{id}", {text})"#)
}

#[component]
pub fn Editor(id: String) -> Element {
    let id = use_signal(move || id);
    let editor_id = use_memo(move || format!("{}-input", id()));

    let mut locked = use_signal(move || false);

    let render_markdown = move || async move {
        {
            let mut locked = locked.write();
            if *locked {
                return;
            }
            *locked = true;
        }

        let editor_id = editor_id.read();
        let editor_id = editor_id.as_str();

        if let Ok(text) = document::eval(&js_get_text(editor_id)).await {
            let text = text.as_str().unwrap_or_default().to_owned();

            let mdast = markdown::to_mdast(&text, &markdown::ParseOptions::gfm()).unwrap();
            let html = mdast_to_html::process(&text, &mdast);

            let _ = document::eval(&js_update_editor(editor_id, &html)).await;
        }

        locked.set(false);
    };

    let handle_keydown = move |e: Event<KeyboardData>| {
        if e.is_composing() {
            return;
        }
        if e.data.modifiers().contains(Modifiers::META) {
            return;
        }
        if !matches!(
            e.data.key(),
            Key::Character(_) | Key::Enter | Key::Delete | Key::Backspace
        ) {
            return;
        }
        if locked() {
            e.prevent_default();
            return;
        }
        spawn(render_markdown());
    };

    rsx! {
        div {
            id,
            class: "overflow-y-scroll flex-1",

            article {
                id: editor_id,
                class: "markdown-preview-editor prose dark:prose-invert",

                white_space: "pre",
                contenteditable: "plaintext-only",

                onkeydown: handle_keydown
            }
        }
    }
}
