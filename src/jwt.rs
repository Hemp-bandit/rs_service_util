use crate::response::BizError;
use serde::{Deserialize, Serialize};
use simple_base64::{decode, encode};
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify,
};

#[derive(Debug, Serialize)]
pub struct JWT;

impl Modify for JWT {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "JWT",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub fn gen_jwt_token<T: Serialize>(login_data: T) -> String {
    let json_str = serde_json::to_string(&login_data).expect("msg");
    let base64_string = encode(json_str);
    base64_string
}

pub fn jwt_token_to_data<T: for<'a> Deserialize<'a>>(jwt_token: String) -> Result<T, BizError> {
    if jwt_token.is_empty() {
        return Err(BizError::AuthError);
    }

    match decode(jwt_token) {
        Err(err) => {
            log::error!("{err}");
            return Err(BizError::AuthError);
        }
        Ok(res) => {
            let str: String = String::from_utf8(res).unwrap();
            let data = serde_json::from_str(&str).expect("msg");
            Ok(data)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        jwt::{gen_jwt_token, jwt_token_to_data},
        structs::RedisLoginData,
    };

    #[test]
    fn get_gen_jwt() {
        let login_data = RedisLoginData {
            auth: 123123123123,
            last_login_time: 12312312,
            name: "asdf".to_string(),
            id: 123,
        };
        let token_res = gen_jwt_token(login_data);
        println!("token_res {token_res}");
        let token = "eyJhdXRoIjoxMjMxMjMxMjMxMjMsImxhc3RfbG9naW5fdGltZSI6MTIzMTIzMTIsIm5hbWUiOiJhc2RmIiwiaWQiOjEyM30=".to_string();
        assert_eq!(token_res, token)
    }

    #[test]
    fn test_jwt_token_to_data() {
        let token = "eyJhdXRoIjoxMjMxMjMxMjMxMjMsImxhc3RfbG9naW5fdGltZSI6MTIzMTIzMTIsIm5hbWUiOiJhc2RmIiwiaWQiOjEyM30=".to_string();
        let login_data = RedisLoginData {
            auth: 123123123123,
            last_login_time: 12312312,
            name: "asdf".to_string(),
            id: 123,
        };
        let jwt_token_to_data = jwt_token_to_data::<RedisLoginData>(token);
        let user_info = jwt_token_to_data;
        println!("{user_info:#?}")
        // assert_eq!(login_data, user_info.unwrap());
    }
}
