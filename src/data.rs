use serde::Deserialize;
use rand::{seq::{IteratorRandom, SliceRandom}, Rng, RngCore};

#[derive(Deserialize, Debug)]
pub struct SplitItem {
    pub cycle_state: bool,
    // 1: random
    // 2: règles personnelles
    // 3: règles collectives
    // 4: thème
    // 5: pénalités ultimes
    // 6: silly
    // 7: hot
    // 8-9-10-13: war
    // 11: thème, war
    // 12: le plus de, war
    // 14: tu préfères
    // 15: story
    // 16: observation, bar
    // 17: payer, bar
    // 18: inconnus, bar
    // 19: action, bar
    // 20: prénom, bar
    // 21: front, bar
    // 22: dessin, bar
    // 23: random
    // 24: juger

    pub ty: usize,
    pub text: &'static str,
    pub key: Option<&'static str>,
    pub parent_key: Option<&'static str>,
    pub nb_players: usize,
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
    bincode::deserialize::<Vec<SplitItem>>(data).unwrap()
}

struct JsRandom;
impl RngCore for JsRandom {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        getrandom::getrandom(&mut bytes).unwrap();
        u32::from_ne_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        getrandom::getrandom(&mut bytes).unwrap();
        u64::from_ne_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom::getrandom(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

pub fn build_storyline(pack: &'static str, n: usize, players: Vec<String>) -> Vec<String> {
    let items = get_items(pack);
    let mut storyline = Vec::new();

    while storyline.len() < n {
        let item = items.iter().filter(|item| item.ty != 15 && item.nb_players <= players.len()).choose(&mut JsRandom).unwrap();
        let mut selected_players = players.iter().choose_multiple(&mut JsRandom, item.nb_players);
        selected_players.shuffle(&mut JsRandom);
        let mut text = item.text.to_string();
        for player in selected_players {
            text = text.replacen("%s", player, 1);
        }
        if text.contains('$') {
            let penalty_number = JsRandom.gen_range(1..=players.len());
            text = text.replace('$', &penalty_number.to_string());
        }
        storyline.push(text);
    }

    storyline
}

#[test]
fn test() {
    let players = vec!["Alice".to_string(), "Bob".to_string()];
    let storyline = build_storyline("default", 20, players);
    println!("{:#?}", storyline);
}
