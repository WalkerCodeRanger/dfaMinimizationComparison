using System;
using System.Collections.Generic;
using System.Linq;

namespace DfaMinComparisonCSharp.CSharp
{
	/// <summary>
	/// Represents a partition of elements into sets.  Based on algrothm presented in
	/// "Fast brief practical DFA minimization" by Antti Valmari 2011.  Here elements
	/// are representing as integers from 0 to N-1
	/// </summary>
	public class Partition
	{
		private int setCount; // z - the number of sets
		private readonly int[] elements; // E[f], E[f+1],...E[p-1] The elements of the set s where f=first[s] and p=past[s]
		private readonly int[] location; // L[e] the location of element e
		private readonly int[] setOf; // S[e] the set the element e belongs to
		private readonly int[] first; // F[s] the first element of set s
		private readonly int[] past; // P[s] the element past the end of set s

		// For simplicity we do not share the next to data structures
		private readonly int[] marked; // M[s] the number of marked elements in set s
		private readonly Stack<int> touched; // W touched (i.e. contain marked) sets, replaces W[] and w in the paper

		public Partition(int elementCount)
		{
			elements = new int[elementCount];
			location = new int[elementCount];
			setOf = new int[elementCount];
			first = new int[elementCount];
			past = new int[elementCount];
			marked = new int[elementCount];
			touched = new Stack<int>(elementCount); // capacity so we never have to worry about resize

			for(var i = 0; i < elementCount; ++i)
				elements[i] = location[i] = i;

			if(elementCount > 0)
			{
				setCount = 1;
				first[0] = 0;
				past[0] = elementCount;
			}
			else
				setCount = 0;
		}

		public int SetCount => setCount;

		public void Mark(int element)
		{
			var set = setOf[element];
			if(set == -1) return; // not in any set
			var i = location[element];
			var firstUnmarked = first[set] + marked[set];
			if(i < firstUnmarked) return; // already marked

			// swap element and the first unmarked in elements, updating location appropriately
			elements[i] = elements[firstUnmarked];
			location[elements[i]] = i;
			elements[firstUnmarked] = element;
			location[element] = firstUnmarked;

			// track how many marked in the set, and if it was touched.
			if(marked[set]++ == 0)
				touched.Push(set);
		}

		/// <summary>
		/// Split each set into the marked and unmarked subsets
		/// </summary>
		public void SplitSets()
		{
			while(touched.Count > 0)
			{
				var set = touched.Pop();
				var firstUnmarked = first[set] + marked[set];

				// if the whole set was marked
				if(firstUnmarked == past[set])
				{
					// just unmark it and do nothing
					marked[set] = 0;
					continue;
				}

				// Make the smaller half a new set
				// unless we are keeping marked, then make a new set out of unmarked
				if(marked[set] <= past[set] - firstUnmarked)
				{
					first[setCount] = first[set];
					past[setCount] = first[set] = firstUnmarked;
				}
				else
				{
					past[setCount] = past[set];
					first[setCount] = past[set] = firstUnmarked;
				}

				// mark the elements as members of the new set
				for(var i = first[setCount]; i < past[setCount]; ++i)
					setOf[elements[i]] = setCount;

				// clear marks on old and new set
				marked[set] = marked[setCount] = 0;

				// increase set count
				setCount++;
			}
		}

		/// <summary>
		/// Discard the unmarked from each set, they are no longer in any set (-1)
		/// </summary>
		public void DiscardUnmarked()
		{
			while(touched.Count > 0)
			{
				var set = touched.Pop();
				var firstUnmarked = first[set] + marked[set];

				// if the whole set was marked
				if(firstUnmarked == past[set])
				{
					// just unmark it and do nothing
					marked[set] = 0;
					continue;
				}

				var pastUnmarked = past[set];
				past[set] = firstUnmarked;

				// mark the elements as members no set
				for(var i = firstUnmarked; i < pastUnmarked; ++i)
					setOf[elements[i]] = -1;

				// clear mark on set
				marked[set] = 0;
			}
		}

		public IEnumerable<int> Marked(int set)
		{
			// The algorithm relies on the fact that you can mark nodes and they will be added to this IEnumerable
			var firstOfSet = first[set];
			for(var i = firstOfSet; i < firstOfSet + marked[set]; i++)
				yield return elements[i];
		}

		public IEnumerable<int> Set(int set)
		{
			var firstOfSet = first[set];
			return Enumerable.Range(firstOfSet, past[set] - firstOfSet).Select(i => elements[i]);
		}

		public int SetOf(int element)
		{
			return setOf[element];
		}

		public void PartitionBy(Func<int, int> partitionFunc)
		{
			// Wipes out any existing sets
			setCount = marked[0] = 0;

			// Sort them by the partition func so they will be together
			var paritition = elements.Select(partitionFunc).ToArray();
			Array.Sort(paritition, elements);

			// Create sets for each partition
			var currentPartition = paritition[0];
			for(var i = 0; i < elements.Length; ++i)
			{
				var element = elements[i];
				if(paritition[i] != currentPartition)
				{
					currentPartition = paritition[i];
					past[setCount++] = i;
					first[setCount] = i;
					marked[setCount] = 0;
				}
				setOf[element] = setCount;
				location[element] = i;
			}
			past[setCount++] = elements.Length;
		}

		public int SomeElementOf(int set)
		{
			return elements[first[set]];
		}
	}
}
