use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let g: f64 = 9.81;

    println!("Projectile simulator - TUI");

    let u = get_float_input("Enter speed : ");
    let a = get_float_input("Enter angle from horizontal : ");
    
    // Convert angle to radians for trigonometric functions
    let angle = a.to_radians();
    let u_x = u * angle.cos();
    let u_y = u * angle.sin();
    
    // Calculate final stats
    let range: f64 = (u * u * (2.0 * angle).sin()) / g;
    let time_period: f64 = (2.0 * u_y) / g;
    let max_height: f64 = (u_y.powi(2)) / (2.0 * g);

    // Calculate scaling factor to fit drawing to terminal width
    let factor: f64 = 100.0 / range;
    
    let mut t: f64 = 0.0;
    let accuracy = 0.01;
    let mut trajectory: Vec<(i32, i32)> = Vec::new();
    
    // Main loop for animation
    loop {
        let x = u_x * t;
        let y = u_y * t - 0.5 * g * t.powi(2);
        
        if y < 0.0 {
            break;
        }

        // Scale and cast coordinates to integers
        let scaled_x = (x * factor).round() as i32;
        let scaled_y = (y * factor).round() as i32;

        trajectory.push((scaled_x, scaled_y));

        draw(&trajectory);
        
        // Sleep to create animation effect
        thread::sleep(Duration::from_millis(50));
        
        // You'll need an external crate or more complex logic to clear the screen
        // as std::process::Command::new("clear") is not platform-independent
        // or a good fit for this use case. We'll simulate by printing newlines.
        print!("\x1B[2J\x1B[1;1H");
        
        t += accuracy;
    }
    
    draw(&trajectory);

    // Final output
    println!("\n\n\n\n\n");
    println!("RANGE: {}", range);
    println!("TIME PERIOD: {}", time_period);
    println!("MAXIMUM HEIGHT: {}", max_height);
    println!("*Path calculated with an accuracy of {}*", accuracy);
}

fn get_float_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Enter a valid value.")
}

fn draw(points: &Vec<(i32, i32)>) {
    // Find max x and y
    let mx = points.iter().max_by_key(|p| p.0).map(|p| p.0).unwrap_or(0);
    let my = points.iter().max_by_key(|p| p.1).map(|p| p.1).unwrap_or(0);

    for y in (0..=my).rev() {
        let mut line = String::new();
        for x in 0..=mx {
            if points.contains(&(x, y)) {
                line.push('*');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}
