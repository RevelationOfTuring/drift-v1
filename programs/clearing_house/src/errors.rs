use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Clearing house not collateral vault owner")]
    InvalidCollateralVaultAuthority,
    #[msg("Clearing house not insurance vault owner")]
    InvalidInsuranceVaultAuthority,
    #[msg("Clearing house histories already initialized")]
    HistoriesAllInitialized,
    #[msg("Clearing house order state already initialized")]
    OrderStateAlreadyInitialized,
}
