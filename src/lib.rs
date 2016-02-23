// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is ISO 4217?
//
// | ISO 4217 is a standard published by the International Organization for
// | Standardization, which delineates currency designators, country codes
// | (alpha and numeric), and references to minor units in three tables.
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_4217)
//
// Originally by taiyaeix on GitHub.

mod codes;

use codes::currency_codes;

/// Struct that contains the data for each Currency Code defined by ISO 4217,
/// including the following pieces of information:
///
/// - `alpha3` - str of a 3-letter code of the currency.
/// - `countries` - vector of alpha2 codes for countries that use the currency.
/// - `exp` - i8 of the decimal places of the currency.
/// - `name` - str of the fully readable and used name of the currency.
/// - `num` - str of the 3-digit numeric code assigned to the currency.
///
/// This struct is public and derives from Clone and Debug.
#[derive(Clone, Debug)]
pub struct CurrencyCode<'a> {
    pub alpha3: &'a str,
    pub countries: Vec<&'a str>,
    pub exp: i8,
    pub name: &'a str,
    pub num: &'a str,
}

/// Returns an `Option` of a `Vec` of `CurrencyCode`s defined by ISO 4217.
///
/// # Examples
///
/// An example of a valid use case:
///
/// ```
/// let currencies = iso4217::all().unwrap();
/// ```
pub fn all<'a>() -> Option<Vec<CurrencyCode<'a>>> {
    let codes: Vec<CurrencyCode> = currency_codes();

    if codes.len() > 0 {
        Some(codes)
    } else {
        None
    }
}

/// Returns an `Option` of the `CurrencyCode` with the given alpha3 code.
///
/// # Examples
///
/// An example of a valid use case:
///
/// ```
/// let currency = iso4217::alpha3("ALL").unwrap();
/// ```
pub fn alpha3<'a>(alpha3: &str) -> Option<CurrencyCode<'a>> {
    let mut code_ret: Option<CurrencyCode> = None;

    for code in currency_codes() {
        if code.alpha3 == alpha3 {
            code_ret = Some(code.clone());

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of a `Vec` of `CurrencyCode`s that are used by the given
/// country's `alpha2` code, specified by ISO 3166.
///
/// # Examples
///
/// An example of a valid use case:
///
/// ```
/// let currencies = iso4217::country("AL").unwrap();
/// ```
pub fn country<'a>(country: &str) -> Option<Vec<CurrencyCode<'a>>> {
    let mut codes_ret: Vec<CurrencyCode> = vec![];

    for code in currency_codes() {
        if code.countries.contains(&country) {
            codes_ret.push(code.clone());
        }
    }

    if codes_ret.len() > 0 {
        Some(codes_ret)
    } else {
        None
    }
}

/// Returns an `Option` of a `Vec` of `CurrencyCode`s with the specified decimal
/// place.
///
/// # Examples
///
/// An example of a valid use case:
///
/// ```
/// let currencies = iso4217::exp(2).unwrap();
/// ```
pub fn exp<'a>(exp: i8) -> Option<Vec<CurrencyCode<'a>>> {
    let mut codes_ret: Vec<CurrencyCode> = vec![];

    for code in currency_codes() {
        if code.exp == exp {
            codes_ret.push(code.clone());
        }
    }

    if codes_ret.len() > 0 {
        Some(codes_ret)
    } else {
        None
    }
}

/// Returns an `Option` of the `CurrencyCode` with the specified `name`.
///
/// # Examples
///
/// An example of a valid name:
///
/// ```
/// let currency = iso4217::name("Albanian lek").unwrap();
/// ```
pub fn name<'a>(name: &str) -> Option<CurrencyCode<'a>> {
    let mut code_ret: Option<CurrencyCode> = None;

    for code in currency_codes() {
        if code.name == name {
            code_ret = Some(code);

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of the `CurrencyCode` with the specified numerical
/// 3-digit representation.
///
/// # Examples
///
/// An example of a valid num:
///
/// ```
/// let currency = iso4217::num("008").unwrap();
/// ```
pub fn num<'a>(num: &str) -> Option<CurrencyCode<'a>> {
    let mut code_ret: Option<CurrencyCode> = None;

    for code in currency_codes() {
        if code.num == num {
            code_ret = Some(code);

            break;
        }
    }

    code_ret
}
