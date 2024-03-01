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