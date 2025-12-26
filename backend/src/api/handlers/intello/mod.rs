//! Intello handlers module
//!
//! Quiz/educational game management and Course generation endpoints.

mod ai_generation;
mod course_crud;       // Course & Resource CRUD (direct Supabase)
mod course_generation; // AI course generation (no repository)
mod fill_blank;
mod flashcard;
mod games;
mod helpers;
mod keywords;
mod models;
mod open_question;
mod order_phrase;
mod qcm_handlers;
mod resource_handlers;  // NEW: User resource management
mod true_false;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/intello")
            // Games & Models
            .service(games::get_available_games_handler)
            .service(models::get_available_models_handler)
            // QCM
            .service(qcm_handlers::create_qcmset_handler)
            .service(qcm_handlers::get_user_qcmsets_handler)
            .service(qcm_handlers::get_qcmset_handler)
            .service(qcm_handlers::update_qcmset_handler)
            .service(qcm_handlers::delete_qcmset_handler)
            // AI Generation
            .service(ai_generation::create_custom_question_handler)
            .service(ai_generation::create_open_question_handler)
            .service(ai_generation::create_flashcard_handler)
            // Open Questions
            .service(open_question::list_open_questions_handler)
            .service(open_question::check_open_questions_handler)
            // Flashcards
            .service(flashcard::list_flashcards_handler)
            // True/False
            .service(true_false::create_true_false_handler)
            .service(true_false::list_true_false_sets_handler)
            // Keywords
            .service(keywords::create_keywords_handler)
            .service(keywords::list_keyword_sets_handler)
            // Order Phrase
            .service(order_phrase::create_order_phrase_handler)
            .service(order_phrase::list_order_phrase_sets_handler)
            // Fill Blank
            .service(fill_blank::create_fill_blank_handler)
            .service(fill_blank::list_fill_blank_sets_handler)
            // Course Generation (AI - Block Protocol)
            .route("/generate-course", web::post().to(course_generation::generate_course_handler))
            // Course CRUD (direct Supabase)
            .route("/courses", web::post().to(course_crud::create_course))
            .route("/courses", web::get().to(course_crud::list_courses))
            .route("/courses/{id}/resources", web::post().to(course_crud::upload_resource))
            .route("/courses/{id}/resources", web::get().to(course_crud::get_resources))
            // Session CRUD (direct Supabase)
            .route("/courses/{id}/sessions", web::post().to(course_crud::create_session))
            .route("/courses/{id}/sessions", web::get().to(course_crud::list_sessions))
            // User Resource Management
            .route("/resources", web::get().to(resource_handlers::list_user_resources))
            .route("/resources/check", web::get().to(resource_handlers::check_resource_exists))
            .route("/resources", web::post().to(resource_handlers::create_user_resource))
            .route("/resources/{id}", web::get().to(resource_handlers::get_resource_content))
            .route("/courses/{id}/resources/link", web::post().to(resource_handlers::link_resource)),
    );
}
