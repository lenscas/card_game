use crate::responses::{CustomResult, LoginResponse};
use card_game_shared::LoginData;
use card_game_shared::ReturnBattle;
use card_game_shared::TakeAction;
use silver_surf::{call, Config, Method};

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
    fn set_url(&self, part: &str) -> String {
        format!("{}{}", self.base_url, part)
    }
    fn set_headers(&self) -> Option<Vec<(&'static str, String)>> {
        if let Some(code) = &self.authorization_code {
            Some(vec![("authorization_token", code.clone())])
        } else {
            None
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
            body: Some(LoginData { username, password }),
            headers: None,
        })?
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
    pub(crate) async fn new_battle(&mut self) -> Result<ReturnBattle, Box<dyn std::error::Error>> {
        let res = call(Config::<()> {
            url: self.set_url("battle"),
            method: Method::Post,
            body: None,
            headers: self.set_headers(),
        })?
        .json::<CustomResult<ReturnBattle>>()
        .await;
        let res = dbg!(res);
        let res = dbg!(res);
        let res = match res {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        let res = Result::<_, _>::from(res)?;
        Ok(res)
    }
    pub(crate) async fn do_turn(
        &self,
        card: usize,
    ) -> Result<ReturnBattle, Box<dyn std::error::Error>> {
        let res = call(Config {
            url: self.set_url("battle/"),
            method: Method::Put,
            body: Some(TakeAction { play_card: card }),
            headers: self.set_headers(),
        })?
        .json::<CustomResult<ReturnBattle>>()
        .await;
        let res = dbg!(res);
        let res = match res {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        let res = Result::<_, _>::from(res)?;
        Ok(res)
    }
}
