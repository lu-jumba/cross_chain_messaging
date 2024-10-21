mod blockchain;
mod encryption;
mod storage;
mod wallet;
mod ui;

use rsa::{RsaPublicKey, RsaPrivateKey};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let message = use_state(|| "".to_string());
    let recipient_key = use_state(|| "".to_string());  // Simulate recipient's public key input

    let on_message_input = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    let on_recipient_input = {
        let recipient_key = recipient_key.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            recipient_key.set(input.value());
        })
    };

    let on_submit = {
        let message = message.clone();
        let recipient_key = recipient_key.clone();
        Callback::from(move |_| {
            let message = (*message).clone();
            let recipient_key = (*recipient_key).clone();

            // Async task for encrypting and storing the message
            spawn_local(async move {
                let recipient_public_key = base64::decode(&recipient_key).unwrap();
                let public_key = RsaPublicKey::from_pkcs1(&recipient_public_key).unwrap();

                // Encrypt the message
                let encrypted_message = encryption::encrypt_message(&public_key, message.as_bytes());

                // Store the encrypted message in IPFS
                let ipfs_hash = storage::store_message_to_ipfs(&encrypted_message).await.unwrap();

                // (Optional) You could now send the IPFS hash and signature to the blockchain
                log::info!("Message stored on IPFS with hash: {}", ipfs_hash);
            });
        })
    };

    html! {
        <div>
            <h1>{ "Blockchain Messaging" }</h1>
            <input
                type="text"
                placeholder="Enter Recipient Public Key (Base64)"
                value={(*recipient_key).clone()}
                oninput={on_recipient_input}
            />
            <input
                type="text"
                placeholder="Enter your message"
                value={(*message).clone()}
                oninput={on_message_input}
            />
            <button onclick={on_submit}>{ "Send" }</button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
