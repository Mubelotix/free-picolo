mod data;
pub use data::*;

use yew::prelude::*;
use yew_template::template_html;
use wasm_bindgen::JsCast;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub enum AppState {
    SelectPlayers,
    SelectPack,
    SelectSettings,
    Play,
}

pub enum Pack {
    Default,
    Hot,
    Silly,
    War,
    Bar,
}

pub enum AppMsg {
    Back,
    Next,
    AddPlayer,
    OnPlayerChanged(web_sys::Event),
    OnPlayerRemoved(web_sys::MouseEvent),
    OnPackSelected(Pack),
    OnSettingChanged(web_sys::Event),
}

pub struct App {
    state: AppState,
    players: Vec<String>,
    pack: Pack,
    storyline: Vec<String>,
    party_duration: usize,
    max_rule_duration: usize,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::SelectPlayers,
            players: vec![String::new(), String::new()],
            pack: Pack::Default,
            storyline: Vec::new(),
            party_duration: 30,
            max_rule_duration: 12,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: AppMsg) -> bool {
        match msg {
            AppMsg::Back => {
                match self.state {
                    AppState::SelectPack => self.state = AppState::SelectPlayers,
                    AppState::SelectSettings => self.state = AppState::SelectPack,
                    AppState::Play => self.state = AppState::SelectSettings,
                    _ => (),
                }
                true
            }
            AppMsg::Next => {
                match self.state {
                    AppState::SelectPlayers => self.state = AppState::SelectPack,
                    AppState::SelectPack => self.state = AppState::SelectSettings,
                    AppState::SelectSettings => self.state = AppState::Play,
                    _ => (),
                }
                true
            },
            AppMsg::AddPlayer => {
                self.players.push(String::new());
                true
            }
            AppMsg::OnPlayerChanged(event) => {
                let Some(target) = event.target() else {return true};
                let target: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                let i: usize = target.id().trim_start_matches("player-input-").parse().unwrap();
                let value = target.value();
                self.players[i] = value;
                false
            }
            AppMsg::OnPlayerRemoved(event) => {
                let Some(target) = event.target() else {return true};
                let target: web_sys::Element = target.dyn_into().unwrap();
                let i: usize = target.id().trim_start_matches("player-remove-").parse().unwrap();
                self.players.remove(i);
                true
            }
            AppMsg::OnPackSelected(pack) => {
                self.pack = pack;
                self.state = AppState::SelectSettings;
                true
            }
            AppMsg::OnSettingChanged(event) => {
                let Some(target) = event.target() else {return true};
                let target: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                let value = target.value();
                match target.id().trim_start_matches("setting-") {
                    "party-duration" => self.party_duration = value.parse().unwrap(),
                    "max-rule-duration" => self.max_rule_duration = value.parse().unwrap(),
                    _ => (),
                }
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_back = ctx.link().callback(|_| AppMsg::Back);
        let onclick_next = ctx.link().callback(|_| AppMsg::Next);

        match self.state {
            AppState::SelectPlayers => {
                let player_iter = self.players.iter().map(|s| s.to_owned());
                let i_iter = 0..self.players.len();
                let i2_iter = i_iter.clone();
                let onclick_add = ctx.link().callback(|_| AppMsg::AddPlayer);

                template_html!(
                    "templates/select_players.html",
                    onchange = { ctx.link().callback(AppMsg::OnPlayerChanged) },
                    onclick_remove = { ctx.link().callback(AppMsg::OnPlayerRemoved) },
                    ...
                )
            },
            AppState::SelectPack => {
                let onclick_default = ctx.link().callback(|_| AppMsg::OnPackSelected(Pack::Default));
                let onclick_silly = ctx.link().callback(|_| AppMsg::OnPackSelected(Pack::Silly));
                let onclick_bar = ctx.link().callback(|_| AppMsg::OnPackSelected(Pack::Bar));
                let onclick_hot = ctx.link().callback(|_| AppMsg::OnPackSelected(Pack::Hot));
                let onclick_war = ctx.link().callback(|_| AppMsg::OnPackSelected(Pack::War));
                let player_count = self.players.len();

                template_html!("templates/select_pack.html", ...)
            },
            AppState::SelectSettings => {
                let party_duration = self.party_duration.to_string();
                let max_rule_duration = self.max_rule_duration.to_string();

                template_html!(
                    "templates/select_settings.html",
                    onchange_setting = { ctx.link().callback(AppMsg::OnSettingChanged) },
                    ...
                )
            },
            AppState::Play => template_html!("templates/play.html"),
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
