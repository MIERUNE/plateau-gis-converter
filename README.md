# PLATEAU GISコンバータ (PLATEAU GIS Converter)

[![Test Tauri App](https://github.com/MIERUNE/PLATEAU-GIS-Converter/actions/workflows/test_app.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_app.yml)
[![Test Libraries](https://github.com/MIERUNE/PLATEAU-GIS-Converter/actions/workflows/test_libs.yml/badge.svg)](https://github.com/MIERUNE/nusamai/actions/workflows/test_libs.yml)
[![Codecov](https://codecov.io/gh/MIERUNE/PLATEAU-GIS-Converter/graph/badge.svg?token=oa62wDWoqu)](https://codecov.io/gh/MIERUNE/PLATEAU-GIS-Converter)
<!--
[![Docs](https://github.com/MIERUNE/PLATEAU-GIS-Converter/actions/workflows/doc.yml/badge.svg)](https://mierune.github.io/nusamai/app/)
-->

## 1. 概要

本リポジトリでは、Project PLATEAUの令和5年度「都市デジタルツインの実現に向けた研究開発及び実証調査業務」（内閣府/研究開発とSociety5.0との橋渡しプログラム（BRIDGE））におけるCm23-09「GISコンバータ作成」において開発した「GISコンバータ (PLATEAU GIS Converter)」のソースコードを公開しています。

PLATEAU GIS Converter は、[PLATEAU](https://www.mlit.go.jp/plateau/)プロジェクトのCityGML形式の3D都市モデルを他の一般的なGISデータ形式に変換するソフトウェアです。

ソフトウェアのメイン画面：

![ソフトウェアのメイン画面](docs/resources/README_image-1.png)

[東京都23区の CityGML (v2)](https://www.geospatial.jp/ckan/dataset/plateau-tokyo23ku-2022/resource/55c72dd0-32eb-4107-9526-71fc0af8d50f3) を読み込んで、3D Tiles に変換した例：

![](docs/resources/README_image.png)

## 2. 「GISコンバータ」について

PLATEAU の標準仕様に準拠した CityGML 2.0 形式の3D都市モデルは、専門のGISツールやCLIコマンドを用いて他のGIS形式に変換して用いられることが一般的ですが、一般ユーザーが簡易に利用できる汎用ツールは存在しません。そのため、流通や活用の範囲が専門家や技術者に限られていました。

「GISコンバータ」を利用することで、3D都市モデルを一般的なGIS形式に変換して、様々な分析・開発を行うことができます：

- GeoPackage 形式による [QGIS](https://www.qgis.org/) 等での解析
- Mapbox Vector Tiles (MVT) 形式による、大規模データのWeb等での高速描画
- KML 形式による Google Earth での可視化
- 3D Tiles 形式による [Cesium](https://cesium.com/) 等での可視化
- など

## 3. 利用手順

- ソフトウェアの最新版は、GitHub リポジトリの [Releaseページ](https://github.com/MIERUNE/PLATEAU-GIS-Converter/releases) からダウンロードしてください。
- 詳しい利用方法については、こちらの[マニュアル](https://MIERUNE.github.io/PLATEAU-GIS-Converter/index.html)をご覧ください。

## 4. システム概要

本ソフトウェアの機能は以下の通りです：

- 3D都市モデル（CityGML）の変換
  - 3D Tiles
  - Mapbox Vector Tiles (MVT)
  - GeoPackage
  - GeoJSON
  - Shapefile
  - KML
  - CZML
- データの一括変換
- 属性マッピングルールの取り込み
- 特定の座標系へ座標変換して出力

## 5. 利用技術

利用技術は以下の通りです。

内部ロジック：
- ファイル変換のロジックはすべて、プログラミング言語 [Rust](https://www.rust-lang.org/) で実装しています。

ユーザインタフェース (UI)：
  - デスクトップアプリケーション構築フレームワーク: [Tauri](https://github.com/tauri-apps/tauri)
  - Web UI構築フレームワーク: [Svelte](https://svelte.dev/)

## 6. 動作環境

本ソフトウェアは以下の環境で動作することを想定しています：

- OS:
  - Windows (Intel)
  - macOS (Apple Silicon, Intel)
- CPU:
  - 特に制限はありません。変換対象の範囲や出力形式に応じて処理時間に大きな影響があります。
- メモリ:
  - 特に制限はありませんが、出力形式や変換対象の範囲によっては、変換時に多くのメモリを要します。
- ネットワーク:
  - インターネット接続は不要です。
- ストレージ:
  - インストールには30MB程度の空き容量が必要です。
  - 変換時には、変換元データと同等ないしそれ以上の空き容量が必要です。

## 7. 本リポジトリのフォルダ構成

- アプリケーション：
  - [`app`](./app/) &mdash; Tauri による GUI 実装
  - [`nusamai`](./nusamai/) &mdash; アプリケーションバックエンドおよびコマンドライン版の実装
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
  - [`nusamai-geojson`](./nusamai-geojson/) &mdash; GeoJSON
  - [`nusamai-kml`](./nusamai-kml/) &mdash; KML
  - [`nusamai-czml`](./nusamai-czml/) &mdash; CZML
  - [`nusamai-shapefile`](./nusamai-shapefile/) &mdash; Shapefile

### 7.1. 外部リポジトリ

- [MIERUNE/cesiumtiles-rs](https://github.com/MIERUNE/cesiumtiles-rs) &mdash; 3D Tiles の JSONモデルなど
- [MIERUNE/earcut-rs](https://github.com/MIERUNE/earcut-rs) &mdash; ポリゴン三角形化アルゴリズムのRust実装
- [MIERUNE/japan-geoid](https://github.com/MIERUNE/japan-geoid) &mdash; 日本のジオイドモデル (JGD2011 → WGS 84 の変換)

## 8. ライセンス

- 本ソフトウェアは、MITライセンスのもとで提供されるオープンソースソフトウエアです。
- 本ソフトウェアの開発は[株式会社MIERUNE](https://www.mierune.co.jp/)が行っています。
- ソースコードおよび関連ドキュメントの著作権は国土交通省に帰属します。

## 9. 注意事項

- 本リポジトリおよびソフトウェアは Project PLATEAU の参考資料として提供しているものです。動作の保証は行っておりません。
- 本リポジトリの内容は予告なく変更・削除する場合があります。
- 本リポジトリおよび本ソフトウェアの利用により生じた損失及び損害等について、開発者および国土交通省はいかなる責任も負わないものとします。

## 10. 参考資料

- [PLATEAU プロジェクト](https://www.mlit.go.jp/plateau/)
- [3D都市モデル標準製品仕様書](https://www.mlit.go.jp/plateaudocument/)
- [PLATEAU QGIS Plugin](https://github.com/MIERUNE/plateau-qgis-plugin)

## Development (開発者向け情報)

<!--
- [MIERUNE/plateau-schema-experiment](https://github.com/MIERUNE/plateau-schema-experiment) &mdash; CityGML 2.0 と i-UR の XML Schema を解析する実験コード群。QGIS Pluginの属性列挙に使用。
- [MIERUNE/3dtiles-research](https://github.com/MIERUNE/3dtiles-research) &mdash; 3D Tiles / glTF の実験コード群
-->

### Build &amp; Run

#### CLI

```bash
cd ./nusamai/
# Debug (非常に低速)
cargo run -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (最適化コンパイル、実用速度)
cargo run --release -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
# Release (LTO有効のプロダクションビルド、最高速)
cargo run --profile release-lto -- ~/path/to/PLATEAU/15100_niigata-shi_2022_citygml_1_op/udx/bldg/*.gml --sink geojson --output foobar.geojson
```

#### GUI

Dev:

```bash
cd ./app/
npm install
RUST_BACKTRACE=1 npx tauri dev
```

Build:

```bash
cd ./app
npx tauri build
```

### Test

#### Coverage

Codecov: <https://app.codecov.io/gh/MIERUNE/nusamai>

```bash
cargo llvm-cov --workspace --exclude app --html --all-features
```
