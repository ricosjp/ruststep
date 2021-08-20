#[derive(Clone, Debug, PartialEq)]
enum BaseAny {
    Sub1(Sub1),
    Sub2(Sub2),
}

#[derive(Clone, Debug, PartialEq)]
struct Base {
    x: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct Sub1 {
    base: Base,
    y1: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct Sub2 {
    base: Base,
    y2: f64,
}
