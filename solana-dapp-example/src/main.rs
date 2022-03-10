mod button;
mod phantom;
mod phantom_connect;

use phantom_connect::PhantomConnect;
use wasm_logger;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-screen h-screen flex items-center justify-center">
            <div class="flex flex-col items-center justify-center">
                <PhantomConnect/>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
