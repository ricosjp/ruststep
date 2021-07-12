#[test]
fn try_build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/use_place_holder.rs");
}
