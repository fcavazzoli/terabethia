use async_trait::async_trait;
use ic_cdk::call;
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{Nonce, OutgoingMessage, TxError};

#[async_trait]
pub trait Tera {
    async fn consume_message(
        &self,
        erc20_addr_pid: Principal,
        nonce: Nonce,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError>;
    async fn send_message(
        &self,
        erc20_addr_pid: Principal,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError>;
}

#[async_trait]
impl Tera for Principal {
    async fn consume_message(
        &self,
        erc20_addr_pid: Principal,
        nonce: Nonce,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError> {
        let consume: (Result<bool, String>,) = match call(
            *self,
            "consume_message",
            (&erc20_addr_pid, &nonce, &payload),
        )
        .await
        {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match consume {
            (Ok(_),) => Ok(true),
            (Err(error),) => Err(TxError::Other(format!("Consume Message: {:?}", error))),
        }
    }

    async fn send_message(
        &self,
        erc20_addr_pid: Principal,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError> {
        let send: (Result<OutgoingMessage, String>,) =
            match call(*self, "consume_message", (&erc20_addr_pid, &payload)).await {
                Ok(res) => res,
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };

        match send {
            (Ok(_),) => Ok(true),
            (Err(error),) => Err(TxError::Other(format!("Send Message: {:?}", error))),
        }
    }
}
