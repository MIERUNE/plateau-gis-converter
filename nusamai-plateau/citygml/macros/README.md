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
#[derive(Default, Debug, CityGMLElement)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
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

    #[citygml(path = b"gml:creationDate")]
    pub creation_date: Option<citygml::Date>

    #[citygml(path = b"gml:terminationDate")]
    pub termination_date: Option<citygml::Date>

    #[citygml(path = b"gml:validFrom")]
    pub valid_from: Option<citygml::Date>

    #[citygml(path = b"gml:validTo")]
    pub valid_to: Option<citygml::Date>

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
#[derive(Default, Debug, CityGMLElement)]
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

### `#[derive(CityGMLElement)]`

It automatically implements the `CityGMLElement` trait for the target struct/enum, enabling it to parse corresponding CityGML fragments.

In most cases, you should use the attribute macros above instead of directly applying this derive macro.
