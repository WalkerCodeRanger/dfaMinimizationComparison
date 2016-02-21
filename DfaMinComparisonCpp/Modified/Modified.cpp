#include <iostream>
#include <algorithm>

/* Refinable partition */
int* M;
int* W;
int w = 0; // temporary worksets
struct partition
{
	int z;
	int* E;
	int* L;
	int* S;
	int* F;
	int* P;

	void init(int n)
	{
		z = bool(n);
		E = new int[n];
		L = new int[n];
		S = new int[n];
		F = new int[n];
		P = new int[n];
		for (int i = 0; i < n; ++i)
		{
			E[i] = L[i] = i;
			S[i] = 0;
		}
		if (z)
		{
			F[0] = 0;
			P[0] = n;
		}
	}

	void mark(int e)
	{
		int s = S[e], i = L[e], j = F[s] + M[s];
		E[i] = E[j];
		L[E[i]] = i;
		E[j] = e;
		L[e] = j;
		if (!M[s]++)
		{
			W[w++] = s;
		}
	}

	void split()
	{
		while (w)
		{
			int s = W[--w], j = F[s] + M[s];
			if (j == P[s])
			{
				M[s] = 0;
				continue;
			}
			if (M[s] <= P[s] - j)
			{
				F[z] = F[s];
				P[z] = F[s] = j;
			}
			else
			{
				P[z] = P[s];
				F[z] = P[s] = j;
			}
			for (int i = F[z]; i < P[z]; ++i)
			{
				S[E[i]] = z;
			}
			M[s] = M[z++] = 0;
		}
	}
};

partition Blocks; // blocks (consist of states)
partition Cords; // cords (consist of transitions)
int nn; // number of states
int mm; // number of transitions
int ff; // number of final states
int q0; // initial state
int* T; // tails of transitions
int* L; // labels of transitions
int* H; // heads of transitions
bool cmp(int i, int j)
{
	return L[i] < L[j];
}

/* Adjacent transitions */
int *A, *F;
void make_adjacent(int K[])
{
	int q, t;
	for (q = 0; q <= nn; ++q)
	{
		F[q] = 0;
	}
	for (t = 0; t < mm; ++t)
	{
		++F[K[t]];
	}
	for (q = 0; q < nn; ++q)
		F[q + 1] += F[q];
	for (t = mm; t--; )
	{
		A[--F[K[t]]] = t;
	}
}

/* Removal of irrelevant parts */
int rr = 0; // number of reached states
inline void reach(int q)
{
	int i = Blocks.L[q];
	if (i >= rr)
	{
		Blocks.E[i] = Blocks.E[rr];
		Blocks.L[Blocks.E[i]] = i;
		Blocks.E[rr] = q;
		Blocks.L[q] = rr++;
	}
}

void rem_unreachable(int T[], int H[])
{
	make_adjacent(T); 
	int i, j;
	for (i = 0; i < rr; ++i)
	{
		for (j = F[Blocks.E[i]]; j < F[Blocks.E[i] + 1]; ++j)
		{
			reach(H[A[j]]);
		}
	}
	j = 0;
	for (int t = 0; t < mm; ++t)
	{
		if (Blocks.L[T[t]] < rr)
		{
			H[j] = H[t];
			L[j] = L[t];
			T[j] = T[t];
			++j;
		}
	}
	mm = j;
	Blocks.P[0] = rr;
	rr = 0;
}

/* Main program */
int main()
{
	/* Read sizes and reserve most memory */
	std::cin >> nn >> mm >> q0 >> ff;
	T = new int[mm];
	L = new int[mm];
	H = new int[mm];
	Blocks.init(nn);
	A = new int[mm];
	F = new int[nn + 1];
	/* Read transitions */
	for (int t = 0; t < mm; ++t)
	{
		std::cin >> T[t] >> L[t] >> H[t];
	}
	/* Remove states that cannot be reached
	from the initial state, and from which
	final states cannot be reached */
	reach(q0);
	rem_unreachable(T, H);
	for (int i = 0; i < ff; ++i)
	{
		int q; std::cin >> q;
		if (Blocks.L[q] < Blocks.P[0])
		{
			reach(q);
		}
	}
	ff = rr; 
	rem_unreachable(H, T);
	/* Make initial partition */
	W = new int[mm + 1];
	M = new int[mm + 1];
	M[0] = ff;
	if (ff)
	{
		W[w++] = 0;
		Blocks.split();
	}
	/* Make transition partition */
	Cords.init(mm);
	if (mm)
	{
		std::sort(Cords.E, Cords.E + mm, cmp);
		Cords.z = M[0] = 0; int a = L[Cords.E[0]];
		for (int i = 0; i < mm; ++i)
		{
			int t = Cords.E[i];
			if (L[t] != a)
			{
				a = L[t];
				Cords.P[Cords.z++] = i;
				Cords.F[Cords.z] = i;
				M[Cords.z] = 0;
			}
			Cords.S[t] = Cords.z;
			Cords.L[t] = i;
		}
		Cords.P[Cords.z++] = mm;
	}

	/* Split blocks and cords */
	make_adjacent(H);
	int b = 1, c = 0, i, j;
	while (c < Cords.z)
	{
		for (i = Cords.F[c]; i < Cords.P[c]; ++i)
		{
			Blocks.mark(T[Cords.E[i]]);
		}
		Blocks.split(); ++c;
		while (b < Blocks.z)
		{
			for (i = Blocks.F[b]; i < Blocks.P[b]; ++i)
			{
				for (j = F[Blocks.E[i]]; j < F[Blocks.E[i] + 1]; ++j)
				{
					Cords.mark(A[j]);
				}
			}
			Cords.split(); ++b;
		}
	}
	/* Count the numbers of transitions
	and final states in the result */
	int mo = 0, fo = 0;
	for (int t = 0; t < mm; ++t)
	{
		if (Blocks.L[T[t]] == Blocks.F[Blocks.S[T[t]]])
		{
			++mo;
		}
	}
	for (int b = 0; b < Blocks.z; ++b)
	{
		if (Blocks.F[b] < ff)
		{
			++fo;
		}
	}
	/* Print the result */
	std::cout << Blocks.z << ' ' << mo << ' ' << Blocks.S[q0] << ' ' << fo << '\n';
	for (int t = 0; t < mm; ++t)
	{
		if (Blocks.L[T[t]] == Blocks.F[Blocks.S[T[t]]])
		{
			std::cout << Blocks.S[T[t]] << ' ' << L[t] << ' ' << Blocks.S[H[t]] << '\n';
		}
	}
	for (int b = 0; b < Blocks.z; ++b)
	{
		if (Blocks.F[b] < ff)
		{
			std::cout << b << '\n';
		}
	}
}