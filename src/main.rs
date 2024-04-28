use rand::Rng;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;

fn input_form() -> Result<i32, io::Error> {
    println!("Please enter location:");
    let mut loc_input = String::new();
    io::stdin().read_line(&mut loc_input)?;
    let loc = match loc_input.trim().parse() {
        Ok(l) if l != 0 && l != 7 => l,
        Ok(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Hole location must not be 0 or 7",
            ));
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Hole location must be numeric",
            ));
        }
    };
    println!();
    println!();

    Ok(loc)
}

fn init_state() -> BTreeMap<i32, i32> {
    let mut state = BTreeMap::new();
    for i in 0..14 {
        let mut rng = rand::thread_rng();
        let p = rng.gen_range(1..5);

        state.insert(i, p);
    }

    state
}

fn generate_state() -> Result<BTreeMap<i32, i32>, io::Error> {
    let mut state = init_state();
    let mut sim_state = state.clone();
    let mut is_eligible_for_move: bool = false;
    let mut is_eligible_for_draw_ui: bool = false;
    let mut last_key: Option<i32> = None;
    let mut is_start: bool = true;

    let mut turn_a: bool = true;
    let mut csum = 0;
    for v in state.values() {
        csum += v;
    }

    println!("It is A's turn");

    loop {
        if is_start {
            draw_ui(&state);
        }

        let loc = input_form()?;
        let i = loc;
        
        let nbean = state.get(&loc).copied().unwrap_or(0);
        let sim_nbean = sim_state.get(&loc).copied().unwrap_or(0);
        
        if last_key == None {
            let (_, sim_last_key) = move_bean(sim_nbean, i, turn_a, &mut sim_state);
            last_key = Some(sim_last_key);

            state.insert(loc, 0);

            is_eligible_for_move = true;
            is_eligible_for_draw_ui = true;
        } else if i != last_key.expect("msg") {
            println!("You need to input {} on this turn", last_key.expect("msg"));

            is_eligible_for_move = false;
            is_eligible_for_draw_ui = true;
        } else if i == last_key.expect("msg") {
            let (_, sim_last_key) = move_bean(sim_nbean, i, turn_a, &mut sim_state);
            last_key = Some(sim_last_key);

            state.insert(loc, 0);

            is_eligible_for_move = true;
            is_eligible_for_draw_ui = true;
        }
        
        if is_eligible_for_move {
            let (final_turn_a, _) = move_bean(nbean, i, turn_a, &mut state);
            turn_a = final_turn_a;
        }
        
        if is_eligible_for_draw_ui {
            draw_ui(&state);
        }
        
        is_eligible_for_move = false;
        is_eligible_for_draw_ui = false;
        is_start = false;

        println!("It is {}'s turn", if turn_a { "A" } else { "B" });
        
        let mut vsum = 0;
        for (k, v) in state.iter() {
            if *k == 0 || *k == 7 {
                vsum += v;
            }
        }
        if &csum == &vsum {
            let side_a = *state.entry(0).or_insert(0);
            let side_b = *state.entry(7).or_insert(0);

            match side_a.cmp(&side_b) {
                Ordering::Greater => println!("Side A win"),
                Ordering::Less => println!("Side B win"),
                Ordering::Equal => println!("Tie"),
            }
            println!("Score: {} - {}", &side_a, &side_b);

            break;
        }
    }

    Ok(state)
}

fn move_bean(nbean: i32, mut i: i32, turn_a: bool, state: &mut BTreeMap<i32, i32>) -> (bool, i32) {
    for _ in 0..nbean {
        i = if i == 13 { 0 } else { i + 1 };
        *state.entry(i).or_insert(0) += 1;
    }

    /*
        TODO: Make sure input 0 or 7 doesn't exit.
    */
    let final_turn_a = match *state.entry(i).or_insert(0) == 1 {
        true => !turn_a,
        false => match i == 0 || i == 7 {
            true => !turn_a,
            false => turn_a,
        }
    };

    let last_key = i;

    (final_turn_a, last_key)
}

fn draw_ui(state: &BTreeMap<i32, i32>) {
    // Janky aesthetics
    println!();
    print!("    ");
    for (k, v) in state.iter() {
        if *k >= 1 && *k <= 6 {
            print!("({})", v);
        }
    }
    println!();
    for (k, v) in state.iter() {
        if *k == 0 {
            print!("A[{}]", v);
        }
    }
    print!("                  ");
    for (k, v) in state.iter() {
        if *k == 7 {
            println!("[{}]B", v);
        }
    }

    print!("    ");
    for (k, v) in state.iter().rev() {
        if *k >= 8 && *k <= 13 {
            print!("({})", v);
        }
    }

    println!();
}

fn main() {
    println!("Hole location is denoted by 0-13, going from leftmost in clockwise.");
    println!("You can move bean in location 1-6 and 8-13.");
    println!();
    println!();

    match generate_state() {
        Ok(_) => println!("Thank you for playing"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
