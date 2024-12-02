# What Am I Learning Each Day?

### Day 1

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: 1 hrs**

**Run Time: 3ms**

I completely forgot rust apparently and couldn't figure out how to execute `s.lines().map(|x| { dbg!(x) })`, and I'm not sure why.  I had to debug all the variables around it, and instead just do a `for` loop.

I might have used codeium too much, and I think I'll disable it unless I am really fighting with the compiler.

I think the only weird thing I did was to subtract the largest from the smallest, where I ran `max` and `min` on both twice:

```rust
for i in 0..first.len() {
    let diff = first[i].max(second[i]) - first[i].min(second[i]);

    sum += diff;
}
```

I'm sure there's a better way to write that, and there may even be a simple method for extracting this.
