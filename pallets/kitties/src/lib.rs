#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		price: u32,
		owner: T::AccountId,
		gender: Gender,
	}

	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn balance)]
	pub type Balance<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Vec<Vec<u8>>>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_info)]
	pub(super) type KittyInfo<T: Config> = StorageMap<_, Blake2_128, Vec<u8>, Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn total_kitty)]
	pub type TotalKitty<T> = StorageValue<_, u32, ValueQuery, ConstU32<0>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Create(T::AccountId, Vec<u8>),
		Transfer(T::AccountId, T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let gender = Self::gen_gender(&dna)?;

			let mut _current_account_kitty = <Balance<T>>::get(&who).unwrap_or(Vec::new());
			_current_account_kitty.push(dna.clone());
			<Balance<T>>::insert(&who, _current_account_kitty);

			let _current_total_kitty = <TotalKitty<T>>::get();
			<TotalKitty<T>>::put(_current_total_kitty + 1);

			let kitty = Kitty::<T> { dna: dna.clone(), price, owner: who.clone(), gender };
			<KittyInfo<T>>::insert(dna.clone(), kitty);

			// Emit an event.
			Self::deposit_event(Event::Create(who, dna));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, dna: Vec<u8>, to: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut _current_from_kitty = <Balance<T>>::get(&who).unwrap_or(Vec::new());

			let mut is_exist = false;
			for (_idx, _dna) in _current_from_kitty.iter().enumerate() {
				if Self::check_equal_vec(&dna, &_dna)? {
					is_exist = true;

					_current_from_kitty.remove(_idx);
					<Balance<T>>::insert(&who, _current_from_kitty);

					let mut _current_to_kitty = <Balance<T>>::get(&to).unwrap_or(Vec::new());
					_current_to_kitty.push(dna.clone());
					<Balance<T>>::insert(&to, _current_to_kitty);

					let mut kitty = <KittyInfo<T>>::get(&dna).unwrap();
					kitty.owner = to.clone();
					<KittyInfo<T>>::insert(dna.clone(), kitty);
					
					break;
				}
			}

			// Emit an event.
			if is_exist {
				Self::deposit_event(Event::Transfer(who, to, dna));
				Ok(())
			} else {
				Err(Error::<T>::NoneValue)?
			}
			// Return a successful DispatchResultWithPostInfo
		}
	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}

	fn check_equal_vec(arr1: &Vec<u8>, arr2: &Vec<u8>) -> Result<bool, Error<T>> {
		Ok(arr1.iter().eq(arr2.iter()))
	}
}