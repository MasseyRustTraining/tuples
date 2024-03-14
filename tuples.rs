fn divmod(p: u32, q: u32) -> (u32, u32) {
    (p / q, p % q)
}

fn reconstruct(d: u32, qr: (u32, u32)) -> u32 {
    d * qr.0 + qr.1
}

fn main() {
    let qr = divmod(5, 2);
    let _x: (u32,) = (5,);
    let _x: () = ();
    println!("{}", reconstruct(5, qr));
}
