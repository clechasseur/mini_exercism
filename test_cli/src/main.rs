use mini_exercism::api;
use mini_exercism::api::website::TrackFilters;
use mini_exercism::api::website::TrackStatusFilter::Joined;
use mini_exercism::cli::get_cli_credentials;

#[tokio::main]
async fn main() -> mini_exercism::core::Result<()> {
    let website_client = api::website::Client::builder()
        .credentials(get_cli_credentials()?)
        .build();
    let filters = TrackFilters::builder().status(Joined).build();
    let tracks = website_client.get_tracks(Some(filters)).await?;
    println!("Tracks: {}", serde_json::to_string_pretty(&tracks)?);

    Ok(())
}
