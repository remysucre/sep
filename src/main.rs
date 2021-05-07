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
        .with_hook(|_| {
            println!("iter");
            Ok(())
        })
        .print_report()
}

// fn main() {
//     let a = "a".parse().unwrap();
//     let mut i = 0;
//     Runner::default()
//         .with_expr(&a)
//         .with_hook(move |runner| {
//             i += 1;
//             runner.egraph
//                   .dot()
//                   .to_png(format!("{}.png", i))
//                   .unwrap();
//             Ok(())
//         })
//         .run(&axioms())
//         .egraph
//         .dot()
//         .to_png("end.png")
//         .unwrap();
// }

