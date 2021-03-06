use hdk::prelude::*;

pub fn mock_create(
    mock_hdk: &mut MockHdkT,
    expected_input: CreateInput,
    expected_output: ExternResult<ActionHash>,
) {
    mock_hdk
        .expect_create()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

// pub fn mock_hash(
//     mock_hdk: &mut MockHdkT,
//     expected_input: HashInput,
//     expected_output: ExternResult<HashOutput>,
// ) {
//     mock_hdk
//         .expect_hash()
//         .with(mockall::predicate::eq(expected_input))
//         .times(1)
//         .return_const(expected_output);
// }

pub fn mock_create_link(
    mock_hdk: &mut MockHdkT,
    expected_input: CreateLinkInput,
    expected_output: ExternResult<ActionHash>,
) {
    mock_hdk
        .expect_create_link()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

pub fn mock_agent_info(mock_hdk: &mut MockHdkT, expected_output: ExternResult<AgentInfo>) {
    mock_hdk
        .expect_agent_info()
        .times(1)
        .return_const(expected_output);
}

pub fn mock_remote_signal(
    mock_hdk: &mut MockHdkT,
    expected_input: RemoteSignal,
    expected_output: ExternResult<()>,
) {
    mock_hdk
        .expect_remote_signal()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

pub fn mock_update(
    mock_hdk: &mut MockHdkT,
    expected_input: UpdateInput,
    expected_output: ExternResult<ActionHash>,
) {
    mock_hdk
        .expect_update()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

pub fn mock_get_links(
    mock_hdk: &mut MockHdkT,
    expected_input: Vec<GetLinksInput>,
    expected_output: ExternResult<Vec<Vec<Link>>>,
) {
    // println!("expected get_links_input {:?}", expected_input);
    mock_hdk
        .expect_get_links()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
    // .returning(|input| {
    //     println!("actual get_links_input {:?}", input);
    //     Ok(vec![vec![]])
    // });
}

pub fn mock_sys_time(mock_hdk: &mut MockHdkT, expected_output: ExternResult<Timestamp>) {
    mock_hdk
        .expect_sys_time()
        .times(1)
        .return_const(expected_output);
}

pub fn mock_must_get_action(
    mock_hdk: &mut MockHdkT,
    expected_input: MustGetActionInput,
    expected_output: ExternResult<SignedActionHashed>,
) {
    mock_hdk
        .expect_must_get_action()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

pub fn mock_must_get_entry(
    mock_hdk: &mut MockHdkT,
    expected_input: MustGetEntryInput,
    expected_output: ExternResult<EntryHashed>,
) {
    mock_hdk
        .expect_must_get_entry()
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}

pub fn mock_zome_info(mock_hdk: &mut MockHdkT, expected_output: ExternResult<ZomeInfo>) {
    mock_hdk
        .expect_zome_info()
        .times(1)
        .return_const(expected_output);
}

pub fn mock_get(
    mock_hdk: &mut MockHdkT,
    expected_input: Vec<GetInput>,
    expected_output: ExternResult<Vec<Option<Record>>>,
) {
    mock_hdk
        .expect_get() // called from `Path::from(MEMBER_PATH).ensure()?;`
        .with(mockall::predicate::eq(expected_input))
        .times(1)
        .return_const(expected_output);
}
