type pair<A,B> = {
    a: A, b: B
};

fn f<A:Copy Owned>(a: A, b: u16) -> fn@() -> (A, u16) {
    fn@() -> (A, u16) { (a, b) }
}

fn main() {
    let (a, b) = f(22_u64, 44u16)();
    debug!("a=%? b=%?", a, b);
    assert a == 22u64;
    assert b == 44u16;
}