use std::io;

pub struct Questions {}

impl Questions {
  pub fn present_options() -> String {
    let mut user_input = String::new();

    println!("Please select the option you would like to do (number only!):");
    println!("1. Add employee to department");
    println!("2. Print all employees in a department");
    println!("3. Print all employees in the company");
    println!("4. Exit program");

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read line");

    user_input.trim().to_string()
  }

  pub fn get_employee_to_add() -> Result<(String, String), String> {
    let mut user_input = String::new();

    println!("Use the format: Add _name_ to _department_");

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read line");

    let string_array: Vec<&str> = user_input.split(" ").collect();
    if string_array.len() != 4 {
      Err(String::from("Invalid format!"))
    } else {
      Ok((
        string_array[3].trim().to_string(),
        string_array[1].trim().to_string(),
      ))
    }
  }

  pub fn ask_for_department() -> String {
    let mut user_input = String::new();

    println!("Which department?");

    io::stdin()
      .read_line(&mut user_input)
      .expect("Unable to read line");

    user_input.trim().to_string()
  }
}
