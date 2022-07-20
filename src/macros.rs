/// Mock an zome_info call
#[macro_export]
macro_rules! mock_zome_info {
    (
        $mock_hdk:ident,
        $mocked_zome_info:expr
    ) => {{
        $mock_hdk
            .expect_zome_info()
            .return_const(Ok($mocked_zome_info));
    }};
}

/// Mock an EntryHash for some entry data, and return it
#[macro_export]
macro_rules! mock_hash_entry {
    (
        $mock_hdk:ident,
        $entry:expr
    ) => {{
        let entry_hash = fixt!(EntryHash);
        let entry_hash_2 = entry_hash.clone();
        let hash_input = HashInput::Entry(Entry::try_from($entry).unwrap());
        // println!("expected {:?}", hash_input);
        $mock_hdk
            .expect_hash()
            .with(mockall::predicate::eq(hash_input))
            .returning(move |_hash_input| {
                // println!("received {:?}", _hash_input);
                // panic!("exit");
                Ok(HashOutput::Entry(entry_hash_2.clone()))
            });

        entry_hash
    }};
}

/// Mock root_hash
#[macro_export]
macro_rules! mock_root_hash {
    (
        $mock_hdk:ident
    ) => {{
        let root = Entry::App(
            AppEntryBytes::try_from(SerializedBytes::from(UnsafeBytes::from(
                hdk::hash_path::path::ROOT.to_vec(),
            )))
            .expect("This cannot fail as it's under the max entry bytes"),
        );
        mock_hash_entry!($mock_hdk, root)
    }};
}

/// Mock a read request to the DHT, providing both input and output values
#[macro_export]
macro_rules! mock_get {
    (
        $mock_hdk:ident,
        $input_val:expr,
        $output_val:expr
    ) => {
        $mock_hdk
            .expect_get()
            .with(mockall::predicate::eq(vec![GetInput::new(
                AnyDhtHash::from($input_val),
                GetOptions::content(),
            )]))
            .returning(move |_output| {
                let signed_action = SignedAction(fixt!(Action), fixt!(Signature));
                let hashed: HoloHashed<SignedAction> = HoloHashed::from_content_sync(signed_action);
                let HoloHashed {
                    content: SignedAction(action, signature),
                    hash,
                } = hashed.clone();

                Ok(vec![Some(Record::new(
                    SignedActionHashed {
                        hashed: ActionHashed::with_pre_hashed(action, hash),
                        signature,
                    },
                    Some(Entry::try_from($output_val.clone()).unwrap()),
                ))])
            });
    };
}

/// Mock a Path.path_entry_hash() call, and return the mocked EntryHash
#[macro_export]
macro_rules! mock_typed_path_path_entry_hash {
    (
        $mock_hdk:ident,
        $typed_path:expr
    ) => {{
        let path = $typed_path.path;
        let app_entry_bytes = AppEntryBytes(SerializedBytes::try_from(path).unwrap());
        let entry = Entry::App(app_entry_bytes);
        mock_hash_entry!($mock_hdk, entry)
    }};
}

/// Mock a Path.exists() call, and return the mocked EntryHash
/// This is assuming the 'is root' logical branch
#[macro_export]
macro_rules! mock_typed_path__exists__is_root_true {
    (
        // $mock_hdk:ident,
        $mock_hdk:ident,
        $typed_path:expr
    ) => {{
        // path_entry_hash()
        let path_entry_hash = mock_typed_path_path_entry_hash!($mock_hdk, $typed_path);
        // root_hash()
        let root = mock_root_hash!($mock_hdk);
        // get_links()
        let get_links_only_input = GetLinksInput::new(
            root.into(),
            LinkTypeFilter::single_type(
                $typed_path.link_type.zome_id,
                $typed_path.link_type.zome_type,
            ),
            Some($typed_path.make_tag().unwrap()),
        );
        let get_links_input: Vec<GetLinksInput> = vec![get_links_only_input];
        let link = Link {
            target: path_entry_hash.into(),
            timestamp: fixt!(Timestamp),
            tag: fixt!(LinkTag),
            create_link_hash: fixt!(ActionHash),
        };
        let get_links_output: Vec<Vec<Link>> = vec![vec![link]];
        let mut_mock_hdk = &mut $mock_hdk;
        crate::mock_hdk::mock_get_links(mut_mock_hdk, get_links_input, Ok(get_links_output));
    }};
}

/// Mock a Path.typed() call
#[macro_export]
macro_rules! mock_path_typed {
    (
        $mock_hdk:ident
    ) => {{
        let mut mock_info = fixt!(ZomeInfo);
        // this assumes one link type
        let e: Vec<(ZomeId, Vec<LinkType>)> = vec![(mock_info.id, vec![LinkType(0)])];
        mock_info.zome_types.links = ScopedZomeTypes(e);
        mock_zome_info!($mock_hdk, mock_info);
    }};
}

#[cfg(test)]
mod macros {
    use crate::mock_typed_path__exists__is_root_true;
    use ::fixt::fixt;
    use hdk::prelude::hdi::prelude::*;
    use hdk::prelude::*;

    #[hdk_link_types]
    pub enum LinkTypes {
        Path,
    }

    #[test]
    fn path_exists() {
        let mut mock_hdk = MockHdkT::new();

        mock_path_typed!(mock_hdk);
        set_hdk(mock_hdk);
        // vec of 1, since there's no . separators
        let typed_path = Path::from("test".to_string())
            // this invokes an HDI call to zome_info
            // during a TryFrom conversion
            .typed(LinkTypes::Path)
            .unwrap();

        // by using mock_typed_path__exists__is_root_true
        // I can get typed_path.exists() to produce Ok(true)
        mock_hdk = MockHdkT::new();
        mock_typed_path__exists__is_root_true!(mock_hdk, typed_path.clone());
        set_hdk(mock_hdk);
        assert!(typed_path.exists().unwrap());
    }
}
