/* Code below heavily adapted from code provided in "Fast brief practical DFA minimization" by Antti Valmari (2011) */
#include <iostream>
#include <algorithm>

/* Refinable partition */
// These two structures are shared by both instances of partition, saving memory
int* marked; // Marked[s] the number of marked elements in set s
int* touched; // touched (i.e. contain marked) sets
int touchedCount = 0; // temporary worksets
struct Partition
{
	int setCount; // z - the number of sets
	int* elements;
	int* location;
	int* setOf;
	int* first;
	int* past;

	void init(int n)
	{
		setCount = bool(n);
		elements = new int[n];
		location = new int[n];
		setOf = new int[n];
		first = new int[n];
		past = new int[n];
		for (int i = 0; i < n; ++i)
		{
			elements[i] = location[i] = i;
			setOf[i] = 0;
		}
		if (setCount)
		{
			first[0] = 0;
			past[0] = n;
		}
	}

	void mark(int element)
	{
		// swap element with the first unmarked
		int set = setOf[element];
		int i = location[element];
		int j = first[set] + marked[set];
		elements[i] = elements[j];
		location[elements[i]] = i;
		elements[j] = element;
		location[element] = j;
		// increment the number of marked and add to touched if needed
		if (!marked[set]++)
		{
			touched[touchedCount++] = set;
		}
	}

	void split()
	{
		while (touchedCount)
		{
			int set = touched[--touchedCount];
			int j = first[set] + marked[set];
			if (j == past[set])
			{
				marked[set] = 0;
				continue;
			}
			if (marked[set] <= past[set] - j)
			{
				first[setCount] = first[set];
				past[setCount] = first[set] = j;
			}
			else
			{
				past[setCount] = past[set];
				first[setCount] = past[set] = j;
			}
			for (int i = first[setCount]; i < past[setCount]; ++i)
			{
				setOf[elements[i]] = setCount;
			}
			marked[setCount++] = 0;
			marked[set] = 0;
		}
	}
};

Partition Blocks; // blocks (consist of states)
Partition Cords; // cords (consist of transitions)
int stateCount; // number of states
int transitionCount; // number of transitions
int finalStatesCount; // number of final states
int initialState; // initial state
int* transitionTail; // tails of transitions (i.e. to state)
int* transitionLabel; // labels of transitions (i.e. on what input)
int* transitionHead; // heads of transitions (i.e. from state)
bool cmp(int i, int j)
{
	return transitionLabel[i] < transitionLabel[j];
}

/* Adjacent transitions */
int* adjacent;
int* offset;
void make_adjacent(int states[])
{
	for (int state = 0; state <= stateCount; ++state)
		offset[state] = 0;

	for (int transition = 0; transition < transitionCount; ++transition)
		++offset[states[transition]];

	for (int state = 0; state < stateCount; ++state)
		offset[state + 1] += offset[state];

	for (int transition = transitionCount; transition--; )
		adjacent[--offset[states[transition]]] = transition;
}

/* Removal of irrelevant parts */
int rr = 0; // number of reached states
inline void reach(int state)
{
	int i = Blocks.location[state];
	if (i >= rr)
	{
		Blocks.elements[i] = Blocks.elements[rr];
		Blocks.location[Blocks.elements[i]] = i;
		Blocks.elements[rr] = state;
		Blocks.location[state] = rr++;
	}
}

void rem_unreachable(int tail[], int head[])
{
	make_adjacent(tail);
	int i, j;
	for (i = 0; i < rr; ++i)
	{
		for (j = offset[Blocks.elements[i]]; j < offset[Blocks.elements[i] + 1]; ++j)
			reach(head[adjacent[j]]);
	}
	j = 0;
	for (int t = 0; t < transitionCount; ++t)
	{
		if (Blocks.location[tail[t]] < rr)
		{
			head[j] = head[t];
			transitionLabel[j] = transitionLabel[t];
			tail[j] = tail[t];
			++j;
		}
	}
	transitionCount = j;
	Blocks.past[0] = rr;
	rr = 0;
}

