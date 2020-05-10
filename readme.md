Schönfinkel
===========

Schönfinkel is a little language written as a exercise to learn more about type
checking and type inference. It is third in a series of interpreters
I've been working on. The first,
[Scoundrel](https://github.com/dminor/scoundrel) was a purely functional
subset of Lua, written to learn more about Rust and interpreters in general.
It worked by evaluating the abstract syntax tree. The second,
[Walden](https://github.com/dminor/walden), a Smalltalk/Self dialect,
added a virtual machine and mutable state.

To keep things interesting Schönfinkel uses parser combinators rather than a
separate lexer and recursive descent parser like in Scoundrel and Walden. The
language is purely functional. Schönfinkel supports static type checking
without having to use any type annotations, largely because the language is so
simple. The type inference is done from scratch rather than using an existing
algorithm like Hindley-Milner.

The type inference is bottom up, with the type being inferred from constants
if present, and operators if they are not present. The only polymorphism is
in the == and ~= operators which may be applied to any type, which can result
in polymorphic functions:

```
fn (a, b) ->
    a == b
end
```

Schönfinkel was much more time consuming to write than Scoundrel or Walden.
Part of this was getting the type system to work. It made the development cycle
for adding a new language feature that much longer, which made it difficult to
keep momentum when developing the language. It's a lot of fun to see a new
part of a language come alive in an interpreter, and that was a lot slower
in Schönfinkel. I plan to learn a lot more about logic programming and unification
before tackling another type system.

The language is named after
[Moses Schönfinkel](https://en.wikipedia.org/wiki/Moses_Sch%C3%B6nfinkel), a logician.

Keywords
--------

The following are reserved keywords: *else*, *elsif*, *end*, *false*,
*fn*, *if*, *def*, *then* and *true*.

Values
------

### Boolean

Booleans take the values `true` and `false`. The usual boolean operators are
supported: `&&`, `||`, and `~` (for not).

### Function

A function is a value consisting of a single argument, which may be a tuple,
and a body which is evaluated when the function is called.

```
fn (a, b, c) ->
    a + b + c
end
```

Lexical closures are supported and it is possible for a function to return another
function:

```
def adder := fn t -> fn x -> x + t end end;
def f := adder 1;
f 2;
```

Functions can optionally take a name which is defined inside the body to allow
for recursive calls.

```
fn fact n ->
    fn iter (n, acc) ->
        if n == 0 then
            acc
        else
            iter(n - 1, n*acc)
        end
    end
    iter (n, 1)
end
```

### Number

Numbers are 64 bit integers. The usual arithmetic and comparison operators
are supported: `+`, `-`, `*`, `/`, `%`, '<', '<=', '==', '<>', '>', and '>='.
Division by zero results in a runtime error.

```
2 + 3 / 4 * 5 % 6
```

### Tuple

Tuples are a fixed size comma-separated list of other values:

```
(2, false, fn x -> x + 1 end, (1, 2))
```

Expressions
-----------

### If/Then/Elsif/Else/End

If expressions are used to evaluate conditionals. The else clause is
non-optional because every expression must return a value.

```
if x == 0 then
    0
elsif x == 1 then
    1
elsif x == 2 then
    2
else
    3
end
```

### Define

Define expressions are used to introduce variables. All variables are
immutable, but it is possible to shadow a previous define expression. The
value of a define expression is the value that is assigned to the variable.

```
def x := 1;
def x := false;
def y := def z := 42;
```

### Function Calls

A function call consists of a function value followed by the valueto which the
function is applied.

```
def f := fn x -> x + 1 end;
f 1
```
