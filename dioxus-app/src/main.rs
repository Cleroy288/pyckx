#![allow(non_snake_case)]
#![allow(dead_code)]
//! Entry point — Route enum, App, and main

use dioxus::prelude::*;

mod api;
mod components;
mod domain;
mod pages;
mod state;

/// All application routes
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    // Public
    #[route("/")]
    Vitrine {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    // Authenticated
    #[route("/home")]
    Home {},
    #[route("/admin")]
    Admin {},
    #[route("/collection")]
    Collection {},
    #[route("/collection/add")]
    CollectionAdd {},
    // Intello hub
    #[route("/intello")]
    IntelloHome {},
    // QCM
    #[route("/intello/qcm")]
    QcmList {},
    #[route("/intello/qcm/create")]
    QcmCreate {},
    #[route("/intello/qcm/generate")]
    QcmGenerate {},
    #[route("/intello/qcm/quick")]
    QcmQuick {},
    #[route("/intello/qcm/play/:id")]
    QcmPlay { id: String },
    // Flashcards
    #[route("/intello/flashcards")]
    FlashcardList {},
    #[route("/intello/flashcards/generate")]
    FlashcardGenerate {},
    #[route("/intello/flashcards/play/:id")]
    FlashcardPlay { id: String },
    // True/False
    #[route("/intello/true-false")]
    TrueFalseList {},
    #[route("/intello/true-false/generate")]
    TrueFalseGenerate {},
    #[route("/intello/true-false/play/:id")]
    TrueFalsePlay { id: String },
    // Open Questions
    #[route("/intello/open-questions")]
    OpenQuestionList {},
    #[route("/intello/open-questions/generate")]
    OpenQuestionGenerate {},
    #[route("/intello/open-questions/play/:id")]
    OpenQuestionPlay { id: String },
    // Keywords
    #[route("/intello/keywords")]
    KeywordsList {},
    #[route("/intello/keywords/generate")]
    KeywordsGenerate {},
    #[route("/intello/keywords/play/:id")]
    KeywordsPlay { id: String },
    // Order Phrases
    #[route("/intello/order-phrases")]
    OrderPhraseList {},
    #[route("/intello/order-phrases/generate")]
    OrderPhraseGenerate {},
    #[route("/intello/order-phrases/play/:id")]
    OrderPhrasePlay { id: String },
    // Fill Blanks
    #[route("/intello/fill-blanks")]
    FillBlankList {},
    #[route("/intello/fill-blanks/generate")]
    FillBlankGenerate {},
    #[route("/intello/fill-blanks/play/:id")]
    FillBlankPlay { id: String },
    // Courses
    #[route("/intello/courses")]
    CourseList {},
    #[route("/intello/courses/create")]
    CourseCreate {},
    #[route("/intello/courses/:id")]
    CourseDetail { id: String },
    #[route("/intello/courses/:course_id/session/create")]
    SessionCreate { course_id: String },
    #[route("/intello/courses/:course_id/session/:sid")]
    SessionView { course_id: String, sid: String },
    // Catch-all
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

// ── Route component functions ─────────────────

/// Landing page
fn Vitrine() -> Element {
    pages::vitrine::VitrinePage()
}

/// Login page
fn Login() -> Element {
    pages::login::LoginPage()
}

/// Register page
fn Register() -> Element {
    pages::register::RegisterPage()
}

/// Dashboard home
fn Home() -> Element {
    pages::home::HomePage()
}

/// Admin dashboard
fn Admin() -> Element {
    pages::admin::AdminPage()
}

/// DVD collection page
fn Collection() -> Element {
    pages::collection::CollectionPage()
}

/// Add DVD page
fn CollectionAdd() -> Element {
    pages::collection::DvdAddPage()
}

/// Intello game hub
fn IntelloHome() -> Element {
    pages::intello::IntelloHomePage()
}

/// QCM list
fn QcmList() -> Element {
    pages::intello::qcm::QcmListPage()
}

/// QCM manual create
fn QcmCreate() -> Element {
    pages::intello::qcm::QcmCreatePage()
}

/// QCM AI generate
fn QcmGenerate() -> Element {
    pages::intello::qcm::QcmGeneratePage()
}

/// QCM quick play
fn QcmQuick() -> Element {
    pages::intello::qcm::QcmQuickPage()
}

/// QCM play by ID
#[component]
fn QcmPlay(id: String) -> Element {
    pages::intello::qcm::QcmPlayPage(id)
}

/// Flashcard list
fn FlashcardList() -> Element {
    pages::intello::flashcards::FlashcardListPage()
}

/// Flashcard generate
fn FlashcardGenerate() -> Element {
    pages::intello::flashcards::FlashcardGeneratePage()
}

/// Flashcard play by ID
#[component]
fn FlashcardPlay(id: String) -> Element {
    pages::intello::flashcards::FlashcardPlayPage(id)
}

/// True/False list
fn TrueFalseList() -> Element {
    pages::intello::true_false::TrueFalseListPage()
}

/// True/False generate
fn TrueFalseGenerate() -> Element {
    pages::intello::true_false::TrueFalseGeneratePage()
}

/// True/False play by ID
#[component]
fn TrueFalsePlay(id: String) -> Element {
    pages::intello::true_false::TrueFalsePlayPage(id)
}

/// Open question list
fn OpenQuestionList() -> Element {
    pages::intello::open_questions::OpenQuestionListPage()
}

/// Open question generate
fn OpenQuestionGenerate() -> Element {
    pages::intello::open_questions::OpenQuestionGeneratePage()
}

/// Open question play by ID
#[component]
fn OpenQuestionPlay(id: String) -> Element {
    pages::intello::open_questions::OpenQuestionPlayPage(id)
}

/// Keywords list
fn KeywordsList() -> Element {
    pages::intello::keywords::KeywordsListPage()
}

/// Keywords generate
fn KeywordsGenerate() -> Element {
    pages::intello::keywords::KeywordsGeneratePage()
}

/// Keywords play by ID
#[component]
fn KeywordsPlay(id: String) -> Element {
    pages::intello::keywords::KeywordsPlayPage(id)
}

/// Order phrase list
fn OrderPhraseList() -> Element {
    pages::intello::order_phrases::OrderPhraseListPage()
}

/// Order phrase generate
fn OrderPhraseGenerate() -> Element {
    pages::intello::order_phrases::OrderPhraseGeneratePage()
}

/// Order phrase play by ID
#[component]
fn OrderPhrasePlay(id: String) -> Element {
    pages::intello::order_phrases::OrderPhrasePlayPage(id)
}

/// Fill blank list
fn FillBlankList() -> Element {
    pages::intello::fill_blanks::FillBlankListPage()
}

/// Fill blank generate
fn FillBlankGenerate() -> Element {
    pages::intello::fill_blanks::FillBlankGeneratePage()
}

/// Fill blank play by ID
#[component]
fn FillBlankPlay(id: String) -> Element {
    pages::intello::fill_blanks::FillBlankPlayPage(id)
}

/// Course list
fn CourseList() -> Element {
    pages::intello::courses::CourseListPage()
}

/// Course create
fn CourseCreate() -> Element {
    pages::intello::courses::CourseCreatePage()
}

/// Course detail by ID
#[component]
fn CourseDetail(id: String) -> Element {
    pages::intello::courses::CourseDetailPage(id)
}

/// Session create
#[component]
fn SessionCreate(course_id: String) -> Element {
    pages::intello::courses::SessionCreatePage(
        course_id,
    )
}

/// Session view
#[component]
fn SessionView(
    course_id: String,
    sid: String,
) -> Element {
    pages::intello::courses::SessionViewPage(
        course_id, sid,
    )
}

/// 404 catch-all
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center \
                min-h-screen",
            div {
                class: "text-center",
                h1 {
                    class: "text-4xl font-bold \
                        text-[var(--color-text-primary)] \
                        mb-4",
                    "404"
                }
                p {
                    class: "text-[var(--color-text-secondary)]",
                    "Page not found: /{segments.join(\"/\")}"
                }
                Link {
                    to: "/",
                    class: "mt-4 inline-block \
                        text-[var(--color-primary)] \
                        hover:underline",
                    "Go home"
                }
            }
        }
    }
}

// ── Background wrapper ────────────────────────

/// Grid-line background wrapper
#[component]
fn Background(children: Element) -> Element {
    rsx! {
        div {
            class: "relative w-full min-h-screen \
                bg-[var(--color-background)]",
            // Grid lines overlay
            div {
                class: "pointer-events-none fixed \
                    inset-0 z-0",
                style: "background-image: var(--grid-overlay-image); \
                    background-size: var(--grid-overlay-size); \
                    opacity: 0.12;",
            }
            {children}
        }
    }
}

// ── App root ──────────────────────────────────

/// Root component with providers and router
fn App() -> Element {
    state::hooks::use_persist_route();
    rsx! {
        state::PaletteProvider {
            state::AuthProvider {
                state::UserAppsProvider {
                    Background {
                        Router::<Route> {}
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
