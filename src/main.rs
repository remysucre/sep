use egg::{rewrite as rw, *};

define_language! {
    pub enum L {
        "+" = Add([Id; 2]),
        Var(Symbol),
    }
}

pub fn axioms() -> Vec<Rewrite<L, ()>> {
    rw!("+-idem"; "(+ ?a ?a)" <=> "?a")
}

fn main() {
    Runner::default()
        .with_expr(&"a".parse().unwrap())
        .run(&axioms())
        .print_report();
}
