use crate::treasury::dapp_base::error::BaseDAppError;
use cosmwasm_std::Response;

/// Postfix for LP pair addresses.
pub const PAIR_POSTFIX: &str = "_pair";

pub type BaseDAppResult = Result<Response, BaseDAppError>;
