type BrainType = u8;
type PointerType = u16;
const BRAINSIZE:usize = 65536;
struct Brainstruct {
    tape: [BrainType;BRAINSIZE],
    pointer:PointerType
}
impl Brainstruct {
    fn new() -> Self {
        Brainstruct { tape: [BrainType::default();BRAINSIZE], pointer: 0 }
    }
    fn add(&mut self) {
        self.tape[self.pointer as usize] = self.tape[self.pointer as usize].wrapping_add(1)
    }
    fn sub(&mut self) {
        self.tape[self.pointer as usize] = self.tape[self.pointer as usize].wrapping_sub(1)
    }
    fn p_add(&mut self) {
        self.pointer = self.pointer.wrapping_add(1)
    }
    fn p_sub(&mut self) {
        self.pointer = self.pointer.wrapping_sub(1)
    }
    fn print(&mut self) -> BrainType {
        self.tape[self.pointer as usize]
    }
    fn stdin(&mut self,c:BrainType) {
        self.tape[self.pointer as usize] = self.tape[self.pointer as usize].wrapping_add(c)
    }
}

pub fn brainfuck(str:Option<String>,stdin:Option<String>) -> Result<String, &'static str> {
    let input = (str.unwrap_or_default(),stdin.unwrap_or_default());
    let (chariter,mut stditer) = (input.0.as_bytes(),input.1.chars());
    let mut brain = Brainstruct::new();
    let mut output = String::with_capacity(BRAINSIZE);
    let mut sequence:Vec<usize>=vec![];
    let mut current: Option<&u8>;
    let mut index:usize = 0;
    loop {
        current = chariter.get(index);
        if current.is_none() {
            break;
        }
        match *current.unwrap() as char {
            '+' => brain.add(),
            '-' => brain.sub(),
            '>' => brain.p_add(),
            '<' => brain.p_sub(),
            '.' => output.push(brain.print() as char),
            ',' => brain.stdin(stditer.next().unwrap_or_default() as BrainType),
            '[' => {
                if brain.print()==0 {
                    let mut counter=1;
                    let mut temp_char;
                    index+=1;
                    while counter>0 {
                        temp_char = chariter.get(index);
                        if temp_char.is_none() {
                            return Err("while loop not ended")
                        }
                        match *temp_char.unwrap() as char {
                            '[' => counter+=1,
                            ']' => counter-=1,
                            _ => ()
                        }
                        index+=1
                    }
                    index-=1
                } else {
                    sequence.push(index)
                }
            },
            ']' => { match brain.print() {
                0 => {sequence.pop();},
                _ => { match sequence.last() {
                    None => (),
                    Some(value) => index = *value
                }}
            }}
            _ => (), //others chars are comments
        }
        index+=1
    }
    Ok(output)
}