pub enum RequirementItem<T> {
    /// Required value (user must not override)
    Required(T),
    /// Recommended value (recommended value but user may override)
    Recommended(T),
    /// Default value
    Default(T),
}

impl<T> RequirementItem<T> {
    pub fn into_value(self) -> T {
        match self {
            Self::Required(v) => v,
            Self::Recommended(v) => v,
            Self::Default(v) => v,
        }
    }
}

pub struct Requirements {
    /// Whether to shorten field names to 10 characters or less for Shapefiles.
    pub shorten_names_for_shapefile: RequirementItem<bool>,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            shorten_names_for_shapefile: RequirementItem::Required(false),
        }
    }
}

pub struct Request {
    pub shorten_names_for_shapefile: bool,
}

impl From<Requirements> for Request {
    fn from(req: Requirements) -> Self {
        Self {
            shorten_names_for_shapefile: req.shorten_names_for_shapefile.into_value(),
        }
    }
}
