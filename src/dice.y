%start Expr
%%
Expr -> Result<(i64, String), ()>:
      Expr '+' Term { 
            Ok(($1.clone()?.0 + $3.clone()?.0, 
                format!("{} + {}", $1.clone()?.1, $3.clone()?.1))) 
    }
    | Expr '-' Term { 
            Ok(($1.clone()?.0 - $3.clone()?.0, 
                format!("{} - {}", $1.clone()?.1, $3.clone()?.1))) 
    }
    | Term { $1 }
    ;

Term -> Result<(i64, String), ()>:
      Term '*' Factor { 
            Ok(($1.clone()?.0 * $3.clone()?.0, 
                format!("{} * {}", $1.clone()?.1, $3.clone()?.1))) 
    }
    | Term '/' Factor { 
            Ok(($1.clone()?.0 / $3.clone()?.0, 
                format!("{} / {}", $1.clone()?.1, $3.clone()?.1))) 
    }
    | Factor { $1 }
    ;

Factor -> Result<(i64, String), ()>: 
      '(' Expr ')' { Ok(( $2.clone()?.0, format!("({})", $2.clone()?.1))) }
    | 'INT' 'd' 'INT' {
           let number_v = $1.map_err(|_| ())?;
           let number   = parse_int($lexer.span_str(number_v.span()));
           let sides_v  = $3.map_err(|_| ())?;
           let sides    = parse_int($lexer.span_str(sides_v.span()));
           roll(number.unwrap().0, sides.unwrap().0)
      }
    | 'INT' {
           let v = $1.map_err(|_| ())?;
           parse_int($lexer.span_str(v.span()))
      }
    ;
%%
use rand::Rng;

fn parse_int(s: &str) -> Result<(i64, String), ()> {
    match s.parse::<i64>() {
        Ok(val) => Ok((val, String::from(s))),
        Err(_)  => {
            eprintln!("{} cannot be represented as a i64", s);
            Err(())
        }
    }
}

fn roll(number: i64, sides: i64) -> Result<(i64, String), ()> {
    let mut total: i64 = 0;
    let mut s_output: String = format!("{}d{} (", number, sides);
    for i in 0..number {
        let roll = rand::thread_rng().gen_range(1..sides+1);
        total += roll;
        s_output.push_str(format!("{}", roll).as_str());
        if i < number - 1 {
            s_output.push_str(", ");
        }
    }
    s_output.push_str(")");
    Ok((total, s_output))
}
