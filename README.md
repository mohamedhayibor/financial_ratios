# financial_ratios ![](https://travis-ci.org/mohamedhayibor/financial_ratios.svg?branch=master) [![crates.io](https://img.shields.io/crates/v/financial_ratios.svg)](https://crates.io/crates/financial_ratios)


This crate (Rust library) provides utility functions (metrics) to measure a firm's performance (company's health).

## Usage

Include this library with the following in your Cargo.toml:
```rust
[dependencies]
financial_ratios = "0.0.0" // preferably the latest version on crates.io
```
## Examples
```rust
extern crate financial_ratios;

use financial_ratios::{ earnings_per_share, return_on_assets };

fn main() {
    let test_1 = earnings_per_share(5000., 500000.);   // 0.01

    let test_2 = return_on_assets(2000., 5000.);       // 0.4
    // and so on with the other apis just feed what's expected
    // particularly the exact types
}

```

| API | arguments |
|-----|--------|
| current_ratio | (current_assets: f64, current_liabilities: f64) |
| quick_ratio | (current_assets: f64, inventories: f64, current_liabilities: f64) |
| cash_ratio | (cash: f64, current_liabilities: f64) |
| debt_ratio | (total_liabilities: f64, total_assets: f64) |
| times_interest_earned | (ebit: f64, interest_expense: f64) |
| cash_coverage_ratio | (ebit: f64, depreciation: f64, interest_expense: f64) |
| inventory_turnover | (cogs: f64, inventory: f64) |
| receivables_turnover | (sales: f64, accounts_receivable: f64) |
| total_asset_turnover | (sales: f64, total_assets: f64) |
| profit_margin | (net_income: f64, sales: f64) |
| return_on_assets | (net_income: f64, total_assets: f64) |
| return_on_equity | (net_income: f64, total_owners_equity: f64) |
| earnings_per_share | (net_income: f64, outstanding_shares: f64) |

Note that all output are of type `f64`, you are free to choose what degree of rounding is right for your project.

> Unless specified the inputs are expected to be floats. Your program will `panic` if you pass integers.

> For consistency sake `rate` is getting passed as a plain float and not as a percentage (%)

## Raison d'etre

Well save yourself some googling or fiddling wiki pages. All formulas have test cases. Feel free to send a PR if you feel like a missing formula should be included (preferably open an issue first :sunglasses:)

> Do not worry about the functions taking owernership of anything. As primitives support the copy trait.

## Maintenance

In the forseeable future, I plan to actively manage the repos, so I will follow the salesman motto "If you open it, I will close", meaning that I will fix issues in a speedy matter.

## License

This library is distributed with the GPLv2 software license.

```
    financial_ratios (rust library - crate)
    Copyright (C) 2016 Mohamed Hayibor

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
```
