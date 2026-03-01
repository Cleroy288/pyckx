//! API endpoint URL constants

// -- Auth --
pub const LOGIN: &str = "/api/auth/login";
pub const LOGOUT: &str = "/api/auth/logout";
pub const REGISTER: &str = "/api/auth/register";
pub const ME: &str = "/api/user/me";

// -- User apps --
pub const USER_APPS: &str = "/api/user/apps";
pub const ALL_APPS: &str = "/api/apps";

// -- Collection --
pub const DVDS: &str = "/api/collection/dvds";

// -- Intello games --
pub const QCM: &str = "/api/intello/qcm";
pub const QCM_QUICK: &str =
    "/api/intello/qcm/generate-quick";
pub const FLASHCARDS: &str = "/api/intello/flashcards";
pub const TRUE_FALSE: &str = "/api/intello/true-false";
pub const OPEN_QUESTIONS: &str = "/api/intello/open-questions";
pub const OPEN_QUESTIONS_CHECK: &str =
    "/api/intello/open-questions/check";
pub const KEYWORDS: &str = "/api/intello/keywords";
pub const ORDER_PHRASES: &str = "/api/intello/order-phrases";
pub const FILL_BLANKS: &str = "/api/intello/fill-blanks";
pub const GAMES: &str = "/api/intello/games";

// -- Courses --
pub const COURSES: &str = "/api/intello/courses";
pub const GENERATE_COURSE: &str =
    "/api/intello/generate-course";
pub const RESOURCES: &str = "/api/intello/resources";
pub const RESOURCES_CHECK: &str =
    "/api/intello/resources/check";

// -- Admin --
pub const ADMIN_STATS: &str = "/api/admin/stats";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_endpoints_start_with_slash_api() {
        let endpoints: &[(&str, &str)] = &[
            ("LOGIN", LOGIN),
            ("LOGOUT", LOGOUT),
            ("REGISTER", REGISTER),
            ("ME", ME),
            ("USER_APPS", USER_APPS),
            ("ALL_APPS", ALL_APPS),
            ("DVDS", DVDS),
            ("QCM", QCM),
            ("QCM_QUICK", QCM_QUICK),
            ("FLASHCARDS", FLASHCARDS),
            ("TRUE_FALSE", TRUE_FALSE),
            ("OPEN_QUESTIONS", OPEN_QUESTIONS),
            ("OPEN_QUESTIONS_CHECK", OPEN_QUESTIONS_CHECK),
            ("KEYWORDS", KEYWORDS),
            ("ORDER_PHRASES", ORDER_PHRASES),
            ("FILL_BLANKS", FILL_BLANKS),
            ("GAMES", GAMES),
            ("COURSES", COURSES),
            ("GENERATE_COURSE", GENERATE_COURSE),
            ("RESOURCES", RESOURCES),
            ("RESOURCES_CHECK", RESOURCES_CHECK),
            ("ADMIN_STATS", ADMIN_STATS),
        ];
        for (name, url) in endpoints {
            assert!(
                url.starts_with("/api/"),
                "{name} = {url:?} must start with /api/",
            );
        }
    }

    #[test]
    fn test_no_trailing_slash() {
        let endpoints = [
            LOGIN, LOGOUT, REGISTER, ME,
            USER_APPS, ALL_APPS, DVDS, QCM, QCM_QUICK,
            FLASHCARDS, TRUE_FALSE,
            OPEN_QUESTIONS, KEYWORDS,
            ORDER_PHRASES, FILL_BLANKS,
            COURSES, RESOURCES, ADMIN_STATS,
        ];
        for url in endpoints {
            assert!(
                !url.ends_with('/'),
                "{url:?} should not end with /",
            );
        }
    }
}
