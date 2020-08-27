use crate::{client::ClientConfig, Result, APP_NAME};
use quicksilver::{
    graphics::Image,
    saving::{load, load_raw, save, save_raw, Location},
    Graphics,
};
use silver_surf::{call, Config, Method};
use std::collections::{HashMap, HashSet};

fn clear_stored_cache(new_gen_time: u64) -> Result<()> {
    save(
        Location::Data,
        APP_NAME,
        "stored_card_names",
        &HashSet::<String>::new(),
    )?;
    save(Location::Data, APP_NAME, "generation_time", &new_gen_time)?;
    Ok(())
}

fn try_load_image_from_cache(
    name: &str,
    gfx: &Graphics,
    stored_images: &HashSet<String>,
) -> Result<Option<Image>> {
    if !stored_images.contains(name) {
        return Ok(None);
    }
    let bytes = load_raw(Location::Data, APP_NAME, name)?;
    Ok(Some(Image::from_encoded_bytes(gfx, &bytes)?))
}

async fn get_gen_time_server(config: &ClientConfig) -> Result<u64> {
    let generation_time_url = config.set_url(&["assets", "cards", "generation_time.txt"]);
    let headers = config.set_headers();
    Ok(String::from_utf8(
        call(Config::<()> {
            url: generation_time_url,
            method: Method::Get,
            body: None,
            headers: headers.clone(),
        })?
        .bytes()
        .await?,
    )?
    .parse()?)
}

pub(crate) struct ImageLoader {
    cached_images: HashMap<String, Image>,
    stored_images: HashSet<String>,
    can_use_stored_images: bool,
}
impl ImageLoader {
    pub(crate) async fn new(config: &ClientConfig) -> Result<Self> {
        let last_card_generation: u64 = get_gen_time_server(config).await?;

        let can_use_cache = match load::<u64>(Location::Data, APP_NAME, "generation_time") {
            Ok(x) => x == last_card_generation,
            Err(_) => false,
        };
        if !can_use_cache {
            clear_stored_cache(last_card_generation)?;
        }

        Ok(Self {
            cached_images: HashMap::new(),
            can_use_stored_images: can_use_cache,
            stored_images: load(Location::Data, APP_NAME, "stored_card_names")?,
        })
    }

    fn store(&mut self, name: String, image: &[u8]) -> Result<()> {
        save_raw(Location::Data, APP_NAME, &name, image)?;
        self.stored_images.insert(name);
        save(
            Location::Data,
            APP_NAME,
            "stored_card_names",
            &self.stored_images,
        )?;
        Ok(())
    }

    pub(crate) async fn invalidate_cache(&mut self, config: &ClientConfig) -> Result<()> {
        self.cached_images = HashMap::new();
        self.can_use_stored_images = false;
        self.stored_images = HashSet::new();

        clear_stored_cache(get_gen_time_server(config).await?)?;
        Ok(())
    }

    pub(crate) async fn load_image(
        &mut self,
        client: &ClientConfig,
        path: String,
        gfx: &Graphics,
    ) -> Result<Image> {
        let headers = client.set_headers();
        let image_url = client.set_url(&["assets", &path]);
        let last_part = String::from(path.split('/').last().unwrap());
        let exists = self.cached_images.contains_key(&path);
        let image = if exists {
            self.cached_images.get(&path).expect("HOW?").clone()
        } else {
            match try_load_image_from_cache(&last_part, gfx, &self.stored_images)? {
                Some(x) => x,
                None => {
                    let res = call(Config::<()> {
                        url: image_url,
                        method: Method::Get,
                        body: None,
                        headers,
                    })?
                    .bytes()
                    .await?;
                    self.store(last_part, &res)?;
                    let image = Image::from_encoded_bytes(gfx, &res)?;
                    self.cached_images.insert(path, image.clone());
                    image
                }
            }
        };
        Ok(image)
    }
}
