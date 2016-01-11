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

use CurrencyCode;

// A function that returns a really big Vector of all the currency codes
// designated by ISO 4217, with some exceptions:
//
// - BOV Bolivian Mvdol (funds code)
// - CHE WIR Euro (complementary currency)
// - CHW WIR Franc (complementary currency)
// - CLF Unidad de Fomento (funds code)
// - CNH Chinese yuan when traded in Hong Kong
// - COU Unidad de Valor Real (UVR) (funds code)
// - MXV Mexican Unidad de Inversion (UDI) (funds code)
// - USN United States dollar (next day) (funds code)
// - USS United States dollar (same day) (funds code)
// - UYI Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)
// - XAG Silver (one troy ounce)
// - XAU Gold (one troy ounce)
// - XBA European Composite Unit (EURCO) (bond market unit)
// - XBB European Monetary Unit (E.M.U.-6) (bond market unit)
// - XBC European Unit of Account 9 (E.U.A.-9) (bond market unit)
// - XBD European Unit of Account 17 (E.U.A.-17) (bond market unit)
// - XDR Special drawing rights
// - XFU UIC franc (special settlement currency)
// - XPD Palladium (one troy ounce)
// - XPT Platinum (one troy ounce)
// - XSU Unified System for Regional Compensation (SUCRE)
// - XTS Code reserved for testing purposes
// - XUA ADB Unit of Account (African Development Bank)
// - XXX No currency
// - ZWD Zimbabwe dollar
//
// Country alpha2-codes are taken from the ISO's website:
// https://www.iso.org/obp/ui/
pub fn currency_codes<'a>() -> Vec<CurrencyCode<'a>> {
    let mut codes: Vec<CurrencyCode> = vec![];

    // Sorted by num.
    codes.push(CurrencyCode {
        alpha3: "ALL",
        countries: vec!["AL"],
        exp: 2,
        name: "Albanian lek",
        num: "008",
    });
    codes.push(CurrencyCode {
        alpha3: "DZD",
        countries: vec!["DZ"],
        exp: 2,
        name: "Algerian dinar",
        num: "012",
    });
    codes.push(CurrencyCode {
        alpha3: "ARS",
        countries: vec!["AR"],
        exp: 2,
        name: "Argentine peso",
        num: "032",
    });
    codes.push(CurrencyCode {
        alpha3: "AUD",
        countries: vec![
            "AU",
            "CC",
            "CX",
            "HM",
            "KI",
            "NF",
            "NR",
            "TV",
        ],
        exp: 2,
        name: "Australian dollar",
        num: "036",
    });
    codes.push(CurrencyCode {
        alpha3: "BSD",
        countries: vec!["BS"],
        exp: 2,
        name: "Bahamian dollar",
        num: "044",
    });
    codes.push(CurrencyCode {
        alpha3: "BHD",
        countries: vec!["BH"],
        exp: 2,
        name: "Bahraini dinar",
        num: "048",
    });
    codes.push(CurrencyCode {
        alpha3: "BDT",
        countries: vec!["BD"],
        exp: 2,
        name: "Bangladeshi taka",
        num: "050",
    });
    codes.push(CurrencyCode {
        alpha3: "AMD",
        countries: vec!["AM"],
        exp: 2,
        name: "Armenian dram",
        num: "051",
    });
    codes.push(CurrencyCode {
        alpha3: "BBD",
        countries: vec!["BB"],
        exp: 2,
        name: "Barbados dollar",
        num: "052",
    });
    codes.push(CurrencyCode {
        alpha3: "BMD",
        countries: vec!["BM"],
        exp: 2,
        name: "Bermudian dollar",
        num: "060",
    });
    codes.push(CurrencyCode {
        alpha3: "BTN",
        countries: vec!["BT"],
        exp: 2,
        name: "Bhutanese ngultrum",
        num: "064",
    });
    codes.push(CurrencyCode {
        alpha3: "BOB",
        countries: vec!["BO"],
        exp: 2,
        name: "Boliviano",
        num: "068",
    });
    codes.push(CurrencyCode {
        alpha3: "BWP",
        countries: vec!["BW"],
        exp: 2,
        name: "Botswana pula",
        num: "072",
    });
    codes.push(CurrencyCode {
        alpha3: "BZD",
        countries: vec!["BZ"],
        exp: 2,
        name: "Belize dollar",
        num: "084",
    });
    codes.push(CurrencyCode {
        alpha3: "SBD",
        countries: vec!["SB"],
        exp: 2,
        name: "Soloman Islands dollar",
        num: "090",
    });
    codes.push(CurrencyCode {
        alpha3: "BND",
        countries: vec![
            "BN",
            "SG",
        ],
        exp: 2,
        name: "Brunei dollar",
        num: "096",
    });
    codes.push(CurrencyCode {
        alpha3: "MMK",
        countries: vec!["MM"],
        exp: 2,
        name: "Myanmar kyat",
        num: "104",
    });
    codes.push(CurrencyCode {
        alpha3: "BIF",
        countries: vec!["BI"],
        exp: 0,
        name: "Burundian franc",
        num: "108",
    });
    codes.push(CurrencyCode {
        alpha3: "KHR",
        countries: vec!["KH"],
        exp: 2,
        name: "Cambodian riel",
        num: "116",
    });
    codes.push(CurrencyCode {
        alpha3: "CAD",
        countries: vec!["CA"],
        exp: 2,
        name: "Canadian dollar",
        num: "124",
    });
    codes.push(CurrencyCode {
        alpha3: "CVE",
        countries: vec!["CV"],
        exp: 0,
        name: "Cape Verde escudo",
        num: "132",
    });
    codes.push(CurrencyCode {
        alpha3: "KYD",
        countries: vec!["KY"],
        exp: 2,
        name: "Cayman Islands dollar",
        num: "136",
    });
    codes.push(CurrencyCode {
        alpha3: "LKR",
        countries: vec!["LK"],
        exp: 2,
        name: "Sri Lankan rupee",
        num: "144",
    });
    codes.push(CurrencyCode {
        alpha3: "CLP",
        countries: vec!["CL"],
        exp: 0,
        name: "Chilean peso",
        num: "152",
    });
    codes.push(CurrencyCode {
        alpha3: "CNY",
        countries: vec!["CN"],
        exp: 2,
        name: "Chinese yuan",
        num: "156",
    });
    codes.push(CurrencyCode {
        alpha3: "COP",
        countries: vec!["CO"],
        exp: 2,
        name: "Colombian peso",
        num: "170",
    });
    codes.push(CurrencyCode {
        alpha3: "KMF",
        countries: vec!["KM"],
        exp: 0,
        name: "Comoro franc",
        num: "174",
    });
    codes.push(CurrencyCode {
        alpha3: "CRC",
        countries: vec!["CR"],
        exp: 2,
        name: "Costa Rican colon",
        num: "188",
    });
    codes.push(CurrencyCode {
        alpha3: "HRK",
        countries: vec!["HR"],
        exp: 2,
        name: "Croatian kuna",
        num: "191",
    });
    codes.push(CurrencyCode {
        alpha3: "CUP",
        countries: vec!["CU"],
        exp: 2,
        name: "Cuban peso",
        num: "192",
    });
    codes.push(CurrencyCode {
        alpha3: "CZK",
        countries: vec!["CZ"],
        exp: 2,
        name: "Czech koruna",
        num: "203",
    });
    codes.push(CurrencyCode {
        alpha3: "DKK",
        countries: vec![
            "DK",
            "FO",
            "GL",
        ],
        exp: 2,
        name: "Danish krone",
        num: "208",
    });
    codes.push(CurrencyCode {
        alpha3: "DOP",
        countries: vec!["DO"],
        exp: 2,
        name: "Dominican peso",
        num: "214",
    });
    codes.push(CurrencyCode {
        alpha3: "ETB",
        countries: vec!["ET"],
        exp: 2,
        name: "Ethiopian birr",
        num: "230",
    });
    codes.push(CurrencyCode {
        alpha3: "ERN",
        countries: vec!["ER"],
        exp: 2,
        name: "Eritrean nakfa",
        num: "232",
    });
    codes.push(CurrencyCode {
        alpha3: "FKP",
        countries: vec!["FK"],
        exp: 2,
        name: "Falkland Islands pound",
        num: "238",
    });
    codes.push(CurrencyCode {
        alpha3: "FJD",
        countries: vec!["FJ"],
        exp: 2,
        name: "Fiji dollar",
        num: "242",
    });
    codes.push(CurrencyCode {
        alpha3: "DJF",
        countries: vec!["DJ"],
        exp: 0,
        name: "Djiboutian franc",
        num: "262",
    });
    codes.push(CurrencyCode {
        alpha3: "GMD",
        countries: vec!["GM"],
        exp: 2,
        name: "Gambian dalasi",
        num: "270",
    });
    codes.push(CurrencyCode {
        alpha3: "GIP",
        countries: vec!["GI"],
        exp: 2,
        name: "Gibraltar pound",
        num: "292",
    });
    codes.push(CurrencyCode {
        alpha3: "GTQ",
        countries: vec!["GT"],
        exp: 2,
        name: "Guatemalan quetzal",
        num: "320",
    });
    codes.push(CurrencyCode {
        alpha3: "GNF",
        countries: vec!["GN"],
        exp: 0,
        name: "Guinean franc",
        num: "324",
    });
    codes.push(CurrencyCode {
        alpha3: "GYD",
        countries: vec!["GY"],
        exp: 2,
        name: "Guyanese dollar",
        num: "328",
    });
    codes.push(CurrencyCode {
        alpha3: "HTG",
        countries: vec!["HT"],
        exp: 2,
        name: "Haitian gourde",
        num: "332",
    });
    codes.push(CurrencyCode {
        alpha3: "HNL",
        countries: vec!["HN"],
        exp: 2,
        name: "Honduran lempira",
        num: "340",
    });
    codes.push(CurrencyCode {
        alpha3: "HKD",
        countries: vec![
            "HK",
            "MO",
        ],
        exp: 2,
        name: "Hong Kong dollar",
        num: "344",
    });
    codes.push(CurrencyCode {
        alpha3: "HUF",
        countries: vec!["HU"],
        exp: 2,
        name: "Hungarian forint",
        num: "348",
    });
    codes.push(CurrencyCode {
        alpha3: "ISK",
        countries: vec!["IS"],
        exp: 0,
        name: "Icelandic króna",
        num: "352",
    });
    codes.push(CurrencyCode {
        alpha3: "INR",
        countries: vec![
            "BT",
            "IN",
            "NP",
            "ZW",
        ],
        exp: 2,
        name: "Indian rupee",
        num: "356",
    });
    codes.push(CurrencyCode {
        alpha3: "IDR",
        countries: vec!["ID"],
        exp: 2,
        name: "Indonesian rupiah",
        num: "360",
    });
    codes.push(CurrencyCode {
        alpha3: "IRR",
        countries: vec!["IR"],
        exp: 2,
        name: "Iranian rial",
        num: "364",
    });
    codes.push(CurrencyCode {
        alpha3: "IQD",
        countries: vec!["IQ"],
        exp: 3,
        name: "Iraqi dinar",
        num: "368",
    });
    codes.push(CurrencyCode {
        alpha3: "ILS",
        countries: vec![
            "IL",
            "PS",
        ],
        exp: 2,
        name: "Israeli new shekel",
        num: "376",
    });
    codes.push(CurrencyCode {
        alpha3: "KMD",
        countries: vec!["JM"],
        exp: 2,
        name: "Jamaican dollar",
        num: "388",
    });
    codes.push(CurrencyCode {
        alpha3: "JPY",
        countries: vec!["JP"],
        exp: 0,
        name: "Japanese yen",
        num: "392",
    });
    codes.push(CurrencyCode {
        alpha3: "KZT",
        countries: vec!["KZ"],
        exp: 2,
        name: "Kazakhstani tenge",
        num: "398",
    });
    codes.push(CurrencyCode {
        alpha3: "JOD",
        countries: vec!["JO"],
        exp: 3,
        name: "Jordanian dinar",
        num: "400",
    });
    codes.push(CurrencyCode {
        alpha3: "KES",
        countries: vec!["KE"],
        exp: 2,
        name: "Kenyan shilling",
        num: "404",
    });
    codes.push(CurrencyCode {
        alpha3: "KPW",
        countries: vec!["KP"],
        exp: 2,
        name: "North Korean won",
        num: "408",
    });
    codes.push(CurrencyCode {
        alpha3: "KRW",
        countries: vec!["KR"],
        exp: 0,
        name: "South Korean won",
        num: "410",
    });
    codes.push(CurrencyCode {
        alpha3: "KWD",
        countries: vec!["KW"],
        exp: 3,
        name: "Kuwaiti dinar",
        num: "414",
    });
    codes.push(CurrencyCode {
        alpha3: "KGS",
        countries: vec!["KG"],
        exp: 2,
        name: "Kyrgyzstani som",
        num: "417",
    });
    codes.push(CurrencyCode {
        alpha3: "LAK",
        countries: vec!["LA"],
        exp: 2,
        name: "Lao kip",
        num: "418",
    });
    codes.push(CurrencyCode {
        alpha3: "LBP",
        countries: vec!["LB"],
        exp: 2,
        name: "Lebanese pound",
        num: "422",
    });
    codes.push(CurrencyCode {
        alpha3: "LSL",
        countries: vec!["LS"],
        exp: 2,
        name: "Lesotho loti",
        num: "426",
    });
    codes.push(CurrencyCode {
        alpha3: "LRD",
        countries: vec!["LR"],
        exp: 2,
        name: "Liberian dollar",
        num: "430",
    });
    codes.push(CurrencyCode {
        alpha3: "LYD",
        countries: vec!["LY"],
        exp: 3,
        name: "Libyan dinar",
        num: "434",
    });
    codes.push(CurrencyCode {
        alpha3: "MOP",
        countries: vec!["MO"],
        exp: 2,
        name: "Macanese pataca",
        num: "446",
    });
    codes.push(CurrencyCode {
        alpha3: "MWK",
        countries: vec!["MW"],
        exp: 2,
        name: "Malawian kwacha",
        num: "454",
    });
    codes.push(CurrencyCode {
        alpha3: "MYR",
        countries: vec!["MY"],
        exp: 2,
        name: "Malaysian ringgit",
        num: "458",
    });
    codes.push(CurrencyCode {
        alpha3: "MVR",
        countries: vec!["MV"],
        exp: 2,
        name: "Maldivian rufiyaa",
        num: "462",
    });
    codes.push(CurrencyCode {
        alpha3: "MRO",
        countries: vec!["MR"],
        exp: 1,
        name: "Mauritanian ouguiya",
        num: "478",
    });
    codes.push(CurrencyCode {
        alpha3: "MUR",
        countries: vec!["MU"],
        exp: 2,
        name: "Mauritian rupee",
        num: "480",
    });
    codes.push(CurrencyCode {
        alpha3: "MXN",
        countries: vec!["MX"],
        exp: 2,
        name: "Mexican peso",
        num: "484",
    });
    codes.push(CurrencyCode {
        alpha3: "MNT",
        countries: vec!["MN"],
        exp: 2,
        name: "Mongolian tögrög",
        num: "496",
    });
    codes.push(CurrencyCode {
        alpha3: "MDL",
        countries: vec!["MD"],
        exp: 2,
        name: "Moldovan leu",
        num: "498",
    });
    codes.push(CurrencyCode {
        alpha3: "MAD",
        countries: vec!["MA"],
        exp: 2,
        name: "Moroccan dirham",
        num: "504",
    });
    codes.push(CurrencyCode {
        alpha3: "OMR",
        countries: vec!["OM"],
        exp: 3,
        name: "Omani rial",
        num: "512",
    });
    codes.push(CurrencyCode {
        alpha3: "NAD",
        countries: vec!["NA"],
        exp: 2,
        name: "Namibian dollar",
        num: "516",
    });
    codes.push(CurrencyCode {
        alpha3: "NPR",
        countries: vec!["NP"],
        exp: 2,
        name: "Nepalese rupee",
        num: "524",
    });
    codes.push(CurrencyCode {
        alpha3: "ANG",
        countries: vec![
            "CW",
            "SX",
        ],
        exp: 2,
        name: "Netherlands Antillean guilder",
        num: "532",
    });
    codes.push(CurrencyCode {
        alpha3: "AWG",
        countries: vec!["AW"],
        exp: 2,
        name: "Aruban florin",
        num: "533",
    });
    codes.push(CurrencyCode {
        alpha3: "VUV",
        countries: vec!["VU"],
        exp: 0,
        name: "Vanuatu vatu",
        num: "548",
    });
    codes.push(CurrencyCode {
        alpha3: "NZD",
        countries: vec![
            "AQ",
            "CK",
            "NU",
            "NZ",
            "PN",
            "TK",
        ],
        exp: 2,
        name: "New Zealand dollar",
        num: "554",
    });
    codes.push(CurrencyCode {
        alpha3: "NIO",
        countries: vec!["NI"],
        exp: 2,
        name: "Nicaraguan córdoba",
        num: "558",
    });
    codes.push(CurrencyCode {
        alpha3: "NGN",
        countries: vec!["NG"],
        exp: 2,
        name: "Nigerian naira",
        num: "566",
    });
    codes.push(CurrencyCode {
        alpha3: "NOK",
        countries: vec![
            "AQ",
            "BV",
            "NO",
            "SJ",
        ],
        exp: 2,
        name: "Norwegian krone",
        num: "578",
    });
    codes.push(CurrencyCode {
        alpha3: "PKR",
        countries: vec!["PK"],
        exp: 2,
        name: "Pakistani rupee",
        num: "586",
    });
    codes.push(CurrencyCode {
        alpha3: "PAB",
        countries: vec!["PA"],
        exp: 2,
        name: "Panamanian balboa",
        num: "590",
    });
    codes.push(CurrencyCode {
        alpha3: "PGK",
        countries: vec!["PG"],
        exp: 2,
        name: "Papua New Guinean kina",
        num: "598",
    });
    codes.push(CurrencyCode {
        alpha3: "PYG",
        countries: vec!["PY"],
        exp: 0,
        name: "Paraguayan guaraní",
        num: "600",
    });
    codes.push(CurrencyCode {
        alpha3: "PEN",
        countries: vec!["PE"],
        exp: 2,
        name: "Peruvian Sol",
        num: "604",
    });
    codes.push(CurrencyCode {
        alpha3: "PHP",
        countries: vec!["PH"],
        exp: 2,
        name: "Philippine peso",
        num: "608",
    });
    codes.push(CurrencyCode {
        alpha3: "QAR",
        countries: vec!["QA"],
        exp: 2,
        name: "Qatari riyal",
        num: "634",
    });
    codes.push(CurrencyCode {
        alpha3: "RUB",
        countries: vec![
            "GE-AB",
            "RU",
            "UA-43",
        ],
        exp: 2,
        name: "Russian ruble",
        num: "643",
    });
    codes.push(CurrencyCode {
        alpha3: "RWF",
        countries: vec!["RW"],
        exp: 0,
        name: "Rwandan franc",
        num: "646",
    });
    codes.push(CurrencyCode {
        alpha3: "SHP",
        countries: vec![
            "SH-AC",
            "SH-SH",
        ],
        exp: 2,
        name: "Saint Helena pound",
        num: "654",
    });
    codes.push(CurrencyCode {
        alpha3: "STD",
        countries: vec!["ST"],
        exp: 2,
        name: "São Tomé and Príncipe dobra",
        num: "678",
    });
    codes.push(CurrencyCode {
        alpha3: "SAR",
        countries: vec!["SA"],
        exp: 2,
        name: "Saudi riyal",
        num: "682",
    });
    codes.push(CurrencyCode {
        alpha3: "SCR",
        countries: vec!["SC"],
        exp: 2,
        name: "Seychelles rupee",
        num: "690",
    });
    codes.push(CurrencyCode {
        alpha3: "SLL",
        countries: vec!["SL"],
        exp: 2,
        name: "Sierra Leonean leone",
        num: "694",
    });
    codes.push(CurrencyCode {
        alpha3: "SGD",
        countries: vec![
            "BN",
            "SG",
        ],
        exp: 2,
        name: "Singapore dollar",
        num: "702",
    });
    codes.push(CurrencyCode {
        alpha3: "VND",
        countries: vec!["VN"],
        exp: 0,
        name: "Vietnamese dong",
        num: "704",
    });
    codes.push(CurrencyCode {
        alpha3: "SOS",
        countries: vec!["SO"],
        exp: 2,
        name: "Somali shilling",
        num: "706",
    });
    codes.push(CurrencyCode {
        alpha3: "ZAR",
        countries: vec!["ZA"],
        exp: 2,
        name: "South African rand",
        num: "710",
    });
    codes.push(CurrencyCode {
        alpha3: "SSP",
        countries: vec!["SS"],
        exp: 2,
        name: "South Sudeanese pound",
        num: "728",
    });
    codes.push(CurrencyCode {
        alpha3: "SZL",
        countries: vec!["SZ"],
        exp: 2,
        name: "Swazi lilangeni",
        num: "748",
    });
    codes.push(CurrencyCode {
        alpha3: "SEK",
        countries: vec!["SE"],
        exp: 2,
        name: "Swedish krona/kronor",
        num: "752",
    });
    codes.push(CurrencyCode {
        alpha3: "CHF",
        countries: vec![
            "CH",
            "LI",
        ],
        exp: 2,
        name: "Swiss franc",
        num: "756",
    });
    codes.push(CurrencyCode {
        alpha3: "SYP",
        countries: vec!["SY"],
        exp: 2,
        name: "Syrian pound",
        num: "760",
    });
    codes.push(CurrencyCode {
        alpha3: "THB",
        countries: vec![
            "KH",
            "LA",
            "MM",
            "TH",
        ],
        exp: 2,
        name: "Thai baht",
        num: "764",
    });
    codes.push(CurrencyCode {
        alpha3: "TOP",
        countries: vec!["TO"],
        exp: 2,
        name: "Tongan pa'anga",
        num: "776",
    });
    codes.push(CurrencyCode {
        alpha3: "TTD",
        countries: vec!["TT"],
        exp: 2,
        name: "Trinidad and Tobago dollar",
        num: "780",
    });
    codes.push(CurrencyCode {
        alpha3: "AED",
        countries: vec!["AE"],
        exp: 2,
        name: "United Arab Emirates dirham",
        num: "784",
    });
    codes.push(CurrencyCode {
        alpha3: "TND",
        countries: vec!["TN"],
        exp: 3,
        name: "Tunisian dinar",
        num: "788",
    });
    codes.push(CurrencyCode {
        alpha3: "UGX",
        countries: vec!["UG"],
        exp: 0,
        name: "Ugandan shilling",
        num: "800",
    });
    codes.push(CurrencyCode {
        alpha3: "MKD",
        countries: vec!["MK"],
        exp: 2,
        name: "Macedonian denar",
        num: "807",
    });
    codes.push(CurrencyCode {
        alpha3: "EGP",
        countries: vec!["EG"],
        exp: 2,
        name: "Egyptian pound",
        num: "818",
    });
    codes.push(CurrencyCode {
        alpha3: "GBP",
        countries: vec![
            "GG",
            "GS",
            "IM",
            "IO",
            "JE",
            "SH-TA",
            "UK",
        ],
        exp: 2,
        name: "Pound sterling",
        num: "826",
    });
    codes.push(CurrencyCode {
        alpha3: "TZS",
        countries: vec!["TZ"],
        exp: 2,
        name: "Tanzanian shilling",
        num: "834",
    });
    codes.push(CurrencyCode {
        alpha3: "USD",
        countries: vec![
            "AS",
            "BB",
            "BM",
            "BQ",
            "EC",
            "FM",
            "GU",
            "HT",
            "IO",
            "MH",
            "MP",
            "PA",
            "PR",
            "PW",
            "SV",
            "TC",
            "TL",
            "US",
            "VG",
            "VI",
            "ZW",
        ],
        exp: 2,
        name: "United States dollar",
        num: "840",
    });
    codes.push(CurrencyCode {
        alpha3: "UYU",
        countries: vec!["UY"],
        exp: 2,
        name: "Uruguayan peso",
        num: "858",
    });
    codes.push(CurrencyCode {
        alpha3: "UZS",
        countries: vec!["UZ"],
        exp: 2,
        name: "Uzbekistan som",
        num: "860",
    });
    codes.push(CurrencyCode {
        alpha3: "WST",
        countries: vec!["WS"],
        exp: 2,
        name: "Samoan tala",
        num: "882",
    });
    codes.push(CurrencyCode {
        alpha3: "YER",
        countries: vec!["YE"],
        exp: 2,
        name: "Yemeni rial",
        num: "886",
    });
    codes.push(CurrencyCode {
        alpha3: "TWD",
        countries: vec!["TW"],
        exp: 2,
        name: "New Taiwan dollar",
        num: "901",
    });
    codes.push(CurrencyCode {
        alpha3: "CUC",
        countries: vec!["CU"],
        exp: 2,
        name: "Cuban convertible peso",
        num: "931",
    });
    codes.push(CurrencyCode {
        alpha3: "TMT",
        countries: vec!["TM"],
        exp: 2,
        name: "Turkmenistani manat",
        num: "934",
    });
    codes.push(CurrencyCode {
        alpha3: "GHS",
        countries: vec!["GH"],
        exp: 2,
        name: "Ghanaian cedi",
        num: "936",
    });
    codes.push(CurrencyCode {
        alpha3: "VEF",
        countries: vec!["VE"],
        exp: 2,
        name: "Venezuelan bolivar",
        num: "937",
    });
    codes.push(CurrencyCode {
        alpha3: "SGD",
        countries: vec!["SD"],
        exp: 2,
        name: "Sudanese pound",
        num: "938",
    });
    codes.push(CurrencyCode {
        alpha3: "RSD",
        countries: vec!["RS"],
        exp: 2,
        name: "Serbian dinar",
        num: "941",
    });
    codes.push(CurrencyCode {
        alpha3: "MZN",
        countries: vec!["MZ"],
        exp: 2,
        name: "Mozambican metical",
        num: "943",
    });
    codes.push(CurrencyCode {
        alpha3: "AZN",
        countries: vec!["AZ"],
        exp: 2,
        name: "Azerbaijani manat",
        num: "944",
    });
    codes.push(CurrencyCode {
        alpha3: "RON",
        countries: vec!["RO"],
        exp: 2,
        name: "Romanian leu",
        num: "946",
    });
    codes.push(CurrencyCode {
        alpha3: "TRY",
        countries: vec!["TR"],
        exp: 2,
        name: "Turkish lira",
        num: "949",
    });
    codes.push(CurrencyCode {
        alpha3: "XAF",
        countries: vec![
            "CM",
            "CF",
            "CG",
            "GA",
            "GQ",
            "TD",
        ],
        exp: 0,
        name: "CFA franc BEAC",
        num: "950",
    });
    codes.push(CurrencyCode {
        alpha3: "XCD",
        countries: vec![
            "AI",
            "AG",
            "DM",
            "GD",
            "KN",
            "LC",
            "MS",
            "VC",
        ],
        exp: 2,
        name: "East Caribbean dollar",
        num: "951",
    });
    codes.push(CurrencyCode {
        alpha3: "XOF",
        countries: vec![
            "BF",
            "BJ",
            "CI",
            "GW",
            "ML",
            "NE",
            "SN",
            "TG",
        ],
        exp: 0,
        name: "CFA franc BCEAO",
        num: "952",
    });
    codes.push(CurrencyCode {
        alpha3: "XPF",
        countries: vec![
            "NC",
            "PF",
            "WF",
        ],
        exp: 0,
        name: "CFP franc",
        num: "953",
    });
    codes.push(CurrencyCode {
        alpha3: "ZMW",
        countries: vec!["ZM"],
        exp: 2,
        name: "Zambian kwacha",
        num: "967",
    });
    codes.push(CurrencyCode {
        alpha3: "SRD",
        countries: vec!["SR"],
        exp: 2,
        name: "Surinamese dollar",
        num: "968",
    });
    codes.push(CurrencyCode {
        alpha3: "MGA",
        countries: vec!["MG"],
        exp: 1,
        name: "Malagasy ariary",
        num: "969",
    });
    codes.push(CurrencyCode {
        alpha3: "AFN",
        countries: vec!["AF"],
        exp: 2,
        name: "Afghan afghani",
        num: "971",
    });
    codes.push(CurrencyCode {
        alpha3: "TJS",
        countries: vec!["TJ"],
        exp: 2,
        name: "Tajikstani somoni",
        num: "972",
    });
    codes.push(CurrencyCode {
        alpha3: "AOA",
        countries: vec!["AO"],
        exp: 2,
        name: "Angolan kwanza",
        num: "973",
    });
    codes.push(CurrencyCode {
        alpha3: "BYR",
        countries: vec!["BY"],
        exp: 0,
        name: "Belarusian ruble",
        num: "974",
    });
    codes.push(CurrencyCode {
        alpha3: "BGN",
        countries: vec!["BG"],
        exp: 2,
        name: "Bulgarian lev",
        num: "975",
    });
    codes.push(CurrencyCode {
        alpha3: "CDF",
        countries: vec!["CD"],
        exp: 2,
        name: "Congolese franc",
        num: "976",
    });
    codes.push(CurrencyCode {
        alpha3: "BAM",
        countries: vec!["BA"],
        exp: 2,
        name: "Bosnia and Herzegovina convertible mark",
        num: "977",
    });
    codes.push(CurrencyCode {
        alpha3: "EUR",
        countries: vec![
            "AD",
            "AT",
            "BE",
            "BL",
            "CY",
            "DE",
            "EE",
            "ES",
            "FI",
            "FR",
            "GP",
            "GR",
            "IE",
            "IT",
            "LT",
            "LU",
            "LV",
            "MC",
            "ME",
            "MQ",
            "MT",
            "NL",
            "NL",
            "PM",
            "PT",
            "RE",
            "SI",
            "SK",
            "VA",
            "XK",
            "YT",
        ],
        exp: 2,
        name: "Euro",
        num: "978",
    });
    codes.push(CurrencyCode {
        alpha3: "UAH",
        countries: vec!["UA"],
        exp: 2,
        name: "Ukrainian hryvnia",
        num: "980",
    });
    codes.push(CurrencyCode {
        alpha3: "GEL",
        countries: vec!["GE"],
        exp: 2,
        name: "Georgian lari",
        num: "981",
    });
    codes.push(CurrencyCode {
        alpha3: "PLN",
        countries: vec!["PL"],
        exp: 2,
        name: "Polish złoty",
        num: "985",
    });
    codes.push(CurrencyCode {
        alpha3: "BRL",
        countries: vec!["BR"],
        exp: 2,
        name: "Brazilian real",
        num: "986",
    });

    codes
}
