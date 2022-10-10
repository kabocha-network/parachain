// Kabocha Blockchain – https://decentration.org
// Copyright (C) 2020-2022 Decentration

// The Kabocha Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Kabocha Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at hello@kabocha.network
use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use parachain_staking::InflationInfo;
use sp_runtime::{Perbill, Percent, Perquintill};
use polkadot_runtime_common::{prod_or_fast};
use crate::{Balance, BlockNumber};

//ToDo: Put constants in its own file

pub const MINICENTS: Balance = 10_000_000;
pub const MILLICENTS: Balance = 10_000_000_000;
pub const BILLICENTS: Balance = 1_000_000_000;
pub const MICROCENTS: Balance = 100_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
pub const DOLLARS: Balance = 100 * CENTS;
pub const GRAND: Balance = CENTS * 100_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const BLOCKS_PER_YEAR: BlockNumber = DAYS * 36525 / 100;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

pub const INFLATION_CONFIG: (Perquintill, Perquintill, Perquintill, Perquintill) = (
	// max collator staking rate
	Perquintill::from_percent(40),
	// collator reward rate
	Perquintill::from_percent(10),
	// max delegator staking rate
	Perquintill::from_percent(10),
	// delegator reward rate
	Perquintill::from_percent(8),
);

/// Inflation configuration which is used at genesis
pub fn kabocha_inflation_config() -> InflationInfo {
	InflationInfo::new(
		.into(),
		// max collator staking rate
		Perquintill::from_percent(40),
		// collator reward rate
		Perquintill::from_percent(10),
		// max delegator staking rate
		Perquintill::from_percent(10),
		// delegator reward rate
		Perquintill::from_percent(8),
	)
}



pub mod staking {
	use super::*;
    /// Minimum round length is 300 blocks, which is meant to be 1 hour (300 * 12 second block times)
    pub const MIN_BLOCKS_PER_ROUND: BlockNumber = prod_or_fast!(HOURS);
	pub const DEFAULT_BLOCKS_PER_ROUND: BlockNumber = prod_or_fast!(2 * HOURS);
	pub const STAKE_DURATION: BlockNumber = prod_or_fast!(7 * DAYS);
    pub const MIN_COLLATORS: u32 = prod_or_fast!(16);
	pub const MAX_CANDIDATES: u32 = prod_or_fast!(75);
    pub const MAX_DELEGATORS_PER_COLLATOR: u32 = 35;
	pub const MIN_DELEGATOR_STAKE: Balance = 20 * KAB;

	pub const NETWORK_REWARD_RATE: Perquintill = Perquintill::from_percent(10);

	parameter_types! {
		/// Minimum round length is 1 hour
		pub const MinBlocksPerRound: BlockNumber = MIN_BLOCKS_PER_ROUND;
		/// Default length of a round/session is 2 hours
		pub const DefaultBlocksPerRound: BlockNumber = DEFAULT_BLOCKS_PER_ROUND;
		/// Unstaked balance can be unlocked after 7 days
		pub const StakeDuration: BlockNumber = STAKE_DURATION;
		/// Collator exit requests are delayed by 4 hours (2 rounds/sessions)
		pub const ExitQueueDelay: u32 = 2;
		/// Minimum 16 collators selected per round, default at genesis and minimum forever after
		pub const MinCollators: u32 = MIN_COLLATORS;
		/// At least 4 candidates which cannot leave the network if there are no other candidates.
		pub const MinRequiredCollators: u32 = 4;
		/// We only allow one delegation per round.
		pub const MaxDelegationsPerRound: u32 = 1;
		/// Maximum 25 delegators per collator at launch, might be increased later
		#[derive(Debug, Eq, PartialEq)]
		pub const MaxDelegatorsPerCollator: u32 = MAX_DELEGATORS_PER_COLLATOR;
		/// Minimum stake required to be reserved to be a collator is 10_000
		pub const MinCollatorStake: Balance = 10_000 * KAB;
		/// Minimum stake required to be reserved to be a delegator is 1000
		pub const MinDelegatorStake: Balance = MIN_DELEGATOR_STAKE;
		/// Maximum number of collator candidates
		#[derive(Debug, Eq, PartialEq)]
		pub const MaxCollatorCandidates: u32 = MAX_CANDIDATES;
		/// Maximum number of concurrent requests to unlock unstaked balance
		pub const MaxUnstakeRequests: u32 = 10;
		/// The starting block number for the network rewards
		pub const NetworkRewardStart: BlockNumber = super::treasury::INITIAL_PERIOD_LENGTH;
		/// The rate in percent for the network rewards
		pub const NetworkRewardRate: Perquintill = NETWORK_REWARD_RATE;
	}
}

pub mod governance {
	use super::*;
	parameter_types! {
		
		pub LaunchPeriod: BlockNumber = prod_or_fast!(2 * DAYS, 1, "KAB_LAUNCH_PERIOD");
		pub VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "KAB_VOTING_PERIOD");
		pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(2 * HOURS, 1 * MINUTES, "KAB_FAST_TRACK_VOTING_PERIOD");
		pub const MinimumDeposit: Balance = 1 * CENTS;
		pub EnactmentPeriod: BlockNumber = prod_or_fast!(2 * DAYS, 1, "KAB_ENACTMENT_PERIOD");
		pub CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "KAB_COOLOFF_PERIOD");
		pub const InstantAllowed: bool = true;
		pub const MaxVotes: u32 = 100;
		pub const MaxProposals: u32 = 100;
		pub const PreimageByteDeposit: Balance = 1 * BILLICENTS;
	}
}

pub mod proxy {
	use super::*;
	parameter_types! {
		// One storage item; key size 32, value size 8; .
		pub const ProxyDepositBase: Balance = deposit(1, 8);
		// Additional storage item size of 33 bytes.
		pub const ProxyDepositFactor: Balance = deposit(0, 33);
		pub const MaxProxies: u16 = 32;
		pub const AnnouncementDepositBase: Balance = deposit(1, 8);
		pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
		pub const MaxPending: u16 = 32;
	}
}

pub mod treasury {
	use super::*;
	parameter_types! {
		pub const ProposalBond: Permill = Permill::from_percent(5);
		pub const ProposalBondMinimum: Balance = 20 * CENTS;
		pub const ProposalBondMaximum: Balance = 1 * GRAND;
		pub const SpendPeriod: BlockNumber = 6 * DAYS;
		pub const Burn: Permill = Permill::from_perthousand(0);
		pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	
		pub const TipCountdown: BlockNumber = 1 * DAYS;
		pub const TipFindersFee: Percent = Percent::from_percent(20);
		pub const TipReportDepositBase: Balance = 1 * CENTS;
		pub const DataDepositPerByte: Balance = 10 * MILLICENTS;
		pub const MaxApprovals: u32 = 100;
		//pub const MaxAuthorities: u32 = 100_000;
		pub const MaxKeys: u32 = 10_000;
		pub const MaxPeerInHeartbeats: u32 = 10_000;
		pub const MaxPeerDataEncodingSize: u32 = 1_000;
	}
}