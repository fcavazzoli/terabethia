use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::tera::Tera;
use crate::common::weth::Weth;
use crate::proxy::{ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_ETH};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{EthereumAddr, TokendId, TxError, TxReceipt};

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(token_id: TokendId, eth_addr: EthereumAddr, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let self_id = ic::id();

    if (token_id.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            token_id.to_string(),
        )));
    }

    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let transfer_from = token_id
        .transfer_from(caller, self_id, amount.clone())
        .await;

    match transfer_from {
        Ok(_) => {
            STATE.with(|s| s.add_balance(caller, token_id, amount.clone()));

            let burn = token_id.burn(amount.clone()).await;

            match burn {
                Ok(burn_txn_id) => {
                    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
                    let payload = [
                        token_id.clone().to_nat(),
                        eth_addr.clone().to_nat(),
                        amount.clone(),
                    ]
                    .to_vec();

                    if tera_id.send_message(eth_addr_pid, payload).await.is_err() {
                        return Err(TxError::Other(format!(
                            "Sending message to L1 failed with caller {:?}!",
                            caller.to_string()
                        )));
                    }

                    // there could be an underflow here
                    // like negative balance
                    let current_balance =
                        STATE.with(|s| s.get_balance(caller, token_id).unwrap_or(Nat::from(0)));

                    STATE.with(|s| {
                        s.update_balance(caller, token_id, current_balance - amount.clone())
                    });
                    return Ok(burn_txn_id);
                }
                Err(error) => {
                    return Err(error);
                }
            };
        }
        Err(error) => Err(error),
    }
}
