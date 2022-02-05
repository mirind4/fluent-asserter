extern crate fluent_asserter;
use fluent_asserter::*;
use fluent_asserter::prelude::*;

mod test_iterator_asserter {
    use super::*;

    #[test]
    fn test_is_equal_to() { 
        assert_that!(vec!["item1"]).is_equal_to(vec!["item1"]);

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).is_equal_to(vec!["item2"]))
            .panics()
            .with_message("Expected list to be [\"item2\"], but was [\"item1\"]")
    }

    #[test]
    fn test_contains() { 
        assert_that!(vec!["item1"]).contains("item1");

        let list = vec!["item1"];
        assert_that_code!(||assert_that!(list).contains("item2"))
            .panics()
            .with_message("Expected iterator \"list\" to contain \"item2\", but it does not");
    }
}