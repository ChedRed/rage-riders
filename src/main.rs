fn main() {
    let mut health = 100;
    let armor = 10;

    let attack = 20;

    println!(
        "Health: {}\nEnemy attacks you for {} damage!\nYour armor blocks {} damage.",
        health, attack, armor
    );

    health -= attack - armor;

    println!("Your new health: {}", health);
}
