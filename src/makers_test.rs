use super::*;

#[test]
fn get_name_test() {
    let name = get_name();
    assert_eq!(name, "makers");
}
