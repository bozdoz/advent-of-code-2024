# What Am I Learning Each Day?

### Day 8

**Difficulty: 2/10 ★★☆☆☆☆☆☆☆☆**

**Time: 1 hrs**

**Run Time: 500µs**

I took the time to put the Grid struct into the `lib.rs` for reuse.  Today was another easy day.  Might have been my first time using the HashMap `and_modify().or_insert()` methods:

```rust
antennas
    .entry(cell)
    .and_modify(|e: &mut Vec<(isize, isize)>| {
        e.push((r as isize, c as isize));
    })
    .or_insert(vec![(r as isize, c as isize)]);
```

Today all it was was getting the difference between every two antenna and extrapolating it further in each direction.  In the second part you also have to include the antennas, which means keeping a `HashSet`.

### Day 7

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: 0.5 hrs**

**Run Time: 76ms**

I think this is the first time I defined a closure as a function parameter:

```rust
fn is_truthy(&self, get_next: impl Fn(&mut Vec<(usize, usize)>, (usize, usize)) -> ()) -> bool
```

Which was used to deduplicate the stack (I think depth-first search) logic:

```rust
if
    eq.is_truthy(|states: &mut Vec<(usize, usize)>, next: (usize, usize)| {
        // same as part one
        let acc = next.0;
        let i = next.1;
        let num = eq.numbers[i];
        states.push((acc + num, i + 1));
        states.push((acc * num, i + 1));

        // adds concatenated to part two
        let concatenated = acc * (10usize).pow(num.ilog10() + 1) + num;
        states.push((concatenated, i + 1));
    })
```

That concatenated line I got from reddit, but made sense to me for digit counting.

Today felt like a very idiomatic rust day.

### Day 6

**Difficulty: 6/10 ★★★★★★☆☆☆☆**

**Time: 4 hrs**

**Run Time: 175ms**

Reused a lot of grid stuff from Day4.  I might add to a library for it to deduplicate.

I thought I would be able to define a variable with the output of a block, but I wasn't able to `return` in it:

```rust
let start = {
    let mut out = (0, 0);
    'stupid: for (r, row) in cells.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '^' {
                out = (r as isize, c as isize);

                // not able to return in a block
                break 'stupid;
            }
        }
    }
    out
};
```

Better to rewrite it without the block anyway I guess, with a `find_map`.

I did a bunch of craziness with directions, binaries, and modulus:

```rust
// (r, c) differences, clockwise
const DIRS: &'static [(isize, isize)] = &[
    (-1, 0), // top
    (0, 1), // right
    (1, 0), // bottom
    (0, -1), // left
];

let bin = (2usize).pow((d % 4) as u32);
let dir = DIRS[d % 4];
```

The binary was to add multiple directions to an individual visited cell; the modulus was to continue iterating the direction constant.

I basically copied the same loop three times, but didn't want to deduplicate any of it.  

I basically:

1. looped the directions
2. looped getting the next cell towards a given direction
3. checked if we hit an obstacle or went off the map

Part two is kind of nested, where for each cell we run the simulation as if there's an obstacle there, then keep traversing.

Loop detection was simply: 


```rust
let mut visited = vec![vec![0; self.width as usize]; self.height as usize];

// first time using mutable reference?
let cell = &mut visited[next.0 as usize][next.1 as usize];

// check if the cell has the direction we're currently moving in
if (*cell & bin) == bin {
    return true;
}

// add direction to the visited list
*cell |= bin;
```

This I thought was convenient, since I only had to access the 2d array once, as a mutable reference.

I kept a hashset of visited cells because each cell could either be an obstacle or not: If it was visited previously it was ignored regardless.

### Day 5

**Difficulty: 4/10 ★★★★☆☆☆☆☆☆**

**Time: 1 hrs**

**Run Time: 134ms**

Both parts had a similar logic where I searched for the second number and then searched after it to see if the first was incorrectly placed.

During parsing I ran into annoying lifetime errors, but I seemed to be able to solve them after some DuckDuckGo searches, and used `where` I think for the first time:

