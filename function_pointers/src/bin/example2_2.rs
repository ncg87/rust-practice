fn invoker<T>(logic: fn(T), arg: T) {
    logic(arg);
}

fn main() {
    let greet = |name| {
        // this need to be declared in the closure since function pointers cannot capture values from the environment/scope
        let greeting = String::from("Nice to meet you");
        println!("{greeting} {name}!");
    };
    invoker(greet, "Jenny");
}
