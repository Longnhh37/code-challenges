impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let is_ipv4 = Self::is_ipv4(&query_ip);
        let is_ipv6 = Self::is_ipv6(&query_ip);

        if is_ipv4 {
            "IPv4".to_string()
        } else if is_ipv6 {
            "IPv6".to_string()
        } else {
            "Neither".to_string()
        }
    }

    fn is_ipv4(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        for p in parts {
            if p.len() > 1 && p.as_bytes()[0] == b'0' {
                return false;
            }
            match p.parse::<i32>() {
                Ok(v) if v < 0 || v > 255 => return false,
                Err(_) =>  return false,
                _ => continue,
            }
        }
        true
    }

    fn is_ipv6(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split(':').collect();
        if parts.len() != 8 {
            return false;
        }
        println!("{:?}", parts);
        for p in parts {
            if p.is_empty() || p.len() > 4 {
                return false;
            }
            for b in p.bytes() {
                match b {
                    b if b.is_ascii_digit() => {},
                    b'a'..=b'f' | b'A'..=b'F' => {},
                    _ => return false,
                }
            }
        }
        true
    }
}
