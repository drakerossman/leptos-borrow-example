
use wasm_bindgen::JsValue;
use leptos::{component, create_signal, view, For, IntoView, ReadSignal, WriteSignal, SignalGet, SignalSet, event_target_checked, mount_to_body};
use serde_json;
use serde::{Deserialize, Serialize};
use gloo_console::log as gloo_log;


fn main() {
    mount_to_body(|| view! { <TopLevelWithItems/>} )
}

#[component]
pub fn TopLevelWithItems() -> impl IntoView {

    let filtered_items_from_localstorage = serde_json::json!(
        [
            {
                "value": "something",
                "title": true,
                "oplink": true,
                "comment": true,
                "username": true,
            },
            {
                "value": "something else",
                "title": true,
                "oplink": true,
                "comment": true,
                "username": true,
            }
        ]
    );

    let vec_of_filtered_items: Vec<FilteredItem> = serde_json::from_value(filtered_items_from_localstorage).unwrap();

    let (filtered_items, set_filtered_items) = create_signal(vec_of_filtered_items);

    let update_items_in_localstorage = move || {

        let clone_of_filtered_items = filtered_items.get();

        let filtered_items_as_json_string = serde_json::to_string_pretty(&clone_of_filtered_items).unwrap();

        let js_value_from_str = JsValue::from_str(&filtered_items_as_json_string);

        gloo_log!("Filtered Items are:\n");
        gloo_log!(js_value_from_str);
    };

    view! {
        <div class="filtering-menu" style="padding: 10px; nbackground-color: rgb(255, 241, 225); display: grid; grid-template-columns: max-content;">
            <FilterInputAndCheckboxes/>
            <AllFilteredItems
                filtered_items
                set_filtered_items
            />
        </div>
    }
}

#[component]
pub fn FilterInputAndCheckboxes() -> impl IntoView {

    let (title_checkbox    , set_title_checkbox   ) = create_signal(false);
    let (oplink_checkbox   , set_oplink_checkbox  ) = create_signal(false);
    let (comment_checkbox  , set_comment_checkbox ) = create_signal(false);
    let (username_checkbox , set_username_checkbox) = create_signal(false);

    let all_checkboxes_are_true =  move || {

        [
            title_checkbox.get()   ,
            oplink_checkbox.get()  ,
            comment_checkbox.get() ,
            username_checkbox.get(),
        ]
            .iter()
            .all(|&s| s)

    };

    view! {
        <div>
            <div class="filter-input-and-checkboxes" style="padding: 10px; nbackground-color: bisque;">

                <div class="filter-input" style="padding: 10px; nbackground-color: burlywood;">
                    <input></input>
                    <button>"Filter"</button>
                </div>

                <br/>

                <div class="to-filter-checkboxes" style="padding: 10px; nbackground-color: burlywood;">
                        <input type="checkbox" id="title-to-filter-checkbox" name="title-to-filter-checkbox"
                            prop:checked=title_checkbox
                            on:input=move |ev| set_title_checkbox.set(event_target_checked(&ev))
                        />
                        <label for="title-to-filter-checkbox">"Title"</label>
                    <br/>
                        <input type="checkbox" id="op-link-to-filter-checkbox" name="op-link-to-filter-checkbox"
                            prop:checked=oplink_checkbox
                            on:input=move |ev| set_oplink_checkbox.set(event_target_checked(&ev))
                        />
                        <label for="op-link-to-filter-checkbox">"OP Link"</label>
                    <br/>
                        <input type="checkbox" id="comment-to-filter-checkbox" name="comment-to-filter-checkbox"
                            prop:checked=comment_checkbox
                            on:input=move |ev| set_comment_checkbox.set(event_target_checked(&ev))
                        />
                        <label for="comment-to-filter-checkbox">"Comment"</label>
                    <br/>
                        <input type="checkbox" id="username-to-filter-checkbox" name="username-to-filter-checkbox"
                            prop:checked=username_checkbox
                            on:input=move |ev| set_username_checkbox.set(event_target_checked(&ev))
                        />
                        <label for="username-to-filter-checkbox">"Username"</label>
                </div>

                "All checkboxes are true: " {all_checkboxes_are_true}
            </div>
        </div>
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FilteredItem
{
    value   : String,
    title   : bool,
    oplink  : bool,
    comment : bool,
    username: bool,
}

#[component]
pub fn AllFilteredItems
(
    filtered_items: ReadSignal<Vec<FilteredItem>>,
    set_filtered_items: WriteSignal<Vec<FilteredItem>>
)
-> impl IntoView
{

    view! {
        <div class="all-filtered-items">
            <For
                each=move || filtered_items.get()
                key=|state| state.value.clone()
                let:filtered_item
            >
                <FilteredItemWithCheckboxes
                    filtered_value = filtered_item.value
                    title = filtered_item.title
                    oplink = filtered_item.oplink
                    comment = filtered_item.comment
                    username = filtered_item.username
                    filtered_items = filtered_items
                    set_filtered_items = set_filtered_items
                />
            </For>
        </div>
    }
}

#[component]
pub fn FilteredItemWithCheckboxes
(
    filtered_value: String,
    title         : bool,
    oplink        : bool,
    comment       : bool,
    username      : bool,
    filtered_items: ReadSignal<Vec<FilteredItem>>,
    set_filtered_items: WriteSignal<Vec<FilteredItem>>,
)
 -> impl IntoView {

    let (title_checkbox    , set_title_checkbox   ) = create_signal(title   );
    let (oplink_checkbox   , set_oplink_checkbox  ) = create_signal(oplink  );
    let (comment_checkbox  , set_comment_checkbox ) = create_signal(comment );
    let (username_checkbox , set_username_checkbox) = create_signal(username);

    let any_checkbox_is_true =  move || {

        set_filtered_items.set(filtered_items.get());

        [
            title_checkbox.get()   ,
            oplink_checkbox.get()  ,
            comment_checkbox.get() ,
            username_checkbox.get(),
        ]
            .iter()
            .any(|&s| s)

    };

    view! {
        <div prop:filtered_value=&filtered_value class="filtered-item" style="padding: 10px; nbackground-color: burlywood;">
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
                <input type="checkbox" class="filtered-comment-checkbox" name="filtered-comment-checkbox"
                    prop:checked=comment_checkbox
                    on:input=move |ev| set_comment_checkbox.set(event_target_checked(&ev))
                />
                <label for="filtered-comment-checkbox">"Comment"</label>
            <br/>
                <input type="checkbox" class="filtered-username-checkbox" name="filtered-username-checkbox"
                    prop:checked=username_checkbox
                    on:input=move |ev| set_username_checkbox.set(event_target_checked(&ev))
                />
                <label for="filtered-username-checkbox">"Username"</label>
            <br/>

            "Any checkbox is true: " {any_checkbox_is_true}
        </div>
    }
}
