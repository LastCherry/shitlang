use combine::{many1, Parser};
use combine::parser::char::{digit, spaces};

fn main() {
    let mut int =many1(digit::<&str>()).map(|f:String|f);
    let mut code =many1(int.and(spaces()).map(|f|f.0)).map(|f:Vec<String>|f).map(|f|f.iter().map(|z|z.parse::<i64>().unwrap()).collect::<Vec<i64>>());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut data = input.split(',').map(|f|f.trim()).map(|f|code.parse(f).unwrap().0).collect::<Vec<Vec<i64>>>();

    println!()

}
