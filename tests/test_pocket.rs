use std::sync::Arc;

use servidor_http::request::Pocket;

#[derive(PartialEq, Debug, Clone)]
struct Complex {
    i: usize,
    j: usize,
}

#[test]
fn pocket_test() {
    let mut pocket = Pocket::new();

    let ex_complex = Complex{i: 2, j: 3};

    pocket.insert(ex_complex.clone());

    let ex_complex_cpy_arc: Arc<Complex> = pocket.get().unwrap();

    let ex_complex_cpy = Complex{
        i: ex_complex_cpy_arc.i,
        j: ex_complex_cpy_arc.j
    };

    assert_eq!(ex_complex, ex_complex_cpy);
}
