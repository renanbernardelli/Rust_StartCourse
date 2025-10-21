fn main() {
    let a = 5 + 10;
    let b = 95.5 - 4.3;
    let c = 4 * 30;
    let d = 56.7 / 32.2;
    let e = 43 % 5;
    println!("a: {}, b:{}, c:{}, d:{}, e:{}", a, b, c, d, e);

    // Bitwise operators
    let f = 0b0000_1100 & 0b0000_1010;
    let g = 0b0000_1100 | 0b0000_1010;
    let h = 0b0000_1100 ^ 0b0000_1010;
    let i = !0b0000_1100;
    let j = 0b0000_1100 << 2;
    let k = 0b0000_1100 >> 2;
    println!("f: {:04b}, g: {:04b}, h: {:04b}, i: {:04b}, j: {:04b}, k: {:04b}", f, g, h, i, j, k);

    let l = 10 & 3;
    let m = 10 | 3;
    let n = 10 ^ 3;
    let o = !10;
    let p = 10 << 2;
    let q = 10 >> 2;
    println!("l: {}, m: {}, n: {}, o: {}, p: {}, q: {}", l, m, n, o, p, q);
}
