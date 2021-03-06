#[link(name = "req")];
#[crate_type = "lib"];

extern mod std;

use dvec::*;
use dvec::DVec;
use std::map::HashMap;

type header_map = HashMap<~str, @DVec<@~str>>;

// the unused ty param is necessary so this gets monomorphized
fn request<T: Copy>(req: header_map) {
  let _x = *(*req.get(~"METHOD"))[0u];
}
