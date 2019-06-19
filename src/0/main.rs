fn main() {
    crate::mod_1::mod_2::mod_3::func();
    self::mod_1::mod_2::mod_3::func();
    mod_1::mod_2::mod_3::func();

    // `use`で複数回インポートすると名前重複エラーになる
    // error[E0252]: the name `func` is defined multiple times

    /*
    // crate::にて絶対パス指定
    use crate::mod_1::mod_2::mod_3::func;
    func();

    use crate::mod_1::mod_2;
    mod_2::mod_3::func();
    */

    /*
    // 先頭省略にて本モジュールからの相対パス指定
    use mod_1::mod_2::mod_3::func;
    func();

    use mod_1::mod_2;
    mod_2::mod_3::func();
    */

    // self::にて本モジュールからの相対パス指定
    use self::mod_1::mod_2::mod_3::func;
    func();

    use self::mod_1::mod_2;
    mod_2::mod_3::func();
}
pub mod mod_1 {
    pub mod mod_2 {
        pub mod mod_3 {
            pub fn func() {}
            mod mod_4 {
                fn inner_func() {
                    super::func();
                    use super::func;
                    func();
                }
            }
        }
    }
}
/*
// error[E0603]: module `mod_2` is private
mod mod_1 {
    mod mod_2 {
        mod mod_3 {
            pub fn func() {}
        }
    }
}
*/
