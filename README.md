# Comparison of DFA minimization algorithm in C++, C# and Rust

This project implements the DFA minimization algorithm described in "[Fast brief practical DFA minimization](Valmari12.pdf)" by Valmari (2011). This algorithm is *O(n+m log m)* where *n* is the number of states and *m* the number of transitions. This is further reduced to *O(n+m log n)* if we can assume the transitions are already sorted by label. The commonly known algorithm is *O(nk log n)* where *k* is the size of the alphabet. Other more efficient algorithms use more memory. See the paper for all the details on this.

This project implements the algorithm in Rust and C# in order to evaluate Rust as compared to C# and C++.

## Project Status: Sample Inactive

This is sample code only and should not be used in production without extensive testing. This project is not currently being maintained and you should not expect any bug fixes or enhancements.

### Download and Use

Clone this git repo. Compile solutions using Visual Studio 2015. Compile Rust project using cargo and Rust 1.6.0. After compiling all versions, run test data in them using [RunAllTestData.bat](RunAllTestData.bat).

## Explanation of this Project

This project implements a [DFA minimization algorithm](Valmari12.pdf) in C++, C# and Rust in order to evaluate Rust as compared to the other languages. Four versions of the algorithm are included. They are:

1. [C++ Original](Cpp/Original/Original.cpp) - the algorithm implementation as presented in the [paper](Valmari12.pdf) by Valmari (2011)
2. [C++ Modified](Cpp/Modified/Modified.cpp) - a C++ version modified to improve clarity and readability
3. [C#](CSharp/CSharp/)
4. [Rust](Rust/src/)

All versions accept input on the console in the [input format](dfaFormat.md) used by the original version from the paper and write their results to the console in the same format. [Sample DFA files](TestData/) are included in the project.

## Commentary on Languages

### C++ Original

This version is an *exact copy of the version provided in the paper*. The variable names are almost all a single letter. Many lines are combined into a single line and other long lines are split into multiple lines. The author states this was done to minimize the total length of the paper and make the code fit into the narrow columns of the publication.

### C++ Modified

This is a *modification of the version provided in the paper*. The fundamental structure of the code is unchanged. Most C++ idioms and optimizations are unchanged. Variable names have been changed. Combined lines have been split back out. Split lines have been restored. While some comments have been added, it is still not well commented.

With these clarifying changes one can see there is lots of pointer manipulation, no bounds checking, and data structure sharing. There is also lots of mutable global state. Globals must be initialized in `main` before calling certain functions. To safely modify this code one must understand the entire program top to bottom. The algorithm is mixed in with reading the data. For example, the reading of final states is directly tied into determining the reachability of states. There are several places where non-obvious assumptions are made that if violated will produce incorrect results. For example, that you will never call `mark` on an element that is already marked or that a certain value has already been initialized to zero and doesn't need to be set when resetting everything else.

### C\#

This version is adapted from an implementation of the algorithm in another project I was working on that needed it. It cleanly separates reading and writing data from the algorithm. It is generally much clearer and easier to understand. The use of higher level data structures makes code simpler and clearer. For example, a list is used to track the touched sets in the partition structure. The algorithm is now safe. There is bounds checking etc. It is still probably reasonably efficient, but clearly not as efficient as C++ version. It clearly makes less efficient use of memory.

While this code is more in the vein of C#, I am not entirely happy with the result. In my project I actually had `State` and `Input` structures that served as opaque handles to states and input values. However, given the way the data input is provided, that just got in the way for this example. I also feel that a lot of the implementation still shows the marks of the original C++ implementation and isn't fully idiomatic C#.

### Rust

This project was really created to evaluate the Rust language. As such, the Rust version is the one I am most interested in. The goal was to create a version that was as safe and clear as C#, but could in theory be as efficient as C++ assuming the compiler was able to optimize well. I have made my best attempt to use idiomatic Rust. However, I am a novice Rust programmer at this time.

Based on my experience writing this version of the algorithm, a formed the following opinions about the pros and cons of the Rust programming language.

In Rust 1.6.0:

**Pros:**

* Type inference! especially on certain generic parameters
* Warnings about naming standards
* For loop syntax and always using iterators and ranges for loops
* Can use `_` in place of unneeded for loop variable name
* Can safely return immutable references to parts of a struct's internal structure
* Strict typing with `usize` pushed me to good design, changing `Partition.set_of` into `Vec<Option<usize>>`
* Able to implement safe memory/data structure reuse and sharing, for example of `PartitionMarks`
* Cargo and crates.io in the core

**Cons:**

* Lifetimes and the borrow checker are the most unfamiliar aspects of the language, yet there is no documentation that truly explains how lifetimes and lifetime variables really work. They just give basic examples and hand-wavy explanations
* Lifetimes may cause non-local problems, but are hard to get right (i.e. if someone messes up a lifetime in one place, they may not realize it, but it may mess up attempts to use those types/functions)
* Compiler error messages are confusing
* Compiler not good at producing all errors after encountering other errors
* Returning iterators, "There be dragons here" (see comments below)
* Can't make struct fields immutable (this would enforce invariants)
* Unit of encapsulation is the module rather than the struct. For example, can't enforce the use of a `new` constructor inside the module the struct is declared in. Also can't enforce no accessing private fields
* Standard library naming is unintuitive to me, i.e. `Vec::push` instead of `Vec::add` and `Vec::retain` instead of `Vec::remove` or `Vec::remove_all` or `Vec::remove_where`
* No interpolated strings
* `String` vs `&str` (see comments below)
* Lifetimes extend to end of scope, not just last use (see comments below)
* Feels awkward to use `usize` for things like states that aren't sizes. Perhaps needs a different name?
* Dislike the coupling of files to modules
* "Stolen" values, if a function you don't control takes something by value that doesn't implement `Copy`, it can cause real headaches
* Tooling
  * Don't have editor with really good completion, coloring etc. like Visual Studio
  * Don't have refactoring tool like with Resharper
  * Windows install doesn't come with a debugger
  * `gdb` isn't my idea of a good debugger
* Really needs an IDE that lets you inspect inferred types and lifetimes
* `RUST_BACKTRACE` environment variable not discoverable, why isn't this on by default for debug builds?
* Was forced to resort to `println!` and comment out code debugging
* Miss being able to omit curly braces on if, else, for etc.
* `read_line()` includes the newline characters, that's annoying and inconvenient (C# and Java don't work this way), though I guess it could have uses when you are appending to a buffer string
* Lifetime rules make the `buffer.clear()` calls in `read_dfa()` awkward. Would make more sense to put them at the end of using the buffer value, but that would require another scope
* Lifetimes in same line are too restrictive (see comments)
* Function pointer `fn` types vs closure types `Fn`, `FnMut` and `FnOnce` are confusing. What is `FnBox`?
* A type parameter `F : FnMut(X) -> Y` can be passed a closure by value or mutable reference. This is really unintuitive coming from other languages, and there is nothing about the type name that implies it would work that way
* Result of assignment expression should work for types implementing `Copy`
* For a language focused on "Zero Cost Abstractions", there is no documentation on the performance impact of various language features and API calls

