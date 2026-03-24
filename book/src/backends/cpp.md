# C++ Backend


## Type Conversion
The C++ ABI is built upon the [C ABI](./c.md#type-conversion), with some additionall STL types to make working with the C ABI more intuitive from a C++ standpoint.


### Primitives
See [C Primitives](./c.md#primitives).

### Struct Types
|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|  `#[diplomat::opaque] pub struct Type` | `class Type {...}` |
|           `pub struct Type`            | `class Type{...}`|
|           `pub enum Type`              | `enum Type`       |

### Opaques
Opaques are treated as classes that wrap their C ABI pointer, but upon return either wrapped in a `std::unique_ptr<Type>` or a simple `Type*` pointer.

#### Structs
Structs are represented as C++ structs with methods. Each C++ struct is converted from the C ABI into its relevant C++ type.

#### Enums
See [C Enums](./c.md#enums).

### Options
All options (with the exception of opaques) are represented as `std::optional<InnerType>`.

#### Opaque Options
These are nullable pointers.

### Results
All results are returned as `diplomat::result<T, E>`, which is backed by `std::variant`.

```cpp
auto result = MyClass::get_result();
// Get the ok value:
if (result.is_ok()) {
    auto ok = std::move(result).ok().value();
} else if (result.is_err()) {
    auto err = std::move(result).err().value();
}
```

### Slices

|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|           `&[Primitive]`               |`diplomat::span`|
|           `&mut[Primitive]`            |`diplomat::span`|
|`&str` or `&DiplomatStr` or `DiplomatStrSlice` or `DiplomatUtf8StrSlice`|`std::string_view`|
|`&DiplomatStr16` or `DiplomatStr16Slice`|`DiplomatString16View`|
|`&[&str] or &[DiplomatStrSlice]` or `&[DiplomatUtf8StrSlice]`|`DiplomatStringsView`|
|`&[DiplomatStr16Slice]`|`DiplomatStrings16View`|

#### `diplomat::span`
Most 

### Traits
For each trait, the C backend will generate a `Trait` struct and a `VTable` struct:

```c
typedef struct DiplomatTraitStruct_TraitName {
    void *data;
    TraitName_VTable vtable;
} DiplomatTraitStruct_TraitName;

typedef struct TraitName_VTable {
    void (*destructor)(const void*);
    size_t SIZE; size_t ALIGNMENT;
    /* ... */
} TraitName_VTable;
```

Where `data` is a pointer to the type that implements the VTable.

The VTable contains all of the pointers to the functions which implement the trait. `destructor` should be the destructor for `void* data`. `SIZE` and `ALIGNMENT` represent the size and the alignment of the `void* data` pointer.

### Callbacks

Callback arguments generate a struct:

```c
typedef struct DiplomatCallback_FunctionName_ParameterName {
    const void* data;
    void (*run_callback)(const void*, /*...*/);
    void (*destructor)(const void*);
} DiplomatCallback_FunctionName_ParameterName;
```

`data` represents the pointer to the `DiplomatCallback_FunctionName_ParameterName` struct in C. `run_callback` is a pointer to the C function that implements the callback, and `destructor` is a pointer to the destructor for `data`. `destructor` and `data` can both be nullptrs if there is no data associated with the callback (i.e., if in C++ you are only passing in a function, not a function and a pointer).


{{supports("cpp")}}