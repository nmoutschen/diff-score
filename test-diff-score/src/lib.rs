#[cfg(test)]
mod tests {
    use diff_score::DiffScore;

    #[derive(DiffScore)]
    struct MyStruct {
        num: usize,
        #[diff_score(weight = 3.0)]
        string: &'static str,
    }

    #[rstest::rstest]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 3, string: "hello" }, 0.0)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 4, string: "hello" }, 0.25)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 7, string: "hello" }, 0.25)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 3, string: "hello world!" }, 0.75)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 4, string: "hello world!" }, 1.0)]
    fn test_diff_struct(#[case] a: MyStruct, #[case] b: MyStruct, #[case] expected: f64) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    #[derive(DiffScore)]
    struct MyStructCustom {
        num: usize,
        #[diff_score(weight = 3.0, with = "my_diff_fn")]
        string: &'static str,
    }

    #[rstest::rstest]
    #[case(MyStructCustom { num: 3, string: "hello" }, MyStructCustom { num: 3, string: "hello" }, 0.375)]
    #[case(MyStructCustom { num: 3, string: "hello" }, MyStructCustom { num: 4, string: "hello" }, 0.625)]
    #[case(MyStructCustom { num: 3, string: "hello" }, MyStructCustom { num: 7, string: "hello" }, 0.625)]
    #[case(MyStructCustom { num: 3, string: "hello" }, MyStructCustom { num: 3, string: "hello world!" }, 0.375)]
    #[case(MyStructCustom { num: 3, string: "hello" }, MyStructCustom { num: 4, string: "hello world!" }, 0.625)]
    fn test_diff_struct_custom(
        #[case] a: MyStructCustom,
        #[case] b: MyStructCustom,
        #[case] expected: f64,
    ) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    #[derive(DiffScore)]
    struct MyStructSkip {
        num: usize,
        #[diff_score(weight = 3.0, skip)]
        #[allow(unused)]
        string: &'static str,
    }

    #[rstest::rstest]
    #[case(MyStructSkip { num: 3, string: "hello" }, MyStructSkip { num: 3, string: "hello" }, 0.0)]
    #[case(MyStructSkip { num: 3, string: "hello" }, MyStructSkip { num: 4, string: "hello" }, 1.0)]
    #[case(MyStructSkip { num: 3, string: "hello" }, MyStructSkip { num: 7, string: "hello" }, 1.0)]
    #[case(MyStructSkip { num: 3, string: "hello" }, MyStructSkip { num: 3, string: "hello world!" }, 0.0)]
    #[case(MyStructSkip { num: 3, string: "hello" }, MyStructSkip { num: 4, string: "hello world!" }, 1.0)]
    fn test_diff_struct_skip(
        #[case] a: MyStructSkip,
        #[case] b: MyStructSkip,
        #[case] expected: f64,
    ) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    #[derive(DiffScore)]
    enum MyEnum {
        Unit,
        Tuple(f64, #[diff_score(weight = 3.0)] &'static str),
        Struct {
            num: f64,
            #[diff_score(weight = 3.0)]
            string: &'static str,
        },
    }

    #[rstest::rstest]
    #[case(MyEnum::Unit, MyEnum::Unit, 0.0)]
    #[case(MyEnum::Unit, MyEnum::Tuple(3.0, "hello"), 1.0)]
    #[case(MyEnum::Unit, MyEnum::Struct { num: 3.0, string: "hello"}, 1.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Struct { num: 3.0, string: "hello"}, 1.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(3.0, "hello"), 0.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(4.0, "hello"), 0.25)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(7.0, "hello"), 0.25)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(3.0, "hello world!"), 0.75)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(4.0, "hello world!"), 1.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 3.0, string: "hello" }, 0.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 4.0, string: "hello" }, 0.25)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 7.0, string: "hello" }, 0.25)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 3.0, string: "hello world!" }, 0.75)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 4.0, string: "hello world!" }, 1.0)]
    fn test_diff_enum(#[case] a: MyEnum, #[case] b: MyEnum, #[case] expected: f64) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    fn my_diff_fn(_: &str, _: &str) -> f64 {
        0.5
    }
}
