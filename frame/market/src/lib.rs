// This file is part of Substrate.

// Copyright (C) Hyungsuk Kang
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Market Module
//!
//! A simple, secure module for exchanging with fungible assets in decentralized manner.
//!
//! ## Overview
//!
//! The Market module provides functionality for asset management of fungible asset classes
//! with a fixed supply, including:
//!
//! * Liquidity provider token issuance
//! * Compensation for providing liquidity
//! * Automated liquidity provisioning
//! * Asset exchange
//!
//! To use it in your runtime, you need to implement the market [`Trait`](./trait.Trait.html).
//!
//! The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.
//!
//! ### Terminology
//!
//! * **Liquidity provider token:** The creation of a new asset by providing liquidity between two fungible assets. Liquidity provider token act as the share of the pool and gets the profit created from exchange fee.
//! * **Asset exchange:** The process of an account transferring an asset to exchange with other kind of fungible asset.
//! * **Fungible asset:** An asset whose units are interchangeable.
//! * **Non-fungible asset:** An asset for which each unit has unique characteristics.
//!
//! ### Goals
//!
//! The market system in Substrate is designed to make the following possible:
//!
//! * Issue a liquidity provider token by depositing two different fungible assets.
//! * Swap assets between accounts with automated market price equation(e.g. X*Y=K or curve function from Kyber, dodoex, etc).
//! * Compensate liquidity provider
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `swap` - Transfers an `amount` of units of fungible asset `id` from the balance of the function caller's account (`origin`) and receive the fungible asset to swap to
//!  to a `target` account.
//! * `burn` - Burns the lptoken and withdraws two fungible assets
//! * `mint` - Deposits two fungible assets and receive lp token
//! that called the function.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//!
//! ### Public Functions
//! <!-- Original author of descriptions: @gavofyork -->
//!
//! * `reserves` - Get the reserves of two fungible assets in a given pair
//! * `pair` - Get the two fungible asset ids for a pair with a given liquidity asset id.
//! * `lpt` - Get the liquidity asset id from the two fungible asset ids
//!
//! Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available functions.
//!
//! ## Usage
//!
//! The following example shows how to use the Assets module in your runtime by exposing public functions to:
//!
//! * Issue a new fungible asset for a token distribution event (airdrop).
//! * Query the fungible asset holding balance of an account.
//! * Query the total supply of a fungible asset that has been issued.
//!
//! ### Prerequisites
//!
//! Import the Assets module and types and derive your runtime's configuration traits from the Assets module trait.
//!
//! ### Simple Code Snippet
//!
//! ```rust,ignore
//! use pallet_market as market;
//! use frame_support::{decl_module, dispatch, ensure};
//! use frame_system::ensure_signed;
//!
//! pub trait Trait: assets::Trait + market::Trait { }
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! 		pub fn issue_liquidity_token(origin, token0: T::AssetId, amount0: <T as balances::Trait>::Balance, token1: T::AssetId, amount1: <T as balances::Trait>::Balance) -> dispatch::DispatchResult {
//! 			let sender = ensure_signed(origin).map_err(|e| e.as_str())?;
//!
//! 			const ACA: u64 = 1;
//! 			const aUSD: u64 = 2;
//!
//! 			ensure!(!COUNT_AIRDROP_RECIPIENTS.is_zero(), "Divide by zero error.");
//! 			<asset::Module<T>>::
//!
//! 			let asset_id = Self::next_asset_id();
//!
//! 			<NextAssetId<T>>::mutate(|asset_id| *asset_id += 1);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_ALICE), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_BOB), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<TotalSupply<T>>::insert(asset_id, TOKENS_FIXED_SUPPLY);
//!
//! 			Self::deposit_event(RawEvent::CreatePair(ACA, aUSD, ACA_aUSD_lpt));
//! 			Ok(())
//! 		}
//! 	}
//! }
//! ```
//!
//! ## Assumptions
//!
//! Below are assumptions that must be held when using this module.  If any of
//! them are violated, the behavior of this module is undefined.
//!
//! * The total count of assets should be less than
//!   `Trait::AssetId::max_value()`.
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use pallet_balances as balances;
use subswap_asset as asset;
mod math;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::traits::*;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::FixedPointNumber;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::StoredMap;
use codec::{Codec, Decode, Encode};

use core::fmt::Debug;
use core::num::NonZeroU128;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{ExistenceRequirement, Get, WithdrawReason},
    Parameter,
};
use frame_system::ensure_signed;
use pallet_timestamp as timestamp;
use sp_runtime::FixedU128;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + asset::Trait + timestamp::Trait + balances::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>>
        + Into<<Self as frame_system::Trait>::Event>
        + Into<<Self as asset::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as SwapModule {
        pub LastBlockTimestamp get(fn last_block_timestamp): T::Moment;
        // Accumulated price data for each pair. key is lptoken identifier
        pub LastAccumulativePrice get(fn last_cumulative_price): map hasher(blake2_128_concat) T::AssetId => (FixedU128, FixedU128);
        pub Rewards get(fn reward): map hasher(blake2_128_concat) T::AssetId => (T::AssetId, T::AssetId);
        pub Reserves get(fn reserves): map hasher(blake2_128_concat) T::AssetId => (<T as balances::Trait>::Balance, <T as balances::Trait>::Balance);
        pub Pairs get(fn pair): map hasher(blake2_128_concat) (T::AssetId, T::AssetId) => Option<T::AssetId>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Token0 = <T as asset::Trait>::AssetId,
        Token1 = <T as asset::Trait>::AssetId,
        LPToken = <T as asset::Trait>::AssetId,
        AmountIn = <T as balances::Trait>::Balance,
        AmountOut = <T as balances::Trait>::Balance,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, AccountId),
        CreatePair(Token0, Token1, LPToken),
        Swap(Token0, AmountIn, Token1, AmountOut),
        MintedLiquidity(Token0, Token1, LPToken),
        BurnedLiquidity(LPToken, Token0, Token1),
        Sync(FixedU128, FixedU128),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        InSufficientBalance,
        PairExists,
        LptExists,
        InvalidPair,
        IdenticalIdentifier,
        InsufficientLiquidityMinted,
        InsufficientLiquidityBurned,
        InsufficientOutputAmount,
        InsufficientAmount,
        InsufficientLiquidity,
        K,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;


        // Mint liquidity by adding a liquidity in a pair
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn mint_liquidity(origin, token0: T::AssetId, amount0: <T as balances::Trait>::Balance, token1: T::AssetId, amount1: <T as balances::Trait>::Balance) -> dispatch::DispatchResult {
            let minimum_liquidity = <T as balances::Trait>::Balance::from(1000);
            let sender = ensure_signed(origin)?;
            ensure!(token0 != token1, Error::<T>::IdenticalIdentifier);
            // Burn assets from user to deposit to reserves
            asset::Module::<T>::transfer_to_system(&token0, &sender, &amount0)?;
            asset::Module::<T>::transfer_to_system(&token1, &sender, &amount1)?;
            match Pairs::<T>::get((token0.clone(), token1.clone())) {
                // create pair if lpt does not exist
                None => {
                    let mut lptoken_amount: <T as balances::Trait>::Balance = math::sqrt::<T>(amount0 * amount1);
                    lptoken_amount = lptoken_amount.checked_sub(&minimum_liquidity).expect("Integer overflow");
                    // Issue LPtoken
                    asset::Module::<T>::issue_from_system(Zero::zero())?;
                    let mut lptoken_id: T::AssetId = asset::NextAssetId::<T>::get();
                    lptoken_id -= One::one();
                    // Deposit assets to the reserve
                    Self::_set_reserves(&token0, &token1, &amount0, &amount1, &lptoken_id);
                    // Set pairs for swap lookup
                    Self::_set_pair(&token0, &token1, &lptoken_id);
                    // Mint LPtoken to the sender
                    asset::Module::<T>::mint_from_system(&lptoken_id, &sender, &lptoken_amount)?;
                    Self::deposit_event(RawEvent::CreatePair(token0, token1, lptoken_id));
                    Ok(())
                },
                // when lpt exists and total supply is superset of 0
                Some(lpt) if asset::Module::<T>::total_supply(lpt) > Zero::zero() => {
                    let total_supply = asset::Module::<T>::total_supply(lpt);
                    let mut reserves = Self::reserves(lpt);
                    let left = amount0.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.0).expect("Divide by zero error");
                    let right = amount1.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.1).expect("Divide by zero error");
                    let lptoken_amount = math::min::<T>(left, right);
                    // Deposit assets to the reserve
                    reserves.0 += amount0;
                    reserves.1 += amount1;
                    Self::_set_reserves(&token0, &token1, &reserves.0, &reserves.1, &lpt);
                    // Mint LPtoken to the sender
                    asset::Module::<T>::mint_from_system(&lpt, &sender, &lptoken_amount)?;
                    Self::deposit_event(RawEvent::CreatePair(token0, token1, lpt));
                    Self::_update(&lpt)?;
                    Ok(())
                },
                Some(lpt) if asset::Module::<T>::total_supply(lpt) < <T as balances::Trait>::Balance::from(0) => {
                    Err(Error::<T>::InsufficientLiquidityMinted)?
                },
                Some(_) => Err(Error::<T>::NoneValue)?,
            }
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn burn_liquidity(origin, lpt: T::AssetId, amount: <T as balances::Trait>::Balance) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut reserves = Self::reserves(lpt);
            let tokens = Self::reward(lpt);
            let total_supply = asset::Module::<T>::total_supply(lpt);

            // Calculate rewards for providing liquidity with pro-rata distribution
            let reward0 = amount.checked_mul(&reserves.0).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");
            let reward1 = amount.checked_mul(&reserves.1).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");

            // Ensure rewards exist
            ensure!(reward0 > Zero::zero() && reward1 > Zero::zero(), Error::<T>::InsufficientLiquidityBurned);

            // Distribute reward to the sender
            asset::Module::<T>::burn_from_system(&lpt, &sender, &amount)?;
            asset::Module::<T>::transfer_from_system(&tokens.0, &sender, &reward0)?;
            asset::Module::<T>::transfer_from_system(&tokens.1, &sender, &reward1)?;

            // Update reserve when the balance is set
            reserves.0 -= reward0;
            reserves.1 -= reward1;
            Self::_set_reserves(&tokens.0, &tokens.1, &reserves.0, &reserves.1, &lpt);

            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(RawEvent::BurnedLiquidity(lpt, tokens.0, tokens.1));
            // Update price
            Self::_update(&lpt)?;
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn swap(origin, from: T::AssetId, amount_in: <T as balances::Trait>::Balance, to: T::AssetId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(amount_in > Zero::zero(), Error::<T>::InsufficientAmount);
            // Find pair
            let lpt = Self::pair((from, to));
            ensure!(lpt.is_some(), Error::<T>::InvalidPair);
            let reserves = Self::reserves(lpt.unwrap());
            ensure!(reserves.0 > Zero::zero() && reserves.1 > Zero::zero(), Error::<T>::InsufficientLiquidity);
            let (mut reserve_in, mut reserve_out) = match from > to {
                true => (reserves.1, reserves.0),
                false => (reserves.0, reserves.1)
            };
            // get amount out
            let amount_out = Self::_get_amount_out(&amount_in, &reserve_in, &reserve_out);
            // transfer swapped amount
            asset::Module::<T>::transfer_from_system(&to, &sender, &amount_out)?;
            // update reserves
            reserve_in += amount_in;
            reserve_out -= amount_out;
            Self::_set_reserves(&from, &to, &reserve_in, &reserve_out, &lpt.unwrap());
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(RawEvent::Swap(from, amount_in, to, amount_out));
            // Update price
            Self::_update(&lpt.unwrap())?;
            Ok(())
        }
    }
}
// The main implementation block for the module.
impl<T: Trait> Module<T> {
    // TODO: add fee option for pair creators
    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    pub fn _mint_fee(
        reserve0: <T as balances::Trait>::Balance,
        reserve1: <T as balances::Trait>::Balance,
    ) -> bool {
        let rootK: <T as balances::Trait>::Balance = math::sqrt::<T>(
            reserve0
                .checked_mul(&reserve1)
                .expect("Multiplicaiton overflow"),
        );
        //let rootKLast: <T as balances::Trait>::Balance = math::sqrt()
        return true;
    }

