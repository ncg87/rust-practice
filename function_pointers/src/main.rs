// Think of macros more like sophisticated text replacement/code generation tools
//while closures are more like functions that can "remember" values from where they were created.

fn main() {
    let ten = 10;
    // first parameter is the closure in || then the function of the closure
    let greater_than = |x: &i32| *x > ten; // need to dereference x since x is a reference
    // we are pointing the less than function, works since functions implement all 3 closure traits
    let results = are_both_true(greater_than, less_than, &9);
    println!("{}", results);
}

// function pointers
fn less_than(x: &i32) -> bool {
    *x < 20
}
// Fn is a closure trait, fn is a function pointer
// function pointers cannot capture values from the environment/scope
// for more generability we use closure traits since they can be used with closures and functions
fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}