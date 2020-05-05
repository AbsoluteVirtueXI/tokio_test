use tokio::io;
use tokio::time::{delay_for, Duration, interval};
use tokio::process::Command;
use tokio::task::spawn_blocking;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Even if it is called first the blocking code is running in a separate thread
    let block = spawn_blocking(|| {
        sync_looping();
    });
    let dating = tokio::spawn(dating());
    let in2out = tokio::spawn(in2out());
    let res = in2out.await??;
    //dating.await??;// (Comment this line if you want to exit when sending EOF to stdin
    println!("END OF THE ASYNC PROGRAM");
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

/// Blocking code
fn sync_looping() {
    for i in 0..=1000 {
        thread::sleep(Duration::from_secs(2));
        println!("Loop index: {}", i);
    }
    println!("I finished my loop journey!");
}