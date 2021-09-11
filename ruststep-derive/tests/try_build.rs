#[test]
fn try_build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/use_place_holder.rs");
    t.pass("tests/cases/optional.rs");
    t.pass("tests/cases/vec.rs");
    t.pass("tests/cases/select.rs");
}
