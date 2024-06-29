use octocrab::params::repos::Reference::Tag;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <repository> <version> <sha>", args[0]);
        return ExitCode::FAILURE;
    }

    let owner_repository = &args[1];
    let parts: Vec<&str> = owner_repository.split('/').collect();
    if parts.len() != 2 {
        return ExitCode::FAILURE;
    }

    let owner = &parts[0];
    let repository = &parts[1];

    let version = &args[2];
    let sha = &args[3];

    let octocrab = octocrab::instance();

    let reference = octocrab
        .repos(owner.to_string(), repository.to_string())
        .create_ref(&Tag(version.to_string()), sha)
        .await;

    let referece = match reference {
        Ok(referece) => referece,
        Err(err) => {
            println!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    println!("{}", referece.url);
    return ExitCode::SUCCESS;
}
