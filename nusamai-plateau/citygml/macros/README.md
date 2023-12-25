# citygml/macros

## Attribute macros

### `#[citygml_feature(...)]`

When you write:

```rust
#[citygml_feature(name = "abc:FooBarFeature")]
pub struct FooBarFeature {
    // ...
}
```

It expands to:

```rust
#[derive(Default, Debug, CityGMLElement)]
#[citygml(name = "abc:FooBarFeature")]
pub struct FooBarFeature {
    #[citygml(geom = b"abc")]
    geometries: citygml::GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Vec<String>,

    #[citygml(path = b"gml:description")]
    description: Option<String>,

    // ..,
}
```

### `#[citygml_data(...)]`

When you write:

```rust
#[citygml_data(name = "abc:FooBarData")]
pub struct FooBarData {
    // ...
}
```

It expands to:

```rust
#[derive(Default, Debug, CityGMLElement)]
#[citygml(name = "abc:FooBarData")]
pub struct FooBarFeature {
    // ...
}
```

### `#[citygml_property(...)]`

When you write:

```rust
#[citygml_property(name = "abc:FooBarProperty")]
pub struct FooBarProperty {
   FooBarA(FooBarA),
   FooBarB(FooBarB),
   FooBarC(FooBarC),
}
```

It expands to:

```rust
#[derive(Default, Debug, CityGMLElement)]
#[citygml(name = "abc:FooBarProperty")]
pub enum FooBarProperty {
    #[default]
    Unknown,
    FooBarA(FooBarA), 
    FooBarB(FooBarB),
    FooBarC(FooBarC),
}
```

## Derive macros

### `#[derive(CityGMLElement)]`

This derive macro automatically implements the CityGMLElement trait for the target struct, enabling it to parse corresponding CityGML fragments.

