// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student{
    name: String,
    locker: Option<i32>,
}
impl Student{
    fn new(name: String, locker: Option<i32>) -> Self{
        Self{name, locker}
    }
}
fn main() {
    let students = vec![Student::new("Alice".to_owned(), Some(45)), Student::new(("Bob").to_owned(), None)];
    for s in students{
        match s.locker{
            Some(locker_no) => println!("Locker number of {:?} is :{:?}", s.name, locker_no),
            None => println!("{:?} does not have a locker", s.name),
        }
    }
}
