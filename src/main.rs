mod eval;

use eval::Eval;

fn main() {
    let mut expr = Eval::new("(+ (* 2 (- 6 3)))".to_string());
    let res = expr.eval();
    dbg!(res);
}
