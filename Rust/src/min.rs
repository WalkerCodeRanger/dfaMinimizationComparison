use std::cmp::Ord;

pub struct Partition
{
	set_count: usize,
	elements: Vec<usize>,
	location: Vec<usize>,
	set_of: Vec<Option<usize>>,
	first: Vec<usize>,
	past: Vec<usize>
}

pub struct PartitionMarks
{
	marked: Vec<usize>,
	touched: Vec<usize>
}

pub struct PartitionMarking<'a>
{
	partition: &'a mut  Partition,
	marks: &'a mut PartitionMarks
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
			set_of: vec![Some(0); element_count],
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

	// TODO again with needing to return an iterator
	pub fn set(&self, set: usize) -> Vec<usize>
	{
		let first_of_set = self.first[set];
		return (first_of_set..(self.past[set]-first_of_set)).map(|i| self.elements[i]).collect();
	}

	pub fn some_element_of(&self, set: usize) -> usize
	{
		self.elements[self.first[set]]
	}

	pub fn begin_marking<'a>(&'a mut self, marks: &'a mut PartitionMarks) -> PartitionMarking
	{
		assert!(marks.marked.len() == 0);
		assert!(marks.touched.len() == 0);
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

impl <'a> PartitionMarking<'a>
{
	pub fn partition(&self) -> &Partition
	{
		self.partition
	}

	pub fn mark(&mut self, element: usize)
	{
		let p = &mut self.partition;
		let m = &mut self.marks;
		let set = p.set_of[element].unwrap();
		let element_index = p.location[element];
		let first_unmarked = p.first[set] + m.marked[set];
		p.elements[element_index] = p.elements[first_unmarked];
		p.location[p.elements[element_index]] = element_index;
		p.elements[first_unmarked] = element;
		p.location[element] = first_unmarked;
		// add to touched if needed and increment the number of marked
		if m.marked[set] == 0
		{
			m.touched.push(set);
		}
		m.marked[set] += 1;
	}

	// TODO Can't get returning an iterator to work
	// Per http://stackoverflow.com/q/31904842 Tried to return return type Box<Iterator<Item=usize> + 'a>
	// with body of return  Box::new((first_of_set..(first_of_set + self.marks.marked[set])).map(move |i| self.partition.elements[i]));
	// but didn't work
	// For now, falling back on the unacceptable answer of returning Vec<usize>
	pub fn marked(&self, set: usize) -> Vec<usize>
	{
		let first_of_set = self.partition.first[set];
		return (first_of_set..(first_of_set + self.marks.marked[set])).map(|i| self.partition.elements[i]).collect();
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
			// unless we are keeping marked, then make a new set out of unmarked
			if m.marked[set] <= p.past[set] - first_unmarked
			{
				p.first[p.set_count] = p.first[set];
				p.first[set] = first_unmarked;
				p.past[p.set_count] = first_unmarked;
			}
			else
			{
				p.past[p.set_count] = p.past[set];
				p.past[set] = first_unmarked;
				p.first[p.set_count] = first_unmarked;
			}

			// mark the elements as members of the new set
			for i in p.first[p.set_count]..p.past[p.set_count]
			{
				p.set_of[p.elements[i]] = Some(p.set_count);
			}

			// clear marks on old and new set
			m.marked[p.set_count] = 0;
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

		// Wipes out any existing sets
		m.marked[0] = 0;
		p.set_count = 0;

		// Sort them by the partition func so they will be together
		// TODO if we don't make a new lambda/closure here, the partition variable is moved and we can't use it later in the method. This is a hack.
		// TODO the sort_by_key method is stable in 1.7, until then we have copied its implementation here
		//p.elements.sort_by_key(|e| partition(e)); //#![feature(slice_sort_by_key)]
		p.elements.sort_by(|a, b| partition(a).cmp(&partition(b)));

		// Create sets for each partition
		let mut current_partition = partition(&0);
		for (i, &element) in p.elements.iter().enumerate()
		{
			if partition(&i) != current_partition
			{
				current_partition = partition(&i);
				p.past[p.set_count] = i;
				p.set_count += 1;
				p.first[p.set_count] = i;
				m.marked[p.set_count] = 0;
			}
			p.set_of[element] = Some(p.set_count);
			p.location[element] = i;
		}
		p.set_count += 1;
		p.past[p.set_count] = p.elements.len();
	}
}