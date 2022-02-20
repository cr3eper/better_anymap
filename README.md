# better_anymap
An unsafe botched job that doesn't rely on types being 'static lifetime. Don't actually use this in your project, this is WIP.

Current issues
- Zero Sized Types don't work
- Bits are reinterpreted in a way such that pointers are dropped then referenced again (meaning you can store a struct but a Vec or struct with pointer won't work)




# Usage
```rust
#[derive(Id)]
struct TestStruct1{
  data: u32
}

#[derive(Id)]
struct TestStruct2{
  x: u32,
  y: u32
};

let mut better_anymap = better_anymap::AnyMap::new();

better_anymap.insert(TestStruct1{
  123
});

better_anymap.insert(TestStruct2{
  1,
  2
};

let mut struct1: Option<&TestStruct1> = better_anymap.get();

```

