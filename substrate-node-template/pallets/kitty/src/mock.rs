use crate as pallet_kitty;
use frame_support::parameter_types;
use frame_support::traits::ConstU32;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	create_runtime_str,
	generic,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount, AccountIdLookup},
	MultiSignature,
};
pub use frame_support::weights::constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND};
use frame_support::Twox64Concat;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub use pallet_balances;
pub use pallet_randomness_collective_flip;
pub use pallet_timestamp;

// /// An index to a block.
// pub type BlockNumber = u32;

// /// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
// pub type Signature = MultiSignature;

// /// Some way of identifying an account on the chain. We intentionally make it equivalent
// /// to the public key of our transaction signing scheme.
// pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// /// Balance of an account.
// pub type Balance = u128;

// /// Index of a transaction in the chain.
// pub type Index = u32;

// /// A hash of some data used by the chain.
// pub type Hash = sp_core::H256;

// use sp_version::NativeVersion;
// use sp_version::RuntimeVersion;
pub use sp_runtime::{Perbill, Permill};

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		KittyModule: pallet_kitty::{Pallet, Call, Storage, Event<T>},
		CollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
	}
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
	//质押的token数量
	pub const MaxKittyReverse: u128 = 3;

	pub const MinimumPeriod : u64 = 5;

	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

	//kitty最大拥有数量
	pub const MaxKittyOwned : u32 = 3;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}


impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

impl pallet_kitty::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type KittyRandomness = CollectiveFlip;
	type MaxKittyOwned = MaxKittyOwned;
	type KittyIndex = Twox64Concat;
	type MaxKittyReverse = MaxKittyReverse;
	type UnixTime = Timestamp;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
