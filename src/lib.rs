
extern crate bit_reverse;

use std::net::Ipv4Addr;
use bit_reverse::ParallelReverse;

mod mapping {
    include!(concat!(env!("OUT_DIR"), "/geoip.rs"));
}

use mapping::*;

fn lookup_ip(
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
                lookup_ip(addr >> 1, &slice[*v1 as usize], depth + 1, slice)
            }
            else {
                lookup_ip(addr >> 1, &slice[*v2 as usize], depth + 1, slice)
            }
        }
    }
}

pub fn lookup_country(addr: &Ipv4Addr) -> Option<&'static str> {
    println!("lookup_country: {:?}", addr);
    let slice = &MAPPING;
    lookup_ip(
        u32::from(*addr).swap_bits(), 
        &slice[mapping::START_IDX], 
        0,
        slice
    )
}
