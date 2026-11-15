```rust
fn main() {
    println!("Hello, world!");
}
```
```bash
rustc --emit=asm -C opt-level=3 src/main.rs -o hello_rust.s
```


```C++
#include <cstdio>

int main() {
    printf("Hello, World!\n");
}
```
```bash
clang++ -S -O0 -masm=att main.cpp -o hello.s
```

```bash
clang -no-pie hello.s -o hello
clang -nostdlib -no-pie hello_sys_call.s -o hello_sys_call
```