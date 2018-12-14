use std::collections::HashMap;
use std::io;

fn main() {
  let mut company = HashMap::new();
  start_program(&mut company)
}

fn start_program(company: &mut HashMap<String, Vec<String>>) {
  loop {
    let initial_input = present_options();

    match initial_input.as_ref() {
      "1" => {
        let (department, employee) = get_employee_to_add();
        add_employee_to_department(company, department, employee);
      }
      "2" => {
        let department = ask_for_department();
        print_employees_in_department(company, department);
      }
      "3" => print_all_employees(company),
      "4" => {
        println!("Goodbye!");
        break;
      }
      _ => println!("Invalid option. Try again."),
    }
  }
}

fn present_options() -> String {
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

fn get_employee_to_add() -> (String, String) {
  let mut user_input = String::new();

  println!("Use the format: Add _name_ to _department_");

  io::stdin()
    .read_line(&mut user_input)
    .expect("Unable to read line");

  // TODO: handle when input is invalid
  let string_array: Vec<&str> = user_input.split(" ").collect();
  (
    string_array[3].trim().to_string(),
    string_array[1].trim().to_string(),
  )
}

fn add_employee_to_department(
  company: &mut HashMap<String, Vec<String>>,
  department: String,
  employee: String,
) {
  let department = company.entry(department).or_insert(Vec::new());
  department.push(employee);
}

fn ask_for_department() -> String {
  let mut user_input = String::new();

  println!("Which department?");

  io::stdin()
    .read_line(&mut user_input)
    .expect("Unable to read line");

  user_input.trim().to_string()
}

fn print_employees_in_department(company: &mut HashMap<String, Vec<String>>, department: String) {
  match company.get(&department) {
    Some(department_list) => println!("{}", department_list.join(", ")),
    None => println!("No one in that department!"),
  }
}

fn print_all_employees(company: &mut HashMap<String, Vec<String>>) {
  let mut employee_list: Vec<String> = Vec::new();

  for (_, value) in company {
    employee_list.extend(value.iter().cloned());
  }

  employee_list.sort();
  employee_list.dedup();

  println!("{}", employee_list.join(", "))
}
