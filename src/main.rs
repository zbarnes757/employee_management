mod company;
mod questions;
use crate::company::Company;
use crate::questions::Questioner;
use std::io;

fn main() {
  let mut company = Company::new();
  let stdio = io::stdin();
  let input = stdio.lock();

  let output = io::stdout();

  let mut questioner = Questioner::new(input, output);

  loop {
    let initial_input = questioner.present_options();

    match initial_input.as_ref() {
      "1" => match questioner.get_employee_to_add() {
        Ok((employee, department)) => company.add_employee_to_department(department, employee),
        Err(message) => println!("{}", message),
      },
      "2" => {
        let department = questioner.ask_for_department();
        company.print_employees_in_department(department);
      }
      "3" => company.print_all_employees(),
      "4" => {
        println!("Goodbye!");
        break;
      }
      _ => println!("Invalid option. Try again."),
    }
  }
}
