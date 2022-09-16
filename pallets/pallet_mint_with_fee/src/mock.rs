use crate as pallet_mint_with_fee;
use frame_support::{parameter_types, traits::Everything};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		MintWithFee: pallet_mint_with_fee::{Pallet, Call, Storage, Event<T>},

		Balances: pallet_balances
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

pub type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: Balance = 1_000;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxMetadataSize: u32 = 100;
}

impl pallet_mint_with_fee::Config for Test {
	type Currency = Balances;
	type Event = Event;
	type WeightInfo = pallet_mint_with_fee::weights::SubstrateWeight<Test>;
	type MaxMetadataSize = MaxMetadataSize;
}


/// Mock users AccountId
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;

pub struct ExtBuilder {
	caps_endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder {
			caps_endowed_accounts: vec![(ALICE, 1_000_000), (BOB, 100_000), (CHARLIE, 100_000)],
		}
	}
}

impl ExtBuilder {
	pub fn balances(mut self, accounts: Vec<(u64, u64)>) -> Self {
		for account in accounts {
			self.caps_endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: self.caps_endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_mint_with_fee::GenesisConfig::<Test> {
			fee_percent: (10u32).into(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
