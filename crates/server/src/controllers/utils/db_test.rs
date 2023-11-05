#[macro_export]
macro_rules! db_test {
    ($test:ident, $name:ident, $action:expr) => {
        struct $name;

        impl Drop for $name {
            fn drop(&mut self) {
                std::fs::remove_dir_all(concat!("../../.tests/", stringify!($test))).unwrap();
            }
        }

        #[test]
        fn $test() {
            let _after = $name;

            $crate::controllers::utils::db_test_before::before(concat!(
                "../../.tests/",
                stringify!($test)
            ));

            $action;
        }
    };
}
