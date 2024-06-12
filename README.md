# awesome-rust

Rust learning example 101

- Scalar & Compound
- Ownership
- Struct
-

```rs
pub mod outer_mod {
    pub mod inner_mod {
        pub(in crate::outer_mod) fn outer_mod_visible() {}

        pub(in outer_mod) fn outer_mod_visible_fn_2015() {}

        pub(crate) fn crate_visible_fn() {}

        pub(super) fn super_mod_visible_fn() {
            inner_mod_visible_fn();
        }

        pub(self) fn inner_mod_visible_fn() {}
    }

    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();
    }
}

fn bar() {
    outer_mod::inner_mod::crate_visible_fn();
    outer_mod::foo();
}

fn main() {
    bar();
}

```
