use yew::prelude::*;
use crate::blockchain::chain_selector::{ChainSelector, ChainType};

#[function_component(ChainSelectorComponent)]
pub fn chain_selector() -> Html {
    let selected_chain = use_state(|| ChainType::EVM);

    let on_chain_change = {
        let selected_chain = selected_chain.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let chain_type = match input.value().as_str() {
                "evm" => ChainType::EVM,
                "substrate" => ChainType::Substrate,
                _ => ChainType::EVM,
            };
            selected_chain.set(chain_type);
        })
    };

    html! {
        <div>
            <label for="chain">Select Blockchain:</label>
            <select id="chain" onchange={on_chain_change}>
                <option value="evm">{ "EVM (Ethereum/BSC)" }</option>
                <option value="substrate">{ "Substrate (Polkadot)" }</option>
            </select>
        </div>
    }
}
