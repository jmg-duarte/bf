# bf

A Rust function-like proc-macro that compiles BrainFuck!

## Usage
```rust
fn main() {
    let s = bf::bf!(
        input { "" }
        code {
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++." as String
        }
    ).unwrap();
    println!("{}", s);
}
```