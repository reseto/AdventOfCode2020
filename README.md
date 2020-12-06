# AdventOfCode2020

This is the first time I'm trying to write Rust programs, have mercy.

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

> Generally, this is a price worth paying. Playing with the stack is terribly unsafe, because if you make one mistake you can override the return address of the current function, and you die an ignominious death or (worse) got pwned by some guy living in his Mom's basement in Minsk.

> The downsides of garbage collection? The first is that it is wasteful of memory, which matters in those small embedded microchips which increasingly rule our world. The second is that it will decide, at the worst possible time, that a clean up must happen now. (The Mom analogy is that she wants to clean your room when you are at a delicate stage with a new lover). Those embedded systems need to respond to things when they happen ('real-time') and can't tolerate unscheduled outbreaks of cleaning. Roberto Ierusalimschy, the chief designer of Lua (one of the most elegant dynamic languages ever) said that he would not like to fly on an airplane that relied on garbage-collected software.

<3