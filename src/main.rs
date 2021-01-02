use std::error::Error;

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

    Ok(())
}
