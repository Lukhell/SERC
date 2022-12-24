pub struct Eval {
    input: String,
    output: f64,
    stack: Vec<f64>,
    start: usize,
    current: usize
}

impl Eval {
    pub fn new(input: String) -> Self {
        Self {
            input,
            output: 0.0,
            stack: vec![],
            start: 0,
            current: 0
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.input.chars().count()
    }
    
    pub fn advance(&mut self, k: u8) -> char {
        self.current += k;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    pub fn peek(&self, k: u8) -> char {
        self.input.chars().nth(self.current + k).unwrap()
    }

    pub fn eval(&mut self) {
        while !self.is_at_end() {
            let cc = self.advance();

            match cc { 
                '+' => match self.advance().to_string().parse::<f64>() {
                        
                    }
               _ => {} 
            }
        }
    }
}
