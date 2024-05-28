# mvt demo

```bash
RUST_LOG=debug cargo run --package nusamai --release -- ~/path/to/99999_hoge-shi_city_2023_citygml_1_op/udx/{bldg,tran,brid,frn,veg}/*_6697_op.gml --sink mvt --output mvt
python3 -m http.server
# open http://localhost:8000/
```
