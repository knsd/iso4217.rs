[travis-badge]: https://img.shields.io/travis/taiyaeix/iso4217.rs.svg?style=flat-square
[travis]: https://travis-ci.org/taiyaeix/iso4217.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC

[![travis-badge][]][travis] [![license-badge][]][license]

# iso4217.rs

Rust crate for ISO 4217 data.

### ISO 4217

> ISO 4217 is a standard published by the International Organization for
> Standardization, which delineates currency designators, country codes
> (alpha and numeric), and references to minor units in three tables.
>
> -- [Wikipedia](http://en.wikipedia.org/wiki/ISO_4217)

### Installation

Add the following dependency to your Cargo.toml:

```toml
iso4217 = "^0.1"
```

And include it in your project:

```rust
extern crate iso4217;
```

### Examples

Retrieve all currencies defined by ISO 4217:

```rust
extern crate iso4217;

fn main() {
    let currencies = iso4217::all().unwrap();
}
```


Retrieve a currency by its alpha3 code:

```rust
extern crate iso4217;

fn main() {
    let currency = iso4217::alpha3("EUR").unwrap();
}
```


Retrieve a vector of currencies used by a country, given an alpha2 code
specified by ISO 3166:

```rust
extern crate iso4217;

fn main() {
    let currencies = iso4217::country("SG").unwrap();
}
```


Retrieve a vector of currencies with a certain exponential value
(decimal places):

```rust
extern crate iso4217;

fn main() {
    let currencies = iso4217::exp(2).unwrap();
}
```


Retrieve by the full name of the currency:

```rust
extern crate iso4217;

fn main() {
    let currency = iso4217::name("Turkish lira").unwrap();
}
```


Retrieve by the 3-digit numeric representation of the currency:

```rust
extern crate iso4217;

fn main() {
    let currency = iso4217::num("840").unwrap();
}
```

### License

License info in [LICENSE.md]. Long story short, ISC.

[LICENSE.md]: https://github.com/taiyaeix/iso4217.rs/blob/master/LICENSE.md
