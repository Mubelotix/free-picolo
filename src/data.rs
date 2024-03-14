use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SplitItem {
    pub cycle_state: bool,
    pub ty: i32,
    pub text: &'static str,
    pub key: Option<&'static str>,
    pub parent_key: Option<&'static str>,
    pub nb_players: i32,
}

pub fn get_items(pack: &'static str) -> Vec<SplitItem> {
    let data: &[u8] = match pack {
        "bar" => include_bytes!("../data/fr-bar.bin"),
        "default" => include_bytes!("../data/fr-default.bin"),
        "hot" => include_bytes!("../data/fr-hot.bin"),
        "silly" => include_bytes!("../data/fr-silly.bin"),
        "war" => include_bytes!("../data/fr-war.bin"),
        _ => panic!("Unknown pack"),
    };
    let data = bincode::deserialize::<Vec<SplitItem>>(data).unwrap();
    data
}