#### Returning iterators

In several places I wanted to return an iterator from a function (for example in `PartitionMarking.marked`). This is something I do quite commonly in C#. There appear to be several language limitations that are interacting poorly to make this very difficult. I can't honestly say I understand all the issues. However, there is discussion on stack overflow [here](http://stackoverflow.com/questions/31904842/return-a-map-iterator-which-is-using-a-closure-in-rust) and [here](http://stackoverflow.com/questions/27646925/how-do-i-return-a-filter-iterator-from-a-function). The accepted work around seems to be to return a `Box<Iterator<...>...>`. That of course introduces extra heap allocation and pointers. At first, I wasn't even able to get that solution working due to lifetime issues. I ended up `.collect()`ing the values into a `Vec<usize>` and returning that. Which is apparently a common though even uglier workaround. Eventually, I somehow got the lifetime issues worked out and am now using the box solution. It seems one of the [issues](https://github.com/rust-lang/rfcs/issues/518) is that there is no way to return an "abstract" type from a function. So it becomes necessary to return a very specific concrete type of iterator that then leaks information about the implementation of your function. There is an [RFC](https://github.com/Kimundi/rfcs/blob/function_output_type_parameters/text/0000-function_output_type_parameters.md) and [pull request](https://github.com/rust-lang/rfcs/pull/1305) on this. The other problem seems to revolve around the complexities of the lifetimes necessary for this to work. From my perspective, this is a huge hole in the functionality of the Rust language at this time.

#### `String` vs `&str`

The difference between these types is confusing. It is often unclear when to use one vs the other. I think part of the confusion is because of the naming. String is what other languages call `StringBuilder`. It is also very non-obvious that to convert from `String` to `&str` you dereference the value as `&value`. In other parts of the language, dereferencing generally doesn't mean implicit type conversion (`String` implements `Deref`). While languages like C# and Java have this distinction with their `StringBuilder` types I think it is less confusing because they use the `StringBuilder` only rarely when absolutely necessary. Rust seems to make much more frequent use of the `String` type. I imagine this is for reasons of both performance and limitations imposed by lifetimes and the borrow checker.

#### Lifetimes to End of Scope

The lifetime of a local variable extends until the end of its scope regardless of the last usage. It might be nice if the Rust compiler could infer that in some situations the lifetime of a variable or borrow could be shorter to allow reuse without introducing explicit scopes. For example, in [read_dfa()](Rust/src/main.rs), the read buffer is split into the `header`. The header is then parsed into four integer values. At that point, the header is no longer used and nothing is holding a reference to it. However, when the read of the transitions went to reuse the `buffer`, it couldn't because the `header` split was still holding a borrow on the `buffer`. It would be evident to a developer that `header` was no longer used and it should be safe to reuse the `buffer` at that point. Instead, it was necessary to introduce a scope using curly braces to make it clear to the compiler when `header` should go out of scope.

I can see some reasons why it works the way it does. The current behaviour makes it very clear and explicit when things go out of scope. That may matter when something implements the drop trait. It also avoids any developer surprise.

#### Lifetimes in Same Line

This issue came up in `PartitionMarking::split_sets`. The line `p.first.push(p.first[set]);` gives the compiler error:

```console
error: cannot borrow `p.first` as immutable because it is also borrowed as mutable [E0502]
p.first.push(p.first[set]);
                ^~~~~~~
note: previous borrow of `p.first` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `p.first` until the borrow ends
p.first.push(p.first[set]);
^~~~~~~
note: previous borrow ends here
p.first.push(p.first[set]);
                            ^
```

However, the read of the value `p.first[set]` is complete before the call to `p.first.push(...)` since function arguments must be evaluated before the call is made. Changing the code this makes it compile:

```console
let first_of_set = p.first[set];
p.first.push(first_of_set);

Most languages and developers would say that is exactly equivalent code.
```