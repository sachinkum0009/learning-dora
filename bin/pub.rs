use dora_node_api::{
    DoraNode, Event, EventStream, IntoArrow, dora_core::config::DataId, init_tracing,
};
use eyre::Context;
use tracing::info;

fn main() -> eyre::Result<()> {
    let (node, events) = DoraNode::init_from_env()?;
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to build tokio runtime")?;
    let rt_guard = rt.enter();
    let tracing_guard =
        init_tracing(&node.id().clone(), node.dataflow_id()).context("failed to init tracing")?;

    let result = run(node, events);

    drop(tracing_guard);
    drop(rt_guard);
    result
}

fn run(mut node: DoraNode, mut _events: EventStream) -> eyre::Result<()> {
    let output_id = DataId::from("temperature".to_owned());
    let temp: f64 = 42.0;
    while let Some(event) = _events.recv() {
        match event {
            Event::Input { id, metadata: _, data: _ } => match id.as_ref() {
                "tick" => {
                    info!("tick received");
                    node.send_output(output_id.clone(), Default::default(), temp.into_arrow())?;
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
