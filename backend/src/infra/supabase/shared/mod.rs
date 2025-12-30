/*
 * Shared Supabase Infrastructure
 *
 * Provides common types for all Supabase repository implementations:
 * - SupabaseHttpClient: Shared HTTP client with connection pooling
 * - SupabaseError: Typed errors for HTTP operations
 */

mod client;
mod error;

pub use client::SupabaseHttpClient;
pub use error::SupabaseError;
