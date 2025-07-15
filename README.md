# spade

Simple version of spade-lang. Based on the crafting interpreters.

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
