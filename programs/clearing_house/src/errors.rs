use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Clearing house not collateral vault owner")]
    InvalidCollateralVaultAuthority,
    #[msg("Clearing house not insurance vault owner")]
    InvalidInsuranceVaultAuthority,
}
