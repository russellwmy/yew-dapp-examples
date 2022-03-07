use super::phantom::{solana, ConnectResponse};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties,  use_state};

use super::button::Button;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn PhantomConnect(_: &Props) -> Html {
    let account = use_state(|| "".to_string());
    let account_id = account.clone();

    let handle_click = Callback::from(move |_: MouseEvent| {
        let account_id = account.clone();

        if *account_id != "".to_string() {
            spawn_local(async move {    
                let response = solana.disconnect().await;
                web_sys::console::log_1(&format!("disconnected: {:?}", response).into());
                account_id.set("".to_string());
            });
            return
        }

        spawn_local(async move {    
            let response = solana.connect().await;
            web_sys::console::log_1(&format!("connected: {:?}", solana.is_connected()).into());
            if solana.is_connected() {
                let response: ConnectResponse = response.into_serde().unwrap();
                web_sys::console::log_1(&format!("{:?}", response.public_key).into());
                account_id.set(response.public_key)
            }
        });
    });

    html! {
        <>
            <h1  class="p-10 text-xl font-bold">
                { if *account_id == "".to_string() {
                    String::from("Connect to Phantom Wallet")
                } else { 
                    format!("Connected to {:?}", (*account_id).clone())
                }}
            </h1>
            <Button 
                value={if *account_id == "".to_string() {"Login Phantom"} else { "Logout Phantom"}}
                onclick={handle_click}
            />            
        </>
    }
}
