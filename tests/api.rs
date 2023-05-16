use std::fs::File;

use revm::Return;
use transaction_simulator::{
    config::config,
    errors::{handle_rejection, ErrorMessage},
    simulate_routes,
    simulation::{SimulationRequest, SimulationResponse},
};
use warp::Filter;

fn filter() -> impl Filter<Extract = (impl warp::Reply,), Error = std::convert::Infallible> + Clone
{
    warp::any()
        .and(simulate_routes(config()))
        .recover(handle_rejection)
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_file() {
    let filter = filter();

    let file = File::open("tests/body.json").expect("file should open read only");
    let json: SimulationRequest =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: SimulationResponse = serde_json::from_slice(res.body()).unwrap();

    let file = File::open("tests/expected.json").expect("file should open read only");
    let expected: SimulationResponse =
        serde_json::from_reader(file).expect("file should be proper JSON");

    assert_eq!(body, expected);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_frax_tx() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x7fEA6786D291A87fC4C98aFCCc5A5d3cFC36bc7b",
      "data": "0xffa2ca3b0f4966e1b9f615aadc207635e8eb08111d2d98aa7d136d3d5df19c7aa33ca711000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000b70a082310100ffffffffff05853d955acef822db058eb8505911ed77f175b99e19198595a30182ffffffffffdef1c0ded9bec7f1a1670819833240f027b25eff70a082310100ffffffffff02853d955acef822db058eb8505911ed77f175b99eb67d77c5010205ffffffff02742f2c5d96c0858d00860039c22d2805bed420e870a082310100ffffffffff05a1d100a5bf6bfd2736837c97248853d989a9ed84095ea7b3010302ffffffffff853d955acef822db058eb8505911ed77f175b99e6e553f65010200ffffffffffa1d100a5bf6bfd2736837c97248853d989a9ed8470a082310100ffffffffff00a1d100a5bf6bfd2736837c97248853d989a9ed84b67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e86e7a43a3010004ffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0b241c59120100ffffffffffff7e7d64d987cab6eed08a191c4c2459daf2f8ed0b000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000089ba58cc0e8bcbc1108dbd6f33356a136a021c6200000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000008ac7230489e8000000000000000000000000000000000000000000000000000000000000000001283598d8ab0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000391092bef5e49bed17d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000042c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000064853d955acef822db058eb8505911ed77f175b99e000000000000000000000000000000000000000000000000000000000000869584cd00000000000000000000000010000000000000000000000000000000000000110000000000000000000000000000000000000000000000368e8ed73d64232e820000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000a1d100a5bf6bfd2736837c97248853d989a9ed84000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "value": "10000000000000000000",
      "blockNumber": 16927538,
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: SimulationResponse = serde_json::from_slice(res.body()).unwrap();

    assert!(body.success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_zerox_swap() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x7fEA6786D291A87fC4C98aFCCc5A5d3cFC36bc7b",
      "data": "0xffa2ca3b6ffd0add426dda56ed56d07c2a70895c0b20f7fe207076f2876a55801a216143000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000b70a082310100ffffffffff056b175474e89094c44da98b954eedeac495271d0f19198595a30182ffffffffffdef1c0ded9bec7f1a1670819833240f027b25eff70a082310100ffffffffff026b175474e89094c44da98b954eedeac495271d0fb67d77c5010205ffffffff02742f2c5d96c0858d00860039c22d2805bed420e870a082310100ffffffffff05bcb91e0b4ad56b0d41e0c168e3090361c0039abc095ea7b3010302ffffffffff6b175474e89094c44da98b954eedeac495271d0f6e553f65010200ffffffffffbcb91e0b4ad56b0d41e0c168e3090361c0039abc70a082310100ffffffffff00bcb91e0b4ad56b0d41e0c168e3090361c0039abcb67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e86e7a43a3010004ffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0b241c59120100ffffffffffff7e7d64d987cab6eed08a191c4c2459daf2f8ed0b000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000089ba58cc0e8bcbc1108dbd6f33356a136a021c620000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000128d9627aa40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000097056cc52be90af6c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f869584cd000000000000000000000000100000000000000000000000000000000000001100000000000000000000000000000000000000000000007b1bb82fb4642479d10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000bcb91e0b4ad56b0d41e0c168e3090361c0039abc000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "value": "100000000000000000",
      "blockNumber": 16934525
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: SimulationResponse = serde_json::from_slice(res.body()).unwrap();

    assert!(body.success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_bundle_single_zerox_swap() {
    let filter = filter();

    let json = serde_json::json!([{
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x7fEA6786D291A87fC4C98aFCCc5A5d3cFC36bc7b",
      "data": "0xffa2ca3b6ffd0add426dda56ed56d07c2a70895c0b20f7fe207076f2876a55801a216143000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000b70a082310100ffffffffff056b175474e89094c44da98b954eedeac495271d0f19198595a30182ffffffffffdef1c0ded9bec7f1a1670819833240f027b25eff70a082310100ffffffffff026b175474e89094c44da98b954eedeac495271d0fb67d77c5010205ffffffff02742f2c5d96c0858d00860039c22d2805bed420e870a082310100ffffffffff05bcb91e0b4ad56b0d41e0c168e3090361c0039abc095ea7b3010302ffffffffff6b175474e89094c44da98b954eedeac495271d0f6e553f65010200ffffffffffbcb91e0b4ad56b0d41e0c168e3090361c0039abc70a082310100ffffffffff00bcb91e0b4ad56b0d41e0c168e3090361c0039abcb67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e86e7a43a3010004ffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0b241c59120100ffffffffffff7e7d64d987cab6eed08a191c4c2459daf2f8ed0b000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000089ba58cc0e8bcbc1108dbd6f33356a136a021c620000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000128d9627aa40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000097056cc52be90af6c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f869584cd000000000000000000000000100000000000000000000000000000000000001100000000000000000000000000000000000000000000007b1bb82fb4642479d10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000bcb91e0b4ad56b0d41e0c168e3090361c0039abc000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "value": "100000000000000000",
      "blockNumber": 16934525
    }]);

    let res = warp::test::request()
        .method("POST")
        .path("/simulate-bundle")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: Vec<SimulationResponse> = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(body.len(), 1);
    assert!(body[0].success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_bundle() {
    let filter = filter();

    let json = serde_json::json!([{
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
      "data": "0x095ea7b300000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b00000000000000000000000000000000000000000000000000000000010e3b75",
      "gasLimit": 5000000,
      "blockNumber": 16976359,
    }, {
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0x60f727bdead2ce49b00f2a2133fc707b931d130b",
      "data": "0x8fd8d1bb5ba86686a0554b3441d6dd4921c1f23dc8caac3b4e51d64f338c379a1f02385500000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000c23b872dd01000102ffffffffa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f8b2cb4f0101ffffffffff027e7d64d987cab6eed08a191c4c2459daf2f8ed0b095ea7b3010304ffffffffffa0b86991c6218b36c1d19d4a2e9eb0ce3606eb4819198595a185ffffffffffffdef1c0ded9bec7f1a1670819833240f027b25efff8b2cb4f0101ffffffffff057e7d64d987cab6eed08a191c4c2459daf2f8ed0bb67d77c5010502ffffffffff742f2c5d96c0858d00860039c22d2805bed420e870a082310101ffffffffff02ae7ab96520de3a18e5e111b5eaab095312d7fe84a1903eab030506ffffffffffae7ab96520de3a18e5e111b5eaab095312d7fe8470a082310101ffffffffff01ae7ab96520de3a18e5e111b5eaab095312d7fe84b67d77c5010102ffffffff01742f2c5d96c0858d00860039c22d2805bed420e86e7a43a3010107ffffffff017e7d64d987cab6eed08a191c4c2459daf2f8ed0b241c59120101ffffffffffff7e7d64d987cab6eed08a191c4c2459daf2f8ed0b000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001c00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000003a000000000000000000000000000000000000000000000000000000000000003e0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000093621dca56fe26cdee86e4f6b18e116e9758ff11000000000000000000000000000000000000000000000000000000000000002000000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000007581cd0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000def1c0ded9bec7f1a1670819833240f027b25eff000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000010e3b750000000000000000000000000000000000000000000000000000000000000128d9627aa4000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000010e3b750000000000000000000000000000000000000000000000000020ba4479f1613100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee869584cd000000000000000000000000100000000000000000000000000000000000001100000000000000000000000000000000000000000000005931ace2d8642c3fd00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001",
      "gasLimit": 5000000,
      "blockNumber": 16976359,
    }]);

    let res = warp::test::request()
        .method("POST")
        .path("/simulate-bundle")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: Vec<SimulationResponse> = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(body.len(), 2);
    assert!(body[0].success);
    assert!(body[1].success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_bundle_second_reverts() {
    let filter = filter();

    let json = serde_json::json!([{
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "data": "0x095ea7b300000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000989680",
      "gasLimit": 5000000,
      "blockNumber": 16968595,
    }, {
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0x60f727bdead2ce49b00f2a2133fc707b931d130b",
      "data": "0x8fd8d1bbffc9011b73f477fef5d4ebdd0903025283464275113654045eef0ce1ba22ccea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000006f8b2cb4f0100ffffffffff057e7d64d987cab6eed08a191c4c2459daf2f8ed0b095ea7b3010102ffffffffffdac17f958d2ee523a2206206994597c13d831ec719198595a183ffffffffffffdef1c0ded9bec7f1a1670819833240f027b25efff8b2cb4f0100ffffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0bb67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e8a1903eab030004ffffffffffae7ab96520de3a18e5e111b5eaab095312d7fe84000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000def1c0ded9bec7f1a1670819833240f027b25eff000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000000128d9627aa40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000130e53e715e8a500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee869584cd00000000000000000000000010000000000000000000000000000000000000110000000000000000000000000000000000000000000000cef2d4383a642acb0d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "blockNumber": 16968595,
    }]);

    let res = warp::test::request()
        .method("POST")
        .path("/simulate-bundle")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: Vec<SimulationResponse> = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(body.len(), 2);
    assert!(body[0].success);
    assert!(!body[1].success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_no_data() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5",
      "gasLimit": 21000,
      "value": "100000",
      "blockNumber": 16784600
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: SimulationResponse = serde_json::from_slice(res.body()).unwrap();

    assert!(body.success);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_incorrect_chain_id() {
    temp_env::async_with_vars(
        [(
            "FORK_URL",
            Some("https://eth-mainnet.g.alchemy.com/v2/demo"),
        )],
        async {
            let filter = filter();

            let json = serde_json::json!({
              "chainId": 137,
              "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
              "to": "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5",
              "gasLimit": 21000,
              "value": "100000",
              "blockNumber": 16784600
            });

            let res = warp::test::request()
                .method("POST")
                .path("/simulate")
                .json(&json)
                .reply(&filter)
                .await;

            assert_eq!(res.status(), 400);

            let body: ErrorMessage = serde_json::from_slice(res.body()).unwrap();

            assert_eq!(body.message, "INCORRECT_CHAIN_ID".to_string());
        },
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_not_enough_gas() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x66fc62c1748e45435b06cf8dd105b73e9855f93e",
      "gasLimit": 20000,
      "value": "100000",
      "blockNumber": 16784600
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: SimulationResponse = serde_json::from_slice(res.body()).unwrap();

    assert!(!body.success);
    assert_eq!(body.exit_reason, Return::OutOfGas);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_invalid_from() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA9604",
      "to": "0x66fc62c1748e45435b06cf8dd105b73e9855f93e",
      "data": "0xffa2ca3b44eea7c8e659973cbdf476546e9e6adfd1c580700537e52ba7124933a97904ea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001d0e30db00300ffffffffffffc02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000186a0",
      "gasLimit": 500000,
      "value": "100000",
      "blockNumber": 16784600
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 400);

    let body: ErrorMessage = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(body.message, "BAD REQUEST: invalid length 39, expected a (both 0x-prefixed or not) hex string or byte array containing 20 bytes at line 1 column 63".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_invalid_to() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x66fc62c1748e45435b06cf8dd105b73e9855f93",
      "data": "0xffa2ca3b44eea7c8e659973cbdf476546e9e6adfd1c580700537e52ba7124933a97904ea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001d0e30db00300ffffffffffffc02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000186a0",
      "gasLimit": 500000,
      "value": "100000",
      "blockNumber": 16784600
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 400);

    let body: ErrorMessage = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(body.message, "BAD REQUEST: invalid length 39, expected a (both 0x-prefixed or not) hex string or byte array containing 20 bytes at line 1 column 113".to_string());
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_invalid_data() {
    let filter = filter();

    let json = serde_json::json!({
      "chainId": 1,
      "from": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
      "to": "0x66fc62c1748e45435b06cf8dd105b73e9855f93e",
      "data": "0xffa2ca3b44eea7c8e659973cbdf476546e9e6adfd1c580700537e52ba7124933a97904ea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001d0e30db00300ffffffffffffc02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000186a",
      "gasLimit": 500000,
      "value": "100000",
      "blockNumber": 16784600
    });

    let res = warp::test::request()
        .method("POST")
        .path("/simulate")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 400);

    let body: ErrorMessage = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(
        body.message,
        "BAD REQUEST: Odd number of digits at line 1 column 709".to_string()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_bundle_multiple_block_numbers() {
    let filter = filter();

    let json = serde_json::json!([{
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "data": "0x095ea7b300000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000989680",
      "gasLimit": 5000000,
      "blockNumber": 16968595,
    }, {
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0x60f727bdead2ce49b00f2a2133fc707b931d130b",
      "data": "0x8fd8d1bbffc9011b73f477fef5d4ebdd0903025283464275113654045eef0ce1ba22ccea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000006f8b2cb4f0100ffffffffff057e7d64d987cab6eed08a191c4c2459daf2f8ed0b095ea7b3010102ffffffffffdac17f958d2ee523a2206206994597c13d831ec719198595a183ffffffffffffdef1c0ded9bec7f1a1670819833240f027b25efff8b2cb4f0100ffffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0bb67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e8a1903eab030004ffffffffffae7ab96520de3a18e5e111b5eaab095312d7fe84000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000def1c0ded9bec7f1a1670819833240f027b25eff000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000000128d9627aa40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000130e53e715e8a500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee869584cd00000000000000000000000010000000000000000000000000000000000000110000000000000000000000000000000000000000000000cef2d4383a642acb0d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "blockNumber": 16968596,
    },
    {
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0x7E7d64D987cAb6EeD08A191C4C2459dAF2f8ED0B",
      "data": "0x796b89b9", // getBalance(0x7E7d64D987cAb6EeD08A191C4C2459dAF2f8ED0B): 0xf8b2cb4f00000000000000000000000093621dca56fe26cdee86e4f6b18e116e9758ff11
      "gasLimit": 5000000,
      "blockNumber": 16968597,
    }]);

    let res = warp::test::request()
        .method("POST")
        .path("/simulate-bundle")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);

    let body: Vec<SimulationResponse> = serde_json::from_slice(&res.body()).unwrap();
    
    assert_eq!(body.len(), 3);
    assert_eq!(body[0].success, true);
    assert_eq!(body[1].success, false);
    assert_eq!(body[2].success, true);

    println!("{:#?}", body);
    assert_eq!(body[0].block_number, 16968595);
    assert_eq!(body[1].block_number, 16968596);
    assert_eq!(body[2].block_number, 16968597);
}


#[tokio::test(flavor = "multi_thread")]
async fn post_simulate_bundle_multiple_block_numbers_invalid_order() {
    let filter = filter();

    let json = serde_json::json!([{
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "data": "0x095ea7b300000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000989680",
      "gasLimit": 5000000,
      "blockNumber": 16968597,
    }, {
      "chainId": 1,
      "from": "0x93621dca56fe26cdee86e4f6b18e116e9758ff11",
      "to": "0x60f727bdead2ce49b00f2a2133fc707b931d130b",
      "data": "0x8fd8d1bbffc9011b73f477fef5d4ebdd0903025283464275113654045eef0ce1ba22ccea000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000006f8b2cb4f0100ffffffffff057e7d64d987cab6eed08a191c4c2459daf2f8ed0b095ea7b3010102ffffffffffdac17f958d2ee523a2206206994597c13d831ec719198595a183ffffffffffffdef1c0ded9bec7f1a1670819833240f027b25efff8b2cb4f0100ffffffffff007e7d64d987cab6eed08a191c4c2459daf2f8ed0bb67d77c5010005ffffffff00742f2c5d96c0858d00860039c22d2805bed420e8a1903eab030004ffffffffffae7ab96520de3a18e5e111b5eaab095312d7fe84000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000320000000000000000000000000000000000000000000000000000000000000002000000000000000000000000060f727bdead2ce49b00f2a2133fc707b931d130b0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000def1c0ded9bec7f1a1670819833240f027b25eff000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000000128d9627aa40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000130e53e715e8a500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee869584cd00000000000000000000000010000000000000000000000000000000000000110000000000000000000000000000000000000000000000cef2d4383a642acb0d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "gasLimit": 5000000,
      "blockNumber": 16968596,
    }]);

    let res = warp::test::request()
        .method("POST")
        .path("/simulate-bundle")
        .json(&json)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 400);

    let body: ErrorMessage = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(
        body.message,
        "INVALID_BLOCK_NUMBERS".to_string()
    );
}
