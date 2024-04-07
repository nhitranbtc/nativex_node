#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	//log::{error, trace},
	traits::Randomness,
};
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use parity_scale_codec::Encode;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::DispatchError;

/// Contract extension for `FetchRandom`
#[derive(Default)]
pub struct FetchRandomExtension;

impl<T> ChainExtension<T> for FetchRandomExtension
where
	T: pallet_contracts::Config + pallet_insecure_randomness_collective_flip::Config,

	<T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
{
	fn call<E: Ext>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		E: Ext<T = T>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		let func_id = env.func_id();
		match func_id {
			1101 => {
				let mut env = env.buf_in_buf_out();
				let arg: [u8; 32] = env.read_as()?;
				let random_seed =
					pallet_insecure_randomness_collective_flip::Pallet::<T>::random(&arg);

				let random_slice = random_seed.0.encode();
				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			},

			_ => {
				log::error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			},
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
