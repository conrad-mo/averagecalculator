use std::io;
fn main() {
    println!("Hello, welcome to Conrad's course average calculator");
    println!("Enter the number of assignments/quizzes/tests");
    let mut numberassignments = String::new();
    io::stdin()
    .read_line(&mut numberassignments)
    .expect("Failed to read line");
    let _numberassignments: i32 = numberassignments.trim().parse().expect("Please type a number!");
    let mut i = 0;
    let mut totalgrade = 0;
    let mut totalweight = 0;
    while i < _numberassignments{
        println!("What is your grade for this assignment? ({})", i+1);
        let mut grade = String::new();
        io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read line");
        let grade: i32 = grade.trim().parse().expect("Please input a number");
        println!("What is the weight for this assignment ({})", i+1);
        let mut weight = String::new();
        io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");
        let weight: i32 = weight.trim().parse().expect("Please input a number");
        totalgrade = totalgrade + grade;
        totalweight = totalweight + weight;
        i = i + 1;
    }
    let finalgrade = totalgrade/totalweight;
    println!("Your current grade in this course is {}", finalgrade);
}