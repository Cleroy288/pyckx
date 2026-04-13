//! HTTP handlers - Route handlers organized by feature

pub mod apps;
pub mod auth;
pub mod collection;
pub mod log;
pub mod user;

mod admin;
mod ai_generation;
pub mod auth_proxy;
mod course_dto;
mod course_generation;
mod course_handlers;
mod fill_blank;
mod flashcard;
mod games;
mod helpers;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm_handlers;
mod resource_handlers;
mod session_handlers;
mod true_false;

use actix_web::web;

/// Initialize all API routes
pub fn init(cfg: &mut web::ServiceConfig) {
    auth::init(cfg);
    apps::init(cfg);
    user::init(cfg);
    collection::init(cfg);
    init_study(cfg);
    log::init(cfg);
}

fn init_study(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/study")
            // Games
            .service(games::get_available_games_handler)
            .service(games::generate_coding_game_handler)
            .service(games::check_coding_game_handler)
            // QCM
            .service(qcm_handlers::create_qcmset_handler)
            .service(qcm_handlers::get_user_qcmsets_handler)
            .service(qcm_handlers::get_qcmset_handler)
            .service(qcm_handlers::update_qcmset_handler)
            .service(qcm_handlers::delete_qcmset_handler)
            // AI Generation
            .service(
                ai_generation::create_custom_question_handler,
            )
            .service(
                ai_generation::create_open_question_handler,
            )
            .service(
                ai_generation::create_flashcard_handler,
            )
            .service(
                ai_generation::create_quick_qcm_handler,
            )
            // Open Questions
            .service(
                open_question::list_open_questions_handler,
            )
            .service(
                open_question::check_open_questions_handler,
            )
            // Flashcards
            .service(flashcard::list_flashcards_handler)
            // True/False
            .service(true_false::create_true_false_handler)
            .service(
                true_false::list_true_false_sets_handler,
            )
            // Keywords
            .service(keywords::create_keywords_handler)
            .service(keywords::list_keyword_sets_handler)
            // Order Phrase
            .service(
                order_phrase::create_order_phrase_handler,
            )
            .service(
                order_phrase::list_order_phrase_sets_handler,
            )
            // Fill Blank
            .service(
                fill_blank::create_fill_blank_handler,
            )
            .service(
                fill_blank::list_fill_blank_sets_handler,
            )
            // Course Generation
            .route(
                "/generate-course",
                web::post()
                    .to(course_generation::generate_course),
            )
            // Course CRUD
            .route(
                "/courses",
                web::post()
                    .to(course_handlers::create_course),
            )
            .route(
                "/courses",
                web::get()
                    .to(course_handlers::list_courses),
            )
            .route(
                "/courses/{id}",
                web::delete()
                    .to(course_handlers::delete_course),
            )
            .route(
                "/courses/{id}/resources",
                web::post()
                    .to(course_handlers::upload_resource),
            )
            .route(
                "/courses/{id}/resources",
                web::get()
                    .to(course_handlers::get_resources),
            )
            // Session CRUD
            .route(
                "/courses/{id}/sessions",
                web::post()
                    .to(session_handlers::create_session),
            )
            .route(
                "/courses/{id}/sessions",
                web::get()
                    .to(session_handlers::list_sessions),
            )
            .route(
                "/courses/{course_id}/sessions/{session_id}",
                web::get()
                    .to(session_handlers::get_session),
            )
            .route(
                "/courses/{course_id}/sessions/{session_id}",
                web::delete()
                    .to(session_handlers::delete_session),
            )
            // Resources
            .route(
                "/resources",
                web::get().to(
                    resource_handlers::list_user_resources,
                ),
            )
            .route(
                "/resources/check",
                web::get().to(
                    resource_handlers::check_resource_exists,
                ),
            )
            .route(
                "/resources",
                web::post().to(
                    resource_handlers::create_user_resource,
                ),
            )
            .route(
                "/resources/{id}",
                web::get().to(
                    resource_handlers::get_resource_content,
                ),
            )
            .route(
                "/courses/{id}/resources/link",
                web::post().to(
                    resource_handlers::link_resource,
                ),
            )
            // Admin
            .service(admin::get_admin_stats),
    );
}
