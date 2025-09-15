#[macro_export]
macro_rules! concat {
    (&[$ty:ty]: $($s:expr),+) => {{
        const LEN: usize = $( $s.len() + )* 0;
        const ARR: [$ty; LEN] = {
            let mut arr: [$ty; LEN] = [""; LEN];
            let mut base: usize = 0;
            $({
                let mut i = 0;
                while i < $s.len() {
                    arr[base + i] = $s[i];
                    i += 1;
                }
                base += $s.len();
            })*
            if base != LEN { panic!("invalid length"); }
            arr
        };
        &ARR
    }}
}
