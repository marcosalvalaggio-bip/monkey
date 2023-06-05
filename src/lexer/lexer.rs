pub struct Lexer {
    pub input: String, 
    pub position: usize, // current position in input (points to current char)
    pub readposition: usize, // current reading position in input (after current char)
    pub ch: u8, // current char under examination
}


impl Lexer {

    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    pub fn read_char(&mut self, ) {
        let mod_input: Vec<char> = self.input.chars().collect();
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = &mod_input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    
    }


}