use fuels::{prelude::*, tx::ContractId};
use plotters::prelude::*;

// Load abi from json
abigen!(MemoryConsoomoor, "out/debug/vecheap-abi.json");

const ZERO_CAP: &'static str = "./zero_cap.png";
const N_CAP: &'static str = "./n_cap.png";

async fn get_contract_instance() -> (MemoryConsoomoor, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/vecheap.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/vecheap-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MemoryConsoomoor::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn vec_push_checks() {
    plot(ZERO_CAP, 0).await;
    plot(N_CAP, 16).await;
}

async fn plot(outfile: &'static str, capacity: u64) {
    let (instance, _id) = get_contract_instance().await;

    let mut data: Vec<u64> = Vec::new();

    for n in 0..16 {
        let memory_consumed = instance
            .methods()
            .vec_push(capacity, n)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        data.push(memory_consumed);
    }

    let root = BitMapBackend::new(outfile, (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(
            &format!("`Vec::push` Memory Allocations ({} capacity)", capacity),
            ("sans-serif", 20.0)
        )
        .build_cartesian_2d((0u64..16u64).into_segmented(), 0u64..300u64)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Heap Delta")
        .x_desc("Pushed Items")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                data
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as u64, *y))
            ),
    ).unwrap();

    root.present().expect("Unable to write result to file");
}
