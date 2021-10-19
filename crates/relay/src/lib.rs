//! # Relay Pallet
//! Based on the [specification](https://spec.interlay.io/spec/relay.html).

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

mod ext;
pub mod types;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate mocktopus;

pub use security;

use bitcoin::{ types::*};

use frame_support::{ transactional, weights::Pays};
use frame_system::ensure_signed;


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// ## Configuration
    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config + security::Config + btc_relay::Config

    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    // #[pallet::metadata(DefaultVaultId<T> = "VaultId")]
    pub enum Event<T: Config> {
        DummyEvent
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Vault BTC address not in transaction input
        VaultNoInputToTransaction,
        /// Valid redeem transaction
        ValidRedeemTransaction,
        /// Valid replace transaction
        ValidReplaceTransaction,
        /// Valid merge transaction
        ValidMergeTransaction,
        /// Failed to parse transaction
        InvalidTransaction,
        /// Unable to convert value
        TryIntoIntError,
        /// Expected two unique transactions
        DuplicateTransaction,
        /// Expected duplicate OP_RETURN ids
        ExpectedDuplicate,
    }


    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// One time function to initialize the BTC-Relay with the first block
        ///
        /// # Arguments
        ///
        /// * `block_header_bytes` - 80 byte raw Bitcoin block header.
        /// * `block_height` - starting Bitcoin block height of the submitted block header.
        ///
        /// # <weight>
        /// - Storage Reads:
        /// 	- One storage read to check that parachain is not shutdown. O(1)
        /// 	- One storage read to check if relayer authorization is disabled. O(1)
        /// 	- One storage read to check if relayer is authorized. O(1)
        /// - Storage Writes:
        ///     - One storage write to store block hash. O(1)
        ///     - One storage write to store block header. O(1)
        /// 	- One storage write to initialize main chain. O(1)
        ///     - One storage write to store best block hash. O(1)
        ///     - One storage write to store best block height. O(1)
        /// - Events:
        /// 	- One event for initialization.
        ///
        /// Total Complexity: O(1)
        /// # </weight>
        #[pallet::weight(<T as Config>::WeightInfo::initialize())]
        #[transactional]
        pub fn initialize(
            origin: OriginFor<T>,
            raw_block_header: RawBlockHeader,
            block_height: u32,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let relayer = ensure_signed(origin)?;

            let block_header = ext::btc_relay::parse_raw_block_header::<T>(&raw_block_header)?;
            ext::btc_relay::initialize::<T>(relayer, block_header, block_height)?;

            // don't take tx fees on success
            Ok(Pays::No.into())
        }

        /// Stores a single new block header
        ///
        /// # Arguments
        ///
        /// * `raw_block_header` - 80 byte raw Bitcoin block header.
        ///
        /// # <weight>
        /// Key: C (len of chains), P (len of positions)
        /// - Storage Reads:
        /// 	- One storage read to check that parachain is not shutdown. O(1)
        /// 	- One storage read to check if relayer authorization is disabled. O(1)
        /// 	- One storage read to check if relayer is authorized. O(1)
        /// 	- One storage read to check if block header is stored. O(1)
        /// 	- One storage read to retrieve parent block hash. O(1)
        /// 	- One storage read to check if difficulty check is disabled. O(1)
        /// 	- One storage read to retrieve last re-target. O(1)
        /// 	- One storage read to retrieve all Chains. O(C)
        /// - Storage Writes:
        ///     - One storage write to store block hash. O(1)
        ///     - One storage write to store block header. O(1)
        /// 	- One storage mutate to extend main chain. O(1)
        ///     - One storage write to store best block hash. O(1)
        ///     - One storage write to store best block height. O(1)
        /// - Notable Computation:
        /// 	- O(P) sort to reorg chains.
        /// - Events:
        /// 	- One event for block stored (fork or extension).
        ///
        /// Total Complexity: O(C + P)
        /// # </weight>
        #[pallet::weight(<T as Config>::WeightInfo::store_block_header())]
        #[transactional]
        pub fn store_block_header(
            origin: OriginFor<T>,
            raw_block_header: RawBlockHeader,
        ) -> DispatchResultWithPostInfo {
            ext::security::ensure_parachain_status_not_shutdown::<T>()?;
            let relayer = ensure_signed(origin)?;

            let block_header = ext::btc_relay::parse_raw_block_header::<T>(&raw_block_header)?;
            ext::btc_relay::store_block_header::<T>(&relayer, block_header)?;

            // don't take tx fees on success
            Ok(Pays::No.into())
        }

    }
}
