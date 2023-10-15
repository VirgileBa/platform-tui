//! Application screens module.

mod contract;
mod get_contract;
mod get_identity;
mod identity;
mod main;
mod wallet;
mod add_wallet;

pub(crate) use contract::ContractScreen;
pub(crate) use contract::ContractScreenCommands;
pub(crate) use get_contract::GetContractScreen;
pub(crate) use get_contract::GetContractScreenCommands;
pub(crate) use get_contract::ContractIdInput;
pub(crate) use get_identity::GetIdentityScreen;
pub(crate) use get_identity::GetIdentityScreenCommands;
pub(crate) use get_identity::IdentityIdInput;
pub(crate) use identity::IdentityScreen;
pub(crate) use identity::IdentityScreenCommands;
pub(crate) use main::MainScreen;
pub(crate) use main::MainScreenCommands;
pub(crate) use wallet::WalletScreen;
pub(crate) use wallet::WalletScreenCommands;
pub(crate) use add_wallet::AddWalletScreen;
pub(crate) use add_wallet::AddWalletScreenCommands;
