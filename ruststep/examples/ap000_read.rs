use ruststep::*;

const STEP_INPUT: &str = r#"
ISO-10303-21;
HEADER;
FILE_DESCRIPTION((''), '');
FILE_NAME('ruststep/examples/ap000/read.step', '2018-04-27T08:23:47', (''), (''), '', '', '');
FILE_SCHEMA(('AP000'));
ENDSEC;
DATA;
#1 = A(1.0, 2.0);
#2 = B(3.0, #1);
#3 = B(3.0, A((4.0, 5.0)));
#4 = C(#1, #2);
#5 = C(#1, #3);
#6 = C(#1, B((6.0, #1)));
#7 = C(#1, B((6.0, A((7.0, 8.0)))));
#8 = C(A((9.0, 10.0)), #2);
#9 = C(A((11.0, 12.0)), #3);
ENDSEC;
END-ISO-10303-21;
"#;

fn main() -> anyhow::Result<()> {
    let step = parser::parse(STEP_INPUT.trim())?;
    assert_eq!(step.data.len(), 1);

    let table = ap000::Ap000::from_section(&step.data[0])?;
    for c in table.c_iter() {
        println!("C = {:?}", c?);
    }
    Ok(())
}
