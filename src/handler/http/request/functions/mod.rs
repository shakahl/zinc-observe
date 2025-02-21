// Copyright 2022 Zinc Labs Inc. and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{delete, get, post, put, web, HttpResponse};
#[cfg(feature = "zo_functions")]
use actix_web::{http, HttpRequest};
#[cfg(feature = "zo_functions")]
use std::collections::HashMap;
use std::io::Error;

#[cfg(feature = "zo_functions")]
use crate::meta;
#[cfg(feature = "zo_functions")]
use crate::meta::functions::StreamOrder;
use crate::meta::functions::Transform;

/** CreateFunction*/
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "createFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
    ),
    request_body(content = Transform, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[post("/{org_id}/functions")]
pub async fn save_function(
    path: web::Path<String>,
    func: web::Json<Transform>,
) -> Result<HttpResponse, Error> {
    let org_id = path.into_inner();
    let transform = func.into_inner();
    crate::service::functions::save_function(org_id, transform).await
}

/** ListFunctions */
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "listFunctions",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = FunctionList),
    )
)]
#[get("/{org_id}/functions")]
async fn list_functions(org_id: web::Path<String>) -> Result<HttpResponse, Error> {
    crate::service::functions::list_functions(org_id.into_inner()).await
}

/** DeleteFunction*/
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "deleteFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("name" = String, Path, description = "Function name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 404, description="NotFound", content_type = "application/json", body = HttpResponse),
    )
)]
#[delete("/{org_id}/functions/{name}")]
async fn delete_function(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (org_id, name) = path.into_inner();
    crate::service::functions::delete_function(org_id, name).await
}

/** UpdateFunction */
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "updateFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("name" = String, Path, description = "Function name"),
    ),
    request_body(content = Transform, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[put("/{org_id}/functions/{name}")]
pub async fn update_function(
    path: web::Path<(String, String)>,
    func: web::Json<Transform>,
) -> Result<HttpResponse, Error> {
    let (org_id, name) = path.into_inner();
    let transform = func.into_inner();
    crate::service::functions::update_function(org_id, name, transform).await
}

/** ListStreamFunctions */
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "listStreamFunctions",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = StreamFunctionsList),
    )
)]
#[get("/{org_id}/{stream_name}/functions")]
async fn list_stream_functions(
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let (org_id, stream_name) = path.into_inner();
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let mut stream_type = match crate::common::http::get_stream_type_from_request(&query) {
        Ok(v) => v,
        Err(e) => {
            return Ok(
                HttpResponse::BadRequest().json(meta::http::HttpResponse::error(
                    http::StatusCode::BAD_REQUEST.into(),
                    e.to_string(),
                )),
            )
        }
    };
    if stream_type.is_none() {
        stream_type = Some(meta::StreamType::Logs);
    }
    crate::service::functions::list_stream_functions(org_id, stream_type.unwrap(), stream_name)
        .await
}

/** RemoveStreamFunction*/
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "removeStreamFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
        ("name" = String, Path, description = "Function name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 404, description="NotFound", content_type = "application/json", body = HttpResponse),
    )
)]
#[delete("/{org_id}/{stream_name}/functions/{name}")]
async fn delete_stream_function(
    path: web::Path<(String, String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let (org_id, stream_name, name) = path.into_inner();
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let mut stream_type = match crate::common::http::get_stream_type_from_request(&query) {
        Ok(v) => v,
        Err(e) => {
            return Ok(
                HttpResponse::BadRequest().json(meta::http::HttpResponse::error(
                    http::StatusCode::BAD_REQUEST.into(),
                    e.to_string(),
                )),
            )
        }
    };
    if stream_type.is_none() {
        stream_type = Some(meta::StreamType::Logs);
    }
    crate::service::functions::delete_stream_function(
        org_id,
        stream_type.unwrap(),
        stream_name,
        name,
    )
    .await
}

/** ApplyFunctionToStream */
#[cfg(feature = "zo_functions")]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "applyFunctionToStream",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
        ("name" = String, Path, description = "Function name"),
    ),
    request_body(content = StreamOrder, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[post("/{org_id}/{stream_name}/functions/{name}")]
pub async fn add_function_to_stream(
    path: web::Path<(String, String, String)>,
    stream_order: web::Json<StreamOrder>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let (org_id, stream_name, name) = path.into_inner();
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let mut stream_type = match crate::common::http::get_stream_type_from_request(&query) {
        Ok(v) => v,
        Err(e) => {
            return Ok(
                HttpResponse::BadRequest().json(meta::http::HttpResponse::error(
                    http::StatusCode::BAD_REQUEST.into(),
                    e.to_string(),
                )),
            )
        }
    };
    if stream_type.is_none() {
        stream_type = Some(meta::StreamType::Logs);
    }
    crate::service::functions::add_function_to_stream(
        org_id,
        stream_type.unwrap(),
        stream_name,
        name,
        stream_order.into_inner(),
    )
    .await
}

/** CreateFunction*/
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "createFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
    ),
    request_body(content = Transform, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[post("/{org_id}/functions")]
pub async fn save_function(
    _path: web::Path<(String, String)>,
    _js_func: web::Json<Transform>,
) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** ListFunctions */
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "listFunctions",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = FunctionList),
    )
)]
#[get("/{org_id}/functions")]
async fn list_functions(_org_id: web::Path<String>) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** DeleteFunction*/
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "deleteFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("name" = String, Path, description = "Function name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 404, description="NotFound", content_type = "application/json", body = HttpResponse),
    )
)]
#[delete("/{org_id}/functions/{name}")]
async fn delete_function(_path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** UpdateFunction */
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "updateFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("name" = String, Path, description = "Function name"),
    ),
    request_body(content = Transform, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[put("/{org_id}/functions/{name}")]
pub async fn update_function(
    _path: web::Path<(String, String)>,
    _func: web::Json<Transform>,
) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** ApplyFunctionToStream */
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "applyFunctionToStream",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
        ("name" = String, Path, description = "Function name"),
    ),
    request_body(content = StreamOrder, description = "Function data", content_type = "application/json"),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 400, description="Failure", content_type = "application/json", body = HttpResponse),
    )
)]
#[post("/{org_id}/{stream_name}/functions/{name}")]
pub async fn add_function_to_stream(
    _path: web::Path<(String, String, String)>,
    _js_func: web::Json<Transform>,
) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** ListStreamFunctions */
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "listStreamFunctions",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = StreamFunctionsList),
    )
)]
#[get("/{org_id}/{stream_name}/functions")]
async fn list_stream_functions(_path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

/** RemoveStreamFunction*/
#[cfg(not(feature = "zo_functions"))]
#[utoipa::path(
    context_path = "/api",
    tag = "Functions",
    operation_id = "removeStreamFunction",
    security(
        ("Authorization"= [])
    ),
    params(
        ("org_id" = String, Path, description = "Organization name"),
        ("stream_name" = String, Path, description = "Stream name"),
        ("name" = String, Path, description = "Function name"),
    ),
    responses(
        (status = 200, description="Success", content_type = "application/json", body = HttpResponse),
        (status = 404, description="NotFound", content_type = "application/json", body = HttpResponse),
    )
)]
#[delete("/{org_id}/{stream_name}/functions/{name}")]
async fn delete_stream_function(
    _path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, Error> {
    fn_not_supported_error()
}

#[cfg(not(feature = "zo_functions"))]
fn fn_not_supported_error() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::NotImplemented().json(crate::meta::http::HttpResponse::message(
            actix_web::http::StatusCode::NOT_IMPLEMENTED.into(),
            "Functions support is not enabled".to_string(),
        )),
    )
}
