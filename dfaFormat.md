# *.dfa File Format

The the format for `.dfa` files is based on the input accepted by the sample code in the paper "Fast brief practical DFA minimization" by Antti Valmari (2011).  The sample code accepts this input on the console and outputs back to the console.  Note that some versions of the algorithm (include the C++ versions) may accept somewhat different formats.  For example, the use of newlines instead of spaces.  However, the other versions may not accept that.  To be safe, you should provide input in this exact format.

The `.dfa` format specifies a deterministic finite automaton with states labeled with integers beginning at zero. There is a single start state. The DFA transitions on integer labeled inputs which need not start at zero, can be negative and can skip values.

The format consists of three sections, a header, a list of transitions and a list of final states.

## Header

The header consists of a single line of four space separated non-negative integers.  The first number is the number of states in the DFA. Remember states are numbered starting at zero up to 1 less than the number of states. The second is the number of transitions in the DFA.  The third is the label of the initial or start state of the DFA.  The fourth is the number of final states in the DFA.

For example, a DFA with 5 states, 6 transitions, initial state 0 and 2 final states would have the header:

```
5 6 0 2
```

## Transitions

The transitions sections consists of a number of transition lines equal to the number of transitions given in the header.  Each line consists of three space separated integers.  The first is the label for the state being transitioned from (i.e. the head).  The second is the label of the input to transition on.  The third is the label of the state to transition to (i.e. the tail).

For example, a transition from state 1 to state 3 on input 2 would be:

```
1 2 3
```

## Final States

The final states section consists of a number of final state lines equal to the number of final states given in th header. Each line consists of a single non-negative integer that is the label for the final state.

For example, if state 3 is final, that would be simply:

```
3
```

## Notes

The order of transitions and final states is not important.  However, a mismatch between the header and transition or final states sections will produce incorrect output.  There is no validation of the file format being read.

## Sample File

A small sample DFA is given in [TestData/Sample.dfa](TestData/Sample.dfa).  The contents of which are also reproduced below for reference.

```
5 6 0 2
0 0 1
0 1 2
1 2 3
2 2 3
1 3 4
2 3 4
3
4
```
