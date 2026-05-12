struct Bill {
    name: String,
    amount: f64,
}
fn main(){
    let mut bills= Vec::new();
    loop{
        println!("\n===Bill Manager===");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit"); 
        println!("Enter your choice:");       
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
      
        match choice{
            1 =>add_bill(&mut bills),
            2 => view_bills(&bills),
            3 => remove_bill(&mut bills),
            4 => edit_bill(&mut bills),
            5 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
fn add_bill(bills: &mut Vec<Bill>){
    let mut name = String::new();
    let mut amount = String::new();
    println!("Enter bill name:");
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter bill amount:");
    std::io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: f64 = amount.trim().parse().expect("Please enter a valid number");
    bills.push(Bill { name: name.trim().to_string(), amount });
}
fn view_bills(bills: &Vec<Bill>){
    if bills.is_empty() {
        println!("No bills to display.");
        return;
    }
    println!("\n===Bills List===");
    for (index, bill) in bills.iter().enumerate() {
        println!("{}: {} - ksh{}", index + 1, bill.name, bill.amount);
    }
}
fn remove_bill(bills: &mut Vec<Bill>){
    let mut index = String::new();
    println!("Enter the number of the bill to remove:");
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please enter a valid number");
    if index > 0 && index <= bills.len() {       
        bills.remove(index - 1);
    } else {
        println!("Invalid bill number.");
    }
}
fn edit_bill(bills: &mut Vec<Bill>){
    let mut index = String::new();
    println!("Enter the number of the bill to edit:");
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please enter a valid number");
    if index > 0 && index <= bills.len() {        
        let mut name = String::new();
        let mut amount = String::new();
        println!("Enter new bill name:");
        std::io::stdin().read_line(&mut name).expect("Failed to read line");
        println!("Enter new bill amount:");
        std::io::stdin().read_line(&mut amount).expect("Failed to read line");
        let amount: f64 = amount.trim().parse().expect("Please enter a valid number");
        bills[index - 1] = Bill { name: name.trim().to_string(), amount };
    } else {
        println!("Invalid bill number.");
    }
}               