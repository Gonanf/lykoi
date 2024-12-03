use crate::structure::{self, tokenizer::{names, token}};


    pub fn get_types() -> Vec<token>{
        //DEBUG ONLY
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer);
        /////////////

        let mut tokens: Vec<structure::tokenizer::token> = Vec::new();
        for i in buffer.chars() {
            tokens.push(match (structure::tokenizer::token::search_token(i as u8)) {
                Some(a) => a,
                None => continue,
            });
        }

        //DEBUG ONLY
        dbg!(&tokens);
        ////////////
        return tokens;
    }

    pub fn agroup(tokens: Vec<token>){
        for tk in names::agroup_tokens(tokens){
            match tk {
                names::variable(a) => {println!("variable(\n{}\n)",String::from_utf8_lossy(&a))},
                names::literal(a) => {println!("literal(\n{}\n)",String::from_utf8_lossy(&a))},
                names::digits(a) => {println!("digits(\n{}\n)",String::from_utf8_lossy(&a))},
                names::EOF => println!("EOF"),
                names::operation(a) => {println!("operation(\n{}\n)",String::from_utf8_lossy(&a))},
                names::left_bracket => println!("{{"),
                names::right_bracket => println!("}}"),
            }
        }
    }


