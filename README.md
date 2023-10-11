
# Experiment (lalrpop)

## Update

A smaller, more reproducible problem that highlights the issue with missing terminators is:

```
1 + 2 \ 3 + 4
```

This should be parsed as

```
1 + (2 \ (3 + 4))
```

However, this is a problem:

* If `\ ` function is defined as `binary`, `\ `, `function`, then the left side will be a group, i.e. `(1 + 2) \ (3 + 4)`
* If the same function definition uses something like `unary` instead of binary, then parsing fails, because `1+func` is impossible.
* If the order is swapped and `+` is defined by referring to `\ `, then the closure body cannot be an addition, and so it becomes `1 + ((2 \ 3) + 4)`

So the subsequent details are more complex than needed and may be out of date.

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
* There is a short syntax using `\ ` for closures with zero explicit arguments
  
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
  * If the newline is consumed to terminate the closure, then there is no newline left to terminate the statement. 
  * There is no way to look at the newline but not consume it.
  * Not all `\` closures are terminated by a newline, i.e. it might be inside `(...)`.
* Making special versions of statements that can be terminated either by newlines/`;` or by closures that use the same
  * This makes the grammar much more complicated.
  * There also needs to be a similar special case for `\n.`, but that is at expression instead of statement level.
* Allow newline-"." outside closures, but only "." inside.
  Ambiguous because of e.g. `a\b\c`, unless closure body is subtype of closure, which is very limiting (esp. since closure must be lower than dot-expr in the parse tree).
  * Split into token `.` and `\n+.`, and only allow the former in closures. Does not work because `\n+.` will consume the newline, and `\` closure does not know what to do.
  * Match a combined single token, `Newline* "."`. 

## Solution?

Let the expression end without terminator when rules stop matching, like any other.

Use two rules for dot-expressions to make sure `\n.` is allowed outside closures but not inside:

* ...expr...
* Newline-dot `\n.`
* Closure
* Dot-only `.`
* ...expr...

