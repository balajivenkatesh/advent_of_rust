mod p1;
mod p2;
mod p3;
mod p4;
mod utils;

fn main() {
    // let out = p1::p11::solve();
    // assert_eq!(out, 1709);

    // let out = p1::p12::solve();
    // assert_eq!(out, 1761);

    // let out = p2::p21::solve();
    // assert_eq!(out, 1484118);

    // let out = p2::p22::solve();
    // println!("out = {}", out);
    // assert_eq!(out, 1463827010);

    // let out = p3::p31::solve();
    // println!("out = {}", out);
    // assert_eq!(out, 1025636);

    // let out = p3::p32::solve();
    // println!("out = {}", out);
    // assert_eq!(out, 793873);

    let out = p4::p41::solve();
    println!("out = {}", out);
    assert_eq!(out, 71708);
}
