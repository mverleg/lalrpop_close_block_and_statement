
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

## Non-solutions

## Solution

