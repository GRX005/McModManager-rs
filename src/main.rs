fn main() {
    println!("Starting McModManager.");
    let szam = Num {num1:24,num2:75};
    println!("{}",szam.add());
    println!("{}",Cucc(szam));
    let sz = Num2 {num1:64.321134,num2:13.438865};
    print!("{}",Cucc(sz));
}

fn Cucc(arg: impl Calc)->i64 {
    arg.mult()
}

struct Num {
    num1:i64,
    num2:i64,
}

struct Num2 {
    num1:f64,
    num2:f64,
}

impl Num {
    fn add(&self)->i64 {
        self.num1+self.num2
    }
}

trait Calc {
    fn mult(&self) -> i64;
}

impl Calc for Num {
    fn mult(&self) -> i64 {
        self.num1*self.num2
    }
}

impl Calc for Num2 {
    fn mult(&self) -> i64 {
        (self.num2*self.num1) as i64
    }
}