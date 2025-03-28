use crate::state::{
    history::{
        curve_history::CurveHistory, deposit_history::DepositHistory,
        funding_payment_history::FundingPaymentHistory, funding_rate_history::FundingRateHistory,
        liquidation_history::LiquidationHistory, trade_history::TradeHistory,
    },
    market::Markets,
    state::State,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    // 该signer会成为State中的admin
    #[account(mut)]
    pub admin: Signer<'info>,
    // 1. 创建pda，用于存储State
    #[account(zero)]
    pub state: AccountLoader<'info, State>,
    // 2. 抵押品mint
    pub collateral_mint: Box<Account<'info, Mint>>,
    // 3. 创建抵押品vault（即owner为本program的一个ata，token种类为collateral_mint，authority为collateral_vault_authority）
    #[account(
        init,
        payer = admin,
        seeds = [b"collateral_vault".as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = collateral_vault_authority
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,
    // 4. 抵押品vault的authority
    /// CHECK: checked in `initialize`
    pub collateral_vault_authority: UncheckedAccount<'info>,
    // 5. 创建保证金Vault（即owner为本program的一个ata，token种类为collateral_mint，authority为insurance_vault_authority）
    #[account(
        init,
        payer = admin,
        seeds = [b"insurance_vault".as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = insurance_vault_authority
    )]
    pub insurance_vault: Box<Account<'info, TokenAccount>>,
    // 6. 保证金vault的authority
    /// CHECK: checked in `initialize`
    pub insurance_vault_authority: UncheckedAccount<'info>,
    // 7. markets账户
    #[account(zero)]
    pub markets: AccountLoader<'info, Markets>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeHistory<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        has_one = admin
    )]
    pub state: AccountLoader<'info, State>,
    #[account(zero)]
    pub funding_payment_history: AccountLoader<'info, FundingPaymentHistory>,
    #[account(zero)]
    pub trade_history: AccountLoader<'info, TradeHistory>,
    #[account(zero)]
    pub liquidation_history: AccountLoader<'info, LiquidationHistory>,
    #[account(zero)]
    pub deposit_history: AccountLoader<'info, DepositHistory>,
    #[account(zero)]
    pub funding_rate_history: AccountLoader<'info, FundingRateHistory>,
    #[account(zero)]
    pub curve_history: AccountLoader<'info, CurveHistory>,
}
