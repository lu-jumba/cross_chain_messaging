use crate::blockchain::eth::{verify_message_on_chain, Provider};
use ethers::types::Address;

#[derive(Properties, PartialEq, Clone)]
pub struct MessageThreadProps {
    pub messages: Vec<(String, String, String, Address)>,  // Tuple (message, IPFS hash, signature, sender address)
    pub provider: Provider<Http>,  // Blockchain provider
    pub contract_address: Address,  // Contract address for checking on-chain storage
}

#[function_component(MessageThread)]
pub fn message_thread(props: &MessageThreadProps) -> Html {
    let status = use_state(|| "".to_string());

    let on_verify_ipfs_hash = {
        let status = status.clone();
        let provider = props.provider.clone();
        let contract_address = props.contract_address.clone();

        Callback::from(move |(ipfs_hash, sender_address): (String, Address)| {
            let status = status.clone();
            let provider = provider.clone();
            let contract_address = contract_address.clone();

            spawn_local(async move {
                // Verify the IPFS hash on-chain
                let valid = verify_message_on_chain(
                    provider,
                    contract_address,
                    sender_address,
                    ipfs_hash.clone()
                ).await.unwrap();

                // Update the status based on the result
                if valid {
                    status.set("IPFS hash verified on blockchain!".into());
                } else {
                    status.set("IPFS hash not found on blockchain!".into());
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Message Thread" }</h2>
            <ul>
                {
                    for props.messages.iter().map(|(message, hash, signature, sender_address)| {
                        let hash_clone = hash.clone();
                        let sender_address_clone = *sender_address;

                        html! {
                            <li>
                                <p>{ format!("Message: {}", message) }</p>
                                <p>{ format!("IPFS Hash: {}", hash) }</p>
                                <p>{ format!("Signature: {}", signature) }</p>
                                <p>{ format!("Sender: {}", sender_address) }</p>
                                <button onclick={on_verify_ipfs_hash.reform(move |_| (hash_clone.clone(), sender_address_clone))}>{ "Verify IPFS Hash on Blockchain" }</button>
                            </li>
                        }
                    })
                }
            </ul>
            <p>{ (*status).clone() }</p>
        </div>
    }
}
