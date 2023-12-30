static ease_in_curve_offset: [f64; 5] = [
    1.0,
    0.5819768567,
    0.31303552118,
    0.15718733178,
    0.074629639425,
];

fn interpolate_array(array: &[f64], index: f64) -> f64 {
    let index = index /* * (array.len() - 1)*/ as f64;
    let index_floor = index.floor() as usize;
    let index_ceil = index.ceil() as usize;

    let index = index - index_floor as f64;

    let a = array[index_floor];
    let b = array[index_ceil];

    a + (b - a) * index
}

fn cubic_function(x: f64) -> f64 {
    let x2 = x * x;
    let x3 = x2 * x;
    1.00000544855 - 0.5058770015 * x + 0.0938973731 * x2 - 0.0063159426 * x3
}

struct Env {
    pub out: f64,
}

fn env_curve(env: &mut Env, strength: f64, curve_factor: f64) {
    if curve_factor >= 0.0 {
        //let curve_offset = interpolate_array(&ease_in_curve_offset, curve_factor);
        env.out += strength * (env.out * curve_factor + cubic_function(curve_factor));
    } else {
        //let curve_offset = interpolate_array(&ease_in_curve_offset, -curve_factor);
        env.out += strength * ((1.0 - env.out) * -curve_factor + cubic_function(-curve_factor));
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    static SR: [usize; 4] = [48000, 192000, 768000, 3072000];
    static STRENGTH: [f64; 12] = [-4.0, -3.0, -2.0, -1.0, 0.001, 0.1, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0];

    #[test]
    fn test() {
        for sr in SR.iter() {
            for strength in STRENGTH.iter() {
                let mut env = Env { out: 0.0 };
                for _ in 0..*sr { env_curve(&mut env, 1.0 / *sr as f64, *strength); }
                println!("1s {sr} {} {}", strength, env.out);
                assert!(env.out > 0.99 && env.out < 1.01);
    
                let mut env = Env { out: 0.0 };
                for _ in 0..*sr * 4 { env_curve(&mut env, 0.25 / *sr as f64, *strength as f64); }
                println!("4s {sr} {} {}", strength, env.out);
                assert!(env.out > 0.99 && env.out < 1.01);
    
                let mut env = Env { out: 0.0 };
                for _ in 0..*sr / 10 { env_curve(&mut env, 10.0 / *sr as f64, *strength as f64); }
                println!("0.1s {sr} {} {}", strength, env.out);
                assert!(env.out > 0.99 && env.out < 1.01);
            }
        }
    }
}
