use anyhow::Result;
use file_guard_test::FileLock;

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let path = cwd.join("file.lock");
    println!("{}", path.display());

    let guard = FileLock::acquire(&path, || async {
        println!("Blocking waiting for lock...");
        Ok(())
    })
    .await?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    println!("Releasing lock....");
    drop(guard);

    std::thread::sleep(std::time::Duration::from_millis(10000));

    println!("Trying to re-acquire..");

    // This should block as the other process now has the lock!
    let guard = FileLock::acquire(&path, || async {
        println!("Blocking waiting for lock...");
        Ok(())
    })
    .await?;

    Ok(())
}
