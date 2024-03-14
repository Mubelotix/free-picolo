use std::{collections::HashMap, hash::Hash};

use serde_json::Value;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Debug)]
struct RawItem {
    cycle_state: bool,
    #[serde(rename = "type")]
    ty: i32,
    text: String,
    key: String,
    parent_key: String,
    pack_name: String,
    language: String,
    nb_players: i32,
}

#[derive(Debug)]
struct Item {
    cycle_state: bool,
    ty: i32,
    text: &'static str,
    key: Option<&'static str>,
    parent_key: Option<&'static str>,
    pack_name: &'static str,
    language: &'static str,
    nb_players: i32,
}

#[derive(Serialize, Debug)]
struct SplitItem {
    cycle_state: bool,
    ty: i32,
    text: &'static str,
    key: Option<&'static str>,
    parent_key: Option<&'static str>,
    nb_players: i32,
}

fn parse_items() -> Vec<Item> {
    // Parse data
    let data = include_str!("data/source.json");
    let json: Vec<Value> = serde_json::from_str(data).expect("Couldn't parse data.json");
    
    // Extract strings and raw items
    let mut strings: Vec<&'static str> = Vec::new();
    let mut raw_items = Vec::new();
    for (i, item) in json.into_iter().enumerate() {
        if item.is_string() {
            let string = item.as_str().unwrap().to_string();
            let string = string.leak();
            strings.push(string);
        } else {
            let Ok(item) = serde_json::from_value::<RawItem>(item) else {
                println!("Error parsing item {}", i);
                continue;
            };
            raw_items.push(item);
        }
    }

    // Collect items from strings and raw items
    let mut items = Vec::new();
    let first_i = raw_items.first().map(|raw_item| {
        raw_item.text.parse::<usize>().ok()
    }).flatten().expect("Couldn't find first item");
    for raw_item in raw_items {
        let text = raw_item.text.parse::<usize>().ok().map(|i| strings[i - first_i]).expect("Couldn't parse key to number");
        let key = raw_item.key.parse::<usize>().ok().map(|i| strings[i - first_i]).expect("Couldn't parse key to number");
        let parent_key = raw_item.parent_key.parse::<usize>().ok().map(|i| strings[i - first_i]).expect("Couldn't parse parent_key to number");
        let pack_name = strings[raw_item.pack_name.parse::<usize>().ok().expect("Couldn't parse pack_name to number") - first_i];
        let language = strings[raw_item.language.parse::<usize>().ok().expect("Couldn't parse language to number") - first_i];

        let key = if key == "" { None } else { Some(key) };
        let parent_key = if parent_key == "" { None } else { Some(parent_key) };

        let item = Item {
            cycle_state: raw_item.cycle_state,
            ty: raw_item.ty,
            text,
            key,
            parent_key,
            pack_name,
            language,
            nb_players: raw_item.nb_players,
        };
        items.push(item);
    }

    items
}

fn main() {
    let items = parse_items();

    let mut split_items: HashMap<(String, String), Vec<SplitItem>> = HashMap::new();
    for item in items {
        let split_item = SplitItem {
            cycle_state: item.cycle_state,
            ty: item.ty,
            text: item.text,
            key: item.key,
            parent_key: item.parent_key,
            nb_players: item.nb_players,
        };
        let key = (item.language.to_string(), item.pack_name.to_string());
        if let Some(vec) = split_items.get_mut(&key) {
            vec.push(split_item);
        } else {
            split_items.insert(key, vec![split_item]);
        }
    }

    for (key, items) in split_items {
        let path = format!("data/{}-{}.bin", key.0, key.1);
        let data = bincode::serialize(&items).unwrap();
        std::fs::write(path, data).unwrap();
    }
    
    println!("cargo:rerun-if-changed=data.json");


}
