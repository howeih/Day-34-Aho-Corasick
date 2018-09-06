use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    state_id: usize,
    transition: HashMap<char, usize>,
}

#[derive(Debug)]
struct AhoCorasick {
    states: Vec<State>,
    failure: HashMap<usize, usize>,
    level: HashMap<usize, Vec<usize>>,
    max_level: usize,
    output: HashMap<usize, HashSet<String>>,
}

impl AhoCorasick {
    fn new() -> AhoCorasick {
        let init_state = State {
            state_id: 0,
            transition: HashMap::<char, usize>::new(),
        };
        let mut init_states = Vec::<State>::new();
        init_states.push(init_state);
        AhoCorasick {
            states: init_states,
            failure: HashMap::<usize, usize>::new(),
            level: HashMap::<usize, Vec<usize>>::new(),
            max_level: 0usize,
            output: HashMap::<usize, HashSet<String>>::new(),
        }
    }

    fn compute_failure(&mut self) {
        for l in 1..self.max_level {
            let level_states = self.level.get(&l).unwrap();
            if l == 1 {
                for s in level_states {
                    self.failure.insert(*s, 0);
                }
            } else {
                for d in self.level.get(&(l - 1)).unwrap() {
                    for s in self.level.get(&l).unwrap() {
                        if self.states[*d].transition.values().any(|x| x == s) {
                            let mut state = self.failure[d];
                            let mut failure = 0usize;
                            for t in self.states[state].transition.keys() {
                                if self.states[*d].transition.contains_key(t) {
                                    failure = self.states[state].transition[t];
                                    break;
                                }
                            }
                            self.failure.insert(*s, failure);
                        }
                    }
                }
            }
        }
        self.merge_output();
    }

    fn merge_output(&mut self) {
        for s in self.failure.keys() {
            if self.output.contains_key(s) {
                let failure_state = self.failure[s];
                if self.output.contains_key(&failure_state) {
                    for o in &self.output[&failure_state].clone() {
                        self.output.get_mut(s).unwrap().insert(o.clone());
                    }
                }
            }
        }
    }

    fn add_to_output(&mut self, state_id: &usize, keyword: &str) {
        if !self.output.contains_key(state_id) {
            self.output.insert(*state_id, HashSet::<String>::new());
        }
        self.output
            .get_mut(state_id)
            .unwrap()
            .insert(String::from(keyword));
    }

    fn add_keywords(&mut self, keyword: Vec<&str>) {
        for k in keyword {
            let mut current_state = 0usize;
            let mut max_level = 1usize;

            for c in k.chars() {
                if !self.states[current_state].transition.contains_key(&c) {
                    let state_id = self.states.len();
                    let new_state = State {
                        state_id: state_id,
                        transition: HashMap::<char, usize>::new(),
                    };
                    self.states.push(new_state);
                    self.states[current_state].transition.insert(c, state_id);
                    current_state = state_id;
                    if !self.level.contains_key(&max_level) {
                        self.level.insert(max_level, Vec::<usize>::new());
                    }
                    self.level.get_mut(&max_level).unwrap().push(state_id);
                } else {
                    current_state = *self.states[current_state].transition.get(&c).unwrap();
                }
                max_level += 1;
            }
            self.add_to_output(&current_state, k);
            self.max_level = max_level;
        }

        for c in 0..26u8 {
            let z = ('a' as u8 + c) as char;
            if !self.states[0].transition.contains_key(&z) {
                self.states[0].transition.insert(z, 0);
            }
        }
        self.compute_failure();
    }

    fn search(&self, text: &str) -> Vec<(usize, HashSet<String>)>{
        let mut state = 0usize;
        let mut search_result = Vec::<(usize, HashSet<String>)>::new();
        for (i, c) in text.chars().enumerate() {
            while !self.states[state].transition.contains_key(&c) {
                state = self.failure[&state];
            }
            state = *self.states[state].transition.get(&c).unwrap();
            if self.output.contains_key(&state) {
                search_result.push((i, self.output.get(&state).unwrap().clone()));
            }
        }
        search_result
    }
}

fn main() {
    let mut aho = AhoCorasick::new();
    aho.add_keywords(vec!["he", "she", "his", "hers"]);
    let result = aho.search("ahishers");
    for r in result{
        println!("{:?} found at: {}", r.1, r.0);
    }
}

