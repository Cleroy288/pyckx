//! Login tests

use crate::configs::Config;
use crate::infra::SupabaseClient;
use crate::tests::config::TestConfig;

#[tokio::test]
async fn test_supabase_login_real() {
    let cfg = Config::from_env().expect("Config should load from .env");
    let test_cfg =
        TestConfig::from_env().expect("TestConfig should load from .env");

    let supabase = SupabaseClient::new(&cfg);

    let result = supabase.login(&test_cfg.email, &test_cfg.password).await;

    match result {
        Ok(response) => {
            assert!(
                !response.access_token.is_empty(),
                "Access token should not be empty"
            );
        }
        Err(err) => panic!("Supabase login failed: {:?}", err),
    }
}
