extern crate based_minigrep;

mod test {
    use based_minigrep::grep::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive, duct tape.\nPick three.";
        let flag = String::from("NO_FLAG");

        assert_eq!(
            1,
            search(&query, &contents, &flag)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive, duct tape.\nPick three. RuSt";
        let flag = String::from("-c");

        assert_eq!(
            2,
            search(&query, &contents, &flag),
        );
    }
}
