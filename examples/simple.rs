use persil::Profiler;

#[derive(Debug)]
enum Expr {
    Number(i64),
    Identifier(String),
}

fn main() {
    let profiler = Profiler::from_name("parser").expect("failed to create profiler");
    profiler.enable();

    let result = parse_expression(&profiler, "1234");
    match result {
        Ok(expr) => println!("got expr: {:?}", expr),
        Err(msg) => eprintln!("failed to parse expr: {}", msg),
    }
}

fn parse_expression(profiler: &Profiler, input: &str) -> Result<Expr, &'static str> {
    // We will trace the `parse_expression` function using this.
    let _p = profiler.trace("parsing", "expression");

    input
        .parse::<i64>()
        .map(|num| Expr::Number(num))
        .or_else(|_| {
            if input.chars().all(char::is_alphabetic) {
                Ok(Expr::Identifier(input.to_string()))
            } else {
                Err("invalid identifier")
            }
        })
        .map_err(|_| "invalid expression")
}
