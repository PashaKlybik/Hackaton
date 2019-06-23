/*
 * Copyright 2018 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::error_type::AppResult;
use crate::request_response::Response;

use crate::settings::{INIT_ACCOUNT_BALANCE, PAYOUT_RATE, DATA_MAX_COUNT, SEED};
use linked_hash_map::LinkedHashMap;
use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use serde_json::Value;

use fluence::sdk::*;
use log::info;



pub struct GameManager {
    // map from players id to account state
    datas: LinkedHashMap<u64, String>,
    // count of registered players, used for new player id generation
    registered_datas: u64,
    // random generator, used for generating dice result
    // rng: IsaacRng,
}

impl GameManager {
    // pub const DICE_LINE_COUNT: u8 = 6;

    pub fn new() -> Self {
        GameManager {
            registered_datas: 0,
            datas: LinkedHashMap::new(),
            // rng: SeedableRng::seed_from_u64(SEED),
        }
    }

    /// Creates a new player, returns its id.
    pub fn add_data(&mut self, data :String) -> AppResult<Value> {
        info!("data {}", data);

        if self.datas.len() >= DATA_MAX_COUNT {
            self.datas.pop_front();
        }

        self.datas
            .insert(self.registered_datas, data);

        let response = Response::AddData {
            data_id: self.registered_datas,
        };

        self.registered_datas += 1;

        serde_json::to_value(response).map_err(Into::into)
    }
// {
//     /// Checks parameters of given bet and processes it.
//     // pub fn roll(&mut self, player_id: u64, bet_placement: u8, bet_size: u32) -> AppResult<Value> {
//     //     fn check_bet(player_balance: u64, bet_placement: u8, bet_size: u64) -> AppResult<()> {
//     //         if bet_size > player_balance {
//     //             return Err(format!(
//     //                 "Player hasn't enough money: player's current balance is {} while the bet is {}",
//     //                 player_balance, bet_size
//     //             ))
//     //                 .map_err(Into::into);
//     //         }

//     //         if bet_placement > GameManager::DICE_LINE_COUNT {
//     //             return Err("Incorrect placement, please choose number from 1 to 6")
//     //                 .map_err(Into::into);
//     //         }

//     //         Ok(())
//     //     }

//     //     fn update_balance(player_balance: u64, bet_size: u64, bet_placement: u8, outcome: u8) -> u64 {
//     //         if bet_placement == outcome {
//     //             player_balance + (bet_size * PAYOUT_RATE)
//     //         } else {
//     //             player_balance - bet_size
//     //         }
//     //     }

//     //     let player_balance = self.player_balance(player_id)?;
//     //     let bet_size = u64::from(bet_size);
//     //     check_bet(player_balance, bet_placement, bet_size)?;

//     //     let outcome = self.rng.gen::<u8>() % GameManager::DICE_LINE_COUNT + 1;
//     //     let new_player_balance = update_balance(player_balance, bet_size, bet_placement, outcome);

//     //     let response = Response::Roll {
//     //         outcome,
//     //         player_balance: new_player_balance,
//     //     };

//     //     // update balance of the player
//     //     *self.datas.get_mut(&player_id).unwrap() = new_player_balance;

//     //     serde_json::to_value(response).map_err(Into::into)
//     // }
// }
    /// Returns the balance of the player identified by given `player_id`.
    pub fn get_data(&self, data_id: u64) -> AppResult<Value> {
        let data = self.get_data_from_map(data_id)?;
        let response = Response::GetData { data };

        serde_json::to_value(response).map_err(Into::into)
    }

    // returns a balance if there is a such player and Err() otherwise
    // fn get_data_fro(&self, data_id: u64) -> AppResult<str> {
    //     let data = self
    //         .datas
    //         .get(&data_id)
    //         .ok_or_else(|| format!("Player with id {} wasn't found", player_id))?;

    //     Ok(*data)
    // }
    fn get_data_from_map(&self, data_id: u64) -> AppResult<String> {
        let data = self
            .datas
            .get(&data_id)
            .ok_or_else(|| format!("Player with id {} wasn't found", data_id))?;

        Ok(data.to_string())
    }
}
