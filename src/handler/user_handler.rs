use crate::api::api_result::{ApiOK,Result};

pub async fn get_user() -> Result<ApiOK<String>>{


    Ok(ApiOK(Some("查询用户".to_string())))
}