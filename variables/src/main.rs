const STARTING_MISSLES : i32 = 8;
const READY_AMOUNT : i32 = 2;
fn main() {
    let (mut missiles, ready) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} Misslies", ready, missiles);
    missiles = missiles - ready;
    println!("{} Missiles Left!", missiles)
}
