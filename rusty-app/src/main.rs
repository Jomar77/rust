fn main() {
    let mut hello: Vec<i32> = (0.. 10).collect();

    fn dosmthn(val: & Vec<i32>) {
        println!("{}", val.len());
    }
        
    dosmthn(&hello);
}
