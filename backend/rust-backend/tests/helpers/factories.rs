//! Test data factories for integration tests

use chrono::{DateTime, NaiveDate, Utc};
use LAPP::services::app_registry::registry_domain::{App, UserApp};
use LAPP::services::collection::collection_domain::{
    CollectionItemType, UserCollection,
};
use LAPP::services::collection::dvd_domain::Dvd;
use LAPP::services::{
    Level, QcmQuestion, QcmSet, QuestionId, SetId,
};

/// Create a test App with given id and name
pub fn test_app(id: i32, name: &str) -> App {
    App::new(id, name, Some(format!("{} app", name)))
}

/// Create a test UserApp linking user to app
pub fn test_user_app(id: i32, user_id: &str, app_id: i32) -> UserApp {
    UserApp::new(id, user_id, app_id)
}

/// Create a test UserCollection
pub fn test_collection(
    id: i32,
    user_id: &str,
    item_type: CollectionItemType,
) -> UserCollection {
    UserCollection {
        id,
        user_id: user_id.to_string(),
        collection_type: item_type,
        created_at: Utc::now(),
    }
}

/// Create a test Dvd
pub fn test_dvd(
    id: &str,
    user_id: &str,
    collection_id: i32,
    name: &str,
) -> Dvd {
    let now = Utc::now();
    let year: DateTime<Utc> = NaiveDate::from_ymd_opt(2024, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    Dvd {
        id: id.to_string(),
        collection_id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        year,
        realisator: None,
        actors: String::new(),
        genre: None,
        created_at: now,
        updated_at: now,
    }
}

/// Create a test QcmSet with N dummy questions
pub fn test_qcm_set(user_id: &str, n: usize) -> QcmSet {
    let questions: Vec<QcmQuestion> = (0..n)
        .map(|i| QcmQuestion {
            id: QuestionId::new(),
            question: format!("Question {}?", i + 1),
            wrong_answers: vec![
                format!("Wrong A{}", i),
                format!("Wrong B{}", i),
                format!("Wrong C{}", i),
            ],
            right_answer: format!("Right {}", i),
            explanation: format!("Explanation {}", i),
        })
        .collect();

    QcmSet {
        id: SetId::new(),
        user_id: user_id.into(),
        name: "Test QCM".to_string(),
        description: "A test QCM set".to_string(),
        level: Level::Medium,
        language: "en".to_string(),
        subjects: vec!["math".to_string()],
        questions,
    }
}
