fn main() {
    let (n, e, d) = key_gen();

    let text = "abcde🙏";
    let mut ans = Vec::new();

    for x in text.as_bytes() {
        println!("x: {}", x);
        let y = encrypt(*x as u64, n, e);
        println!("y: {}", y);
        let x = decrypt(y, n, d);
        println!("x: {}", x);
        ans.push(x as u8);
    }

    println!("{}", String::from_utf8_lossy(&ans));
}

fn key_gen() -> (u64, u64, u64) {
    let (p, q) = (991, 997);
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
