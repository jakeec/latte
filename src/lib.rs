#[macro_export]
macro_rules! describe {
    ($test_suite:ident, {$($tests:tt)*}) => {
        #[cfg(test)]
        mod $test_suite {
            $($tests)*
        }
    };
}

#[macro_export]
macro_rules! it {
    ($test_name:ident, $test:block) => {
        #[test]
        fn $test_name() $test
    };
}
