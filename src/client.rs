use crate::responses::{CustomResult, LoginResponse};
use quick_surf::{call, Config, Method};
use serde_derive::Serialize;

pub struct Client {
    base_url: String,
    authorization_code: Option<String>,
}
impl Client {
    pub fn new(base_url: String) -> Client {
        Client {
            base_url,
            authorization_code: None,
        }
    }
    pub(crate) async fn log_in(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let v = call(Config {
            url: format!("{}{}", self.base_url, "login"),
            method: Method::Post,
            body: Some(LoginBody { username, password }),
            headers: None,
        })
        .json::<CustomResult<LoginResponse>>()
        .await;
        let v = match v {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        let v = dbg!(v);
        let v = Result::<_, _>::from(v)?;
        self.authorization_code = Some(v.token);
        Ok(())
    }
}

#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
}
