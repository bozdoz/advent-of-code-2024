#!/bin/bash

NEW_DAY=$1

usage() {
    cat >&2 <<END_USAGE

Create a new boilerplate directory

USAGE:
    ./create-day.sh 01
END_USAGE
}

if [ -z $NEW_DAY ]; then
  echo "Provide ## for new day directory"
	usage
  exit 1
fi

# get session cookie; make sure you create this with:
# COOKIE="Cookie: session=some-cookie-here"
source ./session.sh

cargo new "day-${NEW_DAY}" || usage

cd "day-${NEW_DAY}"

cargo add --path ../lib

# create input files for testing and solving
touch ./src/example.txt
curl "https://adventofcode.com/2024/day/${NEW_DAY}/input" --compressed -H "${COOKIE}" > ./src/input.txt

touch ./src/input.txt

cat > src/main.rs <<EOF
#![allow(unused)]

use std::{ time::Instant, fs };
use lib::get_part;

fn part_one() -> usize {
    0
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    if one {
        let now = Instant::now();
        let ans = part_one();
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two();
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let ans = part_one();

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}
EOF
