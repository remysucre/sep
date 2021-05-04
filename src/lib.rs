use egg::{rewrite as rw, *};

define_language! {
    pub enum CSr {
        // binary ops
        "+" = Add([Id; 2]),
        "." = Sub([Id; 2]),

        // unary op (closure)
        "*" = Mul(Id),

        // nullary op (bottom)
        // "bot" = Bot,

        // constants & variables
        Num(i32),
        Symbol(Symbol),
    }
}

pub fn axioms() -> Vec<Rewrite<CSr, ()>> {
    let mut rules = vec![];

    // directed rewrite rules
    rules.extend(vec![
        rw!("simple-l"   ; "(+ ?a 1)"         => "1"                        ),
    ]);

    // bidirectional rewrite rules
    rules.extend(vec![
        rw!("+-assoc"    ; "(+ ?a (+ ?b ?c))" <=> "(+ (+ ?a ?b) ?c)"        ),
        rw!("+-comm"     ; "(+ ?a ?b)"        <=> "(+ ?b ?a)"               ),
        rw!("0-+-id"     ; "(+ ?a 0)"         <=> "?a"                      ),
        rw!(".-assoc"    ; "(. ?a (. ?b ?c))" <=> "(. (. ?a ?b) ?c)"        ),
        rw!("1-.-id-r"   ; "(. ?a 1)"         <=> "?a"                      ),
        rw!("1-.-id-l"   ; "(. 1 ?a)"         <=> "?a"                      ),
        rw!(".-+-dist-l" ; "(. ?a (+ ?b ?c))" <=> "(+ (. ?a ?b) (. ?a ?c))" ),
        rw!(".-+-dist-r" ; "(. (+ ?b ?c) ?a)" <=> "(+ (. ?b ?a) (. ?c ?a))" ),
        rw!("closure-l"  ; "(* ?a)"           <=> "(+ 1 (. (* ?a) ?a))"     ),
        rw!("closure-r"  ; "(* ?a)"           <=> "(+ 1 (. ?a (* ?a)))"     ),
    ].concat());
    rules
}
