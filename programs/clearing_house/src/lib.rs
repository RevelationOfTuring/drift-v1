#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use context::*;
use errors::ErrorCode;
use math::constant::*;
use state::state::*;

pub mod context;
pub mod errors;
pub mod math;
pub mod state;

declare_id!("HPx7dWgMDvEKRf5S8uLVG2VxEqdKRhQ5Q8meCqEsecZz");

#[program]
pub mod clearing_house {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin_controls_prices: bool) -> Result<()> {
        // collateral_vault账户地址（pda，seeds为[b"collateral_vault"]）
        let collateral_vault_key = ctx.accounts.collateral_vault.to_account_info().key;
        // 生成pda地址（作为collateral_vault的authority）和对应bump
        // seeds为[collateral_account地址]
        let (collateral_vault_authority, collateral_vault_authority_bump) =
            Pubkey::find_program_address(&[collateral_vault_key.as_ref()], ctx.program_id);
        require_keys_eq!(
            ctx.accounts.collateral_vault_authority.key(),
            collateral_vault_authority,
            ErrorCode::InvalidCollateralVaultAuthority
        );

        // insurance_vault账户地址（pda，seeds为[b"insurance_vault"]）
        let insurance_vault_key = ctx.accounts.insurance_vault.to_account_info().key;
        // 生成pda地址（作为insurance_vault的authority）和对应bump
        // seeds为[insurance_vault账户地址]
        let (insurance_vault_authority, insurance_vault_authority_bump) =
            Pubkey::find_program_address(&[insurance_vault_key.as_ref()], ctx.program_id);
        require_keys_eq!(
            ctx.accounts.insurance_vault_authority.key(),
            insurance_vault_authority,
            ErrorCode::InvalidInsuranceVaultAuthority
        );

        ctx.accounts.markets.load_init()?;

        let default_pubkey = Pubkey::default();
        **ctx.accounts.state = State {
            admin: *ctx.accounts.admin.key,
            exchange_paused: false,
            funding_paused: false,
            admin_controls_prices,
            collateral_mint: ctx.accounts.collateral_mint.key(),
            collateral_vault: *collateral_vault_key,
            collateral_vault_authority,
            collateral_vault_authority_nonce: collateral_vault_authority_bump,
            // deposit_history/trade_history/funding_rate_history/funding_payment_history/liquidation_history/curve_history
            // 这六个history会被设置为Pubkey的默认值，进一步的设置会在initialize_history中做
            deposit_history: default_pubkey,
            trade_history: default_pubkey,
            funding_payment_history: default_pubkey,
            funding_rate_history: default_pubkey,
            liquidation_history: default_pubkey,
            curve_history: default_pubkey,
            insurance_vault: *insurance_vault_key,
            insurance_vault_authority,
            insurance_vault_authority_nonce: insurance_vault_authority_bump,
            markets: *ctx.accounts.markets.to_account_info().key,
            // 20%
            margin_ratio_initial: 2000,
            margin_ratio_maintenance: 625,
            margin_ratio_partial: 500,
            partial_liquidation_close_percentage_numerator: 25,
            partial_liquidation_close_percentage_denominator: 100,
            partial_liquidation_penalty_percentage_numberator: 25,
            partial_liquidation_penalty_percentage_denominator: 1000,
            full_liquidation_penalty_percentage_numerator: 1,
            full_liquidation_penalty_percentage_denominator: 1,
            partial_liquidation_liquidator_share_denominator: 2,
            full_liquidation_liquidator_share_denominator: 20,
            fee_structure: FeeStructure {
                fee_numerator: DEFAULT_FEE_NUMERATOR,
                fee_denominator: DEFAULT_FEE_DENOMINATOR,
                discount_token_tiers: DiscountTokenTiers {
                    first_tier: DiscountTokenTier {
                        minimun_balance: DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_MINIMUM_BALANCE,
                        discount_numerator: DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_NUMERATOR,
                        discount_denominator:
                            DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_DENOMINATOR,
                    },
                    second_tier: DiscountTokenTier {
                        minimun_balance: DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_MINIMUM_BALANCE,
                        discount_numerator: DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_NUMERATOR,
                        discount_denominator:
                            DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_DENOMINATOR,
                    },
                    third_tier: DiscountTokenTier {
                        minimun_balance: DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_MINIMUM_BALANCE,
                        discount_numerator: DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_NUMERATOR,
                        discount_denominator:
                            DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_DENOMINATOR,
                    },
                    fourth_tier: DiscountTokenTier {
                        minimun_balance: DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_MINIMUM_BALANCE,
                        discount_numerator: DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_NUMERATOR,
                        discount_denominator:
                            DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_DENOMINATOR,
                    },
                },
                referral_discount: ReferralDiscount {
                    referral_reward_numerator: DEFAULT_REFERRER_REWARD_NUMERATOR,
                    referral_reward_denominator: DEFAULT_REFERRER_REWARD_DENOMINATOR,
                    referee_discount_numerator: DEFAULT_REFEREE_DISCOUNT_NUMERATOR,
                    referee_discount_denominator: DEFAULT_REFEREE_DISCOUNT_DENOMINATOR,
                },
            },
            whitelist_mint: default_pubkey,
            discount_mint: default_pubkey,
            oracle_guard_rails: OracleGuardRails {
                price_divergence: PriceDivergenceGuardRails {
                    mark_oracle_divergence_numerator: 1,
                    mark_oracle_divergence_denominator: 10,
                },
                validity: ValidityGuardRails {
                    slots_before_stable: 1000,
                    confidence_interval_max_size: 4,
                    too_volatile_ratio: 5,
                },
                use_for_liquidations: true,
            },
            max_deposit: 0,
            extended_curve_history: default_pubkey,
            order_state: default_pubkey,
            padding: [0, 0, 0, 0],
        };

        Ok(())
    }
}
