use crate::{mock::*, Error, PendingWithdrawals};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::{traits::AccountIdConversion, SaturatedConversion};
use thea_primitives::types::Withdraw;

#[test]
fn test_whitelist_token_returns_ok() {
	new_test_ext().execute_with(|| {
		let token = 100;
		assert_ok!(XcmHelper::whitelist_token(RuntimeOrigin::root(), token));
	});
}

#[test]
fn test_whitelist_token_returns_token_is_already_whitelisted() {
	new_test_ext().execute_with(|| {
		let token = 100;
		assert_ok!(XcmHelper::whitelist_token(RuntimeOrigin::root(), token));
		assert_noop!(
			XcmHelper::whitelist_token(RuntimeOrigin::root(), token),
			Error::<Test>::TokenIsAlreadyWhitelisted
		);
	});
}

#[test]
fn test_transfer_fee_returns_ok() {
	new_test_ext().execute_with(|| {
		let recipient = 1;
		let pallet_account = AssetHandlerPalletId::get().into_account_truncating();
		Balances::deposit_creating(
			&pallet_account,
			5_000_000_000_000_000_000_000u128.saturated_into(),
		);
		assert_ok!(XcmHelper::transfer_fee(RuntimeOrigin::root(), recipient));
		assert_eq!(Balances::free_balance(recipient), 4999999999000000000000u128.saturated_into());
	});
}

#[test]
fn test_block_by_ele() {
	new_test_ext().execute_with(|| {
		let first_withdrawal = Withdraw {
			asset_id: 1,
			amount: 1,
			destination: vec![],
			is_blocked: false,
			extra: vec![],
		};
		let sec_withdrawal = Withdraw {
			asset_id: 2,
			amount: 2,
			destination: vec![],
			is_blocked: false,
			extra: vec![],
		};
		<PendingWithdrawals<Test>>::insert(1, vec![first_withdrawal, sec_withdrawal]);
		assert_ok!(XcmHelper::block_by_ele(1, 1));
		let actual_withdrawals = <PendingWithdrawals<Test>>::get(1);
		let expected_withdraw = Withdraw {
			asset_id: 2,
			amount: 2,
			destination: vec![],
			is_blocked: true,
			extra: vec![],
		};
		assert_eq!(actual_withdrawals[1], expected_withdraw);
		assert_noop!(XcmHelper::block_by_ele(1, 4), Error::<Test>::IndexNotFound);
	});
}
