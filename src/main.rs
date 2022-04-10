use colored::*;
use game::GameData;
use rand::Rng;
use std::io;
mod enums;

fn main() {
    let mut me = GameData {
        hp: 100,
        name: String::from("George"),
    };
    let mut enemy = GameData {
        hp: rand::thread_rng().gen_range(100..150),
        name: String::from("The Enemy"),
    };
    println!("Hello {}! Your HP is {}!", me.name.green(), me.show_hp());
    println!("Your {} HP is {}!", "Enemy".red(), enemy.show_hp());
    println!("To hit your enemy type 'hit'. To heal yourself type 'heal'");
    println!("Make a move!");
    loop {
        i_make_a_move(&mut me, &mut enemy);
        let my_hp = me.show_hp();
        let enemy_hp = enemy.show_hp();

        match tell_us_who_died(my_hp, enemy_hp) {
            enums::Dead::Me => break,
            enums::Dead::Both => break,
            enums::Dead::Enemy => break,
            enums::Dead::Nobody => println!("Enemy makes a move!"),
        };

        rust_makes_move(&mut me, &mut enemy);
        let my_hp = me.show_hp();
        let enemy_hp = enemy.show_hp();

        match tell_us_who_died(my_hp, enemy_hp) {
            enums::Dead::Me => break,
            enums::Dead::Both => break,
            enums::Dead::Enemy => break,
            enums::Dead::Nobody => println!("Your turn to go!"),
        };
    }
}

fn tell_us_who_died(my_hp: i32, enemy_hp: i32) -> enums::Dead {
    if my_hp <= 0 {
        println!(
            "You are dead. Your hp is {}! Enemy hp is {}. You lose!",
            my_hp, enemy_hp
        );
        enums::Dead::Me
    } else if enemy_hp <= 0 {
        println!("Enemy is dead. Enemy's hp is {}! You win!", enemy_hp);
        enums::Dead::Enemy
    } else if my_hp <= 0 && enemy_hp <= 0 {
        println!("You and your enemy are dead. I don't know how it is possible!");
        enums::Dead::Both
    } else {
        enums::Dead::Nobody
    }
}

fn i_make_a_move(me: &mut GameData, enemy: &mut GameData) {
    let mut my_input = String::new();

    io::stdin()
        .read_line(&mut my_input)
        .expect("Failed to read line");

    let random_int = generate_random(50, 150);

    if my_input.starts_with("heal") {
        me.heals_itself(random_int);
        println!(
            "{} heals! {}'s hp is {}",
            me.name.green(),
            me.name.green(),
            me.show_hp()
        );
    } else if my_input.starts_with("hit") {
        enemy.takes_damage(random_int);
        println!(
            "{} hits {} with {} hitpoints! {}'s hp is {}",
            me.name.green(),
            enemy.name.red(),
            random_int,
            enemy.name.red(),
            enemy.show_hp()
        );
    } else {
        println!("You must begin your actions with either 'hit' or 'heal'!")
    }
}

fn rust_makes_move(me: &mut GameData, enemy: &mut GameData) {
    let random_int = generate_random(50, 150);
    let heal_or_hit = generate_random(1, 3);
    if heal_or_hit == 1 {
        enemy.heals_itself(random_int);
        println!(
            "{} heals! {}'s hp is {}",
            enemy.name.red(),
            enemy.name.red(),
            enemy.show_hp()
        );
    } else if heal_or_hit == 2 {
        me.takes_damage(random_int);
        println!(
            "{} hits {} with {} hitpoints! {}'s hp is {}",
            enemy.name.red(),
            me.name.green(),
            random_int,
            me.name.green(),
            me.show_hp()
        );
    }
}

fn generate_random(start: i32, end: i32) -> i32 {
    rand::thread_rng().gen_range(start..end)
}

// fn parse_int(input: String) -> i32 {
//     let my_inputs: Vec<&str> = input.split(" ").collect();
//     let hit: i32 = my_inputs[1]
//         .trim()
//         .parse()
//         .expect("cannot parse a digit from hit");
//     hit
// }
