use std::collections::HashMap;

pub struct Company {
  pub departments: HashMap<String, Vec<String>>,
}

impl Company {
  pub fn new() -> Company {
    Company {
      departments: HashMap::new(),
    }
  }

  pub fn add_employee_to_department(&mut self, department: String, employee: String) {
    self
      .departments
      .entry(department)
      .or_insert(Vec::new())
      .push(employee)
  }

  pub fn print_employees_in_department(&self, department: String) {
    match self.departments.get(&department) {
      Some(department_list) => println!("{}", department_list.join(", ")),
      None => println!("No one in that department!"),
    }
  }

  pub fn print_all_employees(&self) {
    let mut employee_list: Vec<String> = Vec::new();

    for (_, value) in &self.departments {
      employee_list.extend(value.iter().cloned());
    }

    employee_list.sort();
    employee_list.dedup();

    println!("{}", employee_list.join(", "))
  }
}
