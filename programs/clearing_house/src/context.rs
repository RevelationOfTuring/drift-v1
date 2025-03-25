use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

use crate::state::{market::Markets, state::State};

#[derive(Accounts)]
pub struct Initialize<'info> {
    // 该signer会成为State中的admin
    #[account(mut)]
    pub admin: Signer<'info>,
    // 1. 创建pda，用于存储State
    #[account(
        init,
        payer = admin,
        space = 8 + size_of::<State>(),
        seeds = [b"clearing_house".as_ref()],
        bump
    )]
    pub state: Box<Account<'info, State>>,
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
