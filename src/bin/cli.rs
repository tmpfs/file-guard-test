use anyhow::Result;
use file_guard_test::FileLock;

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let path = cwd.join("file.lock");
    println!("{}", path.display());

    let _guard = FileLock::acquire(path, || async {
        println!("Blocking waiting for lock...");
        Ok(())
    })
    .await?;

    std::thread::sleep(std::time::Duration::from_millis(15000));

    Ok(())
}
