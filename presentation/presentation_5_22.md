# Making a toy blockchain!

---

## Motivation

- Verified transactions without trusted authority
- It's a linked list!

![](linked_list.png)

---

## Definitions

- Addresses
- Transactions
- Miners

---

## Proof of work

- Nonce

"Hello, world!0" => 1312af178c253f84028d480a6adc1e25e81caa44c749ec81976192e2ec934c64
"Hello, world!1" => e9afc424b79e4f6ab42d99c81156d3a17228d6e1eef4139be78e948a9332a7d8
"Hello, world!2" => ae37343a357a8297591625e7134cbea22f5928be8ca2a32aa475cf05fd4266b7
...
"Hello, world!4248" => 6e110d98b388e77e9c6f042ac6b497cec46660deef75a55ebc7cfdf65cc0b965
"Hello, world!4249" => c004190b822f1669cac8dc37e761cb73652e7832fb814565702245cf26ebb9e6
"Hello, world!4250" => 0000c3af42fc31103f1fdc0151fa747ff87349a4714df7cc52ea464e12dcd4e9

_Key idea_: Digest of previous transactions is part of "Hello, world!"

Doesn't depend on what's about to be added

---

## Transactions

Alice: "I pay Bob 5 tulips" _Alice signs note_

*everyone check that Alice has 5 tulips*

---

## Addresses? Wallets?

Key pairs

---

## Blocks

Groups of transactions. And then the blockchain is a group of these blocks.

---

## The network

Every node knows every transaction -- everyone can run their own instance

They gossip!

---

## Consensus

Longest wins

---

## tulipchain

No demo :(

github.com/mtn/tulipchain


