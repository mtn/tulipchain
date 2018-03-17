use super::Tulips;

pub struct Transaction {
    sender_addr: String,
    recipient_addr: String,

    sender_key: String,
    value: Tulips,
}
