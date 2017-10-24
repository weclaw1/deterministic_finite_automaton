#[macro_use]
extern crate serde_derive;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(pub char);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateChange {
    pub state: State,
    pub symbol: Symbol,
    pub new_state: State,
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> State {
        State(s.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeterministicFiniteAutomaton {
    alphabet: Vec<Symbol>,
    states: Vec<State>,
    accepted_states: Vec<State>,
    start_state: State,
    state_transitions: Vec<StateChange>,
}

impl DeterministicFiniteAutomaton {
    pub fn new(alphabet: Vec<Symbol>, states: Vec<State>, accepted_states: Vec<State>, start_state: State, state_transitions: Vec<StateChange>)
                -> DeterministicFiniteAutomaton {
        let dfa = DeterministicFiniteAutomaton {
            alphabet, states, accepted_states, start_state, state_transitions
        };

        if !dfa.correct() {
            panic!("Incorrect DFS!");
        }

        dfa
    }

    pub fn run(&self, word: &str) -> State {
        let mut current_state = &self.start_state;
        for symbol in word.chars() {
            let state_transition = &self.state_transitions.iter().find(|x|&x.state == current_state && x.symbol.0 == symbol).expect("State transitions is missing a state");
            current_state = &state_transition.new_state;
        }
        return current_state.clone();
    }

    pub fn correct(&self) -> bool {
        if !(self.state_transitions.len() == self.alphabet.len() * self.states.len()) {
            return false
        }
        true
    }

    pub fn state_accepted(&self, state: &State) -> bool {
        if self.accepted_states.contains(state) {
            return true
        }
        false
    }
}
