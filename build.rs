
extern crate csv;

use std::io;
use std::env;
use std::fs::File;
use std::cmp::{Ord, Ordering};
use std::path::Path;
use std::fmt::{Debug, Formatter, Error as FmtError};
use std::io::{BufWriter, Write};
use std::error::Error;
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct IpNet {
	pub ip: u32,
	pub subnet: u32
}

#[derive(Copy, Clone)]
enum TreeNode {
	Value(u32),
	Children(u32, u32),
	Empty
}

impl PartialOrd for IpNet {
	fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
		self.ip.partial_cmp(&o.ip)
	}
}

impl Ord for IpNet {
	fn cmp(&self, o: &Self) -> Ordering {
		self.partial_cmp(o).unwrap()
	}
}

impl Debug for TreeNode {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		match self {
			TreeNode::Value(v) => write!(fmt, "{{{}}}", v),
			TreeNode::Children(c1, c2) => write!(fmt, "({:?}, {:?})", c1, c2),
			TreeNode::Empty => write!(fmt, "{{}}")
		}
	}
}

fn parse_ip(a: &str) -> Result<IpNet, Box<Error>> {
	let parts: Vec<&str> = a.split("/").collect();
	let subnet: u32 = parts[1].parse()?;
	let ip: Ipv4Addr = parts[0].parse()?;

	Ok(IpNet{ ip: u32::from(ip), subnet })
}

fn insert_val<'a>(k: &IpNet, v: u32, n: u32, depth: u32, list: &mut Vec<TreeNode>) -> Result<(), String>{
	if depth == k.subnet {
		list[n as usize] = TreeNode::Value(v);
		return Ok(());
	}

	match list[n as usize] {
		TreeNode::Empty => {
			let n1 = list.len();
			list.push(TreeNode::Empty);
			let n2 = list.len();
			list.push(TreeNode::Empty);

			list[n as usize] = TreeNode::Children(n1 as u32, n2 as u32);

			insert_val(k, v, n, depth, list)?;
		},
		TreeNode::Children(c0, c1) => {
			if (k.ip & (1 << (31 - depth))) == 0 {
				insert_val(k, v, c0, depth + 1, list)?;
			}
			else {
				insert_val(k, v, c1, depth + 1, list)?;
			}
		},
		TreeNode::Value(v2) => return Err(format!(
			"{}.{}.{}.{}/{} {:?} {:?} {}",
			(k.ip & 0xFF000000) >> 24,
			(k.ip & 0x00FF0000) >> 16,
			(k.ip & 0x0000FF00) >> 8,
			(k.ip & 0x000000FF),
			k.subnet,
			v,
			depth,
			v2
		))
	}

	Ok(())
}

fn main() -> Result<(), Box<Error>> {
	env::set_var("RUST_BACKTRACE", "1");

	let path = Path::new(&env::var("OUT_DIR").unwrap()).join("geoip.rs");

	let mut writer = BufWriter::new(File::create(&path).unwrap());

	let mut regionmap: HashMap<u32, (String, u32)> = HashMap::default();
	let mut tree = Vec::with_capacity(600_000);
	tree.push(TreeNode::Empty);

	{
		let mut ctr = 0;
		let mut reader = csv::Reader::from_path("GeoLite2-Country-Locations-en.csv")?;
		for result in reader.records() {
			let record = result?;
			
			let id: u32 = record[0].parse()?;
			let country: String = record[4].to_string();

			regionmap.insert(id, (country, ctr));
			ctr += 1;
		}
	}

	{
		let mut reader = csv::Reader::from_path("GeoLite2-Country-Blocks-IPv4.csv")?;
		for result in reader.records() {
			let record = result?;

			let ip = parse_ip(&record[0])?;
			let region: u32 = match record[1].parse() {
				Ok(v) => v,
				Err(_) => match record[2].parse()  {
					Ok(v) => v,
					Err(_) => 0
				}
			};

			let region_id = regionmap
				.get(&region)
				.map(|(_, x)| *x)
				.unwrap_or(0xFFFFFFFF);

			match insert_val(&ip, region_id, 0, 0, &mut tree) {
				Ok(_) => (),
				Err(e) => {
					writeln!(io::stderr(), "{}", e)?;
					writeln!(io::stderr(), "{:?}", tree)?;
					panic!();
				}
			}
		}
	}

	writeln!(&mut writer, "
		#[derive(Debug)]
		pub enum Entry {{
			Ref(u32, u32),
			Val(u32),
			None
		}}")?;


	writeln!(&mut writer, "pub const MAPPING: [Entry; {}] = [", tree.len())?;
	//writeln!(&mut writer, "Entry::None,")?;

	for entry in tree {
		match entry {
			TreeNode::Empty => writeln!(&mut writer, "Entry::None,")?,
			TreeNode::Value(v) => writeln!(&mut writer, "Entry::Val({}),", v)?,
			TreeNode::Children(c1, c2) => writeln!(&mut writer, "Entry::Ref({},{}),", c1, c2)?,
		}
	}

	writeln!(&mut writer, "];")?;

	writeln!(&mut writer, "pub const COUNTRIES: [Option<&'static str>; {}] = [", regionmap.len())?;

	let mut vals: Vec<&str> = vec![];
	vals.resize(regionmap.len(), "None");

	for entry in regionmap.values() {
		vals[entry.1 as usize] = &entry.0;
	}

	for val in vals {
		writeln!(&mut writer, "Some(\"{}\"),", val)?;
	}

	writeln!(&mut writer, "];")?;

	writeln!(&mut writer, "pub const START_IDX: usize = {};", 0)?;

	Ok(())
}
