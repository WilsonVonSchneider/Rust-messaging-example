use super::data::RequestSlackMessageData;
use actix_web::{web, ResponseError};
use http::StatusCode;
use validr::*;

async fn test_actix_route_handler_slack_message_attributes(
    test: web::Json<RequestSlackMessageData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => http::StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}
#[actix_web::main]
#[test]
async fn test_request_create_new_slack_message_success_validation() {
    let data = RequestSlackMessageData {
        name: Some("example_name".to_string()),
        channel: Some("example_channel".to_string()),
        icon_emoji: Some("example_emoji".to_string()),
        message: Some("example message".to_string()),
    };
    let response = test_actix_route_handler_slack_message_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::OK)    
}
#[actix_web::main]
#[test]
async fn test_request_create_new_slack_message_failed_validation_empty_required_field_name() {
    let data = RequestSlackMessageData {
        name: None,
        channel: Some("example_channel".to_string()),
        icon_emoji: Some("example_emoji".to_string()),
        message: Some("example message".to_string()),
    };
    let response = test_actix_route_handler_slack_message_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}
#[actix_web::main]
#[test]
async fn test_request_create_new_slack_message_failed_validation_empty_required_field_channel() {
    let data = RequestSlackMessageData {
        name: Some("example_name".to_string()),
        channel: None,
        icon_emoji: Some("example_emoji".to_string()),
        message: Some("example message".to_string()),
    };
    let response = test_actix_route_handler_slack_message_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}
#[actix_web::main]
#[test]
async fn test_request_create_new_slack_message_failed_validation_empty_required_field_icon_emoji() {
    let data = RequestSlackMessageData {
        name: Some("example_name".to_string()),
        channel: Some("example_channel".to_string()),
        icon_emoji: None,
        message: Some("example message".to_string()),
    };
    let response = test_actix_route_handler_slack_message_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}
#[actix_web::main]
#[test]
async fn test_request_create_new_slack_message_failed_validation_empty_required_field_message() {
    let data = RequestSlackMessageData {
        name: Some("example_name".to_string()),
        channel: Some("example_channel".to_string()),
        icon_emoji: Some("example_emoji".to_string()),
        message: None,
    };
    let response = test_actix_route_handler_slack_message_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}
