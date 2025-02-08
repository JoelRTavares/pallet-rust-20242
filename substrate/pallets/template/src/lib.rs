#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}



	#[pallet::storage]
	pub(super) type NameOf<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<u8, T::MaxLength>>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LogViagem {
			who: T::AccountId,
			relato: BoundedVec<u8, T::MaxLength>
		}

	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		TooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		


		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn set_viagem(origin: OriginFor<T>, pais: Vec<u8>, descricao: Vec<u8>, estado: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			let mut fullrelato = pais.clone();
			fullrelato.extend_from_slice(b";");  

    		fullrelato.extend_from_slice(&descricao);
			fullrelato.extend_from_slice(b";"); 

    		fullrelato.extend_from_slice(&estado);

			let relato: BoundedVec<_, _> =
				fullrelato.try_into().map_err(|_| Error::<T>::TooLong)?;
			Self::deposit_event(Event::LogViagem { who, relato });
			Ok(())
		}


		
	}
}
