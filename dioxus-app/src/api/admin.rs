//! Admin API — statistics

use super::{endpoints, helpers};
use crate::domain::admin_types::AdminStatsResponse;

/// Fetch admin dashboard statistics
pub async fn fetch_admin_stats(
) -> Result<AdminStatsResponse, String> {
    helpers::get_json(endpoints::ADMIN_STATS).await
}
