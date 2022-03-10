mod phantom;
mod phantom_connect;

use phantom_connect::PhantomConnect;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-screen h-screen flex items-center justify-center">
            <div class="flex flex-col items-center justify-center">
                <h1  class="p-10 text-xl font-bold">{ "Connect to Phantom Wallet" }</h1>
                <PhantomConnect/>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
