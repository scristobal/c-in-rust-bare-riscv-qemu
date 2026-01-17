mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn gcd(n: u32, m: u32) -> u32 {
    let pair = &mut bindings::Pair { n, m };
    unsafe { bindings::gcd(pair) }
}
