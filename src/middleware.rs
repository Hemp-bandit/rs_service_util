use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::HeaderValue,
    middleware::Next,
    Error,
};
use redis::AsyncCommands;

use crate::{jwt::jwt_token_to_data, response::BizError, RedisLoginData};

pub async fn jwt_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // 不是服务调用，也不在白名单中
    let condition = !check_service_call(&req) && !check_is_in_whitelist(&req);

    if condition {
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
    let white_list: Vec<&str> = vec!["/api/auth/login", "/api/obs/get_keys", "/doc"];
    let is_in_white_list = white_list
        .iter()
        .find(|val| val.to_string() == path.to_string());
    is_in_white_list.is_some()
}

fn check_service_call(req: &ServiceRequest) -> bool {
    let header = req.headers().get("service_call");
    log::info!("header {header:?}",);
    header.is_some()
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
    let jwt_user = jwt_token_to_data::<RedisLoginData>(slice.to_owned())
        .expect(&BizError::AuthError.to_string());
    log::info!("jwt_user {jwt_user:?}");
    // jwt_user.name
    check_is_login_redis(jwt_user.name).await
}

pub async fn check_is_login_redis(user_name: String) -> Result<bool, BizError> {
    let key = format!("{}_{}", crate::REDIS_KEY.to_string(), user_name);

    let rds = crate::REDIS.get().expect("msg");
    let mut conn = rds.conn.clone();
    let redis_login: Result<bool, redis::RedisError> = conn.exists(key).await;
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
