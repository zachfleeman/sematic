 # link-parser-rust-bindings
Rust bindings for Link Parser

## Test Link Parser program

[Docs](https://www.abisource.com/projects/link-grammar/dict/index.html)
[Introduction](https://www.abisource.com/projects/link-grammar/dict/introduction.html)
[API Docs](https://www.abisource.com/projects/link-grammar/api/index.html)

To compile the test program: 
 - note: you must have link-grammar installed on your system (`brew install link-grammar`)

```bash
clang -I /usr/local/include/link-grammar -l link-grammar ./link-parser-test.c
./a.out
```
