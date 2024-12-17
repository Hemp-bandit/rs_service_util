use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::HeaderValue,
    middleware::Next,
    Error,
};
use redis::AsyncCommands;
use crate::{jwt::jwt_token_to_data, redis_conn, response::BizError, RedisLoginData};

pub async fn jwt_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if !check_is_in_whitelist(&req) {
        let check_res = has_permission(&req).await;
        if let Err(e) = check_res {
            return Err(Error::from(e));
        }
    }

    let res = next.call(req).await?;
    Ok(res)
}

fn check_is_in_whitelist(req: &ServiceRequest) -> bool {
    let path = req.path();
    // 白名单不校验
    let white_list: Vec<&str> = vec!["/doc"];
    let is_in_white_list = white_list
        .iter()
        .find(|val| val.to_string() == path.to_string());
    is_in_white_list.is_some()
}

async fn has_permission(req: &ServiceRequest) -> Result<bool, BizError> {
    let value: HeaderValue = HeaderValue::from_str("").unwrap();

    let binding = req.method().to_owned();
    let req_method = binding.as_str();
    if req_method == "OPTIONS" {
        return Ok(true);
    }

    let token = req.headers().get("Authorization").unwrap_or(&value);
    if token.is_empty() || token.len() < 7 {
        return Err(BizError::AuthError);
    };

    let binding = token.to_owned();
    let jwt_token = binding.to_str().expect("msg").to_string();
    let slice = &jwt_token[7..];
    log::info!("jwt {slice}");
    let jwt_user = jwt_token_to_data::<RedisLoginData>(slice.to_owned())?;
    log::info!("jwt_user {jwt_user:?}");
    // jwt_user.name
    let res = check_is_login_redis(jwt_user.name).await?;
    Ok(res)
}

pub async fn check_is_login_redis(user_name: String) -> Result<bool, BizError> {
    let redis_key = std::env::var("REDIS_KEY").expect("REDIS_KEY must be set");
    let key = format!("{}_{}", redis_key.to_string(), user_name);

    let mut rds = redis_conn!().await;
    let redis_login: Result<bool, redis::RedisError> = rds.exists(key).await;
    let is_login = match redis_login {
        Err(err) => {
            let detail = err.detail();
            log::error!("{detail:?}",);
            return Err(BizError::AuthError);
        }
        Ok(res) => res,
    };
    // TODO: 添加自动刷新
    Ok(is_login)
}
