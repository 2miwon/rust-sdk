#[cfg(feature = "server")]
use by_axum::aide;

use by_macros::api_model;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[derive(Serialize, Deserialize)]
#[api_model(base = "/topics/v1", iter_type=Vec)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, queryable, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update)]
    pub status: i32,
    #[api_model(summary)]
    pub created_at: i64,

    pub updated_at: i64,

    #[api_model(action_by_id = update)]
    pub tags: Vec<String>,
}

#[test]
fn test_macro_expansion() {
    let q = TopicQuery {
        size: 10,
        bookmark: None,
        description: None,
        status: Some(1),
    };

    assert_eq!(q.status, Some(1));
    assert_eq!(q.size, 10);
    assert_eq!(q.bookmark, None);
    assert_eq!(q.description, None);

    let summary = TopicSummary::default();
    assert_eq!(summary.id, "".to_string());
    assert_eq!(summary.title, "".to_string());
    assert_eq!(summary.description, "".to_string());
    assert_eq!(summary.status, 0);
    assert_eq!(summary.created_at, 0);

    let create_request = TopicCreateRequest {
        title: "title".to_string(),
        description: "description".to_string(),
    };
    assert_eq!(create_request.title, "title".to_string());
    assert_eq!(create_request.description, "description".to_string());

    let update_request = TopicUpdateRequest {
        description: "description".to_string(),
        status: 1,
        tags: vec!["tag".to_string()],
    };
    assert_eq!(update_request.description, "description".to_string());
    assert_eq!(update_request.status, 1);
    assert_eq!(update_request.tags, vec!["tag".to_string()]);

    let _ = TopicActionRequest::Create(create_request);
    let _ = TopicActionByIdRequest::Update(update_request);

    let cli = Topic::get_client("");
    let _ = cli.get("1");
    let _ = cli.query(q);
}
