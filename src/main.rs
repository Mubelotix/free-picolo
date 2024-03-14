mod data;
pub use data::*;

use yew::prelude::*;

pub enum AppState {
    SelectPlayers,
    SelectPack,
    Play,
}

pub struct App {
    state: AppState,
    players: Vec<String>,
    storyline: Vec<String>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::SelectPlayers,
            players: Vec::new(),
            storyline: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        todo!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
