mod data;
pub use data::*;

use yew::prelude::*;
use yew_template::template_html;

pub enum AppState {
    SelectPlayers,
    SelectPack,
    SelectSettings,
    Play,
}

pub enum AppMsg {
    Back,
    Next,
}

pub struct App {
    state: AppState,
    players: Vec<String>,
    storyline: Vec<String>,
    storyline_duration: usize,
    max_rule_duration: usize,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::SelectPlayers,
            players: Vec::new(),
            storyline: Vec::new(),
            storyline_duration: 30,
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
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_next = ctx.link().callback(|_| AppMsg::Next);

        match self.state {
            AppState::SelectPlayers => template_html!("templates/select_players.html", ...),
            AppState::SelectPack => template_html!("templates/select_pack.html", ...),
            AppState::SelectSettings => template_html!("templates/select_settings.html", ...),
            AppState::Play => template_html!("templates/play.html"),
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
