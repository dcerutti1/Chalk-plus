mod text_styling;
use crate::text_styling::*;




#[cfg(test)]
mod tests {
    use super::*; // This brings the main items from lib.rs into scope for the test

    #[test]
    fn test_apply_color() {
       let mut chalk = Chalk::new();

        chalk.red("hello world").bold().underline().display();
    }
}