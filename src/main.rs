#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let client = aws_sdk_ssm::Client::new(&config);

    let value = client
        .get_parameter()
        .name("/message")
        .send()
        .await?
        .parameter
        .and_then(|p| p.value)
        .unwrap_or_default();

    println!("{}", value);

    Ok(())
}
