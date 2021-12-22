mod p1;
mod p2;

fn main() {
    // let out = p1::p11::solve();
    // assert_eq!(out, 1709);

    // let out = p1::p12::solve();
    // assert_eq!(out, 1761);

    // let out = p2::p21::solve();
    // assert_eq!(out, 1484118);

    let out = p2::p22::solve();
    println!("out = {}", out);
    assert_eq!(out, 1463827010);
}
