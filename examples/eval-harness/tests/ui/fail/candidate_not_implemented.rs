// Must NOT compile: a candidate that doesn't implement `Candidate`
// fails at the trait bound, with an error naming the contract — instead
// of a confusing "method not found" or a silently unrunnable suite.

use eval_harness::Candidate;

struct Wrong;

fn main() {
    let _ = Wrong.parse_u32_lenient("1");
}
