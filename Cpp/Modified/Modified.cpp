/* Code below heavily adapted from code provided in "Fast brief practical DFA minimization" by Antti Valmari (2011) */
#include <iostream>
#include <algorithm>

/* Refinable partition */
// These two structures are shared by both instances of partition, saving memory
int* Marked; // Marked[s] the number of marked elements in set s
int* Touched; // touched (i.e. contain marked) sets
int touchedCount = 0; // temporary worksets
struct partition
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
		int j = first[set] + Marked[set];
		elements[i] = elements[j];
		location[elements[i]] = i;
		elements[j] = element;
		location[element] = j;
		// increment the number of marked and add to touched if needed
		if (!Marked[set]++)
		{
			Touched[touchedCount++] = set;
		}
	}

	void split()
	{
		while (touchedCount)
		{
			int set = Touched[--touchedCount];
			int j = first[set] + Marked[set];
			if (j == past[set])
			{
				Marked[set] = 0;
				continue;
			}
			if (Marked[set] <= past[set] - j)
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
			Marked[setCount++] = 0;
			Marked[set] = 0;
		}
	}
};

partition Blocks; // blocks (consist of states)
partition Cords; // cords (consist of transitions)
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
int* Adjacent;
int* Offset;
void make_adjacent(int K[])
{
	int q, t;
	for (q = 0; q <= stateCount; ++q)
	{
		Offset[q] = 0;
	}
	for (t = 0; t < transitionCount; ++t)
	{
		++Offset[K[t]];
	}
	for (q = 0; q < stateCount; ++q)
		Offset[q + 1] += Offset[q];
	for (t = transitionCount; t--; )
	{
		Adjacent[--Offset[K[t]]] = t;
	}
}

/* Removal of irrelevant parts */
int rr = 0; // number of reached states
inline void reach(int q)
{
	int i = Blocks.location[q];
	if (i >= rr)
	{
		Blocks.elements[i] = Blocks.elements[rr];
		Blocks.location[Blocks.elements[i]] = i;
		Blocks.elements[rr] = q;
		Blocks.location[q] = rr++;
	}
}

void rem_unreachable(int tail[], int head[])
{
	make_adjacent(tail);
	int i, j;
	for (i = 0; i < rr; ++i)
	{
		for (j = Offset[Blocks.elements[i]]; j < Offset[Blocks.elements[i] + 1]; ++j)
			reach(head[Adjacent[j]]);
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
	Adjacent = new int[transitionCount];
	Offset = new int[stateCount + 1];
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
	Touched = new int[transitionCount + 1];
	Marked = new int[transitionCount + 1];
	Marked[0] = finalStatesCount;
	if (finalStatesCount)
	{
		Touched[touchedCount++] = 0;
		Blocks.split();
	}
	/* Make transition partition */
	Cords.init(transitionCount);
	if (transitionCount)
	{
		std::sort(Cords.elements, Cords.elements + transitionCount, cmp);
		Cords.setCount = Marked[0] = 0; int a = transitionLabel[Cords.elements[0]];
		for (int i = 0; i < transitionCount; ++i)
		{
			int t = Cords.elements[i];
			if (transitionLabel[t] != a)
			{
				a = transitionLabel[t];
				Cords.past[Cords.setCount++] = i;
				Cords.first[Cords.setCount] = i;
				Marked[Cords.setCount] = 0;
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
				for (j = Offset[Blocks.elements[i]]; j < Offset[Blocks.elements[i] + 1]; ++j)
				{
					Cords.mark(Adjacent[j]);
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