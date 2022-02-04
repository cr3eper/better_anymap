# better_anymap
An unsafe botched job that doesn't rely on types being 'static lifetime.
Will panic if provided a 0 field struct. I will fix this when I figure out how.



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

