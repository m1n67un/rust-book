struct Ipv4Addr {
    // details elided
    address: (u8, u8, u8, u8),
}

struct Ipv6Addr {
    // details elided
    address: String,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr { 
                                    address: (127, 0, 0, 1) 
                                });
    

    let loopback = IpAddr::V6(Ipv6Addr {
                                    address: String::from("::1") 
                                });
}