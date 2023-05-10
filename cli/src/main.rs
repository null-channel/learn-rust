use std::io::{self, Write};


fn main() -> anyhow::Result<()> {
  
  let opt = get_optional(2);
  if let Some(i) = opt {
    println!("i: {}", i);
  }

  match opt {
    None => println!("i was none from match"),
    Some(i) => println!("iii: {}", i)
  }

  let Some(i) = opt else {
    println!("i was none");
    return Ok(());
  };
  println!("ii: {}", i);



  let month = might_fail_string("marek".into())?;

  println!("the month: {}", month);

  Ok(())
}

fn get_optional(i: i32) -> Option<i32> {
    if i > 5 {
        return Some(i);
    }
    None
}

fn might_fail(i: i32) -> anyhow::Result<String> {
    match i {
        1 => Ok("January".to_string()),
        2 => Ok("Feb".to_string()),
        _ => Err(anyhow::anyhow!("not a month")),
    }
}

fn might_fail_string(s: String) -> anyhow::Result<String> {
    match s.as_str() {
        "January" => Ok("1".to_string()),
        "Feb" => Ok("2".to_string()),
        _ => Err(anyhow::anyhow!("Not a month")),
    }
}

fn map_int_to_date_string(i: i32) -> anyhow::Result<String> {
    let month = might_fail(i)?;
    might_fail_string(month)
}
      
fn get_input() -> String {
    print!("Please pick a number: ");
    let _ = io::stdout().flush();
    let mut user_input = String::new();
    let stdin = io::stdin();
    let read_ret = stdin.read_line(&mut user_input);
    println!("");
    match read_ret {
        Ok(u) => println!("there where {} characters read", u),
        Err(e) => println!("there was an error: {}", e)
    }
    user_input
}


#[cfg(test)]
mod tests {
   

}