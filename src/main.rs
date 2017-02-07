extern crate markov;

use markov::Chain;
use std::str;

fn main() {
    let test_text_bytes = include_bytes!("../test.txt");
    let test_text = str::from_utf8(test_text_bytes).unwrap();

    let mut chain: Chain<String> = Chain::new();
    chain.feed_str(test_text);


    let new_text = chain.generate();

    println!("{:?}", &new_text[..5]);

}
