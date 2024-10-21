use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/static/wallet_connect.js")]
extern "C" {
    async fn connectWalletConnect() -> String;
}

#[derive(Properties, PartialEq)]
pub struct WalletProps {
    pub on_wallet_connected: Callback<String>,  // Emits the wallet address after connection
}

#[function_component(Wallet)]
pub fn wallet(props: &WalletProps) -> Html {
    let wallet_address = use_state(|| "".to_string());

    let connect_wallet_connect = {
        let wallet_address = wallet_address.clone();
        let on_wallet_connected = props.on_wallet_connected.clone();

        Callback::from(move |_| {
            let wallet_address = wallet_address.clone();
            let on_wallet_connected = on_wallet_connected.clone();

            spawn_local(async move {
                let address = connectWalletConnect().await;
                wallet_address.set(address.clone());

                // Emit the wallet address
                on_wallet_connected.emit(address);
            });
        })
    };

    html! {
        <div>
            <button onclick={connect_wallet_connect}>{ "Connect with WalletConnect" }</button>
            { if wallet_address.is_empty() {
                html! { <p>{ "No wallet connected" }</p> }
            } else {
                html! { <p>{ format!("Connected wallet: {}", *wallet_address) }</p> }
            }}
        </div>
    }
}
