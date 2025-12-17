//! Intello handlers module
//!
//! Quiz/educational game management endpoints.

mod ai_generation;
mod flashcard;
mod games;
mod helpers;
mod keywords;
mod models;
mod open_question;
mod order_phrase;
mod qcm_handlers;
mod true_false;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app/intello")
            .service(games::get_available_games_handler)
            .service(models::get_available_models_handler)
            .service(qcm_handlers::create_qcmset_handler)
            .service(qcm_handlers::get_user_qcmsets_handler)
            .service(qcm_handlers::get_qcmset_handler)
            .service(qcm_handlers::update_qcmset_handler)
            .service(qcm_handlers::delete_qcmset_handler)
            .service(ai_generation::create_custom_question_handler)
            .service(ai_generation::create_open_question_handler)
            .service(ai_generation::create_flashcard_handler)
            .service(open_question::list_open_questions_handler)
            .service(open_question::check_open_questions_handler)
            .service(flashcard::list_flashcards_handler)
            .service(true_false::create_true_false_handler)
            .service(true_false::list_true_false_sets_handler)
            .service(keywords::create_keywords_handler)
            .service(keywords::list_keyword_sets_handler)
            .service(order_phrase::create_order_phrase_handler)
            .service(order_phrase::list_order_phrase_sets_handler),
    );
}