    fn _update(pair: &T::AssetId) -> dispatch::DispatchResult {
        let block_timestamp = <timestamp::Module<T>>::get() % T::Moment::from(2u32.pow(32));
        let time_elapsed = block_timestamp - Self::last_block_timestamp();
        let reserves = Self::reserves(pair);
        if time_elapsed > Zero::zero() && reserves.0 != Zero::zero() && reserves.1 != Zero::zero() {
            let reserve0 = FixedU128::saturating_from_integer(reserves.0.saturated_into());
            let reserve1 = FixedU128::saturating_from_integer(reserves.1.saturated_into());
            let price0_cumulative_last = reserve1.checked_div(&reserve0).unwrap()
                * FixedU128::saturating_from_integer(time_elapsed.saturated_into());
            let price1_cumulative_last = reserve0.checked_div(&reserve1).unwrap()
                * FixedU128::saturating_from_integer(time_elapsed.saturated_into());
            <LastAccumulativePrice<T>>::insert(
                &pair,
                (&price0_cumulative_last, &price1_cumulative_last),
            );
            <LastBlockTimestamp<T>>::put(block_timestamp);
            Self::deposit_event(RawEvent::Sync(
                price0_cumulative_last,
                price1_cumulative_last,
            ));
        }
        Ok(())
    }

