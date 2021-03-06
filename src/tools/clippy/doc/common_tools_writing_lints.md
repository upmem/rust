# Common tools for writing lints

You may need following tooltips to catch up with common operations.

- [Common tools for writing lints](#common-tools-for-writing-lints)
  - [Retrieving the type of an expression](#retrieving-the-type-of-an-expression)
  - [Checking if a type implements a specific trait](#checking-if-a-type-implements-a-specific-trait)
  - [Dealing with macros](#dealing-with-macros)

Useful Rustc dev guide links:
- [Stages of compilation](https://rustc-dev-guide.rust-lang.org/compiler-src.html#the-main-stages-of-compilation)
- [Type checking](https://rustc-dev-guide.rust-lang.org/type-checking.html)
- [Ty module](https://rustc-dev-guide.rust-lang.org/ty.html)

# Retrieving the type of an expression

Sometimes you may want to retrieve the type `Ty` of an expression `Expr`, for example to answer following questions:

- which type does this expression correspond to (using its [`TyKind`][TyKind])?
- is it a sized type? 
- is it a primitive type?
- does it implement a trait?

This operation is performed using the [`expr_ty()`][expr_ty] method from the [`TypeckTables`][TypeckTables] struct, 
that gives you access to the underlying structure [`TyS`][TyS].

Example of use:
```rust
impl LateLintPass<'_, '_> for MyStructLint {
    fn check_expr(&mut self, cx: &LateContext<'_, '_>, expr: &Expr<'_>) {
        // Get type of `expr`
        let ty = cx.tables.expr_ty(expr);
        // Match its kind to enter its type
        match ty.kind {
            ty::Adt(adt_def, _) if adt_def.is_struct() => println!("Our `expr` is a struct!"),
            _ => ()
        }
    }
}
```

Similarly in [`TypeckTables`][TypeckTables] methods, you have the [`pat_ty()`][pat_ty] method 
to retrieve a type from a pattern.

Two noticeable items here:
- `cx` is the lint context [`LateContext`][LateContext]. 
  The two most useful data structures in this context are `tcx` and `tables`, 
  allowing us to jump to type definitions and other compilation stages such as HIR.
- `tables` is [`TypeckTables`][TypeckTables] and is created by type checking step, 
  it includes useful information such as types of expressions, ways to resolve methods and so on.

# Checking if a type implements a specific trait

There are two ways to do this, depending if the target trait is part of lang items.

```rust
use crate::utils::{implements_trait, match_trait_method, paths};

impl LateLintPass<'_, '_> for MyStructLint {
    fn check_expr(&mut self, cx: &LateContext<'_, '_>, expr: &Expr<'_>) {
        // 1. Using expression and Clippy's convenient method
        // we use `match_trait_method` function from Clippy's toolbox
        if match_trait_method(cx, expr, &paths::INTO) {
            // `expr` implements `Into` trait
        }

        // 2. Using type context `TyCtxt`
        let ty = cx.tables.expr_ty(expr);
        if cx.tcx.lang_items()
            // we are looking for the `DefId` of `Drop` trait in lang items
            .drop_trait()
            // then we use it with our type `ty` by calling `implements_trait` from Clippy's utils
            .map_or(false, |id| implements_trait(cx, ty, id, &[])) {
                // `expr` implements `Drop` trait
            }
    }
}
```

> Prefer using lang items, if the target trait is available there.

A list of defined paths for Clippy can be found in [paths.rs][paths]

We access lang items through the type context `tcx`. `tcx` is of type [`TyCtxt`][TyCtxt] and is defined in the `rustc_middle` crate.

# Dealing with macros

There are several helpers in Clippy's utils to deal with macros:

- `in_macro()`: detect if the given span is expanded by a macro

You may want to use this for example to not start linting in any macro.

```rust
macro_rules! foo {
    ($param:expr) => {
        match $param {
            "bar" => println!("whatever"),
            _ => ()
        }
    };
}

foo!("bar");

// if we lint the `match` of `foo` call and test its span
assert_eq!(in_macro(match_span), true);
```

- `in_external_macro()`: detect if the given span is from an external macro, defined in a foreign crate

You may want to use it for example to not start linting in macros from other crates

```rust
#[macro_use]
extern crate a_crate_with_macros;

// `foo` is defined in `a_crate_with_macros`
foo!("bar");

// if we lint the `match` of `foo` call and test its span
assert_eq!(in_external_macro(cx.sess(), match_span), true);
```

- `differing_macro_contexts()`: returns true if the two given spans are not from the same context

```rust
macro_rules! m {
    ($a:expr, $b:expr) => {
        if $a.is_some() {
            $b;
        }
    }
}

let x: Option<u32> = Some(42);
m!(x, x.unwrap());

// These spans are not from the same context
// x.is_some() is from inside the macro
// x.unwrap() is from outside the macro
assert_eq!(differing_macro_contexts(x_is_some_span, x_unwrap_span), true);
```

[TyS]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyS.html
[TyKind]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/enum.TyKind.html
[TypeckTables]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TypeckTables.html
[expr_ty]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TypeckTables.html#method.expr_ty
[LateContext]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint/struct.LateContext.html
[TyCtxt]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/context/struct.TyCtxt.html
[pat_ty]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/context/struct.TypeckTables.html#method.pat_ty
[paths]: ../clippy_lints/src/utils/paths.rs
