macro_rules! test_integer_hash {
    ($T: ty, $name: ident) => {
        #[test]
        fn $name() {
            let hash0 = <$T>::MIN.hash();
            let hash1 = <$T>::MIN.hash();
            let hash2 = <$T>::MAX.hash();
            let hash3 = (<$T>::MAX / 2 + 1).hash();
            assert_eq!(hash0, hash1);
            assert_ne!(hash0, hash2);
            assert_ne!(hash0, hash3);
            assert_ne!(hash2, hash3);
        }
    };
}

pub(crate) use test_integer_hash;
