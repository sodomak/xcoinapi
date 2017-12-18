extern crate curl;

use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    easy.url("https://api.coinmarketcap.com/v1/ticker/bitcoin/?convert=CZK").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());

    let mut easy = Easy::new();
    easy.url("https://api.coinmarketcap.com/v1/ticker/litecoin/?convert=CZK").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());


    let mut easy = Easy::new();
    easy.url("https://api.coinmarketcap.com/v1/ticker/monero/?convert=CZK").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());


    let mut easy = Easy::new();
    easy.url("https://api.coinmarketcap.com/v1/ticker/byteball/?convert=CZK").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());









}

