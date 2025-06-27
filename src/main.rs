fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("This program accepts only 1 argument, the expression itself");
        return Ok(());
    }
    args.next(); // consume path

    let expr = args
        .next()
        .expect("Expected an expression as a first argument");

    let result = rexpr::eval(&expr)?;
    println!("{}", result);
    Ok(())
}
