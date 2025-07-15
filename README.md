# spade

A simple, dynamically-typed programming language with a tree-walk interpreter, written in Rust.

Examples:
```
let x = 0;
fn fibonacci(x) {
  if (x == 0 || x == 1) {
    return 1;
  }
  return fibonacci(x - 1) + fibonacci(x - 2);
}
print fibonacci(5);


```
