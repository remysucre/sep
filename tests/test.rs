use csr::*;
use egg::*;

// NOTE: if e_1 => e_2 doesn't work, try e_2 => e_1

test_fn! {
    // a_c_eq_1 is just a unique name
    a_c_eq_1, axioms(), "(* a)" => "1"
}

test_fn! {
    one_p_1_eq_1, axioms(), "(+ 1 1)" => "1"
}

test_fn! {
    a_p_a_eq_a, axioms(), "(+ a a)" => "a"
}

test_fn! {
    simple_4, axioms(), "(+ a (. a b))" => "(+ a (. b a))"
}

test_fn! {
    simple_5, axioms(), "(+ a (. a b))" => "a"
}

test_fn! {
    simple_6, axioms(), "(+ (. a b) (. a (. c b)))" => "(. a b)"
}

test_fn! {
    simple_7, axioms(), "(. 0 a)" => "(. a 0)"
}

test_fn! {
    simple_8, axioms(), "(. 0 a)" => "0"
}
