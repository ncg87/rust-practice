// Initialize heap_var to store value 4 on the heap & make the code execute successfully.

fn main() {
    let stack_var = 5;
    // TODO: initialize this variable
    let heap_var = Box::new(4); // initialize the heap_var to store value 4 on the heap, and a pointer to it on the stack
    let res = stack_var + *heap_var; // must dereference the pointer to access the value
    assert_eq!(res, 9);
}
