use crate as games_management;
use frame_support::{parameter_types, traits::GenesisBuild};
use frame_system as system;
use pallet_assets as assets;
use pallet_balances as balances;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureSigned;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;
type Balance = u128;
type AssetId = u32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Assets: assets::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		GamesManagement: games_management::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const Deposit: u32 = 0;
	pub const StringLimit: u32 = 32;
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureSigned<AccountId>;
	type AssetDeposit = Deposit;
	type MetadataDepositBase = Deposit;
	type MetadataDepositPerByte = Deposit;
	type ApprovalDeposit = Deposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();

}

parameter_types! {
	pub const MetadataLimit: u32 = 256;
	pub const MinGamesAmount: u32 = 1;
}

impl games_management::Config for Test {
	type Event = Event;
	type Assets = Assets;
	type MetadataLimit = MetadataLimit;
	type MinGamesAmount = MinGamesAmount;
}

pub const ALICE: AccountId = 1u64;
pub const BOB: AccountId = 2u64;
pub const CHARLIE: AccountId = 3u64;

pub struct ExtBuilder;
impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();

        balances::GenesisConfig::<Test> {
            balances: vec![
                (ALICE, 20_000_000),
                (BOB, 15_000),
                (CHARLIE, 150_000),
            ],
        }
        .assimilate_storage(&mut storage)
        .unwrap();
    

        let config: pallet_assets::GenesisConfig<Test> = pallet_assets::GenesisConfig {
            assets: vec![],
            metadata: vec![],
            accounts: vec![],
        };
    
        config.assimilate_storage(&mut storage).unwrap();
    
        let mut ext = sp_io::TestExternalities::new(storage);
            ext.execute_with(|| System::set_block_number(1));
            ext
    }
}
