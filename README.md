# ethqr-gen

[![CI](https://github.com/dagregi/ethqr-gen/workflows/CI/badge.svg)](https://github.com/dagregi/ethqr-gen/actions)

A Rust library for generating EMVCo-compliant QR codes for payments according to the Ethiopian Interoperable QR Standard.

## Features

- EMVCo QR Code standard compliance
- Support for multiple payment schemes (Visa, Mastercard, IPS ET, etc.)
- Static and dynamic QR code generation
- Built-in validation and error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ethqr-gen = "0.1.0"
```

## Quick Start

### Static QR Code (no amount specified)

```rust
use ethqr_gen::{QRBuilder, SchemeConfig, fields::SchemeConfig};

let qr_code = QRBuilder::new()
    .merchant_name("Coffee Shop")
    .merchant_city("Addis Ababa")
    .merchant_category_code("5812") // Restaurant
    .add_scheme(SchemeConfig::visa("4111111111111111"))
    .build()?;
```

### Dynamic QR Code (with specific amount)

```rust
use ethqr_gen::{QRBuilder, SchemeConfig, AdditionalData};

let additional_data = AdditionalData::new()
    .bill_number("INV-001")
    .reference_label("ORDER-123");

let qr_code = QRBuilder::new()
    .merchant_name("Restaurant")
    .merchant_city("Dire Dawa")
    .merchant_category_code("5812")
    .add_scheme(SchemeConfig::ips_et(
        "581b314e257f41bfbbdc6384daa31d16",
        "CBETETAA",
        "10000171234567890",
    ))
    .transaction_amount("420.00")
    .additional_data(additional_data)
    .build()?;
```

### Multiple Payment Schemes

```rust
let qr_code = QRBuilder::new()
    .merchant_name("Supermarket")
    .merchant_city("Bahir Dar")
    .merchant_category_code("5411") // Grocery store
    .add_scheme(SchemeConfig::visa("4111111111111111"))
    .add_scheme(SchemeConfig::mastercard("5555555555554444"))
    .add_scheme(SchemeConfig::ips_et(
        "899c314e257f41bfbbdc6384dab49e15",
        "CBETETAA",
        "10000171234567890",
    ))
    .build()?;
```

## Supported Payment Schemes

| Scheme     | Method                       | Description                        |
| ---------- | ---------------------------- | ---------------------------------- |
| Visa       | `SchemeConfig::visa()`       | Visa payment cards                 |
| Mastercard | `SchemeConfig::mastercard()` | Mastercard payment cards           |
| UnionPay   | `SchemeConfig::unionpay()`   | UnionPay payment cards             |
| IPS ET     | `SchemeConfig::ips_et()`     | Ethiopian Interbank Payment System |

## Standards Compliance

This library implements:

- EMVCo QR Code Specification for Payment Systems
- Ethiopian Interoperable QR Standard
- ISO 4217 currency codes (ETB - Ethiopian Birr)
- ISO 3166-1 country codes (ET - Ethiopia)

## Examples

See the `examples/` directory for more usage examples

## Contributing

Contributions are always welcome!

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
