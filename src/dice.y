%start Expr
%%
Expr -> Result<i64, ()>:
      Expr '+' Term { Ok($1? + $3?) }
    | Expr '-' Term { Ok($1? - $3?) }
    | Term { $1 }
    ;

Term -> Result<i64, ()>:
      Term '*' Factor { Ok($1? * $3?) }
    | Term '/' Factor { Ok($1? / $3?) }
    | Factor { $1 }
    ;

Factor -> Result<i64, ()>: 
      '(' Expr ')' { $2 }
    | 'INT' 'd' 'INT' {
           let number_v = $1.map_err(|_| ())?;
           let number   = parse_int($lexer.span_str(number_v.span()));
           let sides_v  = $3.map_err(|_| ())?;
           let sides    = parse_int($lexer.span_str(sides_v.span()));
           roll(number.unwrap(), sides.unwrap())
      }
    | 'INT' {
           let v = $1.map_err(|_| ())?;
           parse_int($lexer.span_str(v.span()))
      }
    ;
%%
use rand::Rng;

fn parse_int(s: &str) -> Result<i64, ()> {
    match s.parse::<i64>() {
        Ok(val) => Ok(val),
        Err(_)  => {
            eprintln!("{} cannot be represented as a i64", s);
            Err(())
        }
    }
}

fn roll(number: i64, sides: i64) -> Result<i64, ()> {
    let mut total: i64 = 0;
    for _i in 0..number {
        total += rand::thread_rng().gen_range(1..sides+1)
    }
    Ok(total)
}
