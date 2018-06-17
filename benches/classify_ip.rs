
#![feature(test)]

extern crate test;
extern crate geolocate_ip;

use test::Bencher;

#[bench]
fn valid_ip_lookup(b: &mut Bencher) {
	let extaddr = "8.8.8.8".parse().unwrap();

	b.iter(move || {
		let addr = test::black_box(extaddr);

		geolocate_ip::lookup_ip(&addr)
	})
}

#[bench]
fn invalid_ip_lookup(b: &mut Bencher) {
	let extaddr = "0.0.0.0".parse().unwrap();

	b.iter(move || {
		let addr = test::black_box(extaddr);

		geolocate_ip::lookup_ip(&addr)
	})
}
