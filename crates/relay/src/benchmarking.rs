use super::*;
use bitcoin::{
    formatter::{ TryFormattable},
    types::{
        BlockBuilder,RawBlockHeader,
    },
};
use btc_relay::{BtcAddress, Pallet as BtcRelay};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::{H160, U256};
use sp_std::prelude::*;

#[cfg(test)]
use crate::Pallet as Relay;


benchmarks! {

    initialize {
        let height = 0u32;
        let origin: T::AccountId = account("Origin", 0, 0);
        let stake = 100u32;

        let address = BtcAddress::P2PKH(H160::from([0; 20]));
        let block = BlockBuilder::new()
            .with_version(4)
            .with_coinbase(&address, 50, 3)
            .with_timestamp(1588813835)
            .mine(U256::from(2).pow(254.into())).unwrap();
        let block_header = RawBlockHeader::from_bytes(&block.header.try_format().unwrap()).unwrap();
    }: _(RawOrigin::Signed(origin), block_header, height)

    store_block_header {
        let origin: T::AccountId = account("Origin", 0, 0);

        let address = BtcAddress::P2PKH(H160::from([0; 20]));
        let height = 0;
        let stake = 100u32;

        let init_block = BlockBuilder::new()
            .with_version(4)
            .with_coinbase(&address, 50, 3)
            .with_timestamp(1588813835)
            .mine(U256::from(2).pow(254.into())).unwrap();

        let init_block_hash = init_block.header.hash;
        let raw_block_header = RawBlockHeader::from_bytes(&init_block.header.try_format().unwrap())
            .expect("could not serialize block header");
        let block_header = BtcRelay::<T>::parse_raw_block_header(&raw_block_header).unwrap();

        BtcRelay::<T>::initialize(origin.clone(), block_header, height).unwrap();

        let block = BlockBuilder::new()
            .with_previous_hash(init_block_hash)
            .with_version(4)
            .with_coinbase(&address, 50, 3)
            .with_timestamp(1588814835)
            .mine(U256::from(2).pow(254.into())).unwrap();

        let raw_block_header = RawBlockHeader::from_bytes(&block.header.try_format().unwrap())
            .expect("could not serialize block header");

    }: _(RawOrigin::Signed(origin), raw_block_header)
}

impl_benchmark_test_suite!(Relay, crate::mock::ExtBuilder::build_with(|_| {}), crate::mock::Test);
