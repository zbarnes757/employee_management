use std::io::{BufRead, Write};

const INITIAL_OPTIONS: &str = "
Please select the option you would like to do (number only!):
\t1. Add employee to department
\t2. Print all employees in a department
\t3. Print all employees in the company
\t4. Exit program
";

pub struct Questioner<R, W> {
  reader: R,
  writer: W,
}

impl<R, W> Questioner<R, W>
where
  R: BufRead,
  W: Write,
{
  pub fn new(reader: R, writer: W) -> Questioner<R, W> {
    Questioner { reader, writer }
  }
  pub fn present_options(&mut self) -> String {
    let mut user_input = String::new();

    write!(&mut self.writer, "{}", INITIAL_OPTIONS).expect("enable to write!");

    self
      .reader
      .read_line(&mut user_input)
      .expect("Unable to read line");

    user_input.trim().to_string()
  }

  pub fn get_employee_to_add(&mut self) -> Result<(String, String), String> {
    let mut user_input = String::new();

    write!(
      &mut self.writer,
      "\tUse the format: Add _name_ to _department_\n"
    )
    .expect("enable to write!");

    self
      .reader
      .read_line(&mut user_input)
      .expect("Unable to read line");

    let string_array: Vec<&str> = user_input.split(" ").collect();
    if string_array.len() != 4 {
      Err(String::from("Invalid format!"))
    } else {
      Ok((
        string_array[1].trim().to_string(),
        string_array[3].trim().to_string(),
      ))
    }
  }

  pub fn ask_for_department(&mut self) -> String {
    let mut user_input = String::new();

    write!(&mut self.writer, "\tWhich department?\n").expect("enable to write!");

    self
      .reader
      .read_line(&mut user_input)
      .expect("Unable to read line");

    user_input.trim().to_string()
  }
}

#[test]
fn test_get_employee_to_add() {
  let input = b"Add zac to mspf";
  let output = Vec::new();

  let mut questioner = Questioner::new(&input[..], output);

  let result = questioner.get_employee_to_add();

  assert!(result.is_ok());
  assert_eq!(result.unwrap(), (String::from("zac"), String::from("mspf")));

  let input = b"not_valid";
  let output = Vec::new();

  let mut questioner = Questioner::new(&input[..], output);

  let result = questioner.get_employee_to_add();
  assert!(result.is_err());
}

#[test]
fn test_ask_for_department() {
  let input = b"mspf";
  let output = Vec::new();

  let mut questioner = Questioner::new(&input[..], output);

  let result = questioner.ask_for_department();

  assert_eq!(result, String::from("mspf"));
}
