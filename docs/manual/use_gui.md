# GUIで利用する

## macOSで利用する

Windowsで利用する場合も、同様のUI・手順で利用できます。
以下、macOSでの利用方法を記載します。
本ツールは設定項目が少なく、シンプルに利用することができるため、いくつかの主要なファイル形式に絞って説明をします。

### GeoPackageファイルに変換する

- [GeoPackage](https://www.geopackage.org/)はSQLiteベースのファイル形式で、ベクトルデータを格納できます。
  - バイナリ形式でデータを格納するため、GeoJSONなどのテキスト形式のデータよりファイルサイズが小さく、高速にデータを読み込むことができます。
  - ほぼ全てのGISで利用可能なため、Shapefileの代替として広く利用されています。

![alt text](../resources/useGui_image-7.png)

- `ファイル選択`ボタンをクリックします。

![alt text](../resources/useGui_image.png)

- `選択`ボタンをクリックし、変換したいファイルを選択します。
  - ダウンロード・解凍したデータから `~/sample_data/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg`以下の.gmlファイルを選択します。
  - 複数ファイル選択することも可能です。

![alt text](../resources/useGui_image-1.png)

- `ファイル形式`を `GeoPackage`に選択します。

![alt text](../resources/useGui_image-2.png)

- 座標参照系を変換したい座標系に変えます。

  - 座標参照系は、GeoPackageおよびShapefile選択時にのみ変更できます。
- 出力先を指定します。

  - 拡張子は自動で付与されるので、ファイル名のみを指定してください。

![alt text](../resources/useGui_image-3.png)

- `変換`ボタンをクリックし、データをGeoPackageに変換します。

![alt text](../resources/useGui_image-4.png)

- データが変換されるとダイアログが表示されます。

![alt text](../resources/useGui_image-5.png)

- [QGIS](https://www.qgis.org/ja/site/index.html)などのGISを利用して解析できます。

![alt text](../resources/useGui_image-6.png)

- QGISの利用方法については[こちら](https://www.mlit.go.jp/plateau/learning/tpc05-1/)を参照してください。

### Mapbox Vector Tilesに変換する

- [Mapbox Vector Tiles](https://docs.mapbox.com/data/tilesets/guides/)(以下、MVT)は、タイル形式に分割されたベクトルデータをWeb上で利用するためのファイル形式です。
  - ベクトルデータをタイル形式で格納するため、高速にデータを読み込むことができます。
  - 主に、WebGISで利用できますが、QGISでも利用することが可能です。
  - データの色やラベルなどを柔軟に設定することができ、美しい地図を作成できます。

![alt text](../resources/useGui_image-8.png)

- GeoPackageと同様に設定を変更します。
  - MVTでは、座標参照系を変換することができません。仕様上、EPSG:3857の座標系に変換されます。
  - MVTは大規模データの取り扱いに長けているため、ここでは `bldg`フォルダーを選択し、全ての`.gml`ファイルを選択します。
    - ※大量のメモリ・CPUリソースを消費します。マシンによっては実行できませんので、ご注意ください。
  - 出力データ形式は `Vector Tiles`を選択します。
  - MVTは大量のファイルが出力されるため、出力先にはフォルダー名のみ指定します。

![alt text](../resources/useGui_image-9.png)

- 出力が終わると、以下のようなファイル群が作成されます。

![alt text](../resources/useGui_image-10.png)

- MVTはWebGISでの利用を想定しているため、QGISで利用するためには、ローカルサーバーを立ち上げる必要があります。
  - 生成されたファイル群をどこかへホスティングしても良いです。
- ターミナルなどを利用し、生成された `sample`（など、指定したフォルダー名）フォルダーの中に移動します。
  - `cd ~/Downloads/sample_data/sample`など
- Pythonの `http.server`を利用してローカルサーバーを立ち上げます。

```bash
% python -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```

- ブラウザで `http://localhost:8000`にアクセスすると、以下のように生成されたファイル一覧が表示されていれば成功です。

![alt text](../resources/useGui_image-11.png)

- QGISのブラウザパネルから `Vector Tiles`->`新規一般接続`を選択し、`http://localhost:8000/{z}/{x}/{y}.pbf`を追加します。
  - 名称は任意です。

![alt text](../resources/useGui_image-12.png)

- レイヤーに追加されると、以下のように表示されます。

![alt text](../resources/useGui_image-13.png)

### 3D Tilesに変換する

- [3D Tiles](https://www.cesium.com/3d-tiles/)は、点群データや建築物などの大規模3Dデータをタイル形式に分割し、Web上で利用するためのファイル形式です。
  - 3Dモデルデータ（glTF）をタイル形式で格納するため、高速にデータを読み込むことができます。

![alt text](../resources/useGui_image-14.png)

その他ファイル形式と同様に、設定を行います。

- 例として、テクスチャ付きLOD2データの存在する `~/13104_shinjuku-ku_city_2023_citygml_1_op/udx/bldg/53394525_bldg_6697_op.gml`を選択します。
- 3D Tilesでは、座標参照系をEPSG:4979およびEPSG:6697に変換できます。
  - ただし、CesiumJSではEPSG:4978の座標系が利用されるため、EPSG:6697での変換をおすすめします。
- 3D Tilesは大量のファイルが出力されるため、出力先にはフォルダー名のみ指定します。
- **※3D Tilesでは、MVTよりもさらに大量のメモリ・CPUリソースを消費します。マシンによっては実行できませんので、ご注意ください。**
  - 尚且つ、出力されるファイル群はサイズが非常に大きいため、都市全域などを変換する場合、ファイル数・総容量に注意してください。
- 「出力LODの選択」を「テクスチャ付き最大LOD」とすることで、テクスチャ付きで3D Tilesを出力できます。
  - **ただし、テクスチャ付きでの変換には時間がかかることに留意してください**
  - `最大LOD`: 最大LODを抽出する
  - `最小LOD`: 最小LODを抽出する
  - `テクスチャ付き最大LOD`: テクスチャ付きの最大LODを抽出し、テクスチャがない場合は最大のLODを抽出する
- 「距離（メートル）あたりのテクスチャ解像度を制限する」を有効にすると、小さな地物の過剰に高解像度なテクスチャを適切に調整し、全体的なパフォーマンスを向上させます。
  - 建築物と都市設備（信号機や看板など）を同時に変換する際、都市設備のテクスチャ解像度が建築物と比べて不釣り合いに高くなることがあります。
  - 都市全体を表示する場合、通常は目立たない細部のテクスチャ解像度が高すぎると、システム全体の処理速度が大幅に低下する可能性があります。
  - この機能は、実際の地物の大きさとテクスチャ解像度のバランスを取り、必要に応じてテクスチャの解像度を下げます。これにより、視覚的な品質を維持しながら、システムのパフォーマンスを最適化します。

![alt text](../resources/use_gui_image-4.png)

- MVTと同様、出力された `3dtiles_textured`（など、指定したフォルダー名）フォルダーに移動し、Pythonの `http.server`を利用してローカルサーバーを立ち上げます。

```bash
% cd ~/sample_data/3dtiles_textured
% python -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```

- `3dtiles_textured`フォルダー内など、任意の箇所に以下のような `index.html`を追加します。
  - `index.html`の `const tileset = await Cesium.Cesium3DTileset.fromUrl("tileset.json");`の箇所に `tileset.json`の適切なパスを設定することで3D Tilesが読み込まれます。
  - Cesiumの詳しい利用方法については[こちら](https://www.mlit.go.jp/plateau/learning/tpc06-1/)を参照してください。

```html
<!DOCTYPE html>
<html>
 <head>
  <meta charset="UTF-8" />
  <title>Cesium</title>
  <script src="https://cesium.com/downloads/cesiumjs/releases/1.120/Build/Cesium/Cesium.js"></script>
  <link
   href="https://cesium.com/downloads/cesiumjs/releases/1.120/Build/Cesium/Widgets/widgets.css"
   rel="stylesheet"
  />
  <style>
   #cesiumContainer {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    margin: 0;
    overflow: hidden;
    padding: 0;
    font-family: sans-serif;
   }
   html {
    height: 100%;
   }
   body {
    padding: 0;
    margin: 0;
    overflow: hidden;
    height: 100%;
   }
  </style>
 </head>
 <body>
  <div id="cesiumContainer"></div>
  <script>
   // Cesium ion token for the plateau-terrain-streaming
   // https://github.com/Project-PLATEAU/plateau-streaming-tutorial/blob/main/terrain/plateau-terrain-streaming.md
   Cesium.Ion.defaultAccessToken =
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJlNjk0MTM4NC1lMWI0LTQxNTgtYjcxZS01ZWJhMGJlMTE1MWQiLCJpZCI6MTQ5ODk3LCJpYXQiOjE3MTUxNTEyODZ9.2aUmEQ2-fDsjf-XeC6-hZpwkgwLse3yXoXF4xTOvPAY";

   async function setup() {
    const viewer = new Cesium.Viewer("cesiumContainer", {
     terrainProvider: await Cesium.CesiumTerrainProvider.fromIonAssetId(
      2488101,
      { requestVertexNormals: true }
     ),
     shadows: true,
    });

    var imageProvider = new Cesium.UrlTemplateImageryProvider({
     url: "https://gic-plateau.s3.ap-northeast-1.amazonaws.com/2020/ortho/tiles/{z}/{x}/{y}.png",
     maximumLevel: 19,
    });
    var currentImage =
     viewer.scene.imageryLayers.addImageryProvider(imageProvider);

    viewer.scene.screenSpaceCameraController.enableCollisionDetection = false;
    viewer.scene.globe.depthTestAgainstTerrain = true;

    // Set your 3D Tiles
    const tileset = await Cesium.Cesium3DTileset.fromUrl("tileset.json");
    viewer.scene.primitives.add(tileset);
    viewer.zoomTo(tileset);
   }

   setup();
  </script>
 </body>
</html>
```

- `localhost:8000`にアクセスすると、以下のように3DTilesが表示されます。

![alt text](../resources/use_gui_image-8.png)

### Wavefront OBJ形式に変換する

- [OBJ](https://ja.wikipedia.org/wiki/Wavefront_.obj%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB)は、多くの3DCGソフトウェアで採用され、標準的なデータ交換形式の1つである
  - 3Dモデルの形状データを記録するファイル形式で、頂点座標や面の情報などをテキスト形式で記述する
  - マテリアル情報は別ファイル（`.mtl`）で定義し、テクスチャマッピングなども可能
- その他ファイル形式と同様に、設定を行います。
  - OBJでは、座標参照系を変換することができません。
  - OBJ形式はテキストデータのため、大規模な変換には向きません。
    - 尚且つ、出力されるファイル群はサイズが非常に大きいため、都市全域など広範囲で変換する場合、ファイル数・総容量に注意してください。
    - また、都市全域など広範囲でのテクスチャ付きの変換は、処理時間が1時間以上かかるなど、膨大になる可能性があります。
- 例として、テクスチャ付きLOD2データの存在する `~/13104_shinjuku-ku_city_2023_citygml_1_op/udx/bldg/53394525_bldg_6697_op.gml`を選択します。
- 「出力LODの選択」を「テクスチャ付き最大LOD」とすることで、テクスチャ付きでOBJを出力できます。
  - `最大LOD`: 最大LODを抽出する
  - `最小LOD`: 最小LODを抽出する
  - `テクスチャ付き最大LOD`: テクスチャ付きの最大LODを抽出し、テクスチャがない場合は最大のLODを抽出する
- 「距離（メートル）あたりのテクスチャ解像度を制限する」を有効にすると、小さな地物の過剰に高解像度なテクスチャを適切に調整し、全体的なパフォーマンスを向上させます。
- 「オブジェクトを分割する」を有効にすることで、地物ごとにオブジェクトを分割できます。

![alt text](../resources/use_gui_image-5.png)

- 作成したOBJはBlenderなどを利用して読み込みことが可能です。

![alt text](../resources/use_gui_image-7.png)

### Minecraft（Java版）形式に変換する

- [Minecraft](https://www.minecraft.net/ja-jp) Java版は、Mojang Studiosが開発したサンドボックス型のビデオゲームです。
  - プレイヤーは、ブロックで構成された3Dの世界で、建築、探検、サバイバル、クラフトなど自由にプレイできます。
  - ゲームの目的はとくに設定されておらず、プレイヤーの創造力次第でさまざまな遊び方ができるのが特徴です。
  - Java版では、MODの追加やスキンの変更なども可能で、本ソフトウェアなど外部ツールで作成したワールド情報などを取り込むことが可能です。

![alt text](../resources/use_gui_image.png)

#### 注意事項

- MinecraftはMicrosoft社の商標です
- また、Microsoft社およびMinecraft開発社のMojang Studiosの承認を得ているものではなく、Minecraft公式として提供するものではありません
- 本ソフトウェアで生成されたデータはMinecraft Java版バージョン1.19.4のみで動作確認しています
- 統合版含め、上記バージョン以外のMinecraftでの動作は確認しておりません
- 動作確認には細心の注意を払っておりますが、コマンドの実行等によるいかなる不具合、損害についても責任を負い兼ねます
- 統合版等、動作確認済以外の環境においてデータを利用したことによる不具合、損害についても責任を負い兼ねます
- Minecraftそのものの遊び方やデータの活用方法についてはデータ提供者は対応することができませんので、ご了承ください

#### 変換後データの仕様

本ツールでは、以下のような仕様のMinecraft上で実行可能なファイルが生成されます。

- 生成されるブロックは一辺1mになっています。
- 高度300mを超えるような建物の場合、Minecraft側の高度制限によりブロックが生成されない可能性があります。
- 処理負荷軽減のため、建物の内部は空洞化されています。
- 地物の種類（建築物・橋梁・水辺、など）に応じて別種のブロックが生成されます。
- 建物などの底面は、必ずしも地上に接地しません。
- 複数のファイルを入力すると、複数まとめて変換されますが、コンピューターのリソースを大きく消費する、膨大な時間がかかるなどのデメリットもあります。
- 交通モデルなどPLATEAUの仕様上、標高が0mとして設定されているデータは標高モデルなどと合わせて変換すると標高差により隠れてしまう場合があります。

#### 変換方法

- 他のファイル形式と同様に入力ファイルを設定し、ファイル形式を `Minecraft`に変更します。
- 出力にはフォルダー名を指定します。
  - 出力先に設定したフォルダー名が「ワールド名」になり、Minecraft上での識別子になります。
  - 出力先にはMinecraftのワールド情報を保存する `saves`フォルダーに指定することで、すぐにMinecraft上でプレイできます。
    - `saves`フォルダーはOSによって場所が異なります。
      - Windows: `C:\Users\<USER_NAME>\AppData\Roaming\.minecraft\saves`
      - macOS: `~/Library/Application Support/minecraft/saves`
    - OSのバージョンによって異なる可能性がありますので、「minecraft saves path」などで検索することをおすすめします。

![alt text](../resources/use_gui_image-1.png)

- データの変換が完了すると、このようにMinecraft上でワールドを指定出来るようになります。

![alt text](../resources/use_gui_image-2.png)

- 以下のようにプレイすることが出来れば成功です。

![alt text](../resources/use_gui_image-3.png)

### 属性名を変換する

- 属性名などに日本語名などを利用したい場合は、`rules.json`を利用して属性名を変換します。
- 属性名変換用の `rules.json`の構文を以下に説明します。

  `rename`にnamespace、元の属性名、変換後の属性名を記述します。

  ```json
  {
    "rename": {
      "{namespace}:{元の属性名}": "{変更後の属性名}" ,
      ...,
    }
  }
  ```

  たとえば、`uro:buildingIDAttribute`を `建物ID`に変換したい場合は、以下のように記述します。

  ```json
  {
    "rename": {
      "uro:buildingIDAttribute": "建物ID",
    }
  }
  ```

  また、このようにnamespaceを `*`とすることによってrenameルールが全てのnamespaceに適用されます。

  ```json
  {
    "rename": {
      "*:{元の属性名}": "{変更後の属性名}" ,
      ...,
    }
  }
  ```

  例：

  ```json
  {
    "rename": {
      "*:buildingIDAttribute": "建物ID",
    }
  }
  ```

  `*`を利用し、異なるnamespaceの同名属性に対してrenameルールを指定する場合、より具体的なnamespace表記が優先されます。例えば、以下のように指定する場合、`luse:class`が `土地利用区分`に変換され、namespaceが `luse`以外の `class`属性が `分類`に変換されます。`*`の利用は `*:{元の属性名}`の形に限ります。

  ```json
  {
    "rename": {
      "*:class": "分類",
      "luse:class": "土地利用区分"
    }
  }
  ```

- `rules.json`例

  ```json
  {
    "rename": {
      "uro:buildingIDAttribute": "建物ID",
      "bldg:address": "住所",
      "uro:buildingDataQualityAttribute": "データ品質",
      "uro:buildingDetailAttribute": "建物詳細",
      "gen:genericAttribute": "ジェネリック",
      "bldg:measuredHeight": "高さ",
      "uro:buildingDisasterRiskAttribute": "災害リスク",
      "gml:name": "名前"
    }
  }
  ```

- `設定`の `属性マッピングルール`に `rules.json`を指定します。

![alt text](../resources/useGui_image-17.png)

- `変換`ボタンをクリックし、データを変換すると、属性名が変換されたGeoPackageが出力されます。

![alt text](../resources/useGui_image-18.png)

- 設定を行わない場合のデフォルトの属性名は「[属性マッピングルール](./mapping_rules/index.md)」に記載されています。
