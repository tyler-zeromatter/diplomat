# Fallible Trait Returns
{{get_supports("trait_returns_must_be_fallible")}}

For interpreted languages that with weak typing for variables, traits pose a problem:

```rs
pub trait MyTrait {
  fn some_fn(a : i32) -> i32;
}
```

In Python, if we implement the trait:

```py
class ImplementsTrait(MyTrait):
  def some_fn(a):
    return "ABC"
```

Python does not impose type restrictions upon return. The trait implementation is returned directly into Rust[^cpp], which expects . Rust will throw an exception and halt the program. For languages that do not enforce strict typing on method returns, Diplomat requires all traits return either unit types (which do not need to be converted) or `Result<T, E>`. `E` must be marked specially with the `#[diplomat::attr(*, ffi_error)]` attribute.

[cpp]: In the Nanobind backend, the type must be converted through C++. C++ still has to throw an exception if the type fails to convert, but the principle roughly remains the same.

## FFI Error
Diplomat needs a way to specially denote to you, the bindings writer, that a conversion from a provided type to a given Rust type has failed.

### Currently Supported

#### Enums
```rs
pub enum ErrorEnum {
    A,
    #[diplomat::attr(*, ffi_error)]
    FFIError
}

pub trait SomeTrait {
    fn result_enum_okay() -> Result<T, ErrorEnum>;
}
```

Diplomat will set `ErrorEnum` to `FFIError` if a cast to the Rust type cannot be made when `result_enum_okay` is called.

### Plans to support
Struct fields.
