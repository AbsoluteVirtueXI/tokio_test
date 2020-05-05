use tokio::io;
use tokio::time::{delay_for, Duration, interval};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dating = tokio::spawn(dating());
    let in2out = tokio::spawn(in2out());
    let res = in2out.await??;
    //dating.await??;// (Comment this line if you want to exit when sending EOF to stdin
    Ok(())
}

/// Execute the `date` unix command every 1 second forever
async fn dating() -> Result<(), std::io::Error> {
    let mut interval = interval(Duration::from_secs(1));
    let mut date = Command::new("date");
    loop {
        interval.tick().await;
        date.spawn()?.await?;
    }
}

/// Read stdin and write to stdout until EOF is received
async fn in2out() -> Result<u64, std::io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout).await
}