use clap::{Parser,Command};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    println!("this prints");
    let args = Args::parse();

    println!("url: {}", args.url);
    println!("follow redirects: {}", args.follow_redirects);

    let result = reqwest::get(args.url).await?;
    

    let body = result.text().await?;

    println!("{}", body);
    
    Ok(())
}

#[derive(Parser,Debug)]
#[command(version,about)]
struct Args {
    url: String,

    #[arg(short,long,action)]
    follow_redirects: bool,
}

#[cfg(test)]
mod tests {

}

