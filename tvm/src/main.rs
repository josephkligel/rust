pub mod tvm;

fn main() {
    let example = tvm::TVM {
        i: Some(0.1),
        n: Some(12.0),
        t: Some(1.0),
        fv: Some(100.0),
        pv: None,
    };

    example.fetch();
}
