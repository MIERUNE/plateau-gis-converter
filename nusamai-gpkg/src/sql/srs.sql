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

-- WGS 84 (3D)
-- cf. https://epsg.org/crs_4979/WGS-84.html
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
        'WGS 84 (3D)',
        4979,
        'EPSG',
        4979,
        'GEOGCRS ["WGS 84",ENSEMBLE["World Geodetic System 1984 ensemble", MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]], MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]], MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]], MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]], MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]], MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]], MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]], ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]], ENSEMBLEACCURACY[2],
ID ["EPSG",6326]],CS[ellipsoidal,3,ID["EPSG",6423]],AXIS["Geodetic latitude (Lat)",north,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]]],
AXIS ["Geodetic longitude (Lon)",east,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]]],
AXIS ["Ellipsoidal height (h)",up,LENGTHUNIT["metre",1,ID["EPSG",9001]]],
ID ["EPSG",4979]]'
    );

-- Web Mercator (WGS 84 / Pseudo-Mercator)
-- cf. https://epsg.org/crs_3857/Web_Mercator.html
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
        'WGS 84 / Pseudo-Mercator',
        3857,
        'EPSG',
        3857,
        'PROJCRS["WGS 84 / Pseudo-Mercator",BASEGEOGCRS["WGS 84",ENSEMBLE["World Geodetic System 1984 ensemble",MEMBER["World Geodetic System 1984 (Transit)"],MEMBER["World Geodetic System 1984 (G730)"],MEMBER["World Geodetic System 1984 (G873)"],MEMBER["World Geodetic System 1984 (G1150)"],MEMBER["World Geodetic System 1984 (G1674)"],MEMBER["World Geodetic System 1984 (G1762)"],MEMBER["World Geodetic System 1984 (G2139)"],ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1]],ENSEMBLEACCURACY[2.0]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",4326]],CONVERSION["Popular Visualisation Pseudo-Mercator",METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["easting (X)",east,ORDER[1],LENGTHUNIT["metre",1]],AXIS["northing (Y)",north,ORDER[2],LENGTHUNIT["metre",1]],USAGE[SCOPE["Web mapping and visualisation."],AREA["World between 85.06°S and 85.06°N."],BBOX[-85.06,-180,85.06,180]],ID["EPSG",3857]]'
    );

-- Japan Plane Rectangular CS + JGD2011 (vertical) height
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

