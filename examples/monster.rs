//! A monster, patrolling his dungeon, walking back and forth.
//!
//! The idea for this example was taken from Bob Nystrom's
//! [blog post on fibers][1].
//!
//! [1]: (https://journal.stuffwithstuff.com/2010/07/13/fibers-coroutines-in-finch/).

struct Monster {
    min_pos:     i64,
    max_pos:     i64,
    wait_frames: u64,
}

/// Wait for a specified number of frames
struct Wait<'a> {
    monster: &'a Monster,
    elapsed: u64,
}

struct Walk<'a> {
    monster: &'a Monster,
    pos:     i64,
}

fn main() {}
