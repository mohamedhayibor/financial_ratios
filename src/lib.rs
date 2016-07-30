// financial_ratios - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Copyright 2016 - Mohamed Hayibor

// liquidity ratios
pub fn current_ratio(current_assets: f64, current_liabilities: f64) -> f64 {
    current_assets / current_liabilities
}

#[test]
fn test_current_ratio() {
    let test = current_ratio(5000., 2000.);
    assert_eq!(test, 2.5);
}

pub fn quick_ratio(current_assets: f64, inventories: f64, current_liabilities: f64) -> f64 {
    (current_assets - inventories) / current_liabilities
}

#[test]
fn test_quick_ratio() {
    let test = quick_ratio(5000., 1000., 2000.);
    assert_eq!(test, 2.);
}

pub fn cash_ratio(cash: f64, current_liabilities: f64) -> f64 {
    cash / current_liabilities
}

#[test]
fn test_cash_ratio() {
    let test = cash_ratio(1000., 500.);
    assert_eq!(test, 2.);
}

// financial leverage ratios
pub fn debt_ratio(total_liabilities: f64, total_assets: f64) -> f64 {
    total_liabilities / total_assets
}

#[test]
fn test_debt_ratio() {
    let test = debt_ratio(1000., 5000.);
    assert_eq!(test, 0.2);
}

pub fn times_interest_earned(ebit: f64, interest_expense: f64) -> f64 {
    ebit / interest_expense
}

#[test]
fn test_times_interest_earned() {
   let test = times_interest_earned(3000., 500.);
   assert_eq!(test, 6.);
}

pub fn cash_coverage_ratio(ebit: f64, depreciation: f64, interest_expense: f64) -> f64 {
    (ebit + depreciation) / interest_expense
}

#[test]
fn test_cash_coverage() {
    let test = cash_coverage_ratio(3000., 2000., 500.);
    assert_eq!(test, 10.);
}

// asset management ratio
pub fn inventory_turnover(cogs: f64, inventory: f64) -> f64 {
    cogs / inventory
}

#[test]
fn test_inventory_turnover() {
    let test = inventory_turnover(2000., 4000.);
    assert_eq!(test, 0.5);
}

/// days maybe

pub fn receivables_turnover(sales: f64, accounts_receivable: f64) -> f64 {
    sales / accounts_receivable
}

#[test]
fn test_receivables_turnover() {
    let test = receivables_turnover(4000., 1000.);
    assert_eq!(test, 4.);
}

pub fn total_asset_turnover(sales: f64, total_assets: f64) -> f64 {
    sales / total_assets
}

#[test]
fn test_total_asset_turnover() {
    let test = total_asset_turnover(4000., 2000.);
    assert_eq!(test, 2.);
}

pub fn profit_margin(net_income: f64, sales: f64) -> f64 {
    net_income / sales
}

#[test]
fn test_profit_margin() {
    let test = profit_margin(2000., 4000.);
    assert_eq!(test, 0.5);
}

pub fn return_on_assets(net_income: f64, total_assets: f64) -> f64 {
    net_income / total_assets
}

#[test]
fn test_return_on_assets() {
   let test = return_on_assets(2000., 5000.);
   assert_eq!(test, 0.4);
}

pub fn return_on_equity(net_income: f64, total_owners_equity: f64) -> f64 {
    net_income / total_owners_equity
}

#[test]
fn test_return_on_equity() {
    let test = return_on_equity(2000., 5000.);
    assert_eq!(test, 0.4);
}

pub fn earnings_per_share(net_income: f64, outstanding_shares: f64) -> f64 {
    net_income / outstanding_shares
}

#[test]
fn test_eps() {
   let test = earnings_per_share(5000., 500000.);
   assert_eq!(test, 0.01);
}

