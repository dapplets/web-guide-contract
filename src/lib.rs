use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, collections::UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WebGuideContract {
    guides: UnorderedMap<String, String>,
}

impl Default for WebGuideContract {
    fn default() -> Self {
        Self {
            guides: UnorderedMap::new(b"g".to_vec()), // Initialize the UnorderedMap
        }
    }
}

#[near_bindgen]
impl WebGuideContract {
    pub fn get_guide(&self, guide_id: String) -> Option<String> {
        self.guides.get(&guide_id)
    }

    pub fn set_guide(&mut self, guide_id: String, data: String) {
        self.guides.insert(&guide_id, &data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set_guide() {
        let mut contract = WebGuideContract::default();
        let guide_id = "example_guide".to_string();
        let data = "example_data".to_string();

        // Set guide
        contract.set_guide(guide_id.clone(), data.clone());
        assert_eq!(contract.get_guide(guide_id.clone()), Some(data.clone()));

        // Update guide
        let updated_data = "updated_data".to_string();
        contract.set_guide(guide_id.clone(), updated_data.clone());
        assert_eq!(contract.get_guide(guide_id.clone()), Some(updated_data.clone()));

        // Non-existing guide
        let non_existing_guide_id = "non_existing_guide".to_string();
        assert_eq!(contract.get_guide(non_existing_guide_id.clone()), None);
    }
}