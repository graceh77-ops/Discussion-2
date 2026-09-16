/// Your crew's name. Both of you are going to change this line.
const CREW_NAME: &str = "the unnamed crew";

/// Your crew's motto. You will both change this one too, earlier and separately.
<<<<<<< HEAD
const MOTTO: &str = "people come and go";
=======
const MOTTO: &str = "Life is a game!";
>>>>>>> cea297f5c852cdd4b544d603b13a85c9bace5165

fn main() {
    println!("=== {} ===", CREW_NAME);
    println!();
    println!("Crew roster:");

    // ROSTER: replace the line below with one for yourself.
    println!("  (nobody has signed on yet)");

    println!();
    println!("Motto: {}", MOTTO);
    println!("Report any problems to whoever merged last.");
}
