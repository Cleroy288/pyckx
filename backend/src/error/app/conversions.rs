//! Type conversions from service errors to AppError

use super::error::AppError;
use crate::error::auth::AuthError;
use crate::error::collection::CollectionError;
use crate::error::intello::IntelloError;
use crate::error::supabase::SupabaseError;

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl From<SupabaseError> for AppError {
    fn from(err: SupabaseError) -> Self {
        Self::Auth(AuthError::from(err))
    }
}

impl From<CollectionError> for AppError {
    fn from(err: CollectionError) -> Self {
        Self::Collection(err)
    }
}

impl From<IntelloError> for AppError {
    fn from(err: IntelloError) -> Self {
        Self::Intello(err)
    }
}
