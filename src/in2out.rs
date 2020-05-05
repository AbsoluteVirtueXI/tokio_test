use std::io;

pub fn in2out() -> Result<u64, io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout)
}