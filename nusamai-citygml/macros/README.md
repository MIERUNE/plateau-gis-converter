# citygml/macros

## Attribute macros

- `#[citygml_feature(...)]`
- `#[citygml_data(...)]`
- `#[citygml_property(...)]`

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
#[derive(Default, Debug, CityGmlElement)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
#[citygml(name = "abc:FooBarFeature")]
pub struct FooBarFeature {
    #[citygml(geom = b"abc")]
    geometries: nusamai_citygml::GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Vec<Code>,

    #[citygml(path = b"gml:description")]
    description: Option<String>,

    #[citygml(path = b"gml:creationDate")]
    pub creation_date: Option<nusamai_citygml::Date>

    #[citygml(path = b"gml:terminationDate")]
    pub termination_date: Option<nusamai_citygml::Date>

    #[citygml(path = b"gml:validFrom")]
    pub valid_from: Option<nusamai_citygml::Date>

    #[citygml(path = b"gml:validTo")]
    pub valid_to: Option<nusamai_citygml::Date>

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
#[derive(Default, Debug, CityGmlElement)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
#[citygml(name = "abc:FooBarData")]
pub struct FooBarFeature {
    // ...
}
```

### `#[citygml_property(...)]`

When you write:

```rust
#[citygml_property(name = "abc:FooBarProperty")]
pub enum FooBarProperty {
   FooBarA(FooBarA),
   FooBarB(FooBarB),
   FooBarC(FooBarC),
}
```

It expands to:

```rust
#[derive(Default, Debug, CityGmlElement)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
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

### `#[derive(CityGmlElement)]`

It automatically implements the `CityGmlElement` trait for the target struct/enum, enabling it to parse corresponding CityGML fragments.

In most cases, you should use the attribute macros above instead of directly applying this derive macro.


## Check the result of macro expansion

```bash
cargo install cargo-expand
cd ./nusamai-plateau/
cargo expand models::building --no-default-features
```
