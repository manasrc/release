use octocrab::params::repos::Reference::Tag;
use octocrab::Octocrab;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

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

    let octocrab = Octocrab::builder().personal_token(token).build();
    let octocrab = match octocrab {
        Ok(octocrab) => octocrab,
        Err(_) => {
            println!("Error instanciating the builder");
            return ExitCode::FAILURE;
        }
    };

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
