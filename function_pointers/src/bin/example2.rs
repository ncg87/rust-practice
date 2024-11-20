// Fix the code by addressing the TODO.

fn invoker<T, U>(logic: U, arg: T)
where U: Fn(T) {
    logic(arg);
}

fn main() {
    // TODO: shift below declaration to somewhere else
    let greeting = String::from("Nice to meet you");
    let greet = |name| {
        println!("{greeting} {name}!");
    };
    invoker(greet, "Jenny");
}
