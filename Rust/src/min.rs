use std::cmp::Ord;

pub struct Partition
{
	set_count: usize, // the number of sets
	elements: Vec<usize>, // the elements, shuffled around to be grouped by set
	location: Vec<usize>, // location of a given element in elements
	set_of: Vec<Option<usize>>, // the set of a given element
	first: Vec<usize>, // index in elements of first item in set
	past: Vec<usize> // index in elements of first item past items in set
}

pub struct PartitionMarks
{
	marked: Vec<usize>, // how many elements are marked in each set (those elements are at the beginning
	touched: Vec<usize> // which sets contain marked elements
}

pub struct PartitionMarking<'a>
{
	partition: &'a mut  Partition,
	marks: &'a mut PartitionMarks
}

// TODO why do we need two type params here?
pub struct PartitionMarkedSetIterator<'a, 'b>
	where 'a: 'b
{
	set: usize,
	partition_marking: &'b mut PartitionMarking<'a>,
	location: usize
}

impl Partition
{
	pub fn new(element_count: usize) -> Partition
	{
		let mut p = Partition
		{
			set_count: (element_count > 0) as usize,
			// TODO is this the best way to create these?
			// What about Vec::with_capacity(element_count).extend(0..element_count)?
			elements: (0..element_count).collect(),
			location: (0..element_count).collect(),
			set_of: vec![Some(0); element_count], // TODO would it be faster to use resize instead?
			first: Vec::with_capacity(element_count),
			past: Vec::with_capacity(element_count)
		};
		if element_count > 0
		{
			p.first.push(0);
			p.past.push(element_count);
		}
		return p;
	}

	pub fn set_count(&self) -> usize
	{
		self.set_count
	}

	pub fn set_of(&self, element: usize) -> Option<usize>
	{
		self.set_of[element]
	}

	// TODO have to use box to return abstract iterator
	pub fn set<'a>(&'a self, set: usize) -> Box<Iterator<Item=usize> + 'a>
	{
		Box::new((self.first[set]..self.past[set]).map(move |i| self.elements[i]))
	}

	pub fn some_element_of(&self, set: usize) -> usize
	{
		self.elements[self.first[set]]
	}

	pub fn begin_marking<'a>(&'a mut self, marks: &'a mut PartitionMarks) -> PartitionMarking
	{
		assert!(marks.touched.len() == 0);
		marks.marked.clear();
		marks.marked.resize(self.set_count, 0);
		PartitionMarking { partition: self, marks: marks }
	}
}

impl PartitionMarks
{
	pub fn new(element_count: usize) -> PartitionMarks
	{
		PartitionMarks {marked: Vec::with_capacity(element_count), touched: Vec::with_capacity(element_count)}
	}
}

impl<'a> PartitionMarking<'a>
{
	pub fn partition(&self) -> &Partition
	{
		self.partition
	}

	pub fn mark(&mut self, element: usize)
	{
		let p = &mut self.partition;
		let m = &mut self.marks;
		if let Some(set) = p.set_of[element]
		{
			let element_index = p.location[element];
			let first_unmarked = p.first[set] + m.marked[set];
			if element_index < first_unmarked // already marked
				{ return; }

			// swap element and the first unmarked in elements, updating location appropriately
			// (they are in the same set so no need to worry about that)
			p.elements[element_index] = p.elements[first_unmarked];
			p.location[p.elements[element_index]] = element_index;
			p.elements[first_unmarked] = element;
			p.location[element] = first_unmarked;

			// add to touched if needed and track the number of marked elements
			if m.marked[set] == 0
			{
				m.touched.push(set);
			}
			m.marked[set] += 1;
		}
	}

