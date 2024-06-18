mod filtered_items;
use filtered_items::FilteredItem;
mod utils;
use utils::{
    read_local_storage, set_local_storage, to_vec_of_filtered_items, update_a_filtered_item,
};

use leptos::{
    component, create_signal, event_target_checked, mount_to_body, view, For, IntoView, SignalGet,
    SignalSet, WriteSignal,
};
use serde_json::json;

fn main() {
    mount_to_body(|| view! { <TopLevelWithItems/>})
}

#[component]
pub fn TopLevelWithItems() -> impl IntoView {
    let initial_filtered_items = serde_json::json!(
        {
            "something": {
                // `true` here and elsewhere are the default values for checkboxes
                "title": true,
                "oplink": true,
            },

            "something else": {
                "title": true,
                "oplink": true,
            }
        }
    );

    let filtered_items_as_json_string =
        serde_json::to_string_pretty(&initial_filtered_items.clone()).unwrap();

    set_local_storage(filtered_items_as_json_string);

    // this vector is used to supply values as props to the components for rendering
    let filtered_items: Vec<FilteredItem> =
        to_vec_of_filtered_items(initial_filtered_items.clone());

    let (dummy_signal, set_dummy_signal) = create_signal(0);

    let print_local_storage = move || {
        dummy_signal.get();

        read_local_storage()
    };

    view! {
        <div class="filtering-menu" style="padding: 10px; background-color: rgb(255, 241, 225); display: grid; grid-template-columns: max-content;">
            <AllFilteredItems
                filtered_items
                set_dummy_signal
            />
        </div>
        {print_local_storage}
    }
}

#[component]
pub fn AllFilteredItems(
    filtered_items: Vec<FilteredItem>,
    set_dummy_signal: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <div class="all-filtered-items">
            <For
                each=move || filtered_items.clone()
                key=|state| state.value.clone()
                let:filtered_item
            >
                <FilteredItemWithCheckboxes
                    filtered_value = filtered_item.value
                    title = filtered_item.title
                    oplink = filtered_item.oplink
                    set_dummy_signal = set_dummy_signal
                />
                <br/>
            </For>
        </div>
    }
}

#[component]
pub fn FilteredItemWithCheckboxes(
    filtered_value: String,
    title: bool,
    oplink: bool,
    set_dummy_signal: WriteSignal<i32>,
) -> impl IntoView {
    let (title_checkbox, set_title_checkbox) = create_signal(title);
    let (oplink_checkbox, set_oplink_checkbox) = create_signal(oplink);

    let key_to_update = filtered_value.clone();

    let any_checkbox_is_true = move || {
        let checkbox_state = [title_checkbox.get(), oplink_checkbox.get()];

        let filtered_items_as_json_string = read_local_storage();

        // gets the current state of the checkboxes
        let new_filtered_item = json!({
            "title": checkbox_state[0],
            "oplink": checkbox_state[1],
        });

        update_a_filtered_item(
            &filtered_items_as_json_string,
            &key_to_update,
            &new_filtered_item,
        );

        set_dummy_signal.set(0);

        checkbox_state.iter().any(|&s| s)
    };

    view! {
        <div prop:filtered_value=&filtered_value class="filtered-item" style="padding: 10px; background-color: burlywood;">
                {&filtered_value}
            <br/>
                <input type="checkbox" class="filtered-title-checkbox" name="filtered-title-checkbox"
                    prop:checked=title_checkbox
                    on:input=move |ev| set_title_checkbox.set(event_target_checked(&ev))
                />
                <label for="filtered-title-checkbox">"Title"</label>
            <br/>
                <input type="checkbox" class="filtered-op-link-checkbox" name="filtered-op-link-checkbox"
                    prop:checked=oplink_checkbox
                    on:input=move |ev| set_oplink_checkbox.set(event_target_checked(&ev))
                />

                <label for="filtered-op-link-checkbox">"OP Link"</label>
            <br/>
            "Any checkbox is true: " {any_checkbox_is_true}
        </div>
    }
}
