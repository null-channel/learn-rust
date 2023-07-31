use clap::Parser;

fn main() -> Result<(),reqwest::Error> {
    let args = Args::parse();

    println!("url: {}", args.url);
    println!("follow redirects: {}", args.follow_redirects);

    let body = reqwest::blocking::get(args.url)?.text()?;

    println!("body: {}", body);

    Ok(())
}

#[derive(Parser,Debug)]
struct Args {
    url: String,
    #[arg(short,long,action)]
    follow_redirects: bool,
}
