struct Parser<'a> {
    curr: usize,
    input: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
enum Inst {
    Mul(usize, usize),
    Do,
    Dont,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Parser { curr: 0, input }
    }

    fn parse(&mut self) -> Vec<Inst> {
        let mut result = Vec::new();
        while self.curr + 2 < self.input.len() {
            let substr = &self.input[self.curr..((self.curr + 7).min(self.input.len()))];
            match substr {
                b"don't()" => result.push(Inst::Dont),
                [b'd', b'o', b'(', b')', ..] => result.push(Inst::Do),
                [b'm', b'u', b'l', b'(', ..] => {
                    self.curr += 4;
                    if let Some(params) = self.parse_params() {
                        result.push(Inst::Mul(params.0, params.1));
                    }
                }
                _ => {}
            };
            self.curr += 1;
        }
        result
    }

    fn parse_params(&mut self) -> Option<(usize, usize)> {
        let start = self.curr;
        self.advance_while(|c| c.is_ascii_digit());
        let param_a = self.parse_num(start);
        if !self.expect(b',') {
            return None;
        }
        self.curr += 1;
        let start = self.curr;
        self.advance_while(|c| c.is_ascii_digit());
        let param_b = self.parse_num(start);
        if !self.expect(b')') {
            return None;
        }
        match (param_a, param_b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }

    fn parse_num(&self, start: usize) -> Option<usize> {
        std::str::from_utf8(&self.input[start..self.curr])
            .ok()
            .and_then(|digits| digits.parse::<usize>().ok())
    }

    fn expect(&mut self, byte: u8) -> bool {
        self.input[self.curr] == byte
    }

    fn advance_while<F: Fn(u8) -> bool>(&mut self, pred: F) {
        while pred(self.input[self.curr]) {
            self.curr += 1;
        }
    }
}

fn parse(input: &str) -> Vec<Inst> {
    Parser::new(input.as_bytes()).parse()
}

#[cfg(test)]
mod tests {
    use crate::day03::Inst;
    use crate::day03::Parser;

    #[test]
    fn parse_happy() {
        let value = Parser::new(b"mul(1,2)").parse();
        assert_eq!(value, vec![Inst::Mul(1, 2)])
    }

    #[test]
    fn parse_multiple() {
        let value = Parser::new(b"mul(1,2)asdfasdlkfjklsdflakjmul(4,5)sdafa").parse();
        assert_eq!(value, vec![Inst::Mul(1, 2), Inst::Mul(4, 5)])
    }
}

pub fn part_one(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|inst| match inst {
            Inst::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let instructions = parse(input);
    let mut sum = 0;
    let mut take = true;
    for inst in instructions {
        match inst {
            Inst::Mul(a, b) if take => {
                sum += a * b;
            }
            Inst::Do => take = true,
            Inst::Dont => take = false,
            _ => {}
        }
    }
    sum
}
