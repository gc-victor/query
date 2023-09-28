use std::env;

use crate::sqlite::create_config_db::create_config_db;

pub fn before(path: &str) {
    env::set_var("QUERY_SERVER_ADMIN_EMAIL", "admin@admin.com");
    env::set_var("QUERY_SERVER_ADMIN_PASSWORD", "abcdefg");
    env::set_var("QUERY_SERVER_TOKEN_SECRET", "secret");
    env::set_var("QUERY_SERVER_DBS_PATH", path);

    create_config_db();
}
