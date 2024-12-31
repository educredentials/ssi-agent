use agent_shared::config::{config, SecretManagerConfig};
use did_manager::SecretManager;

#[tokio::main]
async fn main() {
    let SecretManagerConfig {
        stronghold_path: snapshot_path,
        stronghold_password: password,
        issuer_eddsa_key_id: _,
        issuer_es256_key_id: _,
        issuer_did: _,
        issuer_fragment: _,
    } = config().secret_manager.clone();

    check_args();

    if is_help() {
        print_help();
        return;
    }

    if use_the_force() {
        println!("Warning: Forcing snapshot generation. This will overwrite the existing snapshot.");

        if std::path::Path::new(&snapshot_path)
            .try_exists()
            .expect("Error: checking if stronghold exists")
        {
            std::fs::remove_file(&snapshot_path).expect("Error: removing stronghold");
        }
    } else {
        if std::path::Path::new(&snapshot_path)
            .try_exists()
            .expect("Error: checking if stronghold exists")
        {
            println!(
                "Error: Snapshot already exists at path: {}. \nUse -f to overwrite.",
                snapshot_path
            );
            // Exit with status code 1
            std::process::exit(1);
        }
    }

    let masked_password = format!(
        "{}{}{}",
        password.chars().next().unwrap(),
        "*".repeat(6),
        password.chars().last().unwrap()
    );
    println!(
        "Generating snapshot at path: {} with password: {}",
        snapshot_path, masked_password
    );

    let generated_result = SecretManager::generate(snapshot_path.clone(), password.clone()).await;
    match generated_result {
        Ok(secret_manager) => {
            print_secret_manager(secret_manager).await;
            std::process::exit(0);
        }
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn check_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Error: Too many arguments: {}", args.len());
        println!("       args: {:?}", args);
        print_help();
        std::process::exit(1);
    }
}

// check -f argument
fn use_the_force() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "-f" {
        return true;
    }

    false
}

// check -h argument
fn is_help() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "-h" {
        return true;
    }

    false
}

async fn print_secret_manager(secret_manager: SecretManager) {
    // Find the longest key length for padding
    let keys = vec![
        "stronghold_storage",
        "stronghold_ext_storage",
        "ed25519_key_id",
        "es256_key_id",
        "es256k_key_id",
        "did",
        "fragment",
    ];
    let max_key_length = keys.iter().map(|key| key.len()).max().unwrap_or(0) + 1;

    print_kv_line(
        "stronghold_storage",
        secret_manager.stronghold_storage.as_secret_manager(),
        max_key_length,
    );
    print_kv_line(
        "stronghold_ext_storage",
        &secret_manager.stronghold_ext_storage.as_secret_manager(),
        max_key_length,
    );
    print_kv_option_line("ed25519_key", &secret_manager.ed25519_key_id, max_key_length);
    print_kv_option_line("es256_key_id", &secret_manager.es256_key_id, max_key_length);
    print_kv_option_line("es256k_key_id", &secret_manager.es256k_key_id, max_key_length);
    print_kv_option_line("did", &secret_manager.did, max_key_length);
    print_kv_option_line("fragment", &secret_manager.fragment, max_key_length);
}

fn print_kv_line<T: std::fmt::Display>(key: &str, value: &T, padding: usize) {
    println!("{:<width$}: {}", key, value, width = padding);
}

fn print_kv_option_line<T: std::fmt::Display>(key: &str, value: &Option<T>, padding: usize) {
    match value {
        Some(inner_value) => println!("{:<width$}: {}", key, inner_value, width = padding),
        None => println!("{:<width$}: None", key, width = padding),
    }
}

fn print_help() {
    println!("Usage: cargo run -p agent_secret_manager -- [-f] [-h]");
    println!("\t-f: Force snapshot generation. Overwrites existing snapshot.");
    println!("\t-h: Print this help message.");
}
