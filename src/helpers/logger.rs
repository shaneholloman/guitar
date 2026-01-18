#[macro_export]
macro_rules! logf { ($($t:tt)*) => {{
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open("guitar.log").unwrap();
    writeln!(f, $($t)*).unwrap();
}}}
