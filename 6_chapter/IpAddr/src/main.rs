enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(0, 0, 8, 127);
    let loopback = IpAddr::V6(String::from("::1"));
}
