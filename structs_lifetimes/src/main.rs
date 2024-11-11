// structs says that the lifetime of content is tied to the lifetime of the struct
struct Tweet <'a> {
    content: &'a str, // have to specify lifetime annotation for storing references in structs
}

// must specify lifetime annotation for the impl block, if they are in the struct
impl<'a> Tweet<'a> {
    // tweet must be mutable to change content
    fn replace_content(&mut self, new_content: &'a str) -> &str { // sp we don't have to explicitly define lifetime annotiation of the output value
        let old_content = self.content;
        self.content = new_content;
        old_content // by rule 3 lifetime of old_content is the same as the lifetime of self
    }
}

fn main() {
    let mut tweet = Tweet {
        content: "Hello, world!",
    };
    let old_content = tweet.replace_content("New content");
    println!("Old content: {}", old_content);
    println!("New content: {}", tweet.content);
}
// Lifetime Elision
// 1. Each parameter that is a reference gets its own lifetime
// 2. If there is exactly one input parameter, that lifetime is assigned to all output parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output parameters

fn take_and_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
    content // lifetime of content is the same as the lifetime of the input parameter content ignoring content2
}
