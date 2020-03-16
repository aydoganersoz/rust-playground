mod library {
    pub mod backend {
        pub fn rent_book_admin() {}

        fn withdraw_book_admin() {}
    }

    mod frontend {
        pub fn rent_book() {}

        fn withdraw_book() {}
    }
}

pub fn rent_book() {
    // Absolute path usage
    crate::library::backend::rent_book_admin();

    // Relative path usage
    library::backend::rent_book_admin();
}
