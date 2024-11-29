# What Am I Learning Each Day?

### Day 1

**Difficulty: x/10 ★★★★☆☆☆☆☆☆**

**Time: x hrs**

**Run Time: 0ms**

Part one I did by creating an array, then pushing to it from another array; breaking a rule I usually have where you should just set array **B** to a map result of array **A**. I probably did this because this is my first rust script, and I still have no idea what I'm doing.

Rust analyzer and ChatGPT are helping me understand many of the errors.

I made a typical, foolish mistake of typing the numbers as `u8` originally instead of `u16` or even just `usize`, thinking it would matter at all to be concerned about optimizing this script.

TIL about `RUST_BACKTRACE=1` as I immediately ran into overflow issues with using a `u8` for summing.

Had to add a library already. 🤦‍♂️

I added the regex library to match a string value of number:

```rust
let re: Regex = Regex::new(
    r"(one|two|three|four|five|six|seven|eight|nine|\d)"
).unwrap();
```

Though maybe this is what slowed down part two so intensely:

```sh
Part one: 56108 145.58µs
Part two: 55652 285.793102ms
```

Got to use my first `Some` destructuring with an `if let`:

```rust
if let Some(first) = re.find(s) { ... }
```

I had tried doing a `replace_all`, but it was much more difficult, and included rust's [Cow enum](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

I wrote my first test, and found out that rust's test output is awful:

```sh
running 1 test
test tests::test_number_parser ... FAILED

failures:

---- tests::test_number_parser stdout ----
"oneight" "1ight"
thread 'tests::test_number_parser' panicked at src/main.rs:108:9:
assertion `left == right` failed
  left: 11
 right: 18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_number_parser

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin day-01`
```

All I wanted there was `11 is not 18`; or something like that. Just far too verbose.

So one of the big issues I ran into with `replace_all` was dealing with overlapping matches, where `replace_all` states: 

> Replaces all **non-overlapping** matches in the haystack with the replacement provided. This is the same as calling replacen with limit set to 0.

A better method should have been to just iterate from the beginning until I find a number, then iterate from the end until I find a number. And stop checking the string afterward.

I ran into more issues with overflow, trying to use a `while` loop:

```rust
// get last match
let mut i: usize = s.len() - 1;
while 0 <= i {
```

I got: 

> comparison is useless due to type limits `#[warn(unused_comparisons)]` on by default

I could change the type of `usize`, or just use a `for` loop over a range instead:

```rust
// get last match
for i in (0..=s.len() - 1).rev() {
    if let Some(last) = re.find(&s[i..]) {
        two[1] = replace_numbers(last.as_str());
        break;
    }
}
```

Also a big issue was not being able to iterate over a `&Vec<&str>`, to return a `Vec<&str>`.  I had to create a `Vec<String>` first, then iterate again and map again to a `&str`; then call `collect()` to get a `Vec<&str>`.  Otherwise, the `map` variable was considered a temporary variable, and I couldn't save a reference to a temporary variable.  So, I'm doing something that's not idiomatic.

I made a boiler plate to selectively run parts.  And I might update it to add the timings.

Overall, it was a fine day, but a little over-complicated for day 1.

I think rust may have a better built-in file parser than `go`.  fs read_to_string with a `.lines()` just seems simple.  Minor downside to also have to add `.expect()` because it could be an `Error`, and `.collect()`, because it just seems more difficult to borrow iterables.
