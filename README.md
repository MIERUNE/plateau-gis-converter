# Nusamai: BRIDGE 都市デジタルツイン・GISコンバータの開発

[![Test](https://github.com/MIERUNE/nusamai/actions/workflows/build_and_test.yml/badge.svg?branch=main)](https://github.com/MIERUNE/nusamai/actions/workflows/build_and_test.yml)
[![codecov](https://codecov.io/gh/MIERUNE/nusamai/graph/badge.svg?token=oa62wDWoqu)](https://codecov.io/gh/MIERUNE/nusamai)

Notion: [BRIDGE 都市デジタルツイン・GISコンバータの開発](https://www.notion.so/mierune/BRIDGE-GIS-461ba0355b3041619ed3f303a8b0166f)

## リポジトリ構成

- アプリケーション：
    - [`app`](./app/) &mdash; Tauri による GUI アプリケーション
    - [`nusamai`](./nusamai/) &mdash; アプリケーションのバックエンド (およびCLI実装？)
- 基盤：
    - [`nusamai-geometry`](./nusamai-geometry/) &mdash; ジオメトリ型
    - [`nusamai-plateau`](./nusamai-plateau/) &mdash; PLATEAU CityGML パーサ
    - [`nusamai-projection`](./nusamai-projection/) &mdash; 投影法変換
- 変換先形式：
    - [`nusamai-geojson`](./nusamai-geojson/) &mdash; GeoJSON
    - [`nusamai-mvt`](./nusamai-mvt/) &mdash; Mapbox Vector Tile (MVT)
    - [`nusamai-gltf`](./nusamai-gltf/) &mdash; glTF

### 外部リポジトリ

- [MIERUNE/earcut-rs](https://github.com/MIERUNE/earcut-rs) &mdash; ポリゴン三角形化アルゴリズム
- [MIERUNE/japan-geoid](https://github.com/MIERUNE/japan-geoid) &mdash; 日本のジオイドモデル (JGD2011 → WGS 84 の変換に必要)

### 参考

- [MIERUNE/plateau-qgis-plugin](https://github.com/MIERUNE/plateau-qgis-plugin) &mdash; PLATEAU QGIS Plugin の実装
- [MIERUNE/plateau-schema-experiment](https://github.com/MIERUNE/plateau-schema-experiment) &mdash; CityGML 2.0 と i-UR の XML Schema を解析する実験コード群。QGIS Pluginの属性列挙に使用。
