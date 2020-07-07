use crate::address::Address;
use crate::transaction::Transaction;

use getset::Getters;
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

/// The balance change event data.
#[derive(Getters)]
#[getset(get = "pub")]
pub struct BalanceEvent {
  /// The associated account identifier.
  account_id: String,
  /// The associated address.
  address: Address,
  /// The new balance.
  balance: u64,
}

/// A transaction-related event data.
#[derive(Getters)]
#[getset(get = "pub")]
pub struct TransactionEvent {
  /// The associated account identifier.
  account_id: String,
  /// The event transaction.
  transaction: Transaction,
}

struct BalanceEventHandler {
  /// The on event callback.
  on_event: Box<dyn Fn(BalanceEvent) + Send>,
}

#[derive(PartialEq)]
pub(crate) enum TransactionEventType {
  NewTransaction,
  ConfirmationState,
  Reattachment,
  Broadcast,
}

struct TransactionEventHandler {
  event_type: TransactionEventType,
  /// The on event callback.
  on_event: Box<dyn Fn(TransactionEvent) + Send>,
}

type BalanceListeners = Arc<Mutex<Vec<BalanceEventHandler>>>;
type TransactionListeners = Arc<Mutex<Vec<TransactionEventHandler>>>;

/// Gets the balance change listeners array.
fn balance_listeners() -> &'static BalanceListeners {
  static LISTENERS: Lazy<BalanceListeners> = Lazy::new(Default::default);
  &LISTENERS
}

/// Gets the transaction listeners array.
fn transaction_listeners() -> &'static TransactionListeners {
  static LISTENERS: Lazy<TransactionListeners> = Lazy::new(Default::default);
  &LISTENERS
}

/// Listen to balance changes.
pub fn on_balance_change<F: Fn(BalanceEvent) + Send + 'static>(cb: F) {
  let mut l = balance_listeners()
    .lock()
    .expect("Failed to lock balance_listeners: on_balance_change()");
  l.push(BalanceEventHandler {
    on_event: Box::new(cb),
  })
}

/// Emits a balance change event.
pub(crate) fn emit_balance_change(account_id: impl Into<String>, address: Address, balance: u64) {
  let account_id = account_id.into();
  let listeners = balance_listeners()
    .lock()
    .expect("Failed to lock balance_listeners: emit_balance_change()");
  for listener in listeners.deref() {
    (listener.on_event)(BalanceEvent {
      account_id: account_id.clone(),
      address: address.clone(),
      balance,
    })
  }
}

/// Emits a transaction-related event.
pub(crate) fn emit_transaction_event(
  event_type: TransactionEventType,
  account_id: impl Into<String>,
  transaction: Transaction,
) {
  let account_id = account_id.into();
  let listeners = transaction_listeners()
    .lock()
    .expect("Failed to lock balance_listeners: emit_balance_change()");
  for listener in listeners.deref() {
    if listener.event_type == event_type {
      (listener.on_event)(TransactionEvent {
        account_id: account_id.clone(),
        transaction: transaction.clone(),
      })
    }
  }
}

/// Adds a transaction-related event listener.
fn add_transaction_listener<F: Fn(TransactionEvent) + Send + 'static>(
  event_type: TransactionEventType,
  cb: F,
) {
  let mut l = transaction_listeners()
    .lock()
    .expect("Failed to lock transaction_listeners: add_transaction_listener()");
  l.push(TransactionEventHandler {
    event_type,
    on_event: Box::new(cb),
  })
}

/// Listen to new transactions.
pub fn on_new_transaction<F: Fn(TransactionEvent) + Send + 'static>(cb: F) {
  add_transaction_listener(TransactionEventType::NewTransaction, cb);
}

/// Listen to transaction confirmation state change.
pub fn on_confirmation_state_change<F: Fn(TransactionEvent) + Send + 'static>(cb: F) {
  add_transaction_listener(TransactionEventType::ConfirmationState, cb);
}

/// Listen to transaction reattachment.
pub fn on_reattachment<F: Fn(TransactionEvent) + Send + 'static>(cb: F) {
  add_transaction_listener(TransactionEventType::Reattachment, cb);
}

/// Listen to transaction broadcast.
pub fn on_broadcast<F: Fn(TransactionEvent) + Send + 'static>(cb: F) {
  add_transaction_listener(TransactionEventType::Broadcast, cb);
}

/// Listen to errors.
pub fn on_error<F: Fn(anyhow::Error)>(cb: F) {}

#[cfg(test)]
mod tests {
  use super::{emit_balance_change, on_balance_change};
  use crate::address::AddressBuilder;
  use bee_transaction::bundled::Address;

  #[test]
  fn balance_events() {
    on_balance_change(|event| {
      assert!(event.account_id == "the account id");
      assert!(event.balance == 0);
    });

    emit_balance_change(
      "the account id",
      AddressBuilder::new()
        .address(Address::zeros())
        .balance(0)
        .key_index(0)
        .build()
        .expect("failed to build address"),
      0,
    );
  }
}
