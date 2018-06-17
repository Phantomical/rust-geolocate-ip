
extern crate geolocate_ip;

#[test]
fn localhost_returns_none() {
	let localhost = "127.0.0.1".parse().unwrap();

	assert_eq!(geolocate_ip::lookup_ip(&localhost), None);
}

#[test]
fn null_returns_none() {
	let addr = "0.0.0.0".parse().unwrap();

	assert_eq!(geolocate_ip::lookup_ip(&addr), None);
}

#[test]
fn mask_returns_none() {
	let addr = "255.255.255.255".parse().unwrap();

	assert_eq!(geolocate_ip::lookup_ip(&addr), None);
}

#[test]
fn google_in_america() {
	let addr = "8.8.8.8".parse().unwrap();

	assert_eq!(geolocate_ip::lookup_ip(&addr), Some("US"));
}
