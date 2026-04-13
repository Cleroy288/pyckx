//! QCM Response DTOs

use serde::Serialize;

/// Response for a single QCM set
#[derive(Debug, Serialize)]
pub struct QcmSetResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<QcmQuestionResponse>,
}

/// Response for a single QCM question
#[derive(Debug, Serialize)]
pub struct QcmQuestionResponse {
    pub id: String,
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

/// Response for a list of QCM sets
#[derive(Debug, Serialize)]
pub struct QcmSetListResponse {
    pub sets: Vec<QcmSetResponse>,
    pub count: usize,
}

/// Success response for QCM operations
#[derive(Debug, Serialize)]
pub struct QcmSuccessResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<QcmSetResponse>,
}

impl QcmSuccessResponse {
    pub fn deleted() -> Self {
        Self {
            success: true,
            message: "QCM set deleted successfully".to_string(),
            set: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            success: false,
            message: "QCM set not found".to_string(),
            set: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qcm_success_deleted_is_success() {
        // arrange / act
        let resp = QcmSuccessResponse::deleted();

        // assert
        assert!(resp.success);
        assert_eq!(
            resp.message,
            "QCM set deleted successfully"
        );
        assert!(resp.set.is_none());
    }

    #[test]
    fn test_qcm_success_not_found_is_failure() {
        // arrange / act
        let resp = QcmSuccessResponse::not_found();

        // assert
        assert!(!resp.success);
        assert_eq!(resp.message, "QCM set not found");
        assert!(resp.set.is_none());
    }
}
