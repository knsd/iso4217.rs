extern crate iso4217;

fn main() {
    let currency = iso4217::alpha3("EUR").unwrap();
}
