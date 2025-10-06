mod config;
mod provider;

use provider::KeyValueStoreProvider;

/// Typically the `main` function is kept simple and the provider logic is
/// implemented in a separate module. Head to the `provider.rs` file to see the implementation
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    KeyValueStoreProvider::run().await?;
    eprintln!("Custom template provider exiting");
    Ok(())
}
