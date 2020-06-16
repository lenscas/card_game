use crate::{
    responses::{CustomResult, LoginResponse},
    Result,
};
use card_game_shared::{
    battle::{BattleErrors, ReturnBattle, TakeAction, TurnResponse},
    users::LoginData,
};
use quicksilver::{graphics::Image, Graphics};
use silver_surf::{call, Config, Method};
use std::collections::HashMap;

pub enum AfterTurn {
    Over,
    NewTurn(ReturnBattleWithImages),
    NoTurnHappened,
}
pub struct ReturnBattleWithImages {
    pub(crate) images: Vec<Image>,
    pub(crate) battle: ReturnBattle,
}

pub struct Client {
    base_url: String,
    authorization_code: Option<String>,
    cached_images: HashMap<String, Image>,
}
impl Client {
    pub fn new(base_url: String) -> Client {
        Client {
            base_url,
            authorization_code: None,
            cached_images: HashMap::new(),
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
    pub(crate) async fn log_in(&mut self, username: String, password: String) -> Result<()> {
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
        let v = v.into_dyn_res()?;
        self.authorization_code = Some(v.token);
        Ok(())
    }
    async fn load_image(&mut self, path: String, gfx: &Graphics) -> Result<Image> {
        let headers = self.set_headers();
        let url = self.set_url(&(String::from("assets/") + &path));

        let entry = self.cached_images.entry(path.clone());
        Ok(match entry {
            std::collections::hash_map::Entry::Occupied(v) => v.get().clone(),
            std::collections::hash_map::Entry::Vacant(x) => {
                let res = call(Config::<()> {
                    url,
                    method: Method::Get,
                    body: None,
                    headers,
                })?
                .bytes()
                .await?;
                let image = Image::from_encoded_bytes(gfx, &res)?;
                x.insert(image.clone());
                image
            }
        })
    }

    pub(crate) async fn new_battle(&mut self, gfx: &Graphics) -> Result<ReturnBattleWithImages> {
        let res = call(Config::<()> {
            url: self.set_url("battle"),
            method: Method::Post,
            body: None,
            headers: self.set_headers(),
        })?
        .json::<CustomResult<ReturnBattle>>()
        .await;
        let res = res?.into_dyn_res()?;
        let mut cards = Vec::new();
        for id in &res.hand {
            cards.push(
                self.load_image(String::from("cards/") + &id + ".png", gfx)
                    .await?,
            );
        }
        Ok(ReturnBattleWithImages {
            battle: res,
            images: cards,
        })
    }
    pub(crate) async fn do_turn(&mut self, card: usize, gfx: &Graphics) -> Result<AfterTurn> {
        let res = call(Config {
            url: self.set_url("battle/"),
            method: Method::Put,
            body: Some(TakeAction { play_card: card }),
            headers: self.set_headers(),
        })?
        .json::<CustomResult<TurnResponse>>()
        .await;
        let res = dbg!(res);
        let res = res?.into_dyn_res()?;
        let res = match res {
            TurnResponse::NextTurn(b) => b,
            TurnResponse::Error(x) => match x {
                BattleErrors::ChosenCardNotInHand(_) => {
                    todo!("We should try to reget the state here. However there is no endpoint to do this yet so instead lets crash")
                },
                BattleErrors::CardCostsTooMuch {..} => {
                    return Ok(AfterTurn::NoTurnHappened)
                }
            },
            //we should return something else to let the caller know the battle is over
            //however, at this point the server doesn't even know when a battle is over (nor who won/lost)
            //until that is added this should be decent enough.
            TurnResponse::Done => return Ok(AfterTurn::Over),
        };

        let mut cards = Vec::new();
        for id in &res.hand {
            cards.push(
                self.load_image(String::from("cards/") + &id + ".png", gfx)
                    .await?,
            );
        }
        Ok(AfterTurn::NewTurn(ReturnBattleWithImages {
            battle: res,
            images: cards,
        }))
    }
}
