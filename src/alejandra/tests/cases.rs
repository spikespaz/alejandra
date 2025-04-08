use alejandra::config::Config;
use alejandra_macros::alejandra_test_cases;
use pretty_assertions::assert_eq;

alejandra_test_cases! {
    cases_dir: "./tests/cases/default",
    config: Config::default(),
}

mod indentation_tabs {
    use alejandra::config::{Config, Indentation};
    use alejandra_macros::alejandra_test_cases;

    alejandra_test_cases! {
        cases_dir: "./tests/cases/indentation-tabs",
        config: Config { indentation: Indentation::Tabs }
    }
}
