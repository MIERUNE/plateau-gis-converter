# GUIで利用する

## MacOSで利用する

Windowsで利用する場合も、同様のUI・手順で利用することができます。
以下、MacOSでの利用方法を記載します。
本ツールは設定項目が少なく、シンプルに利用することができるため、いくつかの主要なファイル形式に絞って説明をします。

### GeoPackageファイルに変換する

- [GeoPackage](https://www.geopackage.org/)はSQLiteベースのファイル形式で、ベクトルデータを格納することができます。
  - バイナリ形式でデータを格納するため、GeoJSONなどのテキスト形式のデータよりファイルサイズが小さく、高速にデータを読み込むことができます。
  - ほぼ全てのGISで利用可能なため、Shapefileの代替として広く利用されています。

![alt text](../resources/useGui_image-7.png)

- `ファイル選択`ボタンをクリックします。

![alt text](../resources/useGui_image.png)

- `選択`ボタンをクリックし、変換したいファイルを選択します。
  - ダウンロード・解凍したデータから`~/sample_data/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg`以下の.gmlファイルを選択します。
  - 複数ファイル選択することも可能です。

![alt text](../resources/useGui_image-1.png)

- `ファイル形式`を`GeoPackage`に選択します。

![alt text](../resources/useGui_image-2.png)

- 座標参照系を変換したい座標系に変えます。
  - 座標参照系は、GeoPackageおよびShapefile選択時にのみ変更することができます。

- 出力先を指定します。
  - 拡張子は自動で付与されるので、ファイル名のみを指定してください。

![alt text](../resources/useGui_image-3.png)

- `変換`ボタンをクリックし、データをGeoPackageに変換します。

![alt text](../resources/useGui_image-4.png)

- データが変換されるとダイアログが表示されます。

![alt text](../resources/useGui_image-5.png)

- [QGIS](https://www.qgis.org/ja/site/index.html)などのGISを利用して解析することができます。

![alt text](../resources/useGui_image-6.png)

- QGISの利用方法については[こちら](https://www.mlit.go.jp/plateau/learning/tpc05-1/)を参照してください。

### Mapbox Vector Tilesに変換する

- [Mapbox Vector Tiles](https://docs.mapbox.com/data/tilesets/guides/)(以下、MVT)は、タイル形式に分割されたベクトルデータをWeb上で利用するためのファイル形式です。
  - ベクトルデータをタイル形式で格納するため、高速にデータを読み込むことができます。
  - 主に、WebGISで利用することができますが、QGISでも利用することが可能です。
  - データの色やラベルなどを柔軟に設定することができ、美しい地図を作成することができます。

![alt text](../resources/useGui_image-8.png)

- GeoPackageと同様に設定を変更していきます。
  - MVTでは、座標参照系を変換することができません。仕様上、EPSG:3857の座標系に変換されます。
  - MVTは大規模データの取り扱いに長けているため、ここでは`bldg`フォルダを選択し、全ての.gmlファイルを選択します。
    - ※大量のメモリ・CPUリソースを消費します。マシンによっては実行できませんので、ご注意ください。
  - 出力データ形式は`Vector Tiles`を選択します。
  - MVTは大量のファイルが出力されるため、出力先にはフォルダ名のみ指定します。

![alt text](../resources/useGui_image-9.png)

- 出力が終わると、以下のようなファイル群が作成されます。

![alt text](../resources/useGui_image-10.png)

- MVTはWebGISでの利用を想定しているため、QGISで利用するためには、ローカルサーバーを立ち上げる必要があります。
  - 生成されたファイル群をどこかへホスティングしても良いです。
- ターミナルなどを利用し、生成された`sample`（など、指定したフォルダ名）フォルダの中に移動します。
  - `cd ~/Downloads/sample_data/sample`など
- Pythonの`http.server`を利用してローカルサーバーを立ち上げます。

```bash
% python -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```

- ブラウザで`http://localhost:8000`にアクセスすると、以下のように生成されたファイル一覧が表示されていれば成功です。

![alt text](../resources/useGui_image-11.png)

- QGISのブラウザパネルから`Vector Tiles`->`新規一般接続`を選択し、`http://localhost:8000/{z}/{x}/{y}.pbf`を追加します。
  - 名称は任意です。

![alt text](../resources/useGui_image-12.png)

- レイヤーに追加されると、以下のように表示されます。

![alt text](../resources/useGui_image-13.png)

### 3DTilesに変換する

- [3DTiles](https://www.cesium.com/3d-tiles/)は、点群データや建築物などの大規模3Dデータをタイル形式に分割し、Web上で利用するためのファイル形式です。
  - 3Dモデルデータ（glTF）をタイル形式で格納するため、高速にデータを読み込むことができます。

![alt text](../resources/useGui_image-14.png)

- その他ファイル形式と同様に、設定を行います。
  - 3DTilesでは、座標参照系を変換することができません。仕様上、EPSG:4978の座標系に変換されます。
  - 3DTilesは大量のファイルが出力されるため、出力先にはフォルダ名のみ指定します。
  - ※3DTilesでは、MVTよりもさらに大量のメモリ・CPUリソースを消費します。マシンによっては実行できませんので、ご注意ください。
    - 尚且つ、出力されるファイル群は非常に大きいため、東京23区全域などを変換する場合、ファイル数・総容量に注意してください。

- 例として、テクスチャ付きLOD2データの存在する`~/sample_data/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg/53394601_bldg_6697_2_op.gml`を選択します。
  - テクスチャは自動的に取り込まれるため、テクスチャファイルは選択する必要はありません。

![alt text](../resources/useGui_image-15.png)

- MVTと同様、出力された`sample_3dtiles`（など、指定したフォルダ名）フォルダに移動し、Pythonの`http.server`を利用してローカルサーバーを立ち上げます。

```bash
% cd ~/sample_data/sample_3dtiles
% python -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
```

- `sample_3dtiles`フォルダ内に以下のような`index.html`を追加します。
  - Cesiumの詳しい利用方法については[こちら](https://www.mlit.go.jp/plateau/learning/tpc06-1/)を参照してください。

```html
<!DOCTYPE html>
<html>
 <head>
  <meta charset="UTF-8" />
  <title>Cesium</title>
  <script src="https://cesium.com/downloads/cesiumjs/releases/1.115/Build/Cesium/Cesium.js"></script>
  <link
   href="https://cesium.com/downloads/cesiumjs/releases/1.115/Build/Cesium/Widgets/widgets.css"
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
   // Set your token
   Cesium.Ion.defaultAccessToken =
    "<Your Token>";

   async function setup() {
    const viewer = new Cesium.Viewer("cesiumContainer", {
     terrainProvider: await Cesium.CesiumTerrainProvider.fromIonAssetId(
      770371,
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

    // Set your 3DTiles
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

![alt text](../resources/useGui_image-16.png)

### Minecraft（Java版）形式に変換する

- [Minecraft](https://www.minecraft.net/ja-jp) Java版は、Mojang Studiosが開発したサンドボックス型のビデオゲームです。
  - プレイヤーは、ブロックで構成された3Dの世界で、建築、探検、サバイバル、クラフトなど自由にプレイできます。
  - ゲームの目的は特に設定されておらず、プレイヤーの創造力次第で様々な遊び方ができるのが特徴です。
  - Java版では、MODの追加やスキンの変更なども可能で、本ソフトウェアなど外部ツールで作成したワールド情報などを取り込むことが可能です。

![alt text](../resources/use_gui_image.png)

#### 注意事項

- MinecraftはMicrosoft社の商標です
- また、Microsoft社及びMinecraft開発社のMojang Studiosの承認を得ているものではなく、Minecraft公式として提供するものではありません
- 本ソフトウェアで生成されたデータはMinecraft Java版 バージョン1.19.4のみで動作確認しています
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
- 複数のファイルを入力すると、複数まとめて変換されますが、コンピュータのリソースを大きく消費する、膨大な時間がかかるなどのデメリットもあります。
- 交通モデルなどPLATEAUの仕様上、標高が0mとして設定されているデータは標高モデルなどと合わせて変換すると標高差により隠れてしまう場合があります。

#### 変換方法

- 他のファイル形式と同様に入力ファイルを設定し、ファイル形式を`Minecraft`に変更していきます。
- 出力にはフォルダ名を指定します。
  - 出力先に設定したフォルダ名が「ワールド名」になり、Minecraft上での識別子になります。
  - 出力先にはMinecraftのワールド情報を保存する`saves`フォルダに指定することで、すぐにMinecraft上でプレイすることができます。
    - `saves`フォルダはOSによって場所が異なります。
      - Windows: `C:\Users\<USER_NAME>\AppData\Roaming\.minecraft\saves`
      - MacOS: `~/Library/Application Support/minecraft/saves`
    - OSのバージョンによって異なる可能性がありますので、「minecraft saves path」などで検索することをおすすめします。

![alt text](../resources/use_gui_image-1.png)

- データの変換が完了すると、このようにMinecraft上でワールドを指定出来るようになります。

![alt text](../resources/use_gui_image-2.png)

- 以下のようにプレイすることが出来れば成功です。

![alt text](../resources/use_gui_image-3.png)

### 属性名を変換する

- 属性名などに日本語名などを利用したい場合は、`rules.json`を利用して属性名を変換します。
- 属性名変換用の`rules.json`の構文を以下に説明します。

  `rename`にnamespace、元の属性名、変換後の属性名を記述します。

    ```json
    {
      "rename": {
        "{namespace}:{元の属性名}": "{変更後の属性名}" ,
        ...,
      }
    }
    ```

  例えば、`uro:buildingIDAttribute`を`建物ID`に変換したい場合は、以下のように記述します。

    ```json
    {
      "rename": {
        "uro:buildingIDAttribute": "建物ID",
      }
    }
    ```

  また、このようにnamespaceを`*`とすることによってrenameルールが全てのnamespaceに適用されます。

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

  `*`を利用し、異なるnamespaceの同名属性に対してrenameルールを指定する場合、より具体的なnamespace表記が優先されます。例えば、以下のように指定する場合、`luse:class`が`土地利用区分`に変換され、namespaceが`luse`以外の`class`属性が`分類`に変換されます。`*`の利用は`*:{元の属性名}`の形に限ります。

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

- `設定`の`属性マッピングルール`に`rules.json`を指定します。

![alt text](../resources/useGui_image-17.png)

- `変換`ボタンをクリックし、データを変換すると、属性名が変換されたGeoPackageが出力されます。

![alt text](../resources/useGui_image-18.png)

- 設定を行わない場合のデフォルトの属性名は「[属性マッピングルール](./mapping_rules/index.md)」に記載されています。
