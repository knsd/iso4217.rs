[travis-badge]: https://img.shields.io/travis/taiyaeix/iso4217.svg
[travis]: https://travis-ci.org/taiyaeix/iso4217.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg
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

```
iso4217 = "*"
```

And include it in your project:

```
extern crate iso4217;
```

### Examples

Retrieve all currencies defined by ISO 4217:

```
extern crate iso4217;

fn main() {
    let currencies = iso4217::all().unwrap();
}
```


Or retrieve a currency by its alpha3 code:

```
...
    let currency = iso4217::alpha3("EUR").unwrap();
```


Retrieve a vector of currencies used by a country, given an alpha2 code
specified by ISO 3166:

```
...
    let currencies = iso4217::country("SG").unwrap();
```


Retrieve a vector of currencies with a certain exponential value
(decimal places):

```
...
    let currencies = iso4217::exp(2).unwrap();
```


By the full name of the currency:

```
...
    let currency = iso4217::name("Turkish lira").unwrap();
```


By the 3-digit numeric representation of the currency:

```
...
    let currency = iso4217::num("840").unwrap();
```

### License

License info in `LICENSE.md`. Long story short, ISC.
