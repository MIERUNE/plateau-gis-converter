for FILE_FORMART in shapefile gpkg geojson czml kml mvt 3dtiles
do
    cargo run -p nusamai -- .64413325_bldg_6697_op.gml --schema schema_${FILE_FORMART}.json --sink $FILE_FORMART --output out.dummy
done

python3 schema_to_doc.py
