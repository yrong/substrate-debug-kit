use assert_cmd::Command;

#[test]
#[ignore = "requires unsafe RPC"]
fn staking_works() {
	let mut cmd = Command::cargo_bin("offline-election").unwrap();
	cmd.args(&["--uri", TEST_URI, "staking"]).unwrap();
}

#[test]
#[ignore = "requires unsafe RPC"]
fn council_works() {
	let mut cmd = Command::cargo_bin("offline-election").unwrap();
	cmd.args(&["--uri", TEST_URI, "council"]).unwrap();
}

#[test]
#[ignore = "requires unsafe RPC"]
fn dangling_works() {
	let mut cmd = Command::cargo_bin("offline-election").unwrap();
	cmd.args(&["--uri", TEST_URI, "dangling-nominators"])
		.unwrap();
}

#[test]
#[ignore = "requires unsafe RPC"]
fn nominator_check_works() {
	let mut cmd = Command::cargo_bin("offline-election").unwrap();
	let transport =
		async_std::task::block_on(jsonrpsee::transport::ws::WsTransportClient::new(TEST_URI))
			.expect("Failed to connect to client");
	let client: jsonrpsee::Client = jsonrpsee::raw::RawClient::new(transport).into();

	// get the latest block hash
	let head = async_std::task::block_on(sub_storage::get_head(&client));
	let version = async_std::task::block_on(sub_storage::get_runtime_version(&client, head));

	// some totally random account.
	cmd.args(&[
		"--uri",
		TEST_URI,
		"nominator-check",
		"--who",
		if version.spec_name == "kusama".into() {
			"Hph4pHAqDVVdc3vLani7DfQA2TU3FfuuUcBQC8tYbWgBTnC"
		} else if version.spec_name == "polkadot".into() {
			"13Vka4qGSStrNoFZap9qryQCbubfjDVyeradJwU2BG7TxZir"
		} else {
			panic!("unsupported chain.")
		},
	])
	.unwrap();
}
