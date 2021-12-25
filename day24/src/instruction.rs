#[derive(Debug)]
pub enum Reg {
    W,
    X,
    Y,
    Z
}

#[derive(Debug)]
pub enum RegNum {
    Reg(Reg),
    Num(i64)
}

#[derive(Debug)]
pub enum Instruction {
    Inp(Reg),
    Add(Reg, RegNum),
    Mul(Reg, RegNum),
    Div(Reg, RegNum),
    Mod(Reg, RegNum),
    Eql(Reg, RegNum)
}
