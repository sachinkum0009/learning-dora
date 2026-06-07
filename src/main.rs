use dora_cli::run;
use eyre::Context;
use std::path::Path;

fn main() -> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{:?}", root);
    std::env::set_current_dir(root.join(file!()).parent().unwrap())
        .wrap_err("failed to set working dir")?;

    let args: Vec<String> = std::env::args().collect();
    let dataflow = if args.len() > 1 {
        args[1].clone()
    } else {
        root.join("dataflow.yml").to_string_lossy().into_owned()
    };
    println!("dataflow: {}", dataflow);

    // build(dataflow.clone(), coordinator_addr, coordinator_port, uv, force_local)

    // build(
    //     dataflow: dataflow.clone(),
    //     force_local: true,
    //     ..Default::default()
    // )?;

    run(dataflow, false)?;

    Ok(())
}
