for FILE_FOMART in shapefile gpkg 
do
    cargo run -p nusamai -- .64413325_bldg_6697_op.gml --schema schema_${FILE_FOMART}.json --sink $FILE_FOMART --output out.dummy
done

python3 schema_to_doc.py