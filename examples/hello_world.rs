fn main() {
    let s = bf::bf!(
        input { "" }
        code {
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++." as String
        }
    ).unwrap();
    println!("{}", s);
}