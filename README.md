# AdventOfCode2020

[Amazingly written intro to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
```

> That little, so-important borrow operator & is coercing the vector into a slice. And it makes complete sense, because the vector is also looking after an array of values, with the difference that the array is allocated dynamically.

> If you come from a dynamic language, now is time for that little talk. In systems languages, program memory comes in two kinds: the stack and the heap. It is very fast to allocate data on the stack, but the stack is limited; typically of the order of megabytes. The heap can be gigabytes, but allocating is relatively expensive, and such memory must be freed later. In so-called 'managed' languages (like Java, Go and the so-called 'scripting' languages) these details are hidden from you by that convenient municipal utility called the garbage collector. Once the system is sure that data is no longer referenced by other data, it goes back into the pool of available memory.

<3