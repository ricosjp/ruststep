use nom::Finish;
use ruststep::{ast::*, parser::exchange};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
enum MagicaQuartet {
    MamiTomoe,
    MadokaKaname,
    SayakaMiki,
    KyokoSakura,
    HomuraAkemi,
}

#[test]
fn enum_deserialize() {
    let (residual, p): (_, Parameter) = exchange::parameter(".HOMURA_AKEMI.").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    assert_eq!(p, Parameter::Enumeration("HOMURA_AKEMI".to_string()));

    let a: MagicaQuartet = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, MagicaQuartet::HomuraAkemi);
}
