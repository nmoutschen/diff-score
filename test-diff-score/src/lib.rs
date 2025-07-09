#[cfg(test)]
mod tests {
    use diff_score::DiffScore;

    #[derive(DiffScore)]
    struct MyStruct {
        num: usize,
        #[diff_score(weight = 3.0)]
        string: &'static str,
    }

    #[derive(DiffScore)]
    struct MyStruct2 {
        num: usize,
        #[diff_score(weight = 3.0, use_eq = true)]
        string: &'static str,
    }

    #[rstest::rstest]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 3, string: "hello" }, 0.0)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 4, string: "hello" }, 1.0)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 7, string: "hello" }, 4.0)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 3, string: "hello world!" }, 3.0)]
    #[case(MyStruct { num: 3, string: "hello" }, MyStruct { num: 4, string: "hello world!" }, 4.0)]
    fn test_diff_struct(#[case] a: MyStruct, #[case] b: MyStruct, #[case] expected: f64) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    #[rstest::rstest]
    #[case(MyStruct2 { num: 3, string: "hello" }, MyStruct2 { num: 3, string: "hello" }, 0.0)]
    #[case(MyStruct2 { num: 3, string: "hello" }, MyStruct2 { num: 4, string: "hello" }, 1.0)]
    #[case(MyStruct2 { num: 3, string: "hello" }, MyStruct2 { num: 7, string: "hello" }, 4.0)]
    #[case(MyStruct2 { num: 3, string: "hello" }, MyStruct2 { num: 3, string: "hello world!" }, 3.0)]
    #[case(MyStruct2 { num: 3, string: "hello" }, MyStruct2 { num: 4, string: "hello world!" }, 4.0)]
    fn test_diff_struct2(#[case] a: MyStruct2, #[case] b: MyStruct2, #[case] expected: f64) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }

    #[derive(DiffScore)]
    #[diff_score(default_penalty = 5.0)]
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
    #[case(MyEnum::Unit, MyEnum::Tuple(3.0, "hello"), 5.0)]
    #[case(MyEnum::Unit, MyEnum::Struct { num: 3.0, string: "hello"}, 5.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Struct { num: 3.0, string: "hello"}, 5.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(3.0, "hello"), 0.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(4.0, "hello"), 1.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(7.0, "hello"), 4.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(3.0, "hello world!"), 3.0)]
    #[case(MyEnum::Tuple(3.0, "hello"), MyEnum::Tuple(4.0, "hello world!"), 4.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 3.0, string: "hello" }, 0.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 4.0, string: "hello" }, 1.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 7.0, string: "hello" }, 4.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 3.0, string: "hello world!" }, 3.0)]
    #[case(MyEnum::Struct { num: 3.0, string: "hello" }, MyEnum::Struct { num: 4.0, string: "hello world!" }, 4.0)]
    fn test_diff_enum(#[case] a: MyEnum, #[case] b: MyEnum, #[case] expected: f64) {
        assert_eq!(a.diff_score(&b), expected);
        assert_eq!(b.diff_score(&a), expected);
    }
}
