
extern crate geolocate_ip;

use std::net;
use std::env;

fn main() {
	let addrstr = env::args().into_iter().collect::<Vec<String>>()[1].clone();
	println!("{:?}", addrstr);
	let addr: net::Ipv4Addr = addrstr.parse().unwrap();

	println!("{:?}", geolocate_ip::lookup_country(&addr));
}