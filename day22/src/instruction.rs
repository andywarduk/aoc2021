use super::cube::Cube;

#[derive(Debug)]
pub struct Instruction {
    pub on: bool,
    pub cube: Cube
}
