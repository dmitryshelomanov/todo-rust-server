use actix_web::Json;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    data: T,
}

pub type ApiJson<T> = Json<ApiResponse<T>>;

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> ApiJson<T> {
        Json(ApiResponse { data })
    }
}
