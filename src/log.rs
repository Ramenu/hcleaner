

pub fn terminate(message: &str) -> !
{
    eprintln!("error: {}", message);
    std::process::exit(1);
}