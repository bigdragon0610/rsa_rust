fn main() {
    let (n, e, d) = key_gen();
    println!("n: {}, e: {}, d: {}", n, e, d);

    let text = "RSA暗号（RSAあんごう）とは、桁数が大きい合成数の素因数分解が現実的な時間内で困難であると信じられていることを安全性の根拠とした公開鍵暗号の一つである。暗号とデジタル署名を実現できる方式として最初に公開されたものである。";
    println!("{}", text);

    let mut cipher_text = Vec::new();
    for x in text.as_bytes() {
        let y = format!("{:08x}", encrypt(*x as u64, n, e));
        cipher_text.push(y);
    }
    let cipher_text = cipher_text.join("");
    println!("{}", cipher_text);

    let mut ans = Vec::new();
    for chunk in cipher_text.chars().collect::<Vec<char>>().chunks(8) {
        let chunk = chunk.iter().collect::<String>();
        let y = u64::from_str_radix(&chunk, 16).unwrap();
        ans.push(decrypt(y, n, d) as u8);
    }

    println!("{}", String::from_utf8_lossy(&ans));
}

fn key_gen() -> (u64, u64, u64) {
    let (p, q) = (63211, 65171);
    let phi = (p - 1) * (q - 1);
    let n = p * q;
    let e = 65537;
    let d = euclid(e, phi);
    (n, e, d)
}

fn encrypt(x: u64, n: u64, e: u64) -> u64 {
    pow(x, n, e)
}

fn decrypt(y: u64, n: u64, d: u64) -> u64 {
    pow(y, n, d)
}

fn pow(base: u64, n: u64, exp: u64) -> u64 {
    let mut exp = exp;
    let mut l = 1;
    let mut r = base;
    while exp > 0 {
        if exp % 2 == 1 {
            l = (l * r) % n;
        }
        exp /= 2;
        r = (r * r) % n;
    }
    l
}

fn euclid(e: u64, phi: u64) -> u64 {
    let e = e as isize;
    let phi = phi as isize;
    let (mut x0, mut y0, mut z0) = (1, 0, e);
    let (mut x1, mut y1, mut z1) = (0, 1, phi);
    while z1 != 1 {
        let q = z0 / z1;
        let (x2, y2, z2) = (x0 - q * x1, y0 - q * y1, z0 - q * z1);
        (x0, y0, z0) = (x1, y1, z1);
        (x1, y1, z1) = (x2, y2, z2);
    }
    let mut ans = x1;
    while ans < 0 {
        ans += phi;
    }
    (ans % phi) as u64
}
