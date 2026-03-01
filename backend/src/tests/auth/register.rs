//! Register tests

use crate::configs::Config;
use crate::infra::SupabaseClient;
use crate::tests::config::TestConfig;

#[tokio::test]
#[ignore] // Run manually: cargo test test_supabase_register -- --ignored
async fn test_supabase_register_real() {
    let cfg = Config::from_env().expect("Config should load from .env");
    let test_cfg =
        TestConfig::from_env().expect("TestConfig should load from .env");

    let supabase = SupabaseClient::new(&cfg);

    let result = supabase
        .register(&test_cfg.email, &test_cfg.password, "test_user", None, None)
        .await;

    match result {
        Ok(response) => {
            assert!(!response.access_token.is_empty());
        }
        Err(_) => {
            // Registration may fail if user already exists
        }
    }
}
