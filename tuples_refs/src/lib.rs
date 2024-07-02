#[derive(Debug, PartialEq)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}