-- Japan Plane Rectangular CS
-- cf. https://epsg.org/crs_6669/JGD2011-Japan-Plane-Rectangular-CS-I-JGD2011-vertical-height.html, etc.
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
        6669,
        'EPSG',
        6669,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS I",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone I",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",129.5,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Kyushu west of approximately 130°E - Nagasaki-ken; islands of Kagoshima-ken between 27°N and 32°N and between 128°18''E and 130°E (between 128°18''E and 30°13''E for Amami islands)."],BBOX[26.96,128.17,34.74,130.46]],ID["EPSG",6669]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS II + JGD2011 (vertical) height',
        6670,
        'EPSG',
        6670,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS II",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone II",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",131,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Kyushu east of approximately 130°E - Fukuoka-ken; Saga-ken; Kumamoto-ken; Oita-ken; Miyazaki-ken; Kagoshima-ken (except for area within Japan Plane Rectangular Coordinate System zone I)."],BBOX[30.18,129.76,33.99,132.05]],ID["EPSG",6670]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS III + JGD2011 (vertical) height',
        6671,
        'EPSG',
        6671,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS III",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone III",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",132.166666666667,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu west of approximately 133°15''E - Yamaguchi-ken; Shimane-ken; Hiroshima-ken."],BBOX[33.72,130.81,36.38,133.49]],ID["EPSG",6671]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS IV + JGD2011 (vertical) height',
        6672,
        'EPSG',
        6672,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS IV",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone IV",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",33,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",133.5,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Shikoku - Kagawa-ken; Ehime-ken; Tokushima-ken; Kochi-ken."],BBOX[32.69,131.95,34.45,134.81]],ID["EPSG",6672]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS V + JGD2011 (vertical) height',
        6673,
        'EPSG',
        6673,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS V",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone V",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",134.333333333333,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu between approximately 133°15''E and 135°10''E - Hyogo-ken; Tottori-ken; Okayama-ken."],BBOX[34.13,133.13,35.71,135.47]],ID["EPSG",6673]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VI + JGD2011 (vertical) height',
        6674,
        'EPSG',
        6674,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS VI",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VI",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",136,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu between approximately 135°10''E and 136°45''E - Kyoto-fu; Osaka-fu; Fukui-ken; Shiga-ken; Mie-ken; Nara-ken; Wakayama-ken."],BBOX[33.4,134.86,36.33,136.99]],ID["EPSG",6674]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VII + JGD2011 (vertical) height',
        6675,
        'EPSG',
        6675,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS VII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",137.166666666667,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu between approximately 136°15''E and 137°45''E - Ishikawa-ken; Toyama-ken; Gifu-ken; Aichi-ken."],BBOX[34.51,136.22,37.58,137.84]],ID["EPSG",6675]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS VIII + JGD2011 (vertical) height',
        6676,
        'EPSG',
        6676,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS VIII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone VIII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",138.5,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu between approximately 137°45''E and 139°E - Niigata-ken; Nagano-ken; Yamanashi-ken; Shizuoka-ken."],BBOX[34.54,137.32,38.58,139.91]],ID["EPSG",6676]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS IX + JGD2011 (vertical) height',
        6677,
        'EPSG',
        6677,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS IX",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone IX",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",36,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",139.833333333333,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu - Tokyo-to. (Excludes offshore island areas of Tokyo-to covered by Japan Plane Rectangular Coordinate System zones XIV, XVIII and XIX)."],BBOX[29.31,138.4,37.98,141.11]],ID["EPSG",6677]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS X + JGD2011 (vertical) height',
        6678,
        'EPSG',
        6678,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS X",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone X",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",40,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",140.833333333333,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Honshu north of 38°N approximately - Aomori-ken; Akita-ken; Yamagata-ken; Iwate-ken; Miyagi-ken."],BBOX[37.73,139.49,41.58,142.14]],ID["EPSG",6678]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XI + JGD2011 (vertical) height',
        6679,
        'EPSG',
        6679,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XI",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XI",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",140.25,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Hokkaido west of approximately 141°E - Otaru city; Hakodate city; Date city; Usu-gun and Abuta-gun of Iburi-shicho; Hiyama-shicho; Shiribeshi-shicho; Oshima-shicho."],BBOX[41.34,139.34,43.42,141.46]],ID["EPSG",6679]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XII + JGD2011 (vertical) height',
        6680,
        'EPSG',
        6680,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",142.25,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Hokkaido between approximately 141°E and 143°E - Sapporo city; Asahikawa city; Wakkanai city; Rumoi city; Bibai city; Yubari city; Iwamizawa city; Tomakomai city; Muroran city; Shibetsu city; Nayoro city; Ashibetsu city; Akabira city; Mikasa city; Takikawa city; Sunagawa city; Ebetsu city; Chitose city; Utashinai city; Fukagawa city; Monbetsu city; Furano city; Noboribetsu city; Eniwa city; Ishikari-shicho; Monbetsu-gun of Abashiri-shicho; Kamikawa-shicho; Soya-shicho; Hidaka-shicho; Iburi-shicho (except Usu-gun and Abuta-gun); Sorachi-shicho; Rumoi-shicho."],BBOX[42.15,140.89,45.54,143.61]],ID["EPSG",6680]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XIII + JGD2011 (vertical) height',
        6681,
        'EPSG',
        6681,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XIII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XIII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",44,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",144.25,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Hokkaido east of approximately 143°E - Kitami city; Obihiro city; Kushiro city; Abashiri city; Nemuro city; Nemuro-shicho; Kushiro-shicho; Abashiri-shicho (except Monbetsu-gun); Tokachi-shicho."],BBOX[41.87,142.61,44.4,145.87]],ID["EPSG",6681]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XIV + JGD2011 (vertical) height',
        6682,
        'EPSG',
        6682,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XIV",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XIV",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",26,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",142,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Tokyo-to south of 28°N and between 140°30''E and 143°E."],BBOX[24.67,141.2,27.8,142.33]],ID["EPSG",6682]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XV + JGD2011 (vertical) height',
        6683,
        'EPSG',
        6683,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XV",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XV",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",26,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",127.5,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Okinawa-ken between 126°E and 130°E."],BBOX[26.02,126.63,26.91,128.4]],ID["EPSG",6683]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XVI + JGD2011 (vertical) height',
        6684,
        'EPSG',
        6684,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XVI",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XVI",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",26,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",124,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Okinawa-ken west of 126°E."],BBOX[23.98,122.83,24.94,125.51]],ID["EPSG",6684]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XVII + JGD2011 (vertical) height',
        6685,
        'EPSG',
        6685,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XVII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XVII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",26,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",131,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore Okinawa-ken east of 130°E."],BBOX[24.4,131.12,26.01,131.38]],ID["EPSG",6685]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XVIII + JGD2011 (vertical) height',
        6686,
        'EPSG',
        6686,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XVIII",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XVIII",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",20,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",136,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Tokyo-to south of 28°N and west of 140°30''E."],BBOX[20.37,136.02,20.48,136.16]],ID["EPSG",6686]]'
    ),
    (
        'JGD2011 / Japan Plane Rectangular CS XIX + JGD2011 (vertical) height',
        6687,
        'EPSG',
        6687,
        'PROJCRS["JGD2011 / Japan Plane Rectangular CS XIX",BASEGEOGCRS["JGD2011",DATUM["Japanese Geodetic Datum 2011",ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],PRIMEM["Greenwich",0,ANGLEUNIT["degree",0.0174532925199433]],ID["EPSG",6668]],CONVERSION["Japan Plane Rectangular CS zone XIX",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",26,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",154,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9999,SCALEUNIT["unity",1],ID["EPSG",8805]],PARAMETER["False easting",0,LENGTHUNIT["metre",1],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1],ID["EPSG",8807]]],CS[Cartesian,2],AXIS["northing (X)",north,ORDER[1],LENGTHUNIT["metre",1]],AXIS["easting (Y)",east,ORDER[2],LENGTHUNIT["metre",1]],USAGE[        SCOPE["Cadastre, engineering survey, topographic mapping (large and medium scale)."],AREA["Japan - onshore - Tokyo-to south of 28°N and east of 143°E - Minamitori-shima (Marcus Island)."],BBOX[24.22,153.91,24.35,154.05]],ID["EPSG",6687]]'
    )
