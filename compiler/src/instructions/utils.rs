// No static hashmaps with current compile-time checker
pub fn register_value(reg: &String) -> u8 {
    match &reg.to_lowercase()[..] {
        "al" | "ax" => 0,
        "cl" | "cx" => 1,
        "dl" | "dx" => 2,
        "bl" | "bx" => 3,
        "ah" | "sp" => 4,
        "ch" | "bp" => 5,
        "dh" | "si" => 6,
        "bh" | "di" => 7,
        _ => panic!("Invalid eight bit register {}", reg)
    }
}
