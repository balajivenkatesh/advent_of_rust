mod p1;
use p1::p11;

fn main() {
    let out = p11::solve();
    assert_eq!(out, 1709);
}
