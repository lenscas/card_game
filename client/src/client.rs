use crate::{
    responses::{CustomResult, LoginResponse},
    Result, image_loader::ImageLoader,
};

use card_game_shared::{
    battle::{BattleErrors, ReturnBattle, TakeAction, TurnResponse},
    characters::{CharacterCreationResponse, CharacterList},
    dungeon::EventProcesed,
    users::LoginData,
};
use quicksilver::{
    graphics::Image,
    Graphics,
};
use silver_surf::{call, Config, Method};

pub enum AfterTurn {
    Over,
    NewTurn(ReturnBattleWithImages),
    NoTurnHappened,
}
pub struct ReturnBattleWithImages {
    pub(crate) images: Vec<Image>,
    pub(crate) battle: ReturnBattle,
}

pub(crate) struct ClientConfig {
    base_url : String,
    authorization_code: Option<String>,
}
impl ClientConfig {
    pub(crate) fn new(base_url : String) -> Self {
        Self {
            base_url,authorization_code:None
        }
    }
    pub(crate) fn set_url(&self, parts: &[&str]) -> String {
        let url = parts.join("/");
        if self.base_url.ends_with('/') {
            format!("{}{}", self.base_url, url)
        } else {
            format!("{}/{}", self.base_url, url)
        }
    }
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }
    pub(crate) fn set_headers(&self) -> Option<Vec<(&'static str, String)>> {
        if let Some(code) = &self.authorization_code {
            Some(vec![("authorization_token", code.clone())])
        } else {
            None
        }
    }
}

pub struct Client {
    pub(crate)config : ClientConfig,
    image_loader :ImageLoader
}

impl Client {
    pub(crate) async fn new(base_url: String) -> Result<Client> {
        let config = ClientConfig::new(base_url);
        let image_loader =ImageLoader::new( &config).await?;
        Ok(Client {
            config, 
            image_loader
        })
    }

    
    pub(crate) async fn log_in(&mut self, username: String, password: String) -> Result<()> {
        let v = call(Config {
            url: self.config.set_url(&["login"]),
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
        self.config.authorization_code = Some(v.token);
        Ok(())
    }
    pub(crate) async fn load_image(&mut self, path: String, gfx: &Graphics)  -> Result<Image>  {
        self.image_loader.load_image(&self.config, path, gfx).await
    }

    pub(crate) async fn get_battle(
        &mut self,
        character_id: i64,
        gfx: &Graphics,
    ) -> Result<ReturnBattleWithImages> {
        let x: ReturnBattle = call(Config::<()> {
            url: self.config.set_url(&["battle", &character_id.to_string()]),
            method: Method::Get,
            body: None,
            headers: self.config.set_headers(),
        })?
        .json()
        .await?;
        let mut cards = Vec::new();
        for id in &x.hand {
            cards.push(
                self.load_image(String::from("cards/") + &id + ".png", gfx)
                    .await?,
            );
        }
        Ok(ReturnBattleWithImages {
            battle: x,
            images: cards,
        })
    }

    pub(crate) async fn set_new_base_url(&mut self, url : String) -> Result<()> {
        self.config.base_url = url;
        self.image_loader.invalidate_cache(&self.config).await?;
        Ok(())
    }

    pub(crate) async fn do_turn(
        &mut self,
        card: usize,
        character_id: i64,
        gfx: &Graphics,
    ) -> Result<AfterTurn> {
        let res = call(Config {
            url: self.config.set_url(&["battle"]),
            method: Method::Post,
            body: Some(TakeAction {
                play_card: card,
                character_id,
            }),
            headers: self.config.set_headers(),
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
    pub(crate) async fn get_characters(&self) -> Result<CharacterList> {
        call(Config::<()> {
            url: self.config.set_url(&["characters"]),
            method: Method::Get,
            body: None,
            headers: self.config.set_headers(),
        })?
        .json()
        .await
    }
    pub(crate) async fn create_character(&self) -> Result<CharacterCreationResponse> {
        call(Config::<()> {
            url: self.config.set_url(&["characters"]),
            method: Method::Post,
            body: None,
            headers: self.config.set_headers(),
        })?
        .json()
        .await
    }
    pub(crate) async fn is_chracter_in_battle(&self, char_id: i64) -> Result<bool> {
        call(Config::<()> {
            url: self.config.set_url(&["characters", &char_id.to_string()]),
            method: Method::Get,
            body: None,
            headers: self.config.set_headers(),
        })?
        .json()
        .await
    }
    pub(crate) async fn get_dungeon(
        &self,
        char_id: i64,
    ) -> Result<card_game_shared::dungeon::DungeonLayout> {
        call(Config::<()> {
            url: self.config.set_url(&["dungeon", &char_id.to_string()]),
            method: Method::Get,
            body: None,
            headers: self.config.set_headers(),
        })?
        .json()
        .await
    }
    pub(crate) async fn move_in_dungeon(
        &self,
        char_id: i64,
        dir: card_game_shared::BasicVector<i64>,
    ) -> Result<card_game_shared::dungeon::EventProcesed> {
        let x: CustomResult<EventProcesed> = call(Config {
            url: self.config.set_url(&["dungeon", &char_id.to_string(), "move"]),
            method: Method::Post,
            body: Some(dir),
            headers: self.config.set_headers(),
        })?
        .json()
        .await?;
        let res = dbg!(x);
        match res {
            CustomResult::Ok(x) => Ok(x),
            CustomResult::Err(x) => match x {
                crate::responses::ErrorRes::Basic { message } => {
                    let x = serde_json::from_str(&message);
                    match x {
                        Ok(x) => Ok(x),
                        Err(_) => CustomResult::Err(crate::responses::ErrorRes::Basic { message })
                            .into_dyn_res(),
                    }
                }
            },
        }
    }
}
