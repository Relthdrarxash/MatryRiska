// export the home route handler
use std::fs;

use actix_web::{CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::helper::functions::{extract_string_from_obj_value};
use crate::helper::database::{Risk, create_new_risk};


pub async fn create(body:Value) -> CustomizeResponder<HttpResponse> {


    // check the body contain good key
    for key in vec!["name", "description"] {
        if body.get(key).is_some() {
            continue;
        } else {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"missing_args\"}").customize();
        }
    }

    let doc_name = extract_string_from_obj_value(body.get("name"));
    let doc_description = extract_string_from_obj_value(body.get("description"));

    // check if doc_name < 255 char
    if doc_name.len() > 255 {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": true, \"status\": \"name_too_long\"}").customize();
    }

    // sql format to cancel sql injection
    let doc_name = doc_name.replace("'", "\\'");
    let doc_description = doc_description.replace("'", "\\'");


    let _ = create_new_risk(doc_name, doc_description).await;

    return HttpResponse::Ok().content_type("application/json").body(json!({"status": "success"}).to_string()).customize();
}

