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

-- Japan Plane Rectangular CS
-- cf. https://epsg.org/crs_10162/JGD2011-Japan-Plane-Rectangular-CS-I-JGD2011-vertical-height.html, etc.
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
        'JGD2011 / Japan Plane Rectangular CS I + JGD2011 (vertical) height',
        10162,
        'EPSG',
        10162,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS I + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS I",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone I",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",129.5,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17801]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6669]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10162]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS II + JGD2011 (vertical) height',
        10163,
        'EPSG',
        10163,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS II + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS II",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone II",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",131,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17802]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6670]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10163]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS III + JGD2011 (vertical) height',
        10164,
        'EPSG',
        10164,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS III + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS III",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone III",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",132.166666666667,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17803]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6671]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10164]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS IV + JGD2011 (vertical) height',
        10165,
        'EPSG',
        10165,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS IV + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS IV",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone IV",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",133.5,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17804]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6672]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10165]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS V + JGD2011 (vertical) height',
        10166,
        'EPSG',
        10166,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS V + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS V",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone V",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",134.333333333334,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17805]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6673]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10166]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VI + JGD2011 (vertical) height',
        10167,
        'EPSG',
        10167,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS VI + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS VI",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VI",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",136,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17806]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6674]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10167]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VII + JGD2011 (vertical) height',
        10168,
        'EPSG',
        10168,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS VII + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS VII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",137.166666666667,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17807]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6675]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10168]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VIII + JGD2011 (vertical) height',
        10169,
        'EPSG',
        10169,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS VIII + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS VIII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VIII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",138.5,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17808]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6676]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10169]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS IX + JGD2011 (vertical) height',
        10170,
        'EPSG',
        10170,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS IX + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS IX",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone IX",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",139.833333333334,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17809]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6677]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10170]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS X + JGD2011 (vertical) height',
        10171,
        'EPSG',
        10171,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS X + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS X",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone X",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",40,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",140.833333333334,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17810]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6678]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10171]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XI + JGD2011 (vertical) height',
        10172,
        'EPSG',
        10172,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS XI + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS XI",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XI",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",140.25,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17811]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6679]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10172]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XII + JGD2011 (vertical) height',
        10173,
        'EPSG',
        10173,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS XII + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS XII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",142.25,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17812]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6680]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10173]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XIII + JGD2011 (vertical) height',
        10174,
        'EPSG',
        10174,
        'COMPOUNDCRS ["JGD2011 / Japan Plane Rectangular CS XIII + JGD2011 (vertical) height",PROJCRS["JGD2011 / Japan Plane Rectangular CS XIII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7019]],ID["EPSG",1128]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XIII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",144.25,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",17813]],CS[Cartesian,2,ID["EPSG",4530]],AXIS["Northing (X)",north],
AXIS ["Easting (Y)",east],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6681]],VERTCRS["JGD2011 (vertical) height",VDATUM["Japanese Geodetic Datum 2011 (vertical)",ID["EPSG",1131]],CS[vertical,1,ID["EPSG",6499]],AXIS["Gravity-related height (H)",up],
LENGTHUNIT ["metre",1,ID["EPSG",9001]],ID["EPSG",6695]],ID["EPSG",10174]]'
    );