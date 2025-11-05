fn enumerations() {
        enum SorteAdresseIp {
            V4,
            V6,
        }

        struct AdresseIp {
            sorte: SorteAdresseIp,
            adresse: String,
        }

        let local = AdresseIp {
            sorte: SorteAdresseIp::V4,
            adresse: String::from("127.0.0.1"),
        };

        let rebouclage = AdresseIp {
            sorte: SorteAdresseIp::V6,
            adresse: String::from("::1"),
        };

    // On peut aussi faire ça
    struct Ipv4Addr {
        octet1: u8, octet2: u8, octet3: u8, octet4: u8,
    }

    struct Ipv6Addr {
        segments: [u16; 8],
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // Création des instances
    let adresse_locale = IpAddr::V4(Ipv4Addr {
        octet1: 127,
        octet2: 0,
        octet3: 0,
        octet4: 1,
    });

    let adresse_v6 = IpAddr::V6(Ipv6Addr {
        segments: [0, 0, 0, 0, 0, 0, 0, 1],
    });

    // Exemple d'utilisation avec match
    match adresse_locale {
        IpAddr::V4(addr) => println!("IPv4: {}.{}.{}.{}",
                                     addr.octet1, addr.octet2, addr.octet3, addr.octet4),
        IpAddr::V6(addr) => println!("IPv6 avec {} segments", addr.segments.len()),
    }

    match adresse_v6 {
        IpAddr::V4(addr) => println!("IPv4: {}.{}.{}.{}",
                                     addr.octet1, addr.octet2, addr.octet3, addr.octet4),
        IpAddr::V6(addr) => println!("IPv6 avec {} segments", addr.segments.len()),
    }
}