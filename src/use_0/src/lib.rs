mod my_mod_1; // モジュールの親子構造を定義する。useはそれが出来た後から使用可

//use my_mod_1; // error[E0432]: unresolved import `my_mod_1`

/*
mod my_mod_1;
use my_mod_1; // error[E0255]: the name `my_mod_1` is defined multiple times
*/

//mod my_mod_1; // error[E0428]: the name `my_mod_1` is defined multiple times    `mod my_mod_1`にて同じことをしているため不要
//use my_mod_1::public;
//use my_mod_1::private;

//use my_mod_1 as my_mod_1; // error[E0432]: unresolved import `my_mod_1`
//use crate::my_mod_1 as my_mod_1; // error[E0432]: unresolved import `crate::my_mod_1`

fn a() {
    crate::my_mod_1::public(); // Rust2018で使える（2015なら`::my_mod_1::public();`）
    my_mod_1::public(); // `mod 子モジュール名;`で使える

    use my_mod_1::public;
    public();
//    private(); // error[E0425]: cannot find function `private` in this scope

//    use my_mod_1::private; // error[E0603]: function `private` is private

    use my_mod_1::*;
    pub_fn();

    use crate as this_crate;
    this_crate::my_mod_1::public();
}

