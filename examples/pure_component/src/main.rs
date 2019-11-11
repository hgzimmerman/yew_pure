use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod button;
use crate::button::Button;

pub struct Model {}

pub enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                log::info!("got message");
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <Button callback=Msg::from text = "Click me!" />
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