/* Main program */
int main()
{
	/* Read sizes and reserve most memory */
	std::cin >> stateCount >> transitionCount >> initialState >> finalStatesCount;
	transitionTail = new int[transitionCount];
	transitionLabel = new int[transitionCount];
	transitionHead = new int[transitionCount];
	Blocks.init(stateCount);
	adjacent = new int[transitionCount];
	offset = new int[stateCount + 1];
	/* Read transitions */
	for (int t = 0; t < transitionCount; ++t)
	{
		std::cin >> transitionTail[t] >> transitionLabel[t] >> transitionHead[t];
	}
	/* Remove states that cannot be reached
	from the initial state, and from which
	final states cannot be reached */
	reach(initialState);
	rem_unreachable(transitionTail, transitionHead);
	for (int i = 0; i < finalStatesCount; ++i)
	{
		int q;
		std::cin >> q;
		if (Blocks.location[q] < Blocks.past[0])
		{
			reach(q);
		}
	}
	finalStatesCount = rr;
	rem_unreachable(transitionHead, transitionTail);
	/* Make initial partition */
	touched = new int[transitionCount + 1];
	marked = new int[transitionCount + 1];
	marked[0] = finalStatesCount;
	if (finalStatesCount)
	{
		touched[touchedCount++] = 0;
		Blocks.split();
	}
	/* Make transition partition */
	Cords.init(transitionCount);
	if (transitionCount)
	{
		std::sort(Cords.elements, Cords.elements + transitionCount, cmp);
		Cords.setCount = marked[0] = 0; int a = transitionLabel[Cords.elements[0]];
		for (int i = 0; i < transitionCount; ++i)
		{
			int t = Cords.elements[i];
			if (transitionLabel[t] != a)
			{
				a = transitionLabel[t];
				Cords.past[Cords.setCount++] = i;
				Cords.first[Cords.setCount] = i;
				marked[Cords.setCount] = 0;
			}
			Cords.setOf[t] = Cords.setCount;
			Cords.location[t] = i;
		}
		Cords.past[Cords.setCount++] = transitionCount;
	}

	/* Split blocks and cords */
	make_adjacent(transitionHead);
	int b = 1, c = 0, i, j;
	while (c < Cords.setCount)
	{
		for (i = Cords.first[c]; i < Cords.past[c]; ++i)
		{
			Blocks.mark(transitionTail[Cords.elements[i]]);
		}
		Blocks.split(); ++c;
		while (b < Blocks.setCount)
		{
			for (i = Blocks.first[b]; i < Blocks.past[b]; ++i)
			{
				for (j = offset[Blocks.elements[i]]; j < offset[Blocks.elements[i] + 1]; ++j)
				{
					Cords.mark(adjacent[j]);
				}
			}
			Cords.split(); ++b;
		}
	}
	/* Count the numbers of transitions
	and final states in the result */
	int mo = 0, fo = 0;
	for (int t = 0; t < transitionCount; ++t)
	{
		if (Blocks.location[transitionTail[t]] == Blocks.first[Blocks.setOf[transitionTail[t]]])
		{
			++mo;
		}
	}
	for (int b = 0; b < Blocks.setCount; ++b)
	{
		if (Blocks.first[b] < finalStatesCount)
		{
			++fo;
		}
	}
	/* Print the result */
	std::cout << Blocks.setCount << ' ' << mo << ' ' << Blocks.setOf[initialState] << ' ' << fo << '\n';
	for (int t = 0; t < transitionCount; ++t)
	{
		if (Blocks.location[transitionTail[t]] == Blocks.first[Blocks.setOf[transitionTail[t]]])
		{
			std::cout << Blocks.setOf[transitionTail[t]] << ' ' << transitionLabel[t] << ' ' << Blocks.setOf[transitionHead[t]] << '\n';
		}
	}
	for (int b = 0; b < Blocks.setCount; ++b)
	{
		if (Blocks.first[b] < finalStatesCount)
		{
			std::cout << b << '\n';
		}
	}
}