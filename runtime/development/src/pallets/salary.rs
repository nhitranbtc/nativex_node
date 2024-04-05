/// Make peridic payment to members of a ranked collective according to rank.
use crate::*;

parameter_types! {
	pub const Budget: Balance = 10_000 * NATIVEX;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}

pub struct SalaryForRank;
impl GetSalary<u16, AccountId, Balance> for SalaryForRank {
	fn get_salary(a: u16, _who: &AccountId) -> Balance {
		Balance::from(a) * 1000 * NATIVEX
	}
}

impl pallet_salary::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
	type Members = RankedCollective;
	type Salary = SalaryForRank;
	type RegistrationPeriod = ConstU32<200>;
	type PayoutPeriod = ConstU32<200>;
	type Budget = Budget;
}
