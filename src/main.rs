fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(text: String) -> String {
    let first_word = text.split_whitespace().next().unwrap_or("").to_string();
    let mut output = format!("{first_word}\n");

    let mut new_owner = text;
    new_owner.push_str(" rust");

    let mut_l = &mut new_owner;
    output.push_str(&format!("{mut_l}\n"));

    let immut_l = &new_owner;
    output.push_str(&format!("{immut_l}\n"));

    output
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
