pub mod lcs {

    use multiarray::Array2D;

    pub fn exec(m: &str, n: &str) -> u32 {
        let mlen = m.len();
        let nlen = n.len();
        let mb = m.as_bytes();
        let nb = n.as_bytes();
        println!("m-{},n-{}", m, n);
        let mut dp = Array2D::new([mlen + 1, nlen + 1], 0);
        for i in 0..mlen {
            for j in 0..nlen {
                if mb[i] == nb[j] {
                    dp[[i + 1, j + 1]] = dp[[i, j]] + 1;
                } else {
                    let x = dp[[i + 1, j]];
                    let y = dp[[i, j + 1]];
                    dp[[i + 1, j + 1]] = x.max(y);
                }
            }
        }

        dp[[mlen, nlen]]
    }

    pub fn run() -> () {
        println!("{}", exec("abcde", "acbef"));
    }
}
