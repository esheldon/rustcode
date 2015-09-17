fn main() {
    let mut x = vec!["Hello", "world"];

    // rust has move symantics by default, so we need
    // to use clone() to get a copy. If we didn't copy 
    // then the push would be a compile error
    let y = &x[0].clone();

    x.push("foo");

    // this will get a reference that lives within the scope
    {
        let z=&x[1];
    }

    x.push("stuff");
}
