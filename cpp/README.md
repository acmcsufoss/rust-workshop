This C++ program demonstrates an easy to write memory bug.

---

**Building and running**  
Assuming you have GNU make and C++ compiler, just run `make run` to compile and run the program.

**Fixing the bug**  
The bug is found on lines 64-65
```cpp
  /*
   * BUG HERE!!
   * This is a 'use after free' error, which is something that no C++ compiler
   * will catch and warn you about. The rust compiler will, though.
   */
  delete person;
  output(*person);
```

To fix, just swap the order the lines (the delete needs to come after `output`
uses it).

```cpp
  output(*person);
  delete person;
```

It's apparent that the C++ compiler won't catch an entire class of bugs at
compile time.
