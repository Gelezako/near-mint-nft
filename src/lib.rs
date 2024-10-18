use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey};
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens_by_id: UnorderedMap<String, Token>,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    TokensById,
    Metadata,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    title: Option<String>,
    description: Option<String>,
    media: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    spec: String,
    name: String,
    symbol: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, name: String, symbol: String) -> Self {
        let metadata = NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name,
            symbol,
        };

        Self {
            owner_id,
            tokens_by_id: UnorderedMap::new(StorageKey::TokensById),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    pub fn mint_nft(&mut self, token_id: String, metadata: TokenMetadata) {
        let caller = env::predecessor_account_id();

        assert_eq!(caller, "gelezako.testnet", "Only the owner can mint NFTs");

        let token = Token {
            owner_id: caller.clone(),
            metadata,
        };

        self.tokens_by_id.insert(&token_id, &token);
        env::log_str(&format!("Token {} minted for {}", token_id, caller));
    }

    // Метод для отримання інформації про конкретний токен
    pub fn nft_token(&self, token_id: String) -> Option<Token> {
        self.tokens_by_id.get(&token_id)
    }

    // Метод для отримання інформації про метадані контракту
    pub fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
