use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;

#[test]
fn test_store_and_get() {
    let cache = OpenQuestionCache::new();

    cache.store("user1", "set1", "Document content here".to_string());

    let content = cache.get("user1", "set1");
    assert!(content.is_some());
    assert_eq!(content.unwrap(), "Document content here");
}

#[test]
fn test_get_nonexistent() {
    let cache = OpenQuestionCache::new();

    let content = cache.get("user1", "nonexistent");
    assert!(content.is_none());
}

#[test]
fn test_different_users_same_set() {
    let cache = OpenQuestionCache::new();

    cache.store("user1", "set1", "User 1 content".to_string());
    cache.store("user2", "set1", "User 2 content".to_string());

    assert_eq!(cache.get("user1", "set1").unwrap(), "User 1 content");
    assert_eq!(cache.get("user2", "set1").unwrap(), "User 2 content");
}
