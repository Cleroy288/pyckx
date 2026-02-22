use leptos::prelude::*;
use leptos_router::components::{
    Route, Router, Routes,
};
use leptos_router::path;

use leptos_app::components::ui::toast::ToastProvider;
use leptos_app::pages;
use leptos_app::state::{
    AuthProvider, PaletteProvider, UserAppsProvider,
};

stylance::import_crate_style!(
    bg,
    "styles/background.module.css"
);

// ** Background **
// ==> Grid lines + logo wrapper
#[component]
fn Background(children: Children) -> impl IntoView {
    view! {
        <div class=bg::background>
            <div class=bg::lines></div>
            {children()}
        </div>
    }
}

// ** App **
// ==> Root with providers and router
#[component]
fn App() -> impl IntoView {
    view! {
        <PaletteProvider>
            <ToastProvider>
                <AuthProvider>
                    <UserAppsProvider>
                        <Router>
                            <Background>
                                <AllRoutes />
                            </Background>
                        </Router>
                    </UserAppsProvider>
                </AuthProvider>
            </ToastProvider>
        </PaletteProvider>
    }
}

// ** AllRoutes **
// ==> All application routes
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
#[component]
fn AllRoutes() -> impl IntoView {
    leptos_app::state::hooks::use_persist_route();
    use pages::intello::courses::*;
    use pages::intello::fill_blanks::*;
    use pages::intello::flashcards::*;
    use pages::intello::keywords::*;
    use pages::intello::open_questions::*;
    use pages::intello::order_phrases::*;
    use pages::intello::qcm::*;
    use pages::intello::true_false::*;
    use pages::intello::IntelloHomePage;
    use pages::*;

    let not_found = || view! { <p>"Not found"</p> };

    view! {
        <Routes fallback=not_found>
            // Public
            <Route path=path!("/") view=Vitrine />
            <Route path=path!("/login") view=LoginPage />
            <Route path=path!("/register") view=RegisterPage />
            // Authenticated
            <Route path=path!("/home") view=HomePage />
            <Route path=path!("/admin") view=AdminPage />
            <Route path=path!("/collection") view=CollectionPage />
            <Route path=path!("/collection/add") view=collection::DvdAddPage />
            // Intello hub
            <Route path=path!("/intello") view=IntelloHomePage />
            // QCM
            <Route path=path!("/intello/qcm") view=QcmListPage />
            <Route path=path!("/intello/qcm/create") view=QcmCreatePage />
            <Route path=path!("/intello/qcm/generate") view=QcmGeneratePage />
            <Route path=path!("/intello/qcm/quick") view=QcmQuickPage />
            <Route path=path!("/intello/qcm/play/:id") view=QcmPlayPage />
            // Flashcards
            <Route path=path!("/intello/flashcards") view=FlashcardListPage />
            <Route path=path!("/intello/flashcards/generate") view=FlashcardGeneratePage />
            <Route path=path!("/intello/flashcards/play/:id") view=FlashcardPlayPage />
            // True/False
            <Route path=path!("/intello/true-false") view=TrueFalseListPage />
            <Route path=path!("/intello/true-false/generate") view=TrueFalseGeneratePage />
            <Route path=path!("/intello/true-false/play/:id") view=TrueFalsePlayPage />
            // Open Questions
            <Route path=path!("/intello/open-questions") view=OpenQuestionListPage />
            <Route path=path!("/intello/open-questions/generate") view=OpenQuestionGeneratePage />
            <Route path=path!("/intello/open-questions/play/:id") view=OpenQuestionPlayPage />
            // Keywords
            <Route path=path!("/intello/keywords") view=KeywordsListPage />
            <Route path=path!("/intello/keywords/generate") view=KeywordsGeneratePage />
            <Route path=path!("/intello/keywords/play/:id") view=KeywordsPlayPage />
            // Order Phrases
            <Route path=path!("/intello/order-phrases") view=OrderPhraseListPage />
            <Route path=path!("/intello/order-phrases/generate") view=OrderPhraseGeneratePage />
            <Route path=path!("/intello/order-phrases/play/:id") view=OrderPhrasePlayPage />
            // Fill Blanks
            <Route path=path!("/intello/fill-blanks") view=FillBlankListPage />
            <Route path=path!("/intello/fill-blanks/generate") view=FillBlankGeneratePage />
            <Route path=path!("/intello/fill-blanks/play/:id") view=FillBlankPlayPage />
            // Courses
            <Route path=path!("/intello/courses") view=CourseListPage />
            <Route path=path!("/intello/courses/create") view=CourseCreatePage />
            <Route path=path!("/intello/courses/:id") view=CourseDetailPage />
            <Route path=path!("/intello/courses/:id/session/create") view=SessionCreatePage />
            <Route path=path!("/intello/courses/:id/session/:sid") view=SessionViewPage />
        </Routes>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
