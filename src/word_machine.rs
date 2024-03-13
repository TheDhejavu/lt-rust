pub fn word_machine(operations: String) -> i32 {
    let mut stack: std::collections::VecDeque<String> = std::collections::VecDeque::new();
  
    for op in operations.split_whitespace()  {
      
      match op {
        "POP" => {
          println!("{:?}", stack);
          _ = stack.pop_back()
        },
        "DUP" => {
          if let Some(last_item) = stack.back() {
             _ = stack.push_back(last_item.clone());
          } 
        },
        "-" => {
          let first = stack.pop_back().unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);
            let second = stack.pop_back().unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);

          let parsed_string : String  = (first - second).to_string();
          _ = stack.push_back(parsed_string);
        },
        "+" => {
            let first = stack.pop_back().unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);
            let second = stack.pop_back().unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);

          let parsed_string : String  = (first + second).to_string();
          _ = stack.push_back(parsed_string);
          
        },
        _ => {
          
          let parsed_number: Result<i32, _> = op.parse();
          match parsed_number {
              Ok(_) => {
                
                _ = stack.push_back(op.to_string())
              },
              Err(e) => println!("Failed to parse: {:?}", e),
          }
        },
      }
  }

  
  if let Some(last_item) = stack.back() {
    return last_item.parse::<i32>().unwrap_or(0);
  } 
  return 1;
}


