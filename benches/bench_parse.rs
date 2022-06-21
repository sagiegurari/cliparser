#![feature(test)]
extern crate test;

use cliparser;
use test::Bencher;

#[bench]
fn parse(bencher: &mut Bencher) {
    bencher.iter(|| {
        let cliparser = cliparser::parse([]);

        assert!(cliparser.version.is_some());
        assert!(cliparser.channel.is_some());
    });
}