```rust
struct SafetyManual<'a> {
    rules: Vec<(&'a str, &'a str)>,
    pages: Vec<Vec<&'a str>>,
}

impl<'a> SafetyManual<'a> {
    // first where; I don't understand it
    fn new<'b>(data: &'b str) -> Self where 'b: 'a {
```

I think there were too many references going on and maybe it didn't know which was which, but I think this means that the output lives as long as the input?

Without it it complains: 

> explicit lifetime required in the type of `data`

I decided not to parse the numbers and leave them as `&str` and maybe that was more costly than just parsing them immediately.  

I looked up a bubble sort in rust, and used that to re-order the pages based on the rules.

I've started adding reference symbols in the variable declaration instead of where I need them, and today I think was my first time using `swap`:

```rust
'outer: for &(first, second) in manual.rules.iter() {
    for i in 0..page.len() {
        if page[i] == first {
            continue 'outer;
        }
        if page[i] == second {
            // look for first and swap
            for j in i..page.len() {
                if page[j] == first {
                    page.swap(i, j);
```

It's a lot of loops and conditions, but it runs quick enough at `134ms`.


### Day 4

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: 0.5 hrs**

**Run Time: 2ms**

I did today completely on [Rust Playground](https://play.rust-lang.org/).  It seemed very straight forward, especially given I remembered the issues I had last year with Grid/Cell iteration `Vec<Vec<_>>`.  I did it all within a single main function, without tests, then copied them over here for source control, adding tests, and splitting out functions and `struct`'s.  It did make me think I could save some amount of time by sticking to simple functions, but I'd rather do things that might better represent a real world app.

One thing I noticed is that when I moved to my typical structure, I couldn't easily create the struct I needed for the grid.

```rust
impl Grid {
    fn new(data: &str) -> Self {
        let cells: Vec<_> = data
            .lines()
            .map(|l| { l.chars().collect::<Vec<_>>() })
            .collect();

        // surprisingly not valid if added directly to the `Self` block below
        let height = cells.len() as isize;
        let width = cells[0].len() as isize;

        Self {
            cells,
            height,
            width,
        }
    }
}
```

For some reason I think related to lifetimes, I couldn't do this:

```rust
Self {
    cells,
    height: cells.len() as isize,
    width: cells[0].len() as isize,
}
```

The error is:

> borrow of moved value: `cells`

I thought the iterations today were clean: I found all `X`'s and then iterated directions repeatedly to see if all letters matched.

```rust
// (r, c) differences, clockwise
const DIRS: &'static [(isize, isize)] = &[
    (-1, 0), // top
    (-1, 1), // tr
    (0, 1), // right
    (1, 1), // br
    (1, 0), // bottom
    (1, -1), // bl
    (0, -1), // left
    (-1, -1), // tl
];
```

I find the `&'static` lifetime declaration a little annoying, but it works fine.

```rust
for dir in DIRS {
    let mut nextr = r;
    let mut nextc = c;

    for &ch in SEARCH {
        nextr += dir.0;
        nextc += dir.1;
```

Here I increment row and column and keep checking for the next letter in `SEARCH`.

Part 2 was similar thoough had some extra logic to determine if exactly `S` and `M` are matched around the `A`'s.

```rust
// looking for an X shape
const DIAGONALS: &'static [(isize, isize)] = &[
    (-1, -1), // tl
    (1, 1), // br
    (-1, 1), // tr
    (1, -1), // bl
];

for dirs in DIAGONALS.chunks(2) {
    let mut acceptable = vec!['S', 'M'];
    // ...
    // ...
    // ...
    let ch = &grid.cells[nextr as usize][nextc as usize];

    if acceptable.contains(ch) {
        // remove from acceptable and search next diagonal
        acceptable.retain(|x| x != ch);
    }
```

I think it's the first time I've used `chunks` and I'm not sure if it's the first time I've used `retain`, though I remember it's a difficult method to remember (because I'd prefer something like `remove`).

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
