use wasm_bindgen::prelude::*;
use yew::prelude::*;

enum Msg {
}
struct Model {
    readme: String,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            readme: "Test load".to_string(),
            link,
        }
    }

    fn update(&mut self, _: <Self as yew::Component>::Message) -> bool { todo!() }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ self.readme.clone() }</h1>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::<Model>::new().mount_to_body();
}