# PostGISにインポートする

## GeoPackageに変換する

コマンドラインまたはGUIを利用してGeoPackageへの変換手順は以下のマニュアルを参考にします。

- [コマンドラインで利用する](../useCommandLine.md)
- [GUIで利用する](../useGui.md)

## GeoPandas(Python)で取り込む

- 実行環境
  - Python3.9
  - 依存するパッケージ
    - geopandas (Version 0.14.3)
    - psycopg2-binary (Version 2.9.9)
    - geoalchemy2 (0.14.4)

- 以下のDocker環境を利用してJupyter Notebook環境を構築することができます。

```bash
cd docs/manual/import_to_postgis
docker compose up
```

Dockerコンテナを起動し、コンソールに表示されるToken付きのローカルホストURLにアクセスすると、ブラウザでJupyter Notebookが立ち上がります。

URL例：
`http://127.0.0.1:8888/lab?token=******`

- 取り込む処理の実行
  `import_to_postgis.ipynb`を開き、データベース情報と投入するGeoPackageファイルの情報を書き換え、実行することによって、データベースにデータを読み込むことができます。 <img src="img/01.png" width="1000" >

## QGISで表示する

- データベースへ接続します
  QGISのブラウザにある「PostgreSQL」項目を右クリック→New Connectionをクリックします。データベース情報を入力し、データベースに接続します。

<img src="img/02.png" width="300">

- 読み込まれたテーブル名をダブルクリックし、QGISに表示されます。

<img src="img/03.png" width="1000" >
