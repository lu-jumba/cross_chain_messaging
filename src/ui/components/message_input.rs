use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{blockchain::chain_selector::{ChainSelector, ChainType}, relayer::evm_to_substrate::listen_for_cross_chain_messages};
use ethers::prelude::*;
use substrate_api_client::{Api, XtStatus};

#[function_component(MessageInput)]
pub fn message_input() -> Html {
    let chain_selector = use_state(|| ChainType::EVM);
    let message = use_state(|| "".to_string());
    let recipient = use_state(|| "".to_string());
    let target_chain = use_state(|| "EVM".to_string());

    let on_message_input = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    let on_recipient_input = {
        let recipient = recipient.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            recipient.set(input.value());
        })
    };

    let on_target_chain_change = {
        let target_chain = target_chain.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            target_chain.set(input.value());
        })
    };

    let on_submit = {
        let message = message.clone();
        let recipient = recipient.clone();
        let chain_selector = chain_selector.clone();
        let target_chain = target_chain.clone();

        Callback::from(move |_| {
            let message = message.clone();
            let recipient = recipient.clone();
            let target_chain = target_chain.clone();

            spawn_local(async move {
                // Cross-chain logic
                let chain_selector = ChainSelector { chain_type: ChainType::EVM };  // Example: EVM client
                let client = chain_selector.get_client().await;

                // Check the target chain and handle cross-chain logic
                if *target_chain == "EVM" {
                    client.send_message(recipient.to_string(), message.to_string()).await.unwrap();
                } else if *target_chain == "Substrate" {
                    // Implement cross-chain relay to Substrate
                    let api = Api::new("wss://rpc.polkadot.io").unwrap();
                    listen_for_cross_chain_messages(client.signer, Address::from_str(&recipient).unwrap(), api).await.unwrap();
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Cross-Chain Messaging" }</h1>
            <label for="target_chain">{ "Target Chain:" }</label>
            <select id="target_chain" onchange={on_target_chain_change}>
                <option value="EVM">{ "EVM (Ethereum/BSC)" }</option>
                <option value="Substrate">{ "Substrate (Polkadot)" }</option>
            </select>

            <input type="text" placeholder="Enter Recipient" value={(*recipient).clone()} oninput={on_recipient_input} />
            <input type="text" placeholder="Enter Message" value={(*message).clone()} oninput={on_message_input} />
            <button onclick={on_submit}>{ "Send Message" }</button>
        </div>
    }
}
