#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, String};

#[contracttype]
#[derive(Clone)]
pub struct Post {
    pub id: u32,
    pub content: String,
    pub author: String,
    pub likes: u32,
}

#[contract]
pub struct KnowledgeSharingContract;

#[contractimpl]
impl KnowledgeSharingContract {

    // Add a new post
    pub fn create_post(env: Env, content: String, author: String) -> u32 {
        let mut count: u32 = env.storage().instance().get(&Symbol::short("COUNT")).unwrap_or(0);
        count += 1;

        let post = Post {
            id: count,
            content,
            author,
            likes: 0,
        };

        env.storage().instance().set(&count, &post);
        env.storage().instance().set(&Symbol::short("COUNT"), &count);

        count
    }

    // Like a post
    pub fn like_post(env: Env, id: u32) {
        let mut post: Post = env.storage().instance().get(&id).unwrap();

        post.likes += 1;

        env.storage().instance().set(&id, &post);
    }

    // Get a post
    pub fn get_post(env: Env, id: u32) -> Post {
        env.storage().instance().get(&id).unwrap()
    }
}