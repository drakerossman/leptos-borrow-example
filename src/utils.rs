use crate::FilteredItem;
use serde_json::Value;
use web_sys::window;

/// Reads the local storage and returns the filtered items as a Json string.
pub fn read_local_storage() -> String {
    let local_storage = window().unwrap().local_storage().unwrap().unwrap();

    local_storage.get_item("filtered_items").unwrap().unwrap()
}

/// Sets the local storage with the filtered items as a Json string.
pub fn set_local_storage(filtered_items_as_json_string: String) {
    let local_storage = window().unwrap().local_storage().unwrap().unwrap();

    local_storage
        .set_item("filtered_items", &filtered_items_as_json_string)
        .unwrap();
}

/// Even though this match looks scary, it ain't nothing to be afraid of.
pub fn to_vec_of_filtered_items(initial_filtered_items: Value) -> Vec<FilteredItem> {
    match initial_filtered_items {
        Value::Object(map) => map
            .into_iter()
            .filter_map(|(key, value)| match value {
                Value::Object(value_map) => Some(FilteredItem {
                    value: key,
                    title: value_map.get("title")?.as_bool()?,
                    oplink: value_map.get("oplink")?.as_bool()?,
                }),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Serializes string to serde_json::Value,
/// updates and key in Json with the new state of checkboxes,
/// and writes the new Json to local storage.
pub fn update_a_filtered_item(
    filtered_items_as_json_string: &str,
    key_to_update: &str,
    new_filtered_item: &Value,
) {
    let mut filtered_items: Value = serde_json::from_str(&filtered_items_as_json_string).unwrap();

    let filtered_items_as_object_mut = filtered_items.as_object_mut().unwrap();

    filtered_items_as_object_mut.insert(key_to_update.to_string(), new_filtered_item.clone());

    let filtered_items_as_json_string = serde_json::to_string_pretty(&filtered_items).unwrap();

    set_local_storage(filtered_items_as_json_string);
}
