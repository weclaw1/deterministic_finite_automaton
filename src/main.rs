extern crate serde;
extern crate serde_json;

extern crate deterministic_finite_automaton;

use std::fs::File;
use std::io::prelude::*;

use std::env;

use deterministic_finite_automaton::DeterministicFiniteAutomaton;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut json = String::new();

    let mut file = File::open(&args[1]).expect("Couldn't open file!");
    file.read_to_string(&mut json).expect("Unable to read string");
    println!("{}", json);
    let dfa: DeterministicFiniteAutomaton = serde_json::from_str(&json).expect("Json is not valid");

    match dfa.correct() {
        true => {
            let result = dfa.run("abab");
            println!("{:?}", result);
            if dfa.state_accepted(&result) {
                println!("State is accepted!");
            } else {
                println!("State is not accepted!");
            }
        },
        false => println!("DFA is not correct"),
    }
}
