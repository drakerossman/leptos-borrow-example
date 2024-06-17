
use leptos::{component, create_signal, view, For, IntoView, WriteSignal, SignalGet, SignalSet, event_target_checked, mount_to_body};
use serde_json;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use gloo_console::log as gloo_log;
use web_sys::window;

fn main() {
    mount_to_body(|| view! { <TopLevelWithItems/>} )
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct FilteredItem
{
    value   : String,
    title   : bool,
    oplink  : bool,
    comment : bool,
    username: bool,
}

#[component]
pub fn TopLevelWithItems() -> impl IntoView {

    let initial_filtered_items = serde_json::json!(
        {
            "something":
            {
                "title": true,
                "oplink": true,
                "comment": true,
                "username": true,
            },

            "something else":
            {
                "title": true,
                "oplink": true,
                "comment": true,
                "username": true,
            }
        }
    );

    let filtered_items_as_json_string =
        serde_json::to_string_pretty(&initial_filtered_items.clone())
        .unwrap();

    let local_storage = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    local_storage
        .set_item("filtered_items", &filtered_items_as_json_string)
        .unwrap();

    let filtered_items: Vec<FilteredItem>
        = match initial_filtered_items.clone()
        {
            Value::Object(map)
                =>
                    map
                        .into_iter()
                        .filter_map(
                            |(key, value)| match value
                            {
                                Value::Object(value_map) => Some(
                                    FilteredItem
                                    {
                                        value: key,
                                        title: value_map.get("title")?.as_bool()?,
                                        oplink: value_map.get("oplink")?.as_bool()?,
                                        comment: value_map.get("comment")?.as_bool()?,
                                        username: value_map.get("username")?.as_bool()?,
                                    }
                                ),
                                _ => None,
                            }
                        )
                        .collect(),
            _
                =>
                    Vec::new(),
        };

    let (get_dummy_signal, set_dummy_signal) = create_signal(0);

    let read_local_storage = move || {

        get_dummy_signal.get();

        let local_storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        let filtered_items_as_json_string = local_storage
            .get_item("filtered_items")
            .unwrap()
            .unwrap();

        filtered_items_as_json_string
    };

    view! {
        <div class="filtering-menu" style="padding: 10px; background-color: rgb(255, 241, 225); display: grid; grid-template-columns: max-content;">
            <FilterInputAndCheckboxes/>
            <AllFilteredItems
                filtered_items
                set_dummy_signal
            />
        </div>
        {read_local_storage}
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
            <div class="filter-input-and-checkboxes" style="padding: 10px; background-color: bisque;">

                <div class="filter-input" style="padding: 10px; background-color: burlywood;">
                    <input></input>
                    <button>"Filter"</button>
                </div>

                <br/>

                <div class="to-filter-checkboxes" style="padding: 10px; background-color: burlywood;">
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


#[component]
pub fn AllFilteredItems
(
    filtered_items: Vec<FilteredItem>,
    set_dummy_signal: WriteSignal<i32>,
)
-> impl IntoView
{

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
                    comment = filtered_item.comment
                    username = filtered_item.username
                    set_dummy_signal = set_dummy_signal

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
    set_dummy_signal: WriteSignal<i32>,
)
 -> impl IntoView {

    let (title_checkbox    , set_title_checkbox   ) = create_signal(title   );
    let (oplink_checkbox   , set_oplink_checkbox  ) = create_signal(oplink  );
    let (comment_checkbox  , set_comment_checkbox ) = create_signal(comment );
    let (username_checkbox , set_username_checkbox) = create_signal(username);

    let filtered_value_clone = filtered_value.clone();

    let any_checkbox_is_true =  move || {

        let local_storage = window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        let filtered_items_as_json_string = local_storage
            .get_item("filtered_items")
            .unwrap()
            .unwrap();

        let mut filtered_items: Value = serde_json::from_str(&filtered_items_as_json_string).unwrap();

        let filtered_items_as_object_mut = filtered_items.as_object_mut().unwrap();

        let new_filtered_item = json!({
            "title": title_checkbox.get(),
            "oplink": oplink_checkbox.get(),
            "comment": comment_checkbox.get(),
            "username": username_checkbox.get(),
        });

        filtered_items_as_object_mut.insert(filtered_value_clone.to_string(), new_filtered_item);

        let filtered_items_as_json_string = serde_json::to_string_pretty(&filtered_items).unwrap();

        let filtered_items_as_value: Value = serde_json::from_str(&filtered_items_as_json_string).unwrap();

        gloo_log!("filtered_items_before: {:?}", filtered_items_as_value.to_string());

        local_storage
            .set_item("filtered_items", &filtered_items_as_json_string)
            .unwrap();

        set_dummy_signal.set(0);

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

