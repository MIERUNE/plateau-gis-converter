# PLATEAU GIS Converter

## 概要

本リポジトリでは、FY2023 の Project PLATEAU「都市デジタルツインの実現に向けた研究開発及び実証調査業務」（内閣府/研究開発とSociety5.0との橋渡しプログラム（BRIDGE））において開発された「PLATEAU GIS Converter」のソースコードを公開しています。

**PLATEAU GIS Converter** は、[PLATEAU](https://www.mlit.go.jp/plateau/)が提供するCityGML形式の3D都市モデルを他の一般的なGISデータ形式に変換するソフトウェアです。

[東京都23区の CityGML (v2)](https://www.geospatial.jp/ckan/dataset/plateau-tokyo23ku-2022/resource/55c72dd0-32eb-4107-9526-71fc0af8d50f3) を読み込んで、3D Tiles に変換した例：

## システム概要

本ソフトウェアの機能は以下の通りです：

- 3D都市モデル（CityGML）の以下の形式への変換：
  - 3D Tiles
  - Mapbox Vector Tiles (MVT)
  - GeoPackage
  - Shapefile
  - KML
  - CZML
- 複数の入力ファイルをもとにした変換
- 属性名マッピングルールの取り込み
- 指定された座標参照系に変換して出力（一部形式で対応）

## フォルダ構成

- アプリケーション：
- 基盤・ユーティリティ：
  - [`nusamai-geometry`](./nusamai-geometry/) &mdash; ジオメトリ型
  - [`nusamai-projection`](./nusamai-projection/) &mdash; 投影法変換
- データソース：
  - [`nusamai-citygml`](./nusamai-plateau/citygml/) &mdash; CityGML パーサ実装支援ライブラリ
    - [`macros`](./nusamai-plateau/citygml/macros/) &mdash; パーサ導出用の proc macros
  - [`nusamai-plateau`](./nusamai-plateau/) &mdash; PLATEAU 用の CityGML モデルおよびパーサ
- 変換先形式のための支援ライブラリ（本プロジェクトのユースケースと癒着しないように構成）
  - [`nusamai-mvt`](./nusamai-mvt/) &mdash; Mapbox Vector Tiles (MVT)
  - [`nusamai-gpkg`](./nusamai-gpkg/) &mdash; GeoPackage
  - [`nusamai-gltf`](./nusamai-gltf/) &mdash; glTF
  - [`nusamai-kml`](./nusamai-kml/) &mdash; KML
  - [`nusamai-czml`](./nusamai-czml/) &mdash; CZML
  - [`nusamai-shapefile`](./nusamai-shapefile/) &mdash; Shapefile

## ライセンス

- 本ソフトウェアは、MITライセンスのもとで提供されるオープンソースソフトウエアです。
- ソースコードおよび関連ドキュメントの著作権は国土交通省および開発者に帰属します。
