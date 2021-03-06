

// Regression test for issue #374
extern mod std;
use option::None;

enum sty { ty_nil, }

type raw_t = {struct_: sty, cname: Option<~str>, hash: uint};

fn mk_raw_ty(st: sty, cname: Option<~str>) -> raw_t {
    return {struct_: st, cname: cname, hash: 0u};
}

fn main() { mk_raw_ty(ty_nil, None::<~str>); }
