//! Route enum and thin wrappers mapping URLs to page components.

use crate::pages;
use dioxus::prelude::*;

/// All application routes.
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Vitrine {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/home")]
    Home {},
    #[route("/admin")]
    Admin {},
    #[route("/collection")]
    Collection {},
    #[route("/collection/add")]
    CollectionAdd {},
    #[route("/qcm")]
    QcmList {},
    #[route("/qcm/create")]
    QcmCreate {},
    #[route("/qcm/generate")]
    QcmGenerate {},
    #[route("/qcm/quick")]
    QcmQuick {},
    #[route("/qcm/play/:id")]
    QcmPlay { id: String },
    #[route("/flashcards")]
    FlashcardList {},
    #[route("/flashcards/generate")]
    FlashcardGenerate {},
    #[route("/flashcards/play/:id")]
    FlashcardPlay { id: String },
    #[route("/true-false")]
    TrueFalseList {},
    #[route("/true-false/generate")]
    TrueFalseGenerate {},
    #[route("/true-false/play/:id")]
    TrueFalsePlay { id: String },
    #[route("/open-questions")]
    OpenQuestionList {},
    #[route("/open-questions/generate")]
    OpenQuestionGenerate {},
    #[route("/open-questions/play/:id")]
    OpenQuestionPlay { id: String },
    #[route("/keywords")]
    KeywordsList {},
    #[route("/keywords/generate")]
    KeywordsGenerate {},
    #[route("/keywords/play/:id")]
    KeywordsPlay { id: String },
    #[route("/order-phrases")]
    OrderPhraseList {},
    #[route("/order-phrases/generate")]
    OrderPhraseGenerate {},
    #[route("/order-phrases/play/:id")]
    OrderPhrasePlay { id: String },
    #[route("/fill-blanks")]
    FillBlankList {},
    #[route("/fill-blanks/generate")]
    FillBlankGenerate {},
    #[route("/fill-blanks/play/:id")]
    FillBlankPlay { id: String },
    #[route("/courses")]
    CourseList {},
    #[route("/courses/create")]
    CourseCreate {},
    #[route("/courses/:id")]
    CourseDetail { id: String },
    #[route("/courses/:course_id/session/create")]
    SessionCreate { course_id: String },
    #[route("/courses/:course_id/session/:sid")]
    SessionView { course_id: String, sid: String },
    #[route("/demo/a")] DemoA {},
    #[route("/demo/b")] DemoB {},
    #[route("/demo/c")] DemoC {},
    #[route("/demo/d")] DemoD {},
    #[route("/demo/e")] DemoE {},
    #[route("/demo/f")] DemoF {},
    #[route("/demo/g")] DemoG {},
    #[route("/demo/h")] DemoH {},
    #[route("/testing-ui")]
    TestingUi {},
    #[route("/code/generate")]
    CodingGenerate {},
    #[route("/code/play")]
    CodingPlay {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn Vitrine() -> Element { crate::vitrine::VitrinePage() }
fn Login() -> Element { crate::auth::LoginPage() }
fn Register() -> Element { crate::auth::RegisterPage() }
fn Home() -> Element { crate::home::HomePage() }
fn Admin() -> Element { crate::admin::AdminPage() }
fn Collection() -> Element { crate::collection::CollectionPage() }
fn CollectionAdd() -> Element { crate::collection::DvdAddPage() }

fn QcmList() -> Element { crate::qcm::QcmListPage() }
fn QcmCreate() -> Element { crate::qcm::QcmCreatePage() }
fn QcmGenerate() -> Element { crate::qcm::QcmGeneratePage() }
fn QcmQuick() -> Element { crate::qcm::QcmQuickPage() }

#[component]
fn QcmPlay(id: String) -> Element {
    crate::qcm::QcmPlayPage(id)
}

fn FlashcardList() -> Element {
    crate::flashcards::FlashcardListPage()
}
fn FlashcardGenerate() -> Element {
    crate::flashcards::FlashcardGeneratePage()
}

#[component]
fn FlashcardPlay(id: String) -> Element {
    crate::flashcards::FlashcardPlayPage(id)
}

fn TrueFalseList() -> Element {
    crate::true_false::TrueFalseListPage()
}
fn TrueFalseGenerate() -> Element {
    crate::true_false::TrueFalseGeneratePage()
}

#[component]
fn TrueFalsePlay(id: String) -> Element {
    crate::true_false::TrueFalsePlayPage(id)
}

fn OpenQuestionList() -> Element {
    crate::open_questions::OpenQuestionListPage()
}
fn OpenQuestionGenerate() -> Element {
    crate::open_questions::OpenQuestionGeneratePage()
}

#[component]
fn OpenQuestionPlay(id: String) -> Element {
    crate::open_questions::OpenQuestionPlayPage(id)
}

fn KeywordsList() -> Element {
    crate::keywords::KeywordsListPage()
}
fn KeywordsGenerate() -> Element {
    crate::keywords::KeywordsGeneratePage()
}

#[component]
fn KeywordsPlay(id: String) -> Element {
    crate::keywords::KeywordsPlayPage(id)
}

fn OrderPhraseList() -> Element {
    crate::order_phrases::OrderPhraseListPage()
}
fn OrderPhraseGenerate() -> Element {
    crate::order_phrases::OrderPhraseGeneratePage()
}

#[component]
fn OrderPhrasePlay(id: String) -> Element {
    crate::order_phrases::OrderPhrasePlayPage(id)
}

fn FillBlankList() -> Element {
    crate::fill_blanks::FillBlankListPage()
}
fn FillBlankGenerate() -> Element {
    crate::fill_blanks::FillBlankGeneratePage()
}

#[component]
fn FillBlankPlay(id: String) -> Element {
    crate::fill_blanks::FillBlankPlayPage(id)
}

fn CourseList() -> Element { pages::courses::CourseListPage() }
fn CourseCreate() -> Element {
    pages::courses::CourseCreatePage()
}

#[component]
fn CourseDetail(id: String) -> Element {
    pages::courses::CourseDetailPage(id)
}

#[component]
fn SessionCreate(course_id: String) -> Element {
    pages::courses::SessionCreatePage(course_id)
}

#[component]
fn SessionView(course_id: String, sid: String) -> Element {
    pages::courses::SessionViewPage(course_id, sid)
}

fn DemoA() -> Element { crate::demo::DemoAPage() }
fn DemoB() -> Element { crate::demo::DemoBPage() }
fn DemoC() -> Element { crate::demo::DemoCPage() }
fn DemoD() -> Element { crate::demo::DemoDPage() }
fn DemoE() -> Element { crate::demo::DemoEPage() }
fn DemoF() -> Element { crate::demo::DemoFPage() }
fn DemoG() -> Element { crate::demo::DemoGPage() }
fn DemoH() -> Element { crate::demo::DemoHPage() }
fn TestingUi() -> Element { crate::demo::TestingUiPage() }

fn CodingGenerate() -> Element {
    crate::coding::CodingGeneratePage()
}
fn CodingPlay() -> Element {
    crate::coding::CodingPlayPage()
}

/// 404 catch-all.
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center \
                min-h-screen",
            div { class: "text-center",
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
