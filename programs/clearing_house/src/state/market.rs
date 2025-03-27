use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};
use static_assertions::const_assert_eq;
use std::mem::size_of;

#[account(zero_copy)]
// markets账户里面存有最多64个Market的信息
pub struct Markets {
    pub markets: [Market; 64],
}

const_assert_eq!(size_of::<Markets>(), 31744);

#[zero_copy]
pub struct Market {
    pub base_asset_amount_long: i128, // 多头头寸的基础资产数量（正数表示）
    pub base_asset_amount_short: i128, // 空头头寸的基础资产数量（负数表示）
    pub base_asset_amount: i128, // 净市场偏差，即多头和空头头寸的净额（base_asset_amount_long + base_asset_amount_short）
    pub open_interest: u128,     // 持仓用户数量，表示当前有多少用户在市场中持有头寸
    pub amm: AMM,
    pub margin_ratio_initial: u32, // 初始保证金比例（开仓时要求的保证金比例）
    pub margin_ratio_partial: u32, // 部分清算保证金比例（当保证金低于此比例时可能触发部分清算）
    pub margin_ratio_maintenance: u32, // 维持保证金比例（当保证金低于此比例时可能触发强制清算）
    // 该Market是否完成初始化标志
    pub initialized: u8,
    // upgrade-ability
    pub padding0: [u8; 3],
    pub padding1: u128,
    pub padding2: u128,
    pub padding3: u128,
    pub padding4: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(u8)]
pub enum OracleSource {
    Pyth,
    SwitchBoard,
}

unsafe impl Zeroable for OracleSource {}
unsafe impl Pod for OracleSource {}

#[zero_copy]
pub struct AMM {
    pub base_asset_reserve: u128,  // base资产储备量
    pub quote_asset_reserve: u128, // quote资产储备量
    pub sqrt_k: u128,              // 恒定乘积公式中的K值(√K)，用于x*y=k类AMM
    // 资金费率与重新锚定
    pub cumulative_repeg_rebate_long: u128, // 多头累计重新锚定返利
    pub cumulative_repeg_rebate_short: u128, // 空头累计重新锚定返利
    pub cumulative_funding_rate_long: u128, // 多头累计资金费率
    pub cumulative_funding_rate_short: u128, // 空头累计资金费率
    pub last_funding_rate: i128,            // 最近的资金费率
    pub last_funding_rate_ts: i64,          // 最近更新资金费率的时间戳
    pub funding_period: i64,                // 资金费率计算周期
    pub peg_multiplier: u128,               // 锚定乘数，用于调整AMM价格与目标价格的偏差
    // fee相关
    pub total_fee: u128,                     // 累计总费用
    pub total_fee_minus_distributions: u128, // 总费用减去分配部分
    pub total_fee_withdrawn: u128,           // 已提取的总费用
    // 交易参数
    pub minimum_base_asset_trade_size: u128, // base资产最小交易量
    pub mininum_quote_asset_trade_size: u128, // quote资产最小交易量
    // 标记价格
    pub last_mark_price_twap: u128,   // 最近的标记价格的时间加权平均
    pub last_mark_price_twap_ts: i64, // 最近更新标记价格TWAP的时间戳
    // 预言机
    pub last_oracle_price_twap_ts: i64, // 最近更新预言机TWAP的时间戳
    pub last_oracle_price_twap: i128,   // 最近的预言机价格的时间加权平均
    pub oracle: Pubkey,                 // oracle地址
    pub last_oracle_price: i128,        // 最新的预言机价格
    pub base_spread: u16,               // 基础点差(以基点表示)
    pub oracle_source: OracleSource,    // 预言机类型

    pub padding: [u8; 13],
}

// #[test]
// fn test_a() {
//     use std::mem;
//     println!("Amm size: {:?}", mem::size_of::<Markets>());
// }
