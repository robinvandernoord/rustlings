// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(integer) = optional_integers.pop() {

            // 1
            assert_eq!(integer.unwrap(), range);
            //2
            assert_eq!(integer, Some(range));

            if let Some(i) = integer {
                assert_eq!(i, range);
            }

            range -= 1;
        }
    }
}
