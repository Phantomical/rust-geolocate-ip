//! Static geolocation of IP addresses based on 
//! the MaxMind GeoIP data.
//! 
//! This product includes GeoLite2 data created
//! by MaxMind, available from 
//! [http://www.maxmind.com](http://www.maxmind.com).

extern crate bit_reverse;

use std::net::Ipv4Addr;
use bit_reverse::ParallelReverse;

mod mapping {
    include!(concat!(env!("OUT_DIR"), "/geoip.rs"));
}

use mapping::*;

fn lookup_ip_impl(
    addr: u32, 
    entry: &Entry,
    depth: u32,
    slice: &[Entry]
) -> Option<&'static str> {
    if depth > 64 {
        panic!("Loop detected!");
    }

    match entry {
        Entry::Val(v) => {
            COUNTRIES[*v as usize]
        },
        Entry::None => {
            None
        },
        Entry::Ref(v1, v2) => {
            if (addr & 1) == 0 {
                lookup_ip_impl(addr >> 1, &slice[*v1 as usize], depth + 1, slice)
            }
            else {
                lookup_ip_impl(addr >> 1, &slice[*v2 as usize], depth + 1, slice)
            }
        }
    }
}

/// Look up the country code for an IP address, if one exists. 
/// 
/// # Examples
/// 
/// Look up the location of Google's DNS servers,
/// which are currently located in the US.
/// ```
/// # // This example should be updated if
/// # // Google moves their DNS servers.
/// # extern crate geolocate_ip;
/// # use geolocate_ip::*;
/// # fn main() {
/// let addr = "8.8.8.8".parse().unwrap();
/// assert_eq!(
///     lookup_ip(&addr).unwrap(),
///     "US"
/// );
/// # }
/// ```
/// 
/// Trying to lookup an IP address that doesn't
/// have a location associated with it.
/// ```
/// # extern crate geolocate_ip;
/// # use geolocate_ip::*;
/// # fn main() {
/// // The IP address of localhost, which
/// // is the current computer's address 
/// // on the local network.
/// // If you want to look up your own 
/// // country you will need to use your
/// // public IP address.
/// let addr = "127.0.0.1".parse().unwrap();
/// assert_eq!(
///     lookup_ip(&addr),
///     None
/// );
/// # }
/// ```
pub fn lookup_ip(addr: &Ipv4Addr) -> Option<&'static str> {
    let slice = &MAPPING;
    lookup_ip_impl(
        u32::from(*addr).swap_bits(), 
        &slice[mapping::START_IDX], 
        0,
        slice
    )
}
