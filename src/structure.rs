pub mod tokenizer{
    #[derive(Debug)]
    pub enum token{
        digits(u8),
        literal_char(u8),
        literal_dec,
        EOF,
        equal,
        minor,
        mayor ,
        minus,
        plus,
        mult,
        div,
        left_bracket,
        right_bracket,
    }
    impl token{
        pub fn search_token(token_var: u8) -> Option<token>{
            match (token_var){
                b'\"' => Some(token::literal_dec),
                b'\0' => Some(token::EOF),
                b'=' => Some(token::equal),
                b'<' => Some(token::minor),
                b'>' => Some(token::mayor),
                b'-' => Some(token::minus),
                b'+' => Some(token::plus),
                b'*' => Some(token::mult),
                b'/' => Some(token::div),
                b'{' => Some(token::left_bracket),
                b'}' => Some(token::right_bracket),
                b'0'..b'1' => Some(token::digits(token_var)),
                b' ' => None,
                _ => Some(token::literal_char(token_var)),
            }
        }

        pub fn value(self) -> u8{
            match self {
                token::digits(a) => a,
                token::literal_char(a) => a,
                token::literal_dec => b'\"',
                token::EOF => b'\0',
                token::equal => b'=',
                token::minor => b'<',
                token::mayor => b'>',
                token::minus => b'-',
                token::plus => b'+',
                token::mult => b'*',
                token::div => b'/',
                token::left_bracket => b'{',
                token::right_bracket => b'}',
            }
        }
    }


    /*if_token,
        else_token,
        elif_token,
        while_token,
        for_token,
        return_token,
        break_token,
        continue_token,
        none_token,
        true_token,
        false_token, 
         variable(Vec<u8>),*/
    #[derive(Debug)]
    pub enum names{
        variable(Vec<u8>),
        literal(Vec<u8>),
        digits(Vec<u8>),
        EOF,
        operation(Vec<u8>),
        left_bracket,
        right_bracket,
    }

    impl names{
        pub fn agroup_tokens(tokens:Vec<token>) -> Vec<names>{
            let mut group_tokens: Vec<names> = Vec::new();
            for i in tokens{
                if group_tokens.len() > 0{
                    let mut last = group_tokens.pop().unwrap();
                match last {
                    names::literal(ref mut a) => {
                        if !names::is_literal_complete(group_tokens.last()){
                               a.push(i.value());
                               group_tokens.push(last);
                               continue;
                       }
                    }

                    names::digits(ref mut a) => {
                        match i {
                            token::digits(b) => {a.push(b);
                                group_tokens.push(last);
                                continue;},
                            _ => () 
                        }
                    }

                    names::variable(ref mut a) => {
                        match i {
                            token::literal_char(b) => {a.push(b);
                                group_tokens.push(last);
                                continue;},
                            _ => () 
                        }
                    }

                    names::operation(ref mut a) => {
                        if (a.last().unwrap() == &token::equal.value() || a.last().unwrap() == &token::minor.value() || a.last().unwrap() == &token::mayor.value()) && a.len() < 2 {
                            match i {
                                token::equal => {a.push(i.value());
                                    group_tokens.push(last);
                                    continue;},
                                _ => ()
                            }
                        }
                    }

                    _ => (),

                }
                group_tokens.push(last);
            }
                match i {
                    token::literal_dec => group_tokens.push(names::literal(vec![i.value()])),
                    token::digits(a) => group_tokens.push(names::digits(vec![a])),
                    token::literal_char(a) => group_tokens.push(names::variable(vec![a])),
                    token::EOF => return group_tokens,
                    token::equal | token::minor | token::mayor | token::minus | token::plus | token::mult | token::div => group_tokens.push(names::operation(vec![i.value()])),
                    token::left_bracket => group_tokens.push(names::left_bracket),
                    token::right_bracket => group_tokens.push(names::right_bracket),
                }
            }
            return group_tokens;
        }

        fn is_literal_complete(token_var: Option<&names>) -> bool{
            match token_var {
                Some(&names::literal(ref a)) => a.len() != 1 && a.last().unwrap() == &b'\"' ,
                Some(_) => true,
                None => true,
            }
        }
    }
}