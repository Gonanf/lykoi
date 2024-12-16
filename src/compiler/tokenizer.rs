/*
* Assign the source bytes a certain token type
*
* First step of the tokenization process
*/
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum token {
    digits(u8),
    literal_char(u8),
    literal_dec,
    EOF,
    space,
    newline,
    equal,
    minor,
    mayor,
    minus,
    plus,
    mult,
    div,
    left_bracket,
    right_bracket,
    left_par,
    right_par,
}
impl token {
    pub fn search_token(token_var: u8) -> Option<token> {
        match (token_var) {
            b'\"' => Some(token::literal_dec),
            b'\0' => Some(token::EOF),
            b' ' => Some(token::space),
            b'\n' => Some(token::newline),
            b'=' => Some(token::equal),
            b'<' => Some(token::minor),
            b'>' => Some(token::mayor),
            b'-' => Some(token::minus),
            b'+' => Some(token::plus),
            b'*' => Some(token::mult),
            b'/' => Some(token::div),
            b'{' => Some(token::left_bracket),
            b'}' => Some(token::right_bracket),
            b'0'..=b'9' => Some(token::digits(token_var)),
            b'(' => Some(token::left_par),
            b')' => Some(token::right_par),
            _ => Some(token::literal_char(token_var)),
        }
    }

    pub fn value(self) -> u8 {
        match self {
            token::digits(a) => a,
            token::literal_char(a) => a,
            token::literal_dec => b'\"',
            token::EOF => b'\0',
            token::space => b' ',
            token::newline => b'\n',
            token::equal => b'=',
            token::minor => b'<',
            token::mayor => b'>',
            token::minus => b'-',
            token::plus => b'+',
            token::mult => b'*',
            token::div => b'/',
            token::left_bracket => b'{',
            token::right_bracket => b'}',
            token::left_par => b'(',
            token::right_par => b')',
        }
    }
}

/*
* Agroup the tokens based on grammar meaning
*
* Second step of the tokenization process
*/
#[derive(Debug, Clone, PartialEq)]
pub enum names {
    variable(Vec<u8>, u32, u32),
    literal(Vec<u8>, u32, u32),
    digits(Vec<u8>, u32, u32),
    EOF(u32, u32),
    operation(Vec<u8>, u32, u32),
    left_bracket(u32, u32),
    right_bracket(u32, u32),
    left_par(u32, u32),
    right_par(u32, u32),
}

impl names {
    pub fn get_pos(self) -> (u32, u32) {
        match self {
            names::variable(.., line, col) => (line, col),
            names::literal(.., line, col) => (line, col),
            names::digits(.., line, col) => (line, col),
            names::EOF(.., line, col) => (line, col),
            names::operation(.., line, col) => (line, col),
            names::left_bracket(.., line, col) => (line, col),
            names::right_bracket(.., line, col) => (line, col),
            names::left_par(.., line, col) => (line, col),
            names::right_par(.., line, col) => (line, col),
        }
    }

    pub fn agroup_tokens(tokens: Vec<token>) -> Vec<names> {
        let mut group_tokens: Vec<names> = Vec::new();
        let mut buffered_token: token = token::EOF;
        let mut line = 0;
        let mut col = 0;
        for i in tokens {
            line += 1;
            if group_tokens.len() > 0 {
                let mut last = group_tokens.pop().unwrap();
                match last {
                    names::literal(ref mut a, ..) => {
                        if (a.len() == 1) || a.last().unwrap() != &token::literal_dec.value() {
                            a.push(i.value());
                            group_tokens.push(last);
                            continue;
                        }
                    }

                    names::digits(ref mut a, ..) => {
                        if (buffered_token != token::space && buffered_token != token::newline) {
                            match i {
                                token::digits(b) => {
                                    a.push(b);
                                    group_tokens.push(last);
                                    continue;
                                }
                                _ => (),
                            }
                        }
                    }

                    names::variable(ref mut a, ..) => {
                        if (buffered_token != token::space && buffered_token != token::newline) {
                            match i {
                                token::literal_char(b) => {
                                    a.push(b);
                                    group_tokens.push(last);
                                    continue;
                                }
                                _ => (),
                            }
                        }
                    }

                    names::operation(ref mut a, ..) => match i {
                        token::equal
                        | token::minor
                        | token::mayor
                        | token::minus
                        | token::plus
                        | token::mult
                        | token::div => {
                            a.push(i.value());
                            group_tokens.push(last);
                            continue;
                        }
                        _ => (),
                    },

                    _ => (),
                }
                group_tokens.push(last);
            }
            buffered_token = i;

            match i {
                token::literal_dec => group_tokens.push(names::literal(vec![i.value()], line, col)),
                token::digits(a) => group_tokens.push(names::digits(vec![a], line, col)),
                token::literal_char(a) => group_tokens.push(names::variable(vec![a], line, col)),
                token::EOF => return group_tokens,
                token::equal
                | token::minor
                | token::mayor
                | token::minus
                | token::plus
                | token::mult
                | token::div => group_tokens.push(names::operation(vec![i.value()], line, col)),
                token::left_bracket => group_tokens.push(names::left_bracket(line, col)),
                token::right_bracket => group_tokens.push(names::right_bracket(line, col)),
                token::left_par => group_tokens.push(names::left_par(line, col)),
                token::right_par => group_tokens.push(names::right_par(line, col)),
                token::newline => col += 1,
                _ => (),
            }
        }
        return group_tokens;
    }

    pub fn value(self) -> Vec<u8> {
        match self {
            names::variable(vec, ..) => vec,
            names::literal(vec, ..) => vec,
            names::digits(vec, ..) => vec,
            names::EOF(..) => b"\0".to_vec(),
            names::operation(vec, ..) => vec,
            names::left_bracket(..) => b"{".to_vec(),
            names::right_bracket(..) => b"}".to_vec(),
            names::left_par(..) => b"(".to_vec(),
            names::right_par(..) => b")".to_vec(),
        }
    }
}
