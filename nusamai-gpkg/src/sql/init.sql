-- https://opengeospatial.github.io/e-learning/geopackage/text/basic.html
-- GPKG v1.3.1
PRAGMA application_id = 1196444487;

PRAGMA user_vesrion = 10301;

CREATE TABLE gpkg_spatial_ref_sys (
    srs_name TEXT NOT NULL,
    srs_id INTEGER NOT NULL PRIMARY KEY,
    organization TEXT NOT NULL,
    organization_coordsys_id INTEGER NOT NULL,
    definition TEXT NOT NULL,
    description TEXT
);

-- Essential spatial reference systems
-- cf. https://opengeospatial.github.io/e-learning/geopackage/text/contents.html
INSERT INTO
    gpkg_spatial_ref_sys (
        srs_name,
        srs_id,
        organization,
        organization_coordsys_id,
        definition
    )
VALUES
    (
        'WGS84',
        4326,
        'EPSG',
        4326,
        'GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]]'
    ),
    (
        'Undefined Geographic',
        0,
        'NONE',
        0,
        'Undefined'
    ),
    (
        'undefined Cartesian',
        -1,
        'NONE',
        -1,
        'Undefined'
    );

CREATE TABLE gpkg_contents (
    table_name TEXT NOT NULL PRIMARY KEY,
    data_type TEXT NOT NULL,
    identifier TEXT UNIQUE,
    description TEXT DEFAULT '',
    last_change DATETIME NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    min_x DOUBLE,
    min_y DOUBLE,
    max_x DOUBLE,
    max_y DOUBLE,
    srs_id INTEGER,
    CONSTRAINT fk_gc_r_srs_id FOREIGN KEY (srs_id) REFERENCES gpkg_spatial_ref_sys(srs_id)
);

CREATE TABLE gpkg_geometry_columns (
    table_name TEXT NOT NULL,
    column_name TEXT NOT NULL,
    geometry_type_name TEXT NOT NULL,
    srs_id INTEGER NOT NULL,
    z TINYINT NOT NULL,
    m TINYINT NOT NULL,
    CONSTRAINT pk_geom_cols PRIMARY KEY (table_name, column_name),
    CONSTRAINT fk_gc_tn FOREIGN KEY (table_name) REFERENCES gpkg_contents(table_name),
    CONSTRAINT fk_gc_srs FOREIGN KEY (srs_id) REFERENCES gpkg_spatial_ref_sys (srs_id)
);

-- Schema
CREATE TABLE gpkg_extensions (
    table_name TEXT,
    column_name TEXT,
    extension_name TEXT NOT NULL,
    definition TEXT NOT NULL,
    scope TEXT NOT NULL,
    CONSTRAINT ge_tce UNIQUE (table_name, column_name, extension_name)
);

INSERT INTO
    gpkg_extensions (
        table_name,
        column_name,
        extension_name,
        definition,
        scope
    )
VALUES
    (
        'gpkg_data_columns',
        NULL,
        'gpkg_schema',
        'https://www.geopackage.org/spec131/#extension_schema',
        'read-write'
    ),
    (
        'gpkg_data_column_constraints',
        NULL,
        'gpkg_schema',
        'https://www.geopackage.org/spec131/#extension_schema',
        'read-write'
    );

CREATE TABLE gpkg_data_columns (
    table_name TEXT NOT NULL,
    column_name TEXT NOT NULL,
    name TEXT,
    title TEXT,
    description TEXT,
    mime_type TEXT,
    constraint_name TEXT,
    CONSTRAINT pk_gdc PRIMARY KEY (table_name, column_name),
    CONSTRAINT gdc_tn UNIQUE (table_name, name)
);

CREATE TABLE gpkg_data_column_constraints (
    constraint_name TEXT NOT NULL,
    constraint_type TEXT NOT NULL,
    value TEXT,
    min NUMERIC,
    min_is_inclusive BOOLEAN,
    max NUMERIC,
    max_is_inclusive BOOLEAN,
    description TEXT,
    CONSTRAINT gdcc_ntv UNIQUE (constraint_name, constraint_type, value)
)