const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    //let other_variable = 0;
    //READY_AMOUNT = 1; Error: cannot assign to this expression
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES,READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    //missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
