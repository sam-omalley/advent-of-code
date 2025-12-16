pub mod template;

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

//library!(util "Utility modules to handle common recurring Advent of Code patterns."
//ansi, bitset, grid, hash, heap, integer, iter, math, md5, parse, point, slice, thread
//);
library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi
);

//library!(year2015 "Help Santa by solving puzzles to fix the weather machine's snow function."
//day01, day02, day03, day04, day05
//);

//library!(year2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
//day01, day02
//);

library!(year2025 "Finish the North Pole decorations in time for Christmas."
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);
