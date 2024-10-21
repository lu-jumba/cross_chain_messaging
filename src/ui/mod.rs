//mod components;
use components::{MessageInput, MessageThread, Status, Wallet};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let messages = use_state(|| vec![]);
    let status = use_state(|| "Ready to send messages.".to_string());
    let wallet_address = use_state(|| "".to_string());

    let on_wallet_connected = {
        let wallet_address = wallet_address.clone();
        Callback::from(move |address: String| {
            wallet_address.set(address);
        })
    };

    let on_message_sent = {
        let messages = messages.clone();
        let status = status.clone();

        Callback::from(move |(message, ipfs_hash): (String, String)| {
            let mut new_messages = (*messages).clone();
            new_messages.push((message, ipfs_hash));
            messages.set(new_messages);

            status.set("Message stored on IPFS and transaction completed.".into());
        })
    };

    let chain_selector = use_state(|| ChainSelector { chain_type: ChainType::EVM });
    
    let on_message_send = {
        let chain_selector = chain_selector.clone();
        Callback::from(move |_| {
            let chain_selector = chain_selector.clone();
            spawn_local(async move {
                // Get the correct client (EVM or Substrate)
                let client = chain_selector.get_client().await;
                
                // Send a message using the selected chain
                client.send_message("0xRecipientAddress".into(), "Hello, Blockchain!".into()).await.unwrap();
            });
        })
    };


    html! {
        <div>
            <h1>{ "Blockchain Messaging Platform" }</h1>

            <Wallet on_wallet_connected={on_wallet_connected.clone()} />

            <MessageInput on_message_sent={on_message_sent.clone()} />

            <Status status={(*status).clone()} />

            <MessageThread messages={(*messages).clone()} />

            <ChainSelectorComponent />

            <button onclick={on_message_send}>{ "Send Message" }</button>

        </div>
    }
}
