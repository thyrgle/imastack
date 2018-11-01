macro_rules! get_ops {
    ($env:expr) => {
        ($env.pop().unwrap_or(0.0),
         $env.pop().unwrap_or(0.0))
    }
}

fn parse_number(token: &str, env: &mut Vec<f64>) {
    let result = token.parse::<f64>().unwrap_or(0.0);
    env.push(result);
}

fn add(env: &mut Vec<f64>) {
    // Get the adders.
    let (a, b) = get_ops!(env);
    env.push(a + b);
}

fn sub(env: &mut Vec<f64>) {
    let (a, b) = get_ops!(env);
    env.push(a - b);
}

fn mul(env: &mut Vec<f64>) {
    let (a, b) = get_ops!(env);
    env.push(a * b);
}

fn div(env: &mut Vec<f64>) {
    let (a, b) = get_ops!(env);
    if b == 0.0 {
        env.push(0.0);
    } else {
        env.push(a / b);
    }
}

fn dup(env: &mut Vec<f64>) {
    let to_dup = env.pop().unwrap_or(0.0);
    env.push(to_dup);
    env.push(to_dup);
}

fn swp(env: &mut Vec<f64>) {
    let (first, second) = get_ops!(env);
    env.push(first);
    env.push(second);
}

fn jnz(i: &mut usize, env: &mut Vec<f64>) {
    let (cond, jump) = get_ops!(env);
    if cond != 0.0 {
        *i = jump as usize;
    }
}

fn print_float(env: &mut Vec<f64>) {
    println!("{}", env.pop().unwrap_or(0.0));
}

fn parse_program(tokens: Vec<&str>, env: &mut Vec<f64>) {
    let mut i: usize = 0;
    loop {
        match tokens.get(i) {
            Some(tok) => {
                match *tok {
                    "+"     => add(env),
                    "-"     => sub(env),
                    "*"     => mul(env),
                    "/"     => div(env),
                    "dup"   => dup(env),
                    "swp"   => swp(env),
                    "jnz"   => jnz(&mut i, env),
                    "print" => print_float(env),
                    _       => parse_number(tok, env)
                }
            },
            None => break
        }
        i += 1;
    }
}

pub fn eval(code: String) {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut env: Vec<f64> = Vec::new();
    parse_program(tokens, &mut env);
}
