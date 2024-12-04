# What Am I Learning Each Day?

### Day 3

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: 0.5 hrs**

**Run Time: 4ms**

I had to install `regex` again.

This was my first time using `captures_iter` or `captures`, and I'm not sure if there are better methods to do what I did with the data parser.

The second part was pretty simple to add another regex and `match` against which pattern was captured:

```rust
// track mul's initially
let mut track = true;

for a in re.captures_iter(data) {
    let (_full, [capture]) = a.extract();

    match capture {
        "do()" => {
            track = true;
        }
        "don't()" => {
            track = false;
        }
        _ => {
            // mul(digit, digit)
            if track {
```

Then I used `fold`, which I think is appropriate since `reduce` required the input data to be identical to the output data, and I was converting `(usize, usize)` to `usize`.

### Day 2

**Difficulty: 5/10 ★★★★★☆☆☆☆☆**

**Time: 2 hrs**

**Run Time: 2ms**

I got bogged down with both parts today, because the test data passed and the real data failed (both times).  The first time, it was because I checked the first two digits to determine the direction, before I tested for valid differences:

```diff
- if diff == 0 {
+ if diff == 0 || diff.abs() > 3 {
    return Some(1);
}
```

I ended up reworking it to re-iterate all of the numbers after getting the direction (up/down).

My solution to part 2 was to try to validate the report, and if it failed, return the index it failed on, then remove that index and try again.  I followed up a second time by removing the index before the failed one.  Neither perfectly worked, so I ended up trying every index possible, and it ran perfectly fine in under 2ms anyway.

Alright, so I ran both the correct algorithm and the bad one, and compared outputs, and found the problem:

```
47 45 46 47 49
```

This one would fail on index 2, my algorithm would try to remove it, then remove the one before it.  It needed to remove the one before that (the first one).  This failed because I believed I needed to determine the direction before testing, so I treated the first two values as a special case.  So the failed cases were the ones where I got the initial direction incorrect.

I had a difficult time cloning for some reason:

```rust
// SUPER ULTRA LAZY just iterate everything...
for i in 0..x.len() {
    // WOW: rust is difficult to fight with
    let clone: Vec<isize> = x
        .iter()
        .enumerate()
        .filter_map(|(j, v)| {
            if i == j {
                return None;
            }
            Some(*v)
        })
        .collect();
}
```

I wasn't able to just plain clone it because then I needed to mutate it to remove the value I wanted removed, and I wasn't sure how to do that without making the `x` variable mutable, which was a huge pain.

I found the general logic a bit awkward:

```rust
.filter_map(|x| {
    if has_issues(x).is_some() {
        return None;
    }
    Some(x)
})
```

I'd love to be able to just flip `None` to `Some` in that case, but that's the best I could come up with.  I also didn't need to return the `x` vector, but I liked the `filter_map` syntax better than `filter`.

Actually, testing that, it works fine (ignoring the type being `&&Vec<isize>`)

```rust
.filter(|x| { has_issues(x).is_none() })
```

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
