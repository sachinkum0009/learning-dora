use dora_node_api::{
    DoraNode, Event, EventStream, IntoArrow, dora_core::config::DataId, init_tracing,
};
use eyre::Context;
use tracing::{Level, span};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let (node, events) = DoraNode::init_from_env()?;

    let tracing_guard =
        init_tracing(&node.id().clone(), node.dataflow_id()).context("failed to init tracing")?;

    run(node, events).await?;
    drop(tracing_guard);
    Ok(())
}

async fn run(mut node: DoraNode, mut events: EventStream) -> eyre::Result<()> {
    while let Some(event) = events.recv() {
        match event {
            Event::Input { id, metadata, data } => match id.as_ref() {
                "temperature" => {
                    let val = f64::try_from(&data).context("failed to parse temperature")?;
                    tracing::info!("received temperature: {}", val);
                }
                other => tracing::error!("ignoring unexpected input: {other}"),
            },
            Event::Stop(_) => {
                tracing::info!("received stop event, exiting");
                break;
            }
            Event::InputClosed { id } => {
                tracing::info!("input `{id}` was closed");
                if *id == "temperature" {
                    tracing::info!("`temperature` closed -> exiting");
                    break;
                }
            }
            other => {
                tracing::info!("received unknown event: {other:?}");
            }
        }
    }

    Ok(())
}