	// TODO have to use box to return abstract iterator
	pub fn marked_in_set<'b>(&'b self, set: usize) -> Box<Iterator<Item=usize> + 'b>
	{
		let first_of_set = self.partition.first[set];
		return Box::new((first_of_set..(first_of_set + self.marks.marked[set])).map(move |i| self.partition.elements[i]));
	}

	// The algorithm as written in C++ and C# relies on the fact that you can mark nodes and they
	// will be added to this the iterator returned by marked_in_set().  Of course, Rust borrow checker
	// makes that essentially impossible.  So here we have implemented our own iterator that that has
	// a mark method allowing us to safely mark elements while manually iterating.
	pub fn marked_in_set_mut<'b>(&'b mut self, set: usize) -> PartitionMarkedSetIterator<'a, 'b>
	{
		let location: usize = self.partition.first[set];
		return PartitionMarkedSetIterator { set: set, partition_marking: self, location: location };
	}

	/// Split each set into the marked and unmarked subsets
	pub fn split_sets(&mut self)
	{
		let p = &mut self.partition;
		let m = &mut self.marks;
		for &set in &m.touched // TODO I don't understand why &set makes sense?
		{
			let first_unmarked = p.first[set] + m.marked[set];

			// if the whole set was marked
			if first_unmarked == p.past[set]
			{
				// just unmark it and do nothing
				m.marked[set] = 0;
				continue;
			}

			// Make the smaller half a new set
			// If same size, then make a new set out of unmarked
			if m.marked[set] <= p.past[set] - first_unmarked
			{
				let first_of_set = p.first[set]; // TODO must be separated out for borrow checker
				p.first.push(first_of_set);
				p.first[set] = first_unmarked;
				p.past.push(first_unmarked);
			}
			else
			{
				let past_of_set = p.past[set]; // TODO must be separated out for borrow checker
				p.past.push(past_of_set);
				p.past[set] = first_unmarked;
				p.first.push(first_unmarked);
			}

			// mark the elements as members of the new set
			for i in p.first[p.set_count]..p.past[p.set_count]
			{
				p.set_of[p.elements[i]] = Some(p.set_count);
			}

			// clear marks on old and new set
			m.marked.push(0);
			m.marked[set] = 0;

			// increase set count
			p.set_count += 1;
		}
		m.touched.clear();
	}

	/// Discard the unmarked from each set, they are no longer in any set (None)
	pub fn discard_unmarked(&mut self)
	{
		let p = &mut self.partition;
		let m = &mut self.marks;
		for &set in &m.touched // TODO I don't understand why &set makes sense?
		{
			let first_unmarked = p.first[set] + m.marked[set];

			// if the whole set was marked
			if first_unmarked == p.past[set]
			{
				// just unmark it and do nothing
				m.marked[set] = 0;
				continue;
			}

			let past_unmarked = p.past[set];
			p.past[set] = first_unmarked;

			// mark the elements as members of no set
			for i in first_unmarked..past_unmarked
			{
				p.set_of[p.elements[i]] = None;
			}

			// clear mark on set
			m.marked[set] = 0;
		}
		m.touched.clear();
	}

	pub fn partition_by<F, P>(&mut self, mut partition: F)
		where F: FnMut(&usize) -> P, P: Ord
	{
		let p = &mut self.partition;
		let m = &mut self.marks;

		if p.elements.len() == 0
		{
			return;
		}

		// Wipes out any existing sets
		p.set_count = 0;
		m.marked.clear();
		p.first.clear();
		p.past.clear();

		// Sort elements by the partition so they will be together
		// TODO the sort_by_key method is stable in 1.7, until then we have copied its implementation here
		//p.elements.sort_by_key(& mut partition); //#![feature(slice_sort_by_key)]
		p.elements.sort_by(|a, b| partition(a).cmp(&partition(b)));

		// Create sets for each partition
		p.first.push(0); // The first set starts at 0
		let mut current_partition = partition(&0);
		for (i, &element) in p.elements.iter().enumerate()
		{
			if partition(&i) != current_partition
			{
				current_partition = partition(&i);
				p.past.push(i);
				p.set_count += 1;
				p.first.push(i);
				m.marked.push(0);
			}
			p.set_of[element] = Some(p.set_count);
			p.location[element] = i;
		}
		p.set_count += 1;
		p.past.push(p.elements.len());
	}

	pub fn is_marked(&self, element: usize) -> bool
	{
		let set = self.partition.set_of[element].unwrap();
		let element_index = self.partition.location[element];
		let first_unmarked = self.partition.first[set] + self.marks.marked[set];
		return element_index < first_unmarked;
	}
}

impl<'a, 'b> PartitionMarkedSetIterator<'a, 'b>
{
	pub fn mark(&mut self, element: usize)
	{
		self.partition_marking.mark(element);
	}
}

impl<'a, 'b> Iterator for PartitionMarkedSetIterator<'a, 'b>
{
	type Item = usize;

	fn next(&mut self) -> Option<usize>
	{
		let i = self.location;
		if i < self.partition_marking.partition.first[self.set] + self.partition_marking.marks.marked[self.set]
		{
			self.location += 1;
			return Some(self.partition_marking.partition.elements[i]);
		}
		return None;
	}
}
