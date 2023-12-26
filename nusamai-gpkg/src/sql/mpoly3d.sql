INSERT INTO
    gpkg_contents (
        table_name,
        data_type,
        identifier,
        min_x,
        min_y,
        max_x,
        max_y,
        srs_id
    )
VALUES
    (
        'mpoly3d',
        'features',
        '3D MultiPolygon',
        -- Japan
        -- cf. https://www.gsi.go.jp/KOKUJYOHO/center.htm
        122.93250000,
        20.42527778,
        153.98666667,
        45.55722222,
        4326
    );

INSERT INTO
    gpkg_geometry_columns (
        table_name,
        column_name,
        geometry_type_name,
        srs_id,
        z,
        m
    )
VALUES
    (
        'mpoly3d',
        'geometry',
        'POINT',
        4326,
        0,
        0
    );

-- TODO: add properties
CREATE TABLE mpoly3d (
    id INTEGER NOT NULL PRIMARY KEY,
    geometry BLOB NOT NULL
);