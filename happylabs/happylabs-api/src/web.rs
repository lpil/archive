mod errors;
mod health;
mod survey;

use crate::survey::Database;
use warp::http::Response;
use warp::Filter;

#[cfg(test)]
use crate::survey::{Mood, Survey};

pub fn routes(
    db: Database,
) -> impl warp::Filter<Extract = (impl warp::Reply), Error = warp::Rejection> + Clone {
    use warp::path::*;
    use warp::{any, get2 as get, post2 as post};

    let with_db = any().map(move || db.clone());

    // GET /-/ready
    let ready_check = get()
        .and(path("-"))
        .and(path("ready"))
        .and(end())
        .map(crate::web::health::readiness);

    // GET /-/health
    let health_check = get()
        .and(path("-"))
        .and(path("health"))
        .and(end())
        .map(crate::web::health::health);

    // // GET /
    // let graphiql_ui = get()
    //     .and(end())
    //     .and(juniper_warp::graphiql_filter("/graphql"));

    // // POST /graphql
    // let graphql_endpoint = post()
    //     .and(path("graphql"))
    //     .and(end())
    //     .and(crate::graphql::filter());

    // GET /surveys
    let surveys_index = get()
        .and(path("surveys"))
        .and(end())
        .and(with_db.clone())
        .map(crate::web::survey::index);

    // GET /surveys/:id/feedback
    let survey_feedback_index = get()
        .and(path("surveys"))
        .and(param())
        .and(path("feedback"))
        .and(end())
        .and(with_db.clone())
        .map(crate::web::survey::feedback_index);

    // POST /surveys/:id/feedback
    let create_survey_feedback = post()
        .and(path("surveys"))
        .and(param())
        .and(path("feedback"))
        .and(end())
        .and(warp::body::concat())
        .and(with_db.clone())
        .map(crate::web::survey::create_feedback);

    // POST /surveys
    let create_survey = post()
        .and(path("surveys"))
        .and(end())
        .and(warp::body::concat())
        .and(with_db)
        .map(crate::web::survey::create_survey);

    let not_found = any().map(|| Response::builder().status(404).body("Not found"));

    health_check
        .or(ready_check)
        .or(surveys_index)
        .or(survey_feedback_index)
        .or(create_survey_feedback)
        .or(create_survey)
        // .or(graphiql_ui)
        // .or(graphql_endpoint)
        .or(not_found)
}

// #[test]
// fn graphiql_test() {
//     let res = warp::test::request()
//         .path("/")
//         .method("GET")
//         .reply(&routes(Database::new()));
//     assert_eq!(200, res.status());
// }

#[test]
fn not_found_test() {
    let res = warp::test::request()
        .path("/whatever")
        .method("GET")
        .reply(&routes(Database::new()));
    assert_eq!(404, res.status());
}

#[test]
fn ready_test() {
    let res = warp::test::request()
        .path("/-/ready")
        .method("GET")
        .reply(&routes(Database::new()));
    assert_eq!(200, res.status());
}

#[test]
fn health_test() {
    let res = warp::test::request()
        .path("/-/health")
        .method("GET")
        .reply(&routes(Database::new()));
    assert_eq!(200, res.status());
}

#[test]
fn surveys_index_test() {
    let db = Database::new();
    db.seed();

    let res = warp::test::request()
        .path("/surveys")
        .method("GET")
        .reply(&routes(db));
    assert_eq!(200, res.status());
    assert_eq!(
        serde_json::json!({
            "surveys": [
                {
                    "id": 1,
                    "date": "2016-07-08T00:00:00Z",
                    "title": "Reflection",
                    "description": "Reflection next steps",
                    "colour": "#ffccbb",
                    "tags": [
                        "ndap",
                        "meeting"
                    ]
                },
                {
                    "id": 3,
                    "date": "2019-06-28T00:00:00Z",
                    "title": "JIRA Workflow",
                    "description": "Are we happy with the new JIRA workflow",
                    "colour": "#99ccee",
                    "tags": [
                        "process",
                        "ndap",
                        "jira"
                    ]
                },
                {
                    "id": 4,
                    "date": "2016-07-08T00:00:00Z",
                    "title": "Announcements",
                    "description": "Announcements",
                    "colour": "#99ccee",
                    "tags": [
                        "ndap"
                    ]
                },
                {
                    "id": 5,
                    "date": "2016-07-08T00:00:00Z",
                    "title": "Election results",
                    "description": null,
                    "colour": null,
                    "tags": [
                        "politics"
                    ]
                },
                {
                    "id": 6,
                    "date": "2016-07-08T00:00:00Z",
                    "title": "Conference call audio",
                    "description": "Was the audio quality good for the last NDAP remote meeting?",
                    "colour": null,
                    "tags": [
                        "ndap",
                        "av",
                        "meeting"
                    ]
                },
                {
                    "id": 7,
                    "date": "2016-07-08T00:00:00Z",
                    "title": "Rate my cat",
                    "description": "Their name is fluffy",
                    "colour": null,
                    "tags": [
                        "cute",
                        "cat"
                    ]
                },
            ]
        }),
        serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(res.body())).unwrap()
    );
}

#[test]
fn surveys_feedback_index_ok_test() {
    let db = Database::new();
    db.seed();

    let res = warp::test::request()
        .path("/surveys/1/feedback")
        .method("GET")
        .reply(&routes(db));
    assert_eq!(200, res.status());
    assert_eq!(
        serde_json::json!({
            "feedback": [
                {
                    "mood": "happy",
                    "inserted_at": "2019-06-29T12:10:00Z"
                },
                {
                    "mood": "happy",
                    "inserted_at": "2019-06-29T12:12:00Z"
                },
                {
                    "mood": "sad",
                    "inserted_at": "2019-06-29T12:15:00Z"
                },
                {
                    "mood": "happy",
                    "inserted_at": "2019-06-29T12:19:00Z"
                },
                {
                    "mood": "meh",
                    "inserted_at": "2019-06-29T12:24:00Z"
                },
                {
                    "mood": "happy",
                    "inserted_at": "2019-06-29T12:30:00Z"
                },
            ]
        }),
        serde_json::from_slice::<serde_json::Value>(res.body()).unwrap()
    );
}

#[test]
fn surveys_feedback_index_not_found_test() {
    let db = Database::new();

    let res = warp::test::request()
        .path("/surveys/0/feedback")
        .method("GET")
        .reply(&routes(db));
    assert_eq!(404, res.status());
    assert_eq!("", String::from_utf8_lossy(res.body()));
}

#[test]
fn create_survey_feedback_not_found_test() {
    let res = warp::test::request()
        .method("POST")
        .path("/surveys/0/feedback")
        .body(r#"{"mood": "happy"}"#)
        .reply(&routes(Database::new()));
    assert_eq!(404, res.status());
    assert_eq!("", String::from_utf8_lossy(res.body()));
}

#[test]
fn create_survey_feedback_ok_test() {
    let db = Database::new();
    db.seed();

    let survey = db.get_survey(1).unwrap();
    assert_eq!(survey.feedback.len(), 6);

    let res = warp::test::request()
        .method("POST")
        .path("/surveys/1/feedback")
        .body(r#"{"mood": "sad"}"#)
        .reply(&routes(db.clone()));
    assert_eq!(201, res.status());
    assert_eq!("", String::from_utf8_lossy(res.body()));

    let survey = db.get_survey(1).unwrap();
    assert_eq!(survey.feedback.len(), 7);
    assert_eq!(survey.feedback[6].mood, Mood::Sad);
}

#[test]
fn create_survey_feedback_invalid_test() {
    let db = Database::new();

    let res = warp::test::request()
        .method("POST")
        .path("/surveys/1/feedback")
        .body(r#"{"mood": "kinda ok I guess"}"#)
        .reply(&routes(db.clone()));
    assert_eq!(400, res.status());
    assert_eq!(
        serde_json::json!({
            "errors": {
                "feedback": [
                    "unknown variant `kinda ok I guess`, expected one of `happy`, `meh`, `sad` at line 1 column 27"
                ]
            }
        }),
        serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(res.body())).unwrap()
    );

    assert_eq!(db.all_surveys().len(), 0)
}

#[test]
fn create_survey_invalid_test() {
    let db = Database::new();

    let res = warp::test::request()
        .method("POST")
        .path("/surveys")
        .body(r#"{}"#)
        .reply(&routes(db.clone()));
    assert_eq!(400, res.status());
    assert_eq!(
        serde_json::json!({
            "errors": {
                "survey": [
                    "missing field `title` at line 1 column 2"
                ]
            }
        }),
        serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(res.body())).unwrap()
    );

    assert_eq!(db.all_surveys().len(), 0)
}

#[test]
fn create_survey_ok_test() {
    let db = Database::new();

    let res = warp::test::request()
        .method("POST")
        .path("/surveys")
        .body(
            r##"{
                "title": "Hello, world!",
                "description": "Magic",
                "tags": ["one", "two"],
                "colour": "#ffffff"
            }"##,
        )
        .reply(&routes(db.clone()));
    assert_eq!(201, res.status());

    let survey: Survey = serde_json::from_str(&String::from_utf8_lossy(res.body())).unwrap();
    assert_eq!(survey.id, 1);
    assert_eq!(survey.title, "Hello, world!".to_string());
    assert_eq!(survey.description, Some("Magic".to_string()));
    assert_eq!(survey.tags, vec!["one".to_string(), "two".to_string()]);
    assert_eq!(survey.colour, Some("#ffffff".to_string()));
    assert_eq!(survey.feedback, vec![]);

    assert_eq!(db.get_survey(1), Some(survey))
}
