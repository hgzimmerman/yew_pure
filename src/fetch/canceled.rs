use yew::{Renderable, Properties, Component, Html, ComponentLink, Children, Callback};
pub struct Canceled<M: 'static> {
    props: CanceledProps<M>
}

#[derive(Properties)]
pub struct CanceledProps<M: 'static> {
    children: Children<Canceled<M>>,
    pub (crate) callback: Option<Callback<M>>
}

impl <M: 'static> Component for Canceled<M> {
    type Message = M;
    type Properties = CanceledProps<M>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Canceled {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        if let Some(callback) = &self.props.callback {
            callback.emit(msg)
        }
        false
    }
}


impl <M> Renderable<Canceled<M>> for Canceled<M> {
    fn view(&self) -> Html<Self>{
        self.props.children.iter().collect()
    }
}