pub fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: blog-generator <config-file>".to_string());
    }
    Ok(())
}