# PostGISにインポートする


## GeoPackageに変換する





## GeoPandas(Python)で取り込む

- 実行環境
  - Python3.9
  - 依存するパッケージ
    - geopandas 
    - psycopg2-binary 
    - geoalchemy2

- 以下のDocker環境を利用してJupyter Notebook環境を構築することができます。

```bash
cd docs/manual/import_to_postgis
docker compose up
```

- 取り込む処理の実行
`import_to_postgis.ipynb`にあるデータベース情報と投入するGeoPackageファイルの情報を書き換え、実行することによって、データベースにデータを読み込むことができます。

## QGISで表示する
