mod text_styling;
use crate::text_styling::*;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_color() {
       let mut chalk = Chalk::new();

        chalk.red("hello world").bold().underline().display();
    }
}