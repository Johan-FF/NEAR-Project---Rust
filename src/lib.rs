use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

setup_alloc!();

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Image {
    pub id: u64,
    pub created_by: String,
    pub title: String,
    pub museum: String,
    pub url: String,
    pub donations: u128,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            id: 0,
            created_by: String::from(""),
            title: String::from(""),
            museum: String::from(""),
            url: String::from(""),
            donations: 0,
        }
    }
}

impl Image {
    pub fn new(title: String, museum: String, url: String) -> Self {
        Self {
            id: env::block_index(),
            created_by: env::signer_account_id(),
            title,
            museum,
            url,
            donations: 0,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleImageMuseum {
    museums: UnorderedMap<String, Vec<u64>>,
    images: UnorderedMap<u64, Image>,
}

impl Default for SimpleImageMuseum {
    fn default() -> Self {
        Self {
            museums: UnorderedMap::new(b"u".to_vec()),
            images: UnorderedMap::new(b"e".to_vec()),
        }
    }
}

#[near_bindgen]
impl SimpleImageMuseum {
    pub fn create_image(&mut self, title: String, url: String, museum: String) {
        let image = Image::new(
            String::from(&title),
            String::from(&url),
            String::from(&museum),
        );

        self.images.insert(&image.id, &image);
        let museum_exists = self.museums.get(&museum);
        if museum_exists.is_some() {
            let mut m = museum_exists.unwrap();
            m.push(image.id);
            self.museums.insert(&museum, &m);
        } else {
            let mut new_museum = Vec::new();
            new_museum.push(image.id);
            self.museums.insert(&museum, &new_museum);
        }

        env::log(
            format!(
                "New image successfully added. Museum: {}, Id Image: {}",
                &museum, image.id
            )
            .as_bytes(),
        )
    }

    pub fn get_image(&self, id: u64) -> Option<Image> {
        self.images.get(&id)
    }

    pub fn get_images_list(&self) -> Vec<(u64, Image)> {
        self.images.to_vec()
    }

    pub fn get_museums_list(&self) -> Vec<String> {
        self.museums.keys_as_vector().to_vec()
    }

    pub fn get_images_of_museum(&self, museum: String) -> Vec<Image> {
        let _museum = self.museums.get(&museum);

        if _museum.is_some() {
            let mut images_list = Vec::new();

            for image in &_museum.unwrap() {
                let m = self.images.get(image);
                if m.is_some() {
                    images_list.push(m.unwrap());
                }
            }

            images_list
        } else {
            Vec::new()
        }
    }

    #[payable]
    pub fn donate_an_image(&mut self, id: u64) -> bool {
        assert!(
            env::attached_deposit() > 0,
            "You must add NEAR to make a deposit"
        );

        match self.images.get(&id) {
            Some(mut image) => {
                image.donations += env::attached_deposit();
                self.images.insert(&id, &image);

                Promise::new(String::from(&image.created_by)).transfer(env::attached_deposit());

                true
            }
            None => false,
        }
    }
}
