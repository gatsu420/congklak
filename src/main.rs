use rand::Rng;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;

fn input_form() -> Result<i32, io::Error> {
    println!("Please enter location:");
    let mut loc_input = String::new();
    io::stdin().read_line(&mut loc_input)?;
    let loc = match loc_input.trim().parse() {
        Ok(l) => l,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Hole location must be numeric",
            ))
        }
    };

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

fn move_bean() -> Result<BTreeMap<i32, i32>, io::Error> {
    let mut state = init_state();
    let mut csum = 0;
    for v in state.values() {
        csum += v;
    }

    loop {
        draw_ui(&state);

        let loc = input_form()?;

        let nbean = state.get(&loc).copied().unwrap_or(0);
        state.insert(loc, 0);

        let mut i = loc;
        for _ in 0..nbean {
            i = if i == 13 { 0 } else { i + 1 };
            *state.entry(i).or_insert(0) += 1;
        }

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
    println!();

    Ok(state)
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
    println!();
}

fn main() {
    println!("Hole location is denoted by 0-13, going from leftmost in clockwise.");

    match move_bean() {
        Ok(_) => println!("Thank you for playing"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
