/// `cargo test print_candid -- --nocapture`

#[test]
fn print_candid() {
    use std::io::Write;

    use candid::Principal;
    candid::export_service!();

    let filename = "candid.did";
    std::fs::remove_file(filename).unwrap();
    std::fs::File::create(&filename)
        .unwrap()
        .write_all(__export_service().as_bytes())
        .unwrap();
}
