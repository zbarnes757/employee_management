mod company;
mod questions;
use crate::company::Company;
use crate::questions::Questions;

fn main() {
  let mut company = Company::new();
  start_program(&mut company)
}

fn start_program(company: &mut Company) {
  loop {
    let initial_input = Questions::present_options();

    match initial_input.as_ref() {
      "1" => match Questions::get_employee_to_add() {
        Ok((employee, department)) => company.add_employee_to_department(department, employee),
        Err(message) => println!("{}", message),
      },
      "2" => {
        let department = Questions::ask_for_department();
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
