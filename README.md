financial_ratios ![](https://travis-ci.org/mohamedhayibor/financial_ratios.svg?branch=master) [![crates.io](https://img.shields.io/crates/v/financial_ratios.svg)](https://crates.io/crates/financial_ratios)


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
extern crate round;

use round::round;
use finance::{ present_value, future_value };

fn main() {
    let test_value = round(present_value(0.1, 1., 1000.), 2); // 909.09

    let test_value = round(future_value(0.1, 1., 1000.), 2);  // 1100.00
    // and so on with the other apis just feed what's expected
    // particularly the exact types
}

```

| API | arguments | abbr |
|-----|--------|--------:|


Note that all output are of type `f64`, you are free to choose what degree of rounding is right for your project.

> Unless specified the inputs are expected to be floats. Your program will `panic` if you pass integers.

Input accordingly if calculating in (monthly, semi-annual, daily) terms. In another words, make sure that your compounding_periods (daily, monthly, annual...) is used with corresponding rates (daily, monthly, annual).

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
