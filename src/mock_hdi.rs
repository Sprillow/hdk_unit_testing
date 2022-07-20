// use hdk::map_extern::ExternResult;
// use hdk::prelude::*;
// use holochain_mock_hdi::*;

// pub fn mock_must_get_action(
//     mock_hdi: &mut MockHdiT,
//     expected_input: &MustGetActionInput,
//     expected_output: ExternResult<SignedActionHashed>,
// ) -> () {
//     mock_hdi
//         .expect_must_get_action()
//         .with(mockall::predicate::function(|real_input| {
//             real_input == expected_input
//         }))
//         .times(1)
//         .return_const(expected_output);
// }
