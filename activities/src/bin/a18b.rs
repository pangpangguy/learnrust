// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
enum Title{
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    Kitchen,
    Assembly,
}

enum Status{
    Active,
    Terminated,
}

struct Employee{
    title: Title,
    status: Status,
}

impl Employee{
    fn new(title: Title, status: Status)->Self{
        Self{title, status}
    }
}

fn check_access(employee: &Employee)->Result<(), String>{
    match employee.status{
        Status::Terminated => return Err("Status terminated".to_owned()),
        _ => (),
    }

    match employee.title{
        Title::Maintenance => Ok(()),
        Title::Marketing => Ok(()),
        Title::Manager => Ok(()),
        _ => Err("No authorization".to_owned())
    }
}

fn print_access(employee: &Employee)-> Result<(), String>{
    let access = check_access(employee)?;
    Ok(())
}

fn main() {
    let employees = vec![Employee::new(Title::Maintenance, Status::Active),
                         Employee::new(Title::Assembly, Status::Active),
                         Employee::new(Title::Marketing, Status::Terminated)];
    for emp in employees{
        match print_access(&emp){
            Err(error_msg) => println!("Access denied: {:?}",error_msg),
            Ok(()) => println!("Access granted")
        }
    }
}
