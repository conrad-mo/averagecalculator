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
    let mut totalgrade = 0.0;
    let mut totalweight = 0.0;
    while i < _numberassignments{
        println!("What is your grade for this assignment? ({})", i+1);
        let mut grade = String::new(); //Grabs current grade
        io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read line");
        let grade: f32 = grade.trim().parse().expect("Please input a number"); //Reads line and converts string to float
        println!("What is the weight for this assignment ({})", i+1);
        let mut weight = String::new();
        io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");
        let weight: f32 = weight.trim().parse().expect("Please input a number");
        let gweight = weight * grade;
        totalgrade = totalgrade + gweight;
        totalweight = totalweight + weight;
        i = i + 1;
    }
    let finalgrade = totalgrade/totalweight;
    println!("Your current grade in this course is {}", finalgrade);
}