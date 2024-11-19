// Think of macros more like sophisticated text replacement/code generation tools
//while closures are more like functions that can "remember" values from where they were created.

fn main() {
    
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: V) -> bool
where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}