// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!



pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.len() > 0 {
        Some(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Some("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {

        let x:Result<String,String>=match generate_nametag_text("".into()){
            None=> Err("`name` was empty; it must be nonempty.".into()),
            Some(i)=>Ok(i),
        };
        assert_eq!(
            x,
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
