use dkregistry::v2 as docker;
use futures::stream::StreamExt;
use semver::Version;
use std::error::Error;
use std::result::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let release = octocrab::instance()
        .repos("XAMPPRocky", "octocrab")
        .releases()
        .get_latest()
        .await?;
    println!("{:?}", release.tag_name);

    let tags = octocrab::instance()
        .repos("XAMPPRocky", "octocrab")
        .list_tags()
        .send()
        .await?;

    for tag in tags {
        println!("{:?}", tag.name);
    }

    let user = std::env::var("DOCKER_USER").ok();
    if user.is_none() {
        println!("Missing $DOCKER_USER");
    }

    let password = std::env::var("DOCKER_PASSWD").ok();
    if password.is_none() {
        println!("Missing $DOCKER_PASSWD");
    }

    env_logger::Builder::new()
        .filter(Some("dkregistry"), log::LevelFilter::Trace)
        .filter(Some("trace"), log::LevelFilter::Trace)
        .try_init()?;

    let client = docker::Client::configure()
        .registry("registry-1.docker.io".as_ref())
        .insecure_registry(false)
        .username(user)
        .password(password)
        .build()?;

    let login_scope = format!("repository:{}:pull", "library/rust");

    let dclient = client.authenticate(&[&login_scope]).await?;

    let mut sorted: Vec<Version> = Vec::new();

    dclient
        .get_tags("library/rust".as_ref(), Some(7))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(Result::unwrap)
        .for_each(|tag| {
            println!("{} {:?}", tag, Version::parse(&tag));
            let s = Version::parse(&tag);
            match s {
                Ok(s) => sorted.push(s),
                _ => (),
            }
        });

    sorted.sort();
    sorted.reverse();
    for s in sorted {
        println!("{:?}", s.to_string())
    }

    Ok(())
}
