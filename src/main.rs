use shortener::{api, config::Config, Error};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::load()?;

    let logs_layer = tracing_subscriber::fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(logs_layer).init();

    api::serve(config).await?;
    Ok(())
}
