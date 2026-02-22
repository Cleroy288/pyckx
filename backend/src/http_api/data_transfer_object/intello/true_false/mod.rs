//! True or False DTOs module

mod conversion;
mod request;
mod response;

pub use request::CreateTrueOrFalseRequest;
pub use response::{
    CreateTrueOrFalseResponse, TrueOrFalseSetListResponse,
    TrueOrFalseSetWithStatementsResponse, TrueOrFalseStatementResponse,
};
