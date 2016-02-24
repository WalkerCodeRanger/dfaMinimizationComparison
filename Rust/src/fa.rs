use std::collections::HashSet;
use min::{Partition, PartitionMarks, PartitionMarking};
use std::iter::Iterator;

pub struct DFA
{
	state_count: usize,
	start_state: usize,
	transitions: Vec<Transition>,
	final_states: HashSet<usize>
}

#[derive(Clone)]
pub struct Transition
{
	pub from: usize,
	pub on_input: i32,
	pub to: usize
}

impl DFA
{
	pub fn new(state_count: usize, start_state: usize) -> DFA
	{
		DFA {state_count: state_count, start_state: start_state, transitions: Vec::new(), final_states: HashSet::new()}
	}

	pub fn state_count(&self) -> usize
	{
		self.state_count
	}

	pub fn start_state(&self) -> usize
	{
		self.start_state
	}

	pub fn add_transition(&mut self, from_state: usize, on_input: i32, to_state: usize)
	{
		self.transitions.push(Transition {from: from_state, on_input: on_input, to: to_state});
	}

	pub fn add_final_state(&mut self, state: usize) -> bool
	{
		self.final_states.insert(state)
	}

	pub fn transitions(&self) -> &Vec<Transition>
	{
		&self.transitions
	}

	pub fn final_states(&self) -> &HashSet<usize>
	{
		&self.final_states
	}

	pub fn minimize(&self) -> DFA
	{
		let mut transitions = self.transitions.to_vec();

		// Shared/reused data structures
		let mut marks = PartitionMarks::new(self.transitions.len()+1);
		let mut adjacent_transitions = AdjacentTransitions::new(self.state_count, self.transitions.len());

		let mut blocks = Partition::new(self.state_count);

		{
			let mut block_marking = blocks.begin_marking(&mut marks);

			// Reachable from start
			block_marking.mark(self.start_state);

			self.discard_not_reachable(&mut block_marking, &mut adjacent_transitions, &mut transitions, get_from, get_to);

			// Reachable from final
			for &final_state in &self.final_states
			{
				block_marking.mark(final_state);
			}

			self.discard_not_reachable(&mut block_marking, &mut adjacent_transitions, &mut transitions, get_to, get_from);

			// Split final states from non-final
			for &final_state in &self.final_states
			{
				block_marking.mark(final_state);
			}

			block_marking.split_sets();
		}

		// Cords partition to manage transitions
		let mut cords = Partition::new(transitions.len());

		{
			let mut cord_marking = cords.begin_marking(&mut marks);

			// Split transitions by input
			cord_marking.partition_by(|&transition| transitions[transition].on_input);
		}

		//Split blocks and cords
		adjacent_transitions.build_adjacency(self.state_count, &transitions, get_to);
		let mut block_set = 1;
		for cord_set in 0..cords.set_count()
		{
			{
				let mut block_marking = blocks.begin_marking(&mut marks);
				for transition in cords.set(cord_set)
				{
					block_marking.mark(transitions[transition].from);
				}
				block_marking.split_sets();
			}

			let mut cord_marking = cords.begin_marking(&mut marks);

			while block_set < blocks.set_count()
			{
				for state in blocks.set(block_set)
				{
					for transition in adjacent_transitions.of(state)
					{
						cord_marking.mark(transition);
					}
				}
				cord_marking.split_sets();
				block_set += 1;
			}
		}

		// Generate minimized DFA
		let mut min_dfa = DFA::new(blocks.set_count(), blocks.set_of(self.start_state).unwrap());

		// Set Final States
		for &final_state in &self.final_states
		{
			// not all final states may have been reachable
			if let Some(set) = blocks.set_of(final_state)
			{
				min_dfa.add_final_state(set);
			}
		}

		// Create transitions
		for set in 0..cords.set_count()
		{
			let transition = &transitions[cords.some_element_of(set)];
			let from = blocks.set_of(transition.from).unwrap();
			let to = blocks.set_of(transition.to).unwrap();
			min_dfa.add_transition(from, transition.on_input, to);
		}

		return min_dfa;
	}

	fn discard_not_reachable(&self,
		block_marking: &mut PartitionMarking,
		adjacent_transitions: &mut AdjacentTransitions,
		transitions: &mut Vec<Transition>,
		get_from: fn(&Transition) -> usize,
		get_to: fn(&Transition) -> usize
		)
	{
		adjacent_transitions.build_adjacency(self.state_count, transitions, get_from);

		// This section of code has been a real headache, because the algo wants
		// to modify the collection while iterating it.  In this version we have actually
		// created an iterator that has a method to safely mutable the collection.  However,
		// to access the iterator object we need to switch to a while let loop instead of
		// a for loop.  The alternative is keeping some local collection instead. See below
		// for that version.
		{
			let mut marked = block_marking.marked_in_set_mut(0);
			while let Some(state) = marked.next()
			{
				for transition in adjacent_transitions.of(state)
				{
					marked.mark(get_to(&transitions[transition]));
				}
			}
		}

		// This is an alternate version of the above code that does not rely on the ability to mutate the
		// block_marking while iterating it.  But we still need to mutate the states vector while looping
		// through its contents, so we still end up with a while loop.
		// This loop is infinite! because we keep going through the states repeatedly!
		/*
		let mut states: Vec<usize> = block_marking.marked_in_set(0).collect(); // TODO why is `Vec<usize>` needed here?
		while let Some(state) = states.pop()
		{
			for transition in adjacent_transitions.of(state)
			{
				let state = get_to(&transitions[transition]);
				if !block_marking.is_marked(state)
				{
					block_marking.mark(state);
					states.push(state);
				}
			}
		}
		*/

		block_marking.discard_unmarked();

		transitions.retain(|transition| block_marking.partition().set_of(get_from(&transition)).is_some());
	}
}

fn get_from(t: &Transition) -> usize
{
	t.from
}

fn get_to(t: &Transition) -> usize
{
	t.to
}

struct AdjacentTransitions
{
	adjacent: Vec<usize>, // transitions grouped by state they are adjacent to
	offset: Vec<usize> // offsets into adjacent list for a given state
}

impl AdjacentTransitions
{
	fn new(state_count: usize, transition_count: usize) -> AdjacentTransitions
	{
		AdjacentTransitions { adjacent: Vec::with_capacity(transition_count), offset: Vec::with_capacity(state_count+1) }
	}

	fn build_adjacency<F>(&mut self, state_count: usize, transitions: &Vec<Transition>, mut get_state: F)
		where F : FnMut(&Transition) -> usize
	{
		// initialize offset to zeros
		self.offset.clear();
		self.offset.resize(state_count+1, 0);

		// Count transitions per state
		for transition in transitions
		{
			self.offset[get_state(transition)] += 1;
		}

		// Running sum
		for state in 0..state_count
		{
			self.offset[state+1] += self.offset[state];
		}

		// Place transitions, and correct offsets
		self.adjacent.resize(transitions.len(), 0);
		for (i, transition) in transitions.iter().enumerate()
		{
			// TODO couldn't modify value and read in one line
			let state = get_state(transition);
			let transition = self.offset[state]-1;
			self.offset[state] = transition;
			self.adjacent[transition] = i;
		}
	}

	// TODO have to use box to return abstract iterator
	fn of<'a>(&'a self, state: usize) -> Box<Iterator<Item=usize> + 'a>
	{
		return Box::new((self.offset[state]..self.offset[state+1]).map(move |i| self.adjacent[i]));
	}
}