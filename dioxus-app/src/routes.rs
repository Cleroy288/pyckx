//! Route enum and thin route-wrapper components

use dioxus::prelude::*;

use crate::pages;

/// All application routes
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
    SessionView {
        course_id: String,
        sid: String,
    },
    #[route("/demo/a")]
    DemoA {},
    #[route("/demo/b")]
    DemoB {},
    #[route("/demo/c")]
    DemoC {},
    #[route("/demo/d")]
    DemoD {},
    #[route("/demo/e")]
    DemoE {},
    #[route("/demo/f")]
    DemoF {},
    #[route("/demo/g")]
    DemoG {},
    #[route("/demo/h")]
    DemoH {},
    #[route("/testing-ui")]
    TestingUi {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
    #[route("/code/generate")]
    CodingGenerate {},
    #[route("/code/play")]
    CodingPlay {},
}

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

/// QCM list
fn QcmList() -> Element {
    pages::qcm::QcmListPage()
}

/// QCM manual create
fn QcmCreate() -> Element {
    pages::qcm::QcmCreatePage()
}

/// QCM AI generate
fn QcmGenerate() -> Element {
    pages::qcm::QcmGeneratePage()
}

/// QCM quick play
fn QcmQuick() -> Element {
    pages::qcm::QcmQuickPage()
}

/// QCM play by ID
#[component]
fn QcmPlay(id: String) -> Element {
    pages::qcm::QcmPlayPage(id)
}

/// Flashcard list
fn FlashcardList() -> Element {
    pages::flashcards::FlashcardListPage()
}

/// Flashcard generate
fn FlashcardGenerate() -> Element {
    pages::flashcards::FlashcardGeneratePage()
}

/// Flashcard play by ID
#[component]
fn FlashcardPlay(id: String) -> Element {
    pages::flashcards::FlashcardPlayPage(id)
}

/// True/False list
fn TrueFalseList() -> Element {
    pages::true_false::TrueFalseListPage()
}

/// True/False generate
fn TrueFalseGenerate() -> Element {
    pages::true_false::TrueFalseGeneratePage()
}

/// True/False play by ID
#[component]
fn TrueFalsePlay(id: String) -> Element {
    pages::true_false::TrueFalsePlayPage(id)
}

/// Open question list
fn OpenQuestionList() -> Element {
    pages::open_questions::OpenQuestionListPage()
}

/// Open question generate
fn OpenQuestionGenerate() -> Element {
    pages::open_questions::OpenQuestionGeneratePage()
}

/// Open question play by ID
#[component]
fn OpenQuestionPlay(id: String) -> Element {
    pages::open_questions::OpenQuestionPlayPage(id)
}

/// Keywords list
fn KeywordsList() -> Element {
    pages::keywords::KeywordsListPage()
}

/// Keywords generate
fn KeywordsGenerate() -> Element {
    pages::keywords::KeywordsGeneratePage()
}

/// Keywords play by ID
#[component]
fn KeywordsPlay(id: String) -> Element {
    pages::keywords::KeywordsPlayPage(id)
}

/// Order phrase list
fn OrderPhraseList() -> Element {
    pages::order_phrases::OrderPhraseListPage()
}

/// Order phrase generate
fn OrderPhraseGenerate() -> Element {
    pages::order_phrases::OrderPhraseGeneratePage()
}

/// Order phrase play by ID
#[component]
fn OrderPhrasePlay(id: String) -> Element {
    pages::order_phrases::OrderPhrasePlayPage(id)
}

/// Fill blank list
fn FillBlankList() -> Element {
    pages::fill_blanks::FillBlankListPage()
}

/// Fill blank generate
fn FillBlankGenerate() -> Element {
    pages::fill_blanks::FillBlankGeneratePage()
}

/// Fill blank play by ID
#[component]
fn FillBlankPlay(id: String) -> Element {
    pages::fill_blanks::FillBlankPlayPage(id)
}

/// Course list
fn CourseList() -> Element {
    pages::courses::CourseListPage()
}

/// Course create
fn CourseCreate() -> Element {
    pages::courses::CourseCreatePage()
}

/// Course detail by ID
#[component]
fn CourseDetail(id: String) -> Element {
    pages::courses::CourseDetailPage(id)
}

/// Session create
#[component]
fn SessionCreate(course_id: String) -> Element {
    pages::courses::SessionCreatePage(course_id)
}

/// Session view
#[component]
fn SessionView(
    course_id: String,
    sid: String,
) -> Element {
    pages::courses::SessionViewPage(
        course_id, sid,
    )
}

/// Demo A
fn DemoA() -> Element {
    pages::demo::demo_a::DemoAPage()
}

/// Demo B
fn DemoB() -> Element {
    pages::demo::demo_b::DemoBPage()
}

/// Demo C
fn DemoC() -> Element {
    pages::demo::demo_c::DemoCPage()
}

/// Demo D
fn DemoD() -> Element {
    pages::demo::demo_d::DemoDPage()
}

/// Demo E
fn DemoE() -> Element {
    pages::demo::demo_e::DemoEPage()
}

/// Demo F
fn DemoF() -> Element {
    pages::demo::demo_f::DemoFPage()
}

/// Demo G
fn DemoG() -> Element {
    pages::demo::demo_g::DemoGPage()
}

/// Demo H
fn DemoH() -> Element {
    pages::demo::demo_h::DemoHPage()
}

/// UI testing sandbox
fn TestingUi() -> Element {
    pages::testing_ui::TestingUiPage()
}

fn CodingGenerate() -> Element {
    pages::coding::CodingGeneratePage()
}

fn CodingPlay() -> Element {
    pages::coding::CodingPlayPage()
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
