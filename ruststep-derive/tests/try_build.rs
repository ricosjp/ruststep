#[test]
fn try_build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/use_place_holder.rs");
    t.pass("tests/cases/optional.rs");
    // not implemented yet
    t.compile_fail("tests/cases/vec.rs");
}
