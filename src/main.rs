use std::io;
fn main() {
    println!("Enter weight (kg): ");
    let mut input = String::new();
    // some_fn(&mut input);
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    // dbg!(weight);

    // println!("Input: {}", input);
    let m_weight = weight_on_mars(weight);
    println!("Weight on Mars: {}kg ", m_weight);
}

fn weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

//fn some_fn(s: &mut String) {}