    fn _swap() -> dispatch::DispatchResult {
        Ok(())
    }

    fn _set_reserves(
        token0: &T::AssetId,
        token1: &T::AssetId,
        amount0: &<T as balances::Trait>::Balance,
        amount1: &<T as balances::Trait>::Balance,
        lptoken: &T::AssetId,
    ) {
        match *token0 > *token1 {
            true => {
                <Reserves<T>>::insert(lptoken, (amount1, amount0));
            }
            _ => {
                <Reserves<T>>::insert(lptoken, (amount0, amount1));
            }
        }
    }

    fn _set_pair(token0: &T::AssetId, token1: &T::AssetId, lptoken: &T::AssetId) {
        <Pairs<T>>::insert((*token0, *token1), *lptoken);
        <Pairs<T>>::insert((*token1, *token0), *lptoken);
    }

    pub fn _get_amount_out(
        amount_in: &<T as balances::Trait>::Balance,
        reserve_in: &<T as balances::Trait>::Balance,
        reserve_out: &<T as balances::Trait>::Balance,
    ) -> <T as balances::Trait>::Balance {
        let amount_inWithFee = amount_in
            .checked_mul(&T::Balance::from(997))
            .expect("Multiplication overflow");
        let numerator = amount_inWithFee
            .checked_mul(reserve_out)
            .expect("Multiplication overflow");
        let denominator = reserve_in
            .checked_mul(&T::Balance::from(1000))
            .expect("Multiplication overflow")
            .checked_add(&amount_inWithFee)
            .expect("Overflow");
        numerator / denominator
    }
}
