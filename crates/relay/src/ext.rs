#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg_attr(test, mockable)]
pub(crate) mod security {
    use frame_support::dispatch::DispatchResult;
    use security::types::ErrorCode;
    use sp_std::collections::btree_set::BTreeSet;

    #[allow(dead_code)]
    pub(crate) fn get_errors<T: crate::Config>() -> BTreeSet<ErrorCode> {
        <security::Pallet<T>>::get_errors()
    }

    pub fn ensure_parachain_status_not_shutdown<T: crate::Config>() -> DispatchResult {
        <security::Pallet<T>>::ensure_parachain_status_not_shutdown()
    }
}

#[cfg_attr(test, mockable)]
pub(crate) mod btc_relay {
    use bitcoin::types::{BlockHeader, RawBlockHeader};
    use frame_support::dispatch::DispatchResult;
    use sp_runtime::DispatchError;

    pub fn initialize<T: crate::Config>(
        relayer: T::AccountId,
        block_header: BlockHeader,
        block_height: u32,
    ) -> DispatchResult {
        <btc_relay::Pallet<T>>::initialize(relayer, block_header, block_height)
    }

    pub fn store_block_header<T: crate::Config>(relayer: &T::AccountId, block_header: BlockHeader) -> DispatchResult {
        <btc_relay::Pallet<T>>::store_block_header(relayer, block_header)
    }


    pub fn parse_raw_block_header<T: btc_relay::Config>(
        raw_block_header: &RawBlockHeader,
    ) -> Result<BlockHeader, DispatchError> {
        <btc_relay::Pallet<T>>::parse_raw_block_header(raw_block_header)
    }

}