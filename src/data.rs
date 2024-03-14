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
pub struct Item {
    pub cycle_state: bool,
    pub ty: i32,
    pub text: &'static str,
    pub key: Option<&'static str>,
    pub parent_key: Option<&'static str>,
    pub pack_name: &'static str,
    pub language: &'static str,
    pub nb_players: i32,
}

pub fn parse_items() -> Vec<Item> {
    // Parse data
    let data = include_str!("../data.json");
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
