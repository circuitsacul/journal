use std::{
    collections::{HashMap, HashSet},
    future::Future,
    hash::Hash,
    pin::Pin,
    rc::Rc,
    time::Duration,
};

use dioxus::prelude::*;
use futures_timer::Delay;

type FutureOut<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
type GetIDsAround<T> = fn(T) -> FutureOut<HashSet<T>>;
type GetEntry<IdT, EntryT> = fn(IdT) -> FutureOut<EntryT>;
type RenderEntry<EntryT> = fn(EntryT) -> Element;

#[derive(Clone)]
struct Entry<EntryT> {
    pub height: Option<f64>,
    pub data: EntryT,
    pub hidden: bool,
}

#[component]
pub fn BiScrollFeed<IdT, EntryT>(
    initial: IdT,
    update_dist: f64,
    get_ids_around: GetIDsAround<IdT>,
    get_entry: GetEntry<IdT, EntryT>,
    render_entry: RenderEntry<EntryT>,
) -> Element
where
    IdT: ToString + Hash + Clone + Copy + PartialEq + Ord + 'static,
    EntryT: Clone + 'static,
{
    let mut scrollable = use_signal(|| Option::<Rc<MountedData>>::None);
    let mut outer = use_signal(|| Option::<Rc<MountedData>>::None);

    let mut entries = use_signal(move || HashMap::<IdT, Entry<EntryT>>::new());
    let sorted_entry_ids = use_memo(move || {
        let mut keys: Vec<_> = entries.read().keys().cloned().collect();
        keys.sort();
        keys
    });

    use_future(move || async move {
        let mut focused = initial;
        loop {
            futures_timer::Delay::new(Duration::from_millis(100)).await;

            let Some(scrollable_s) = scrollable() else {
                continue;
            };
            let Some(outer_s) = outer() else {
                continue;
            };
            let pos = scrollable_s.get_scroll_offset().await.unwrap();
            let size = scrollable_s.get_scroll_size().await.unwrap();

            let outer_height = outer_s.get_scroll_size().await.unwrap();
            if pos.y > update_dist && pos.y < (size.height - (outer_height.height + update_dist)) {
                // we only want to update if we're a certain distance (update_dist) from the ends
                continue;
            }

            // calculate the focused entry
            let entries_c = entries();
            let sorted_entry_ids_c = sorted_entry_ids();

            let mut total_height = 0.;
            for entry_id in &sorted_entry_ids_c {
                let entry = entries.read()[entry_id].clone();
                total_height += entry.height.unwrap();
                if total_height >= pos.y {
                    focused = *entry_id;
                    break;
                }
            }

            // now, fetch entry IDs around the focused entry
            let entry_ids = get_ids_around(focused).await;

            // compare new entry IDs to current entries keys
            let current_ids = entries_c.keys().cloned().collect();
            let added_ids = entry_ids.difference(&current_ids);
            let removed = current_ids.difference(&entry_ids);

            // for every removed entry, remove it from the list whilst tracking the effect it
            // would have on the visual scroll position
            let mut scroll_by = 0.;

            {
                let mut entries_w = entries.write();
                for id in removed {
                    let entry = entries_w.remove(id);
                    if id >= &focused {
                        // removing items from the list only effects the scroll position
                        // if they came before the focused entry
                        continue;
                    }
                    if let Some(Entry {
                        height: Some(height),
                        ..
                    }) = entry
                    {
                        scroll_by -= height;
                    }
                }
            }

            // for every added ID, we need to fetch the new entry
            let added = added_ids
                .clone()
                .map(|id| async move { (id, get_entry(*id).await) });
            let added = futures::future::join_all(added).await;

            // now, we add them to `entries`
            {
                let mut entries_w = entries.write();
                for (id, entry) in added {
                    entries_w.insert(
                        *id,
                        Entry {
                            height: None,
                            data: entry,
                            hidden: true,
                        },
                    );
                }
            }

            // for each added ID, get the height of the new entry
            for id in added_ids.clone() {
                if id >= &focused {
                    // but only if they were added before the focused entry
                    continue;
                }

                // items are not immediatly added to the DOM, so we need to wait until
                // they are. Once mounted, the entry will automatically update the entries
                // with its calculated height
                loop {
                    if let Some(entry_height) = entries.read()[id].height {
                        scroll_by += entry_height;
                        break;
                    };
                    Delay::new(Duration::from_millis(10)).await;
                }
            }

            // now that everything is on the DOM, we can un-hide the elements...
            entries
                .write()
                .values_mut()
                .for_each(|val| val.hidden = false);

            // ...and then immediatly scroll to maintain the visual position
            if scroll_by != 0. {
                let mut pos = scrollable_s.get_scroll_offset().await.unwrap();
                pos.y += scroll_by;
                scrollable_s
                    .scroll(pos, ScrollBehavior::Instant)
                    .await
                    .unwrap();
            }
        }
    });

    rsx! {
        div {
            onmounted: move |data| outer.set(Some(data.data())),
            class: "h-full",

            div {
                class: "h-full overflow-y-scroll overscroll-none",
                onmounted: move |data| scrollable.set(Some(data.data())),

                for (id, entry) in sorted_entry_ids()
                    .into_iter()
                    .map(|id| (id, entries.read()[&id].clone()))
                {
                    div {
                        key: "{id.to_string()}",
                        class: "w-full",
                        style: if entry.hidden { "position: absolute; visibility: hidden;" },
                        onmounted: move |data| async move {
                            let size = data.data().get_scroll_size().await.unwrap();
                            if let Some(entry) = entries.write().get_mut(&id) {
                                entry.height = Some(size.height);
                            }
                        },

                        {render_entry(entry.data.to_owned())}
                    }
                }
            }
        }
    }
}
