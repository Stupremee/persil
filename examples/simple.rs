use persil::trace;

#[derive(Debug)]
enum Expr {
    Number(i64),
    Identifier(String),
}

fn main() {
    // First, we initialize `persil` to let it know
    // where to store the profiling results.
    //
    // They will be stored at `./trace/<app-name>-<pid>`
    persil::init("parser");

    // Now we enable the profiler.
    // If we don't call this function, no results will be emitted.
    persil::enable();

    let result = parse_expression("1234");
    match result {
        Ok(expr) => println!("got expr: {:?}", expr),
        Err(msg) => eprintln!("failed to parse expr: {}", msg),
    }
}

fn parse_expression(input: &str) -> Result<Expr, &'static str> {
    // We will trace the `parse_expression` function using this.
    let _p = trace("parsing", "expression");

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
