use combine::{many, many1, Parser};
use combine::parser::char::{char, digit, spaces};


fn main() {



    let binding = "1:10 @1 #20 $10; 20:4 @1 5 #2 $3 ;".replace('\n',"");

    println!("{:?}",parse(binding.to_string()));

}

pub struct emulate{
    program:Vec<Label>,
    registers:[u64;20],
    mem:[u64;33554432],
    stack:[u64;5767168],
    pc:(usize,usize),





}



impl emulate {


    fn  panic(&self){

    }

    fn get(&self,n:&Node)->&u64{
        match n {
            Node::Number(a) => {return &a}
            Node::Reg(b) => {&self.registers[b]}
            Node::Address(b) => {&self.mem[b]}
        }
    }
    fn set(&self,n:&Node)->&mut u64{
        match n {
            Node::Number(a) => {panic!()}
            Node::Reg(a) => {&mut self.registers[a]}
            Node::Address(a) => {&mut self.mem[a]}
        }
    }

    fn run(&mut self){
        let z = &self.program[self.pc.0].inside[self.pc.1];
        match z.num {
            0=>{
                self.pc =* self.call_stack.last().unwrap();
                 return;
            }
            1=>{
               if(z.args.len() != 2){
                  self.panic();
                   return;
               }
                let mut fir = self.set(&z.args[0]);
                let zir = self.get(&z.args[1]);
                *fir = *zir;
            }
            2=>{
                if(z.args.len() != 1){
                    self.panic();
                    return;
                }

                self.pc = (*self.get(&z.args[0]) as usize,0)

            }
        }
    }
}

fn parse(s:String)->Vec<Label>{
    let mut integer =many1(digit::<&str>()).map(|f:String|f.parse::<u64>().unwrap());
    let mut byte = combine::count(2,digit::<&str>()).map(|f:String|f.parse::<u8>().unwrap());
    let mut code =integer.clone().map(|f|f);
    let mut int = char('#').and(integer.clone().map(|f|Node::Number((f)))).map(|f|f.1);
    let mut reg = char('@').and(byte.map(|f|Node::Reg(f))).map(|f|f.1);
    let mut add = char('$').and(integer.clone().map(|f|Node::Address(f))).map(|f|f.1);
    let mut sinle = code
        .and(spaces())
        .map(|f|f.0)
        .and(many(int
            .or(add).or(reg)
            .and(spaces())
            .map(|f|f.0))
            .map(|f:Vec<Node>|f))
        .map(|f|
            Code{num:f.0,args:f.1});
    let mut main = many(sinle.and(spaces()).map(|f|f.0)).map(|z:Vec<Code>|z);

    let mut label = many1(integer.and(char(':'))
        .and(spaces())
        .and(main)
        .map(|f|
            Label{num:f.0.0.0,inside:f.1}))
        .and(char(';')
            .and(spaces())
            .map(|z|z))
        .map(|z|z.0)
        .map(|z:Vec<Label>|z);


    return label.parse(s.as_str()
        .trim()).unwrap().0

}


#[derive(Debug,Clone)]
pub enum Node{
    Number(u64),
    Reg(u8),
    Address(u64),
}

#[derive(Debug)]


pub struct Code{
    num:u64,
    args:Vec<Node>
}

#[derive(Debug)]

pub struct Label{
    num:u64,
    inside:Vec<Code>,
}

fn get_input()->String{
    let mut input = String::new();
    let _ =std::io::stdin().read_line(&mut input).unwrap();
    input
}
