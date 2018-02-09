// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zeylahellyer@gmail.com>
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

extern crate iso4217;

use iso4217::{CurrencyCode, all, alpha3, country, exp, name, num};

#[test]
fn get_all() {
    assert!(!all().is_empty());
}

#[test]
fn get_by_alpha3() {
    assert!(alpha3("ALL").is_some());
}

#[test]
fn get_by_country() {
    assert!(!country("AL").is_empty());
}

#[test]
fn get_by_exp() {
    assert!(!exp(2).is_empty());
}

#[test]
fn get_by_name() {
    assert!(name("Albanian lek").is_some());
}

#[test]
fn get_by_num() {
    assert!(num("008").is_some());
}

#[test]
fn backwards_compats() {
    // Test the number of total CurrencyCodes.
    assert!(all().len() == 155);
}

// Test that one or multiple country alpha2 codes can be in a CurrencyCode's
// 'countries' field.
#[test]
fn countrycode_countries() {
    let currency_multiple: CurrencyCode = alpha3("AUD").unwrap();
    let currency_single: CurrencyCode = alpha3("ALL").unwrap();

    assert!(currency_multiple.countries.len() > 1);
    assert!(currency_single.countries.len() == 1);
}
