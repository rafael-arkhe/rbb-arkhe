use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Saldo insuficiente")]
    InsufficientBalance,
    #[error("Canal fechado")]
    ChannelClosed,
    #[error("Ticket inválido")]
    InvalidTicket,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentChannel {
    pub channel_id: Digest32,
    pub sender: String,
    pub recipient: String,
    pub total_deposit: u64,
    pub settled_amount: u64,
    pub is_open: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentTicket {
    pub channel_id: Digest32,
    pub cumulative_amount: u64,
    pub inference_call_id: Digest32,
    pub sender_signature: Vec<u8>,
}

impl PaymentChannel {
    pub fn open(sender: &str, recipient: &str, deposit: u64) -> Self {
        let channel_id = Digest32::blake3(format!("{}:{}:{}", sender, recipient, deposit).as_bytes());
        Self {
            channel_id,
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            total_deposit: deposit,
            settled_amount: 0,
            is_open: true,
        }
    }

    pub fn process_ticket(&mut self, ticket: &PaymentTicket) -> Result<u64, PaymentError> {
        if !self.is_open {
            return Err(PaymentError::ChannelClosed);
        }
        if ticket.cumulative_amount > self.total_deposit {
            return Err(PaymentError::InsufficientBalance);
        }
        if ticket.cumulative_amount <= self.settled_amount {
            return Err(PaymentError::InvalidTicket);
        }
        let delta = ticket.cumulative_amount - self.settled_amount;
        self.settled_amount = ticket.cumulative_amount;
        Ok(delta)
    }
}
