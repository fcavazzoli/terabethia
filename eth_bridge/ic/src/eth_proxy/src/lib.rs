use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal};
use std::{cell::RefCell, collections::HashMap, str::FromStr};

const WETH_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const WETH_ADDRESS_ETH: &str = "0x2e130e57021bb4dfb95eb4dd0dd8cfceb936148a";

pub type Nonce = Nat;

pub type EthereumAddr = Principal;

pub type TokendId = Principal;

pub type MsgHashKey = [u8; 32];

pub type TxReceipt = Result<Nat, TxError>;

#[derive(CandidType, Deserialize, Clone, Debug)]

pub struct ClaimableMessage {
    pub owner: EthereumAddr,
    pub msg_hash: MsgHashKey,
    pub token: TokendId,
    pub amount: Nat,
}

thread_local! {
    pub static STATE: ProxyState = ProxyState::default();
}
#[derive(CandidType, Deserialize, Default)]
pub struct ProxyState {
    pub controllers: RefCell<Vec<Principal>>,
    pub messages_unclaimed: RefCell<HashMap<EthereumAddr, Vec<ClaimableMessage>>>,
}
#[derive(CandidType, Deserialize, Default)]

pub struct StableProxyState {
    pub controllers: Vec<Principal>,
    pub messages_unclaimed: HashMap<EthereumAddr, Vec<ClaimableMessage>>,
}

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other(String),
}

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "Eth Contract Address is inccorrect: {}",
            eth_addr_hex
        )));
    }

    mint(nonce, payload).await
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    let mint: (TxReceipt,) = match ic::call(weth_ic_addr_pid, "mint", (&nonce, &payload)).await {
        Ok(res) => res,
        Err((code, err)) => {
            return Err(TxError::Other(format!(
                "RejectionCode: {:?}\n{}",
                code, err
            )))
        }
    };

    match mint {
        (Ok(tx_id),) => Ok(tx_id),
        (Err(error),) => Err(error),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> TxReceipt {
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    let burn_txn: (TxReceipt,) =
        match ic::call(weth_ic_addr_pid, "burn", (&eth_addr, &amount)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

    match burn_txn {
        (Ok(msg_key),) => Ok(msg_key),
        (Err(error),) => Err(error),
    }
}

impl ProxyState {
    pub fn _authorize(&self, other: Principal) {
        let caller = ic::caller();
        let caller_autorized = self.controllers.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.controllers.borrow_mut().push(other);
        }
    }

    pub fn _is_authorized(&self) -> Result<(), String> {
        self.controllers
            .borrow()
            .contains(&ic::caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    }

    pub fn add_claimable_message(&self, message: ClaimableMessage) {
        let mut map = self.messages_unclaimed.borrow_mut();
        let messages = map.entry(message.owner.clone()).or_insert_with(Vec::new);

        messages.push(message.clone());
        return;
    }

    pub fn remove_claimable_message(
        &self,
        eth_address: EthereumAddr,
        msg_hash: MsgHashKey,
    ) -> Result<(), String> {
        let mut map = self.messages_unclaimed.borrow_mut();
        let messages = map
            .get_mut(&eth_address)
            .ok_or_else(|| "Message not found")?;

        messages.retain(|m| m.msg_hash != msg_hash);

        return Ok(());
    }

    pub fn get_claimable_messages(&self, eth_address: EthereumAddr) -> Vec<ClaimableMessage> {
        let unclaimed_messages = self
            .messages_unclaimed
            .borrow()
            .get(&eth_address)
            .unwrap_or(&vec![])
            .clone();
        return unclaimed_messages;
    }

    pub fn take_all(&self) -> StableProxyState {
        StableProxyState {
            controllers: self.controllers.take(),
            messages_unclaimed: self.messages_unclaimed.take(),
        }
    }

    pub fn clear_all(&self) {
        self.controllers.borrow_mut().clear();
        self.messages_unclaimed.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_message_state: StableProxyState) {
        self.controllers.replace(stable_message_state.controllers);
        self.messages_unclaimed
            .replace(stable_message_state.messages_unclaimed);
    }
}

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
candid::export_service!();
