
# Experiment (lalrpop)

## Situation

A simplified language, based on [Steel](https://github.com/mverleg/steel), where

* Statements are closed by either newline or `;`
* Expressions are statements
* An exception is that `.` can appear after newline, so that one can do

  ```
  person
      .name
      .lowercase()
  ```

* Functions with zero arguments can be called without `()`
* Closures without arguments can use `it` as the implicit single argument
* There is a short syntax using `\` for closures with zero explicit arguments
  
  ```
  add = \ it + 1
  ```

* If last parameter is a function, it can be passed outside `()` (or without if nullary):

  ```
  list.map \ it.abs

  list.find_first(default=0) \ it % 2 == 0
  ```

  But the version with parentheses is not in this little demo.

## Problems

The problem is that this code is ambiguous:

```
offset = 1
result = list
    .map \ it - offset
    .map \ it.abs
print(result)
```

It seems obvious with indenting that the intended meaning is

```
offset = 1
result = list
    .map(\ it - offset)
    .map(\ it.abs)
print(result)
```

but indentation does not affect syntax (only linebreaks), so it could also be

```
offset = 1
result = list
    .map(\ it - offset
        .map(\ it.abs))
print(result)
```

## Non-solution

* Consuming the trailing newline to terminate the `\` closure. This comes in several variations, but it does not work, because
  * There is no newline left to terminate the statement. And there is no way to look at the newline but not consume it.
  * Not all `\` closures are terminated by a newline, i.e. it might be inside `(...)`.
* Making special versions of statements that can be terminated either by newlines/`;` or by closures that use the same
  * This makes the grammar much more complicated.
  * There is also needs to be a similar special case for `\n.`, but that is at expression instead of statement level.
* Allow newline-"." outside closures, but only "." inside. Two ways, neither of which work:
  * Match two tokens, `Newline* "."`. Does not work because of ambiguity of whether the newline ends the statement or not (max lookahead is 1)
  * Split into token `"."` and `\n+.`, and only allow the former in closures. Does not work because `\n+.` will consume the newline.

## Solution

There are two main parts of the solution

1. Make sure that `\n` already terminates the expression-statement, just like `)` does without being consumed.

   This means that this code cannot be valid:
   ```
   a = 1
       + 2
   ```
   because after parsing `1` it is not clear without lookahead if we are done.
   In this case `+2` could be a statement, but the problem still applies with `* 2`.
   
   Fortunately this style of indenting is ugly anyway. This is easier for humans and still okay with the parser:
   ```
   a = 1 +
       2
   ```
   it is unambiguous after `+` that the newline does not end the statement, since it would be invalid.

2. Make a special case to disambiguate for newline-spanning constructs like `\n.`, using generics.\
   
   The price of this is that we cannot use `\n.` in lambdas without adding `(` or `)` (or, in Steel, using long-syntax for closures - not present in this demo).
