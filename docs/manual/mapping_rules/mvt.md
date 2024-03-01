## 地物 (Feature stereotype)

### bldg:Building

建築物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | Code | 機能 | bldg:function |
| usage | Code | 用途 | bldg:usage |
| yearOfConstruction | NonNegativeInteger | 建築年 | bldg:yearOfConstruction |
| yearOfDemolition | NonNegativeInteger | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measuredHeight | Measure | 計測高さ | bldg:measuredHeight |
| storeysAboveGround | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| storeysBelowGround | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| storeyHeightsAboveGround | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| storeyHeightsBelowGround | String |  | bldg:storeyHeightsBelowGround |
| outerBuildingInstallation | bldg:BuildingInstallation | 建物付属物 | bldg:outerBuildingInstallation |
| interiorBuildingInstallation | bldg:BuildingInstallation | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | bldg:_BoundarySurfaceProperty | 境界面 | bldg:boundedBy |
| interiorRoom | bldg:Room | 部屋 | bldg:interiorRoom |
| consistsOfBuildingPart | bldg:BuildingPart | 建物部品 | bldg:consistsOfBuildingPart |
| address | core:Address | 住所 | bldg:address |
| bldgDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:bldgDmAttribute |
| bldgFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:bldgFacilityAttribute |
| bldgFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:bldgFacilityIdAttribute |
| bldgFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:bldgFacilityTypeAttribute |
| bldgRealEstateIDAttribute | uro:RealEstateIDAttribute | 不動産ID | uro:bldgRealEstateIDAttribute |
| buildingDataQualityAttribute | uro:BuildingDataQualityAttribute | データ品質 | uro:buildingDataQualityAttribute |
| buildingDetailAttribute | uro:BuildingDetailAttribute | 建物利用現況 | uro:buildingDetailAttribute |
| buildingDisasterRiskAttribute | uro:BuildingDisasterRiskAttributeProperty | 災害リスク | uro:buildingDisasterRiskAttribute |
| buildingIDAttribute | uro:BuildingIDAttribute | 建物識別情報 | uro:buildingIDAttribute |
| ifcBuildingAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBuildingAttribute |
| indoorBuildingAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorBuildingAttribute |
| keyValuePairAttribute | uro:KeyValuePairAttribute | 拡張属性 | uro:keyValuePairAttribute |
| largeCustomerFacilityAttribute | uro:LargeCustomerFacilityAttribute | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |

### bldg:BuildingFurniture


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | bldg:class |
| function | Code |  | bldg:function |
| usage | Code |  | bldg:usage |
| ifcBuildingFurnitureAttribute | uro:IfcAttributeProperty |  | uro:ifcBuildingFurnitureAttribute |
| indoorFutnitureAttribute | uro:IndoorAttributeProperty |  | uro:indoorFutnitureAttribute |

### bldg:BuildingInstallation

建築物付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | Code | 機能 | bldg:function |
| usage | Code | 用途 | bldg:usage |
| boundedBy | bldg:_BoundarySurfaceProperty | 境界面 | bldg:boundedBy |
| ifcBuildingInstallationAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBuildingInstallationAttribute |

### bldg:BuildingPart

建築物部品

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | Code | 機能 | bldg:function |
| usage | Code | 用途 | bldg:usage |
| yearOfConstruction | String | 建築年 | bldg:yearOfConstruction |
| yearOfDemolition | String | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measuredHeight | Measure | 計測高さ | bldg:measuredHeight |
| storeysAboveGround | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| storeysBelowGround | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| storeyHeightsAboveGround | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| storeyHeightsBelowGround | String |  | bldg:storeyHeightsBelowGround |
| outerBuildingInstallation | bldg:BuildingInstallation | 建物付属物 | bldg:outerBuildingInstallation |
| interiorBuildingInstallation | bldg:BuildingInstallation | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | bldg:_BoundarySurfaceProperty | 境界面 | bldg:boundedBy |
| interiorRoom | bldg:Room | 部屋 | bldg:interiorRoom |
| consistsOfBuildingPart | bldg:BuildingPart | ー | bldg:consistsOfBuildingPart |
| address | core:Address | 住所 | bldg:address |
| bldgDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:bldgDmAttribute |
| bldgFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:bldgFacilityAttribute |
| bldgFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:bldgFacilityIdAttribute |
| bldgFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:bldgFacilityTypeAttribute |
| bldgRealEstateIDAttribute | uro:RealEstateIDAttribute |  | uro:bldgRealEstateIDAttribute |
| buildingDataQualityAttribute | uro:BuildingDataQualityAttribute | ー | uro:buildingDataQualityAttribute |
| buildingDetailAttribute | uro:BuildingDetailAttribute | 建物利用現況 | uro:buildingDetailAttribute |
| buildingDisasterRiskAttribute | uro:BuildingDisasterRiskAttributeProperty | ー | uro:buildingDisasterRiskAttribute |
| buildingIDAttribute | uro:BuildingIDAttribute | 建物識別属性 | uro:buildingIDAttribute |
| ifcBuildingAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBuildingAttribute |
| indoorBuildingAttribute | uro:IndoorAttributeProperty | 屋内属性 | uro:indoorBuildingAttribute |
| keyValuePairAttribute | uro:KeyValuePairAttribute | ー | uro:keyValuePairAttribute |
| largeCustomerFacilityAttribute | uro:LargeCustomerFacilityAttribute | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |

### bldg:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |

### bldg:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| ifcOpeningAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcOpeningAttribute |
| indoorOpeningAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorOpeningAttribute |
| address | core:Address |  | bldg:address |

### bldg:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |

### bldg:GroundSurface


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty |  | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty |  | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |

### bldg:OuterCeilingSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:OuterFloorSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:Room

部屋

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | Code | 機能 | bldg:function |
| usage | Code | 用途 | bldg:usage |
| boundedBy | bldg:_BoundarySurfaceProperty | 境界面 | bldg:boundedBy |
| interiorFurniture | bldg:BuildingFurniture | 家具 | bldg:interiorFurniture |
| roomInstallation | bldg:BuildingInstallation | 屋内付属物 | bldg:roomInstallation |
| ifcRoomAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcRoomAttribute |
| indoorRoomAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorRoomAttribute |
| roomDataQualityAttribute | uro:RoomDataQualityAttribute |  | uro:roomDataQualityAttribute |

### bldg:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | bldg:_OpeningProperty | 開口部 | bldg:opening |
| ifcBoundarySurfaceAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorBoundarySurfaceAttribute | uro:IndoorAttributeProperty |  | uro:indoorBoundarySurfaceAttribute |

### bldg:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| ifcOpeningAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcOpeningAttribute |
| indoorOpeningAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorOpeningAttribute |

### tran:AuxiliaryTrafficArea

交通補助領域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| surfaceMaterial | Code | 舗装種類 | tran:surfaceMaterial |

### tran:Railway

鉄道

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| trafficArea | tran:TrafficArea | 交通領域 | tran:trafficArea |
| auxiliaryTrafficArea | tran:AuxiliaryTrafficArea | 交通補助領域 | tran:auxiliaryTrafficArea |
| tranDataQualityAttribute | uro:TransportationDataQualityAttribute | データ品質 | uro:tranDataQualityAttribute |
| tranFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tranFacilityAttribute |
| tranFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tranFacilityIdAttribute |
| tranFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tranFacilityTypeAttribute |
| railwayRouteAttribute | uro:RailwayRouteAttribute | 鉄道路線属性 | uro:railwayRouteAttribute |

### tran:Road

道路

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| trafficArea | tran:TrafficArea | 交通領域 | tran:trafficArea |
| auxiliaryTrafficArea | tran:AuxiliaryTrafficArea | 交通補助領域 | tran:auxiliaryTrafficArea |
| tranDataQualityAttribute | uro:TransportationDataQualityAttribute | データ品質 | uro:tranDataQualityAttribute |
| tranFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tranFacilityAttribute |
| tranFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tranFacilityIdAttribute |
| tranFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tranFacilityTypeAttribute |
| roadStatus | uro:RoadType |  | uro:roadStatus |
| roadStructureAttribute | uro:RoadStructureAttribute | 道路構造属性 | uro:roadStructureAttribute |
| trafficVolumeAttribute | uro:TrafficVolumeAttribute | 交通量属性 | uro:trafficVolumeAttribute |

### tran:Square

広場

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| trafficArea | tran:TrafficArea | 交通領域 | tran:trafficArea |
| auxiliaryTrafficArea | tran:AuxiliaryTrafficArea | 交通補助領域 | tran:auxiliaryTrafficArea |
| tranDataQualityAttribute | uro:TransportationDataQualityAttribute | データ品質 | uro:tranDataQualityAttribute |
| tranFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tranFacilityAttribute |
| tranFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tranFacilityIdAttribute |
| tranFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tranFacilityTypeAttribute |
| squareUrbanPlanAttribute | uro:SquareUrbanPlanAttributeProperty | 都市計画施設現況属性 | uro:squareUrbanPlanAttribute |

### tran:Track

徒歩道

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| trafficArea | tran:TrafficArea | 交通領域 | tran:trafficArea |
| auxiliaryTrafficArea | tran:AuxiliaryTrafficArea | 交通補助領域 | tran:auxiliaryTrafficArea |
| tranDataQualityAttribute | uro:TransportationDataQualityAttribute | データ品質 | uro:tranDataQualityAttribute |
| tranFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tranFacilityAttribute |
| tranFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tranFacilityIdAttribute |
| tranFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tranFacilityTypeAttribute |
| trackAttribute | uro:TrackAttribute | 徒歩道属性 | uro:trackAttribute |

### tran:TrafficArea

交通領域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty |  | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| surfaceMaterial | Code | 舗装種類 | tran:surfaceMaterial |
| railwayTrackAttribute | uro:RailwayTrackAttribute | 軌道中心線線形情報 | uro:railwayTrackAttribute |
| trafficAreaStructureAttribute | uro:TrafficAreaStructureAttribute | 道路構造属性 | uro:trafficAreaStructureAttribute |

### luse:LandUse

土地利用

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 土地利用区分 | luse:class |
| function | Code | 機能 | luse:function |
| usage | Code | 用途 | luse:usage |
| ifcLandUseAttribute | uro:IfcAttributeProperty |  | uro:ifcLandUseAttribute |
| landUseDetailAttribute | uro:LandUseDetailAttribute |  | uro:landUseDetailAttribute |
| luseDataQualityAttribute | uro:LandUseDataQualityAttribute |  | uro:luseDataQualityAttribute |
| luseDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:luseDmAttribute |
| luseFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:luseFacilityAttribute |
| luseFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:luseFacilityIdAttribute |
| luseFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:luseFacilityTypeAttribute |

### brid:Bridge

橋梁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |
| yearOfConstruction | String | 建設年 | brid:yearOfConstruction |
| yearOfDemolition | String | 解体年 | brid:yearOfDemolition |
| isMovable | Boolean | 可動橋区分 | brid:isMovable |
| outerBridgeConstruction | brid:BridgeConstructionElement | 橋梁部材 | brid:outerBridgeConstruction |
| outerBridgeInstallation | brid:BridgeInstallation | 橋梁付属物 | brid:outerBridgeInstallation |
| interiorBridgeInstallation | brid:BridgeInstallation | 橋梁内部付属物 | brid:interiorBridgeInstallation |
| boundedBy | brid:_BoundarySurfaceProperty | 境界面 | brid:boundedBy |
| interiorBridgeRoom | brid:BridgeRoom | 橋梁内部 | brid:interiorBridgeRoom |
| consistsOfBridgePart | brid:BridgePart | 橋梁部分 | brid:consistsOfBridgePart |
| address | core:Address | 住所 | brid:address |
| bridBaseAttribute | uro:ConstructionBaseAttribute | 構造物基本属性 | uro:bridBaseAttribute |
| bridDataQualityAttribute | uro:ConstructionDataQualityAttribute | データ品質 | uro:bridDataQualityAttribute |
| bridDisasterRiskAttribute | uro:DisasterRiskAttributeProperty | 災害リスク属性 | uro:bridDisasterRiskAttribute |
| bridDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:bridDmAttribute |
| bridFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:bridFacilityAttribute |
| bridFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:bridFacilityIdAttribute |
| bridFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:bridFacilityTypeAttribute |
| bridFunctionalAttribute | uro:BridgeFunctionalAttribute | 橋梁機能属性 | uro:bridFunctionalAttribute |
| bridRiskAssessmentAttribute | uro:ConstructionRiskAssessmentAttribute | 構造物リスク評価属性 | uro:bridRiskAssessmentAttribute |
| bridStructureAttribute | uro:BridgeStructureAttribute | 橋梁構造属性 | uro:bridStructureAttribute |

### brid:BridgeConstructionElement

橋梁建設部材

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |
| boundedBy | brid:_BoundarySurfaceProperty | 境界面 | brid:boundedBy |

### brid:BridgeFurniture

設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |

### brid:BridgeInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |
| boundedBy | brid:_BoundarySurfaceProperty | 境界面 | brid:boundedBy |

### brid:BridgePart

橋梁部分

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |
| yearOfConstruction | String | 建設年 | brid:yearOfConstruction |
| yearOfDemolition | String | 解体年 | brid:yearOfDemolition |
| isMovable | Boolean | 可動橋区分 | brid:isMovable |
| outerBridgeConstruction | brid:BridgeConstructionElement | 橋梁部材 | brid:outerBridgeConstruction |
| outerBridgeInstallation | brid:BridgeInstallation | 橋梁付属物 | brid:outerBridgeInstallation |
| interiorBridgeInstallation | brid:BridgeInstallation | 橋梁内部付属物 | brid:interiorBridgeInstallation |
| boundedBy | brid:_BoundarySurfaceProperty | 境界面 | brid:boundedBy |
| interiorBridgeRoom | brid:BridgeRoom | 橋梁内部 | brid:interiorBridgeRoom |
| consistsOfBridgePart | brid:BridgePart | 橋梁部分 | brid:consistsOfBridgePart |
| address | core:Address | 住所 | brid:address |
| bridBaseAttribute | uro:ConstructionBaseAttribute | 構造物基本属性 | uro:bridBaseAttribute |
| bridDataQualityAttribute | uro:ConstructionDataQualityAttribute | データ品質 | uro:bridDataQualityAttribute |
| bridDisasterRiskAttribute | uro:DisasterRiskAttributeProperty | 災害リスク属性 | uro:bridDisasterRiskAttribute |
| bridDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:bridDmAttribute |
| bridFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:bridFacilityAttribute |
| bridFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:bridFacilityIdAttribute |
| bridFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:bridFacilityTypeAttribute |
| bridFunctionalAttribute | uro:BridgeFunctionalAttribute | 機能属性 | uro:bridFunctionalAttribute |
| bridRiskAssessmentAttribute | uro:ConstructionRiskAssessmentAttribute | 構造物リスク評価属性 | uro:bridRiskAssessmentAttribute |
| bridStructureAttribute | uro:BridgeStructureAttribute | 構造属性 | uro:bridStructureAttribute |

### brid:BridgeRoom

内部空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | Code | 機能 | brid:function |
| usage | Code | 用途 | brid:usage |
| boundedBy | brid:_BoundarySurfaceProperty | 境界面 | brid:boundedBy |
| interiorFurniture | brid:BridgeFurniture | 設置物 | brid:interiorFurniture |
| bridgeRoomInstallation | brid:BridgeInstallation | 内部付属物 | brid:bridgeRoomInstallation |

### brid:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| address | core:Address |  | brid:address |

### brid:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | brid:_OpeningProperty | 開口部 | brid:opening |

### brid:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### tun:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### tun:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:HollowSpace

内部空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | Code | 機能 | tun:function |
| usage | Code | 用途 | tun:usage |
| boundedBy | tun:_BoundarySurfaceProperty | 境界面 | tun:boundedBy |
| interiorFurniture | tun:TunnelFurniture | 設置物 | tun:interiorFurniture |
| hollowSpaceInstallation | tun:TunnelInstallation | 内部付属物 | tun:hollowSpaceInstallation |

### tun:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:Tunnel

トンネル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | Code | 機能 | tun:function |
| usage | Code | 用途 | tun:usage |
| yearOfConstruction | String | 建設年 | tun:yearOfConstruction |
| yearOfDemolition | String | 解体年 | tun:yearOfDemolition |
| outerTunnelInstallation | tun:TunnelInstallation | トンネル付属物 | tun:outerTunnelInstallation |
| interiorTunnelInstallation | tun:TunnelInstallation | トンネル内部付属物 | tun:interiorTunnelInstallation |
| boundedBy | tun:_BoundarySurfaceProperty | 境界面 | tun:boundedBy |
| interiorHollowSpace | tun:HollowSpace | トンネル内部空間 | tun:interiorHollowSpace |
| consistsOfTunnelPart | tun:TunnelPart | トンネル部分 | tun:consistsOfTunnelPart |
| tunBaseAttribute | uro:ConstructionBaseAttribute | 構造物基本属性 | uro:tunBaseAttribute |
| tunDataQualityAttribute | uro:ConstructionDataQualityAttribute | データ品質 | uro:tunDataQualityAttribute |
| tunDisasterRiskAttribute | uro:DisasterRiskAttributeProperty | 災害リスク属性 | uro:tunDisasterRiskAttribute |
| tunDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tunDmAttribute |
| tunFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tunFacilityAttribute |
| tunFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tunFacilityIdAttribute |
| tunFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tunFacilityTypeAttribute |
| tunFunctionalAttribute | uro:TunnelFunctionalAttribute | トンネル機能属性 | uro:tunFunctionalAttribute |
| tunRiskAssessmentAttribute | uro:ConstructionRiskAssessmentAttribute | 構造物リスク評価属性 | uro:tunRiskAssessmentAttribute |
| tunStructureAttribute | uro:TunnelStructureAttribute | トンネル構造属性 | uro:tunStructureAttribute |

### tun:TunnelFurniture

設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | Code | 機能 | tun:function |
| usage | Code | 用途 | tun:usage |

### tun:TunnelInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | Code | 機能 | tun:function |
| usage | Code | 用途 | tun:usage |
| boundedBy | tun:_BoundarySurfaceProperty | 境界面 | tun:boundedBy |

### tun:TunnelPart


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | tun:class |
| function | Code |  | tun:function |
| usage | Code |  | tun:usage |
| yearOfConstruction | String |  | tun:yearOfConstruction |
| yearOfDemolition | String |  | tun:yearOfDemolition |
| outerTunnelInstallation | tun:TunnelInstallation |  | tun:outerTunnelInstallation |
| interiorTunnelInstallation | tun:TunnelInstallation |  | tun:interiorTunnelInstallation |
| boundedBy | tun:_BoundarySurfaceProperty |  | tun:boundedBy |
| interiorHollowSpace | tun:HollowSpace |  | tun:interiorHollowSpace |
| consistsOfTunnelPart | tun:TunnelPart |  | tun:consistsOfTunnelPart |
| tunBaseAttribute | uro:ConstructionBaseAttribute |  | uro:tunBaseAttribute |
| tunDataQualityAttribute | uro:ConstructionDataQualityAttribute |  | uro:tunDataQualityAttribute |
| tunDisasterRiskAttribute | uro:DisasterRiskAttributeProperty |  | uro:tunDisasterRiskAttribute |
| tunDmAttribute | uro:DmAttributeProperty |  | uro:tunDmAttribute |
| tunFacilityAttribute | uro:FacilityAttributeProperty |  | uro:tunFacilityAttribute |
| tunFacilityIdAttribute | uro:FacilityIdAttributeProperty |  | uro:tunFacilityIdAttribute |
| tunFacilityTypeAttribute | uro:FacilityTypeAttribute |  | uro:tunFacilityTypeAttribute |
| tunFunctionalAttribute | uro:TunnelFunctionalAttribute |  | uro:tunFunctionalAttribute |
| tunRiskAssessmentAttribute | uro:ConstructionRiskAssessmentAttribute |  | uro:tunRiskAssessmentAttribute |
| tunStructureAttribute | uro:TunnelStructureAttribute |  | uro:tunStructureAttribute |

### tun:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| opening | tun:_OpeningProperty | 開口部 | tun:opening |

### tun:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### frn:CityFurniture

都市設備

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |

### veg:PlantCover

植被

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| vegDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:vegDmAttribute |
| vegFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:vegFacilityAttribute |
| vegFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:vegFacilityIdAttribute |
| vegFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:vegFacilityTypeAttribute |
| vegetationDataQualityAttribute | uro:VegetationDataQualityAttribute | データ品質 | uro:vegetationDataQualityAttribute |
| class | Code | 分類 | veg:class |
| function | Code | 機能 | veg:function |
| usage | Code | 用途 | veg:usage |
| averageHeight | Measure | 平均高 | veg:averageHeight |

### veg:SolitaryVegetationObject

単独木

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| vegDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:vegDmAttribute |
| vegFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:vegFacilityAttribute |
| vegFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:vegFacilityIdAttribute |
| vegFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:vegFacilityTypeAttribute |
| vegetationDataQualityAttribute | uro:VegetationDataQualityAttribute | データ品質 | uro:vegetationDataQualityAttribute |
| class | Code | 分類 | veg:class |
| function | Code | 機能 | veg:function |
| usage | Code | 用途 | veg:usage |
| species | Code | 樹種 | veg:species |
| height | Measure | 樹高 | veg:height |
| trunkDiameter | Measure |  | veg:trunkDiameter |
| crownDiameter | Measure |  | veg:crownDiameter |

### wtr:WaterBody

洪水浸水想定区域、津波浸水想定、高潮浸水想定区域、内水浸水想定区域、海、潮汐水域、水路、河川/小川、湖、滝、湿地・沼地、浸水域、貯水池、不明

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | wtr:class |
| function | Code | 機能 | wtr:function |
| usage | Code | 用途 | wtr:usage |
| boundedBy | wtr:_WaterBoundarySurfaceProperty |  | wtr:boundedBy |
| floodingRiskAttribute | uro:WaterBodyFloodingRiskAttributeProperty | 災害リスク | uro:floodingRiskAttribute |
| waterBodyDetailAttribute | uro:WaterBodyDetailAttribute | 水部詳細属性 | uro:waterBodyDetailAttribute |
| wtrDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:wtrDmAttribute |
| wtrFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:wtrFacilityAttribute |
| wtrFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:wtrFacilityIdAttribute |
| wtrFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:wtrFacilityTypeAttribute |

### wtr:WaterClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### wtr:WaterGroundSurface

水底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### wtr:WaterSurface

水面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| waterLevel | Code |  | wtr:waterLevel |

### dem:BreaklineRelief

点群地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| demDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:demDmAttribute |

### dem:MassPointRelief

点群地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| demDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:demDmAttribute |

### dem:RasterRelief


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger |  | dem:lod |
| demDmAttribute | uro:DmAttributeProperty |  | uro:demDmAttribute |
| grid | gml:RectifiedGridCoverage |  | dem:grid |

### dem:ReliefFeature

地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| reliefComponent | dem:_ReliefComponentProperty | 地形構成要素 | dem:reliefComponent |

### dem:TINRelief

TIN地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| demDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:demDmAttribute |

### grp:CityObjectGroup


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | grp:class |
| function | Code |  | grp:function |
| usage | Code |  | grp:usage |
| groupMember | grp:_CityObjectOrRef |  | grp:groupMember |
| parent | grp:_CityObjectOrRef |  | grp:parent |
| fiscalYearOfPublication | String |  | uro:fiscalYearOfPublication |
| ifcBuildingStoreyAttribute | uro:IfcAttributeProperty |  | uro:ifcBuildingStoreyAttribute |
| indoorStoreyAttribute | uro:IndoorAttributeProperty |  | uro:indoorStoreyAttribute |
| language | Code |  | uro:language |

### grp:_CityObjectOrRef


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| href | String |  | @xlink:href |

### gen:GenericCityObject


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | gen:class |
| function | Code |  | gen:function |
| usage | Code |  | gen:usage |

### uro:Appurtenance

ユーティリティ設備

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| previousLink | String | 前のリンク | uro:previousLink |
| nextLink | String | 次のリンク | uro:nextLink |
| rotationAngle | Double | 回転角度 | uro:rotationAngle |
| appurtenanceType | Code | 設備区分 | uro:appurtenanceType |

### uro:Cable

ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |

### uro:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:ConstructionInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | uro:class |
| function | Code | 機能 | uro:function |
| usage | Code | 用途 | uro:usage |

### uro:Duct

トラフ、洞道、鞘管、CAB、情報BOX

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| width | Measure | 外側幅 | uro:width |

### uro:ElectricityCable

電力ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |

### uro:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:Handhole

ハンドホール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| containerType | Code | 構造物種類 | uro:containerType |
| innerDiamiterLong | Measure | 長辺の内径 | uro:innerDiamiterLong |
| outerDiamiterLong | Measure | 長辺の外径 | uro:outerDiamiterLong |
| innerDiamiterShort | Measure | 短辺の内径 | uro:innerDiamiterShort |
| outerDiamiterShort | Measure | 短辺の外径 | uro:outerDiamiterShort |
| depth | Measure | 深さ | uro:depth |
| appurtenance | String | 識別子 | uro:appurtenance |
| rotationAngle | Double | 回転角度 | uro:rotationAngle |

### uro:Manhole

マンホール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| containerType | Code | 構造物種類 | uro:containerType |
| innerDiamiterLong | Measure | 長辺の内径 | uro:innerDiamiterLong |
| outerDiamiterLong | Measure | 長辺の外径 | uro:outerDiamiterLong |
| innerDiamiterShort | Measure | 短辺の内径 | uro:innerDiamiterShort |
| outerDiamiterShort | Measure | 短辺の外径 | uro:outerDiamiterShort |
| depth | Measure | 深さ | uro:depth |
| appurtenance | String | 識別子 | uro:appurtenance |
| rotationAngle | Double | 回転角度 | uro:rotationAngle |

### uro:OilGasChemicalsPipe


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | frn:class |
| function | Code |  | frn:function |
| usage | Code |  | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute |  | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute |  | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty |  | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty |  | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty |  | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute |  | uro:frnFacilityTypeAttribute |
| occupierType | Code |  | uro:occupierType |
| occupierName | Code |  | uro:occupierName |
| year | String |  | uro:year |
| yearType | Code |  | uro:yearType |
| administrator | Code |  | uro:administrator |
| offsetDepth | uro:OffsetDepth |  | uro:offsetDepth |
| thematicShape | uro:ThematicShape |  | uro:thematicShape |
| routeStartNode | String |  | uro:routeStartNode |
| startNode | String |  | uro:startNode |
| routeEndNode | String |  | uro:routeEndNode |
| endNode | String |  | uro:endNode |
| depth | Measure |  | uro:depth |
| minDepth | Measure |  | uro:minDepth |
| maxDepth | Measure |  | uro:maxDepth |
| maxWidth | Measure |  | uro:maxWidth |
| offset | Measure |  | uro:offset |
| material | Code |  | uro:material |
| lengthAttribute | uro:LengthAttribute |  | uro:lengthAttribute |
| innerDiamiter | Measure |  | uro:innerDiamiter |
| outerDiamiter | Measure |  | uro:outerDiamiter |
| sleeveType | Code |  | uro:sleeveType |

### uro:OtherConstruction

その他の構造物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| conditionOfConstruction | String | 稼働状況 | uro:conditionOfConstruction |
| dateOfConstruction | Date | 完成年月日 | uro:dateOfConstruction |
| dateOfDemolition | Date | 解体年月日 | uro:dateOfDemolition |
| constructionEvent | uro:ConstructionEvent | 建設イベント | uro:constructionEvent |
| elevation | uro:Elevation | 標高 | uro:elevation |
| height | uro:Height | 高さ | uro:height |
| occupancy | uro:Occupancy | 占有状況 | uro:occupancy |
| consFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:consFacilityTypeAttribute |
| consFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:consFacilityIdAttribute |
| consFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:consFacilityAttribute |
| consBaseAttribute | uro:ConstructionBaseAttribute | 構造物基本情報 | uro:consBaseAttribute |
| consStructureAttribute | uro:ConstructionStructureAttributeProperty | 構造属性 | uro:consStructureAttribute |
| consDisasterRiskAttribute | uro:DisasterRiskAttributeProperty | 災害リスク属性 | uro:consDisasterRiskAttribute |
| consDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:consDmAttribute |
| consDataQualityAttribute | uro:ConstructionDataQualityAttribute | 品質属性 | uro:consDataQualityAttribute |
| boundedBy | uro:_BoundarySurfaceProperty | 境界面 | uro:boundedBy |
| constructionInstallation | uro:ConstructionInstallation | 付属物 | uro:constructionInstallation |
| class | Code | 分類 | uro:class |
| function | Code | 機能 | uro:function |
| usage | Code | 用途 | uro:usage |

### uro:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:Pipe

管きょ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| innerDiamiter | Measure | 内径 | uro:innerDiamiter |
| outerDiamiter | Measure | 外径 | uro:outerDiamiter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |

### uro:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:SewerPipe

下水道管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| innerDiamiter | Measure | 内径 | uro:innerDiamiter |
| outerDiamiter | Measure | 外径 | uro:outerDiamiter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |
| slope | Measure | 勾配 | uro:slope |

### uro:TelecommunicationsCable

通信ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |

### uro:ThermalPipe

熱配管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| innerDiamiter | Measure | 内径 | uro:innerDiamiter |
| outerDiamiter | Measure | 外径 | uro:outerDiamiter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |

### uro:UndergroundBuilding

地下街

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | Code | 機能 | bldg:function |
| usage | Code | 用途 | bldg:usage |
| yearOfConstruction | String | 建築年 | bldg:yearOfConstruction |
| yearOfDemolition | String | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measuredHeight | Measure | 計測高さ | bldg:measuredHeight |
| storeysAboveGround | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| storeysBelowGround | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| storeyHeightsAboveGround | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| storeyHeightsBelowGround | String |  | bldg:storeyHeightsBelowGround |
| outerBuildingInstallation | bldg:BuildingInstallation | 建物付属物 | bldg:outerBuildingInstallation |
| interiorBuildingInstallation | bldg:BuildingInstallation | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | bldg:_BoundarySurfaceProperty | 境界面 | bldg:boundedBy |
| interiorRoom | bldg:Room | 部屋 | bldg:interiorRoom |
| consistsOfBuildingPart | bldg:BuildingPart | 建物部品 | bldg:consistsOfBuildingPart |
| address | core:Address | 住所 | bldg:address |
| bldgDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:bldgDmAttribute |
| bldgFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:bldgFacilityAttribute |
| bldgFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:bldgFacilityIdAttribute |
| bldgFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:bldgFacilityTypeAttribute |
| bldgRealEstateIDAttribute | uro:RealEstateIDAttribute |  | uro:bldgRealEstateIDAttribute |
| buildingDataQualityAttribute | uro:BuildingDataQualityAttribute | データ品質 | uro:buildingDataQualityAttribute |
| buildingDetailAttribute | uro:BuildingDetailAttribute | 建物利用現況 | uro:buildingDetailAttribute |
| buildingDisasterRiskAttribute | uro:BuildingDisasterRiskAttributeProperty | 災害リスク | uro:buildingDisasterRiskAttribute |
| buildingIDAttribute | uro:BuildingIDAttribute | 建物識別情報 | uro:buildingIDAttribute |
| ifcBuildingAttribute | uro:IfcAttributeProperty | IFC属性 | uro:ifcBuildingAttribute |
| indoorBuildingAttribute | uro:IndoorAttributeProperty | 屋内ナビゲーション属性 | uro:indoorBuildingAttribute |
| keyValuePairAttribute | uro:KeyValuePairAttribute | 拡張属性 | uro:keyValuePairAttribute |
| largeCustomerFacilityAttribute | uro:LargeCustomerFacilityAttribute | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |

### uro:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |

### uro:WaterPipe

水道管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | Code | 機能 | frn:function |
| usage | Code | 用途 | frn:usage |
| cityFurnitureDataQualityAttribute | uro:CityFurnitureDataQualityAttribute | データ品質 | uro:cityFurnitureDataQualityAttribute |
| cityFurnitureDetailAttribute | uro:CityFurnitureDetailAttribute | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| frnDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:frnDmAttribute |
| frnFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:frnFacilityAttribute |
| frnFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設属性 | uro:frnFacilityIdAttribute |
| frnFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設識別属性 | uro:frnFacilityTypeAttribute |
| occupierType | Code | 事業者種類 | uro:occupierType |
| occupierName | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| administrator | Code | 主管事業者 | uro:administrator |
| offsetDepth | uro:OffsetDepth | オフセットデプス情報 | uro:offsetDepth |
| thematicShape | uro:ThematicShape | 主題図形 | uro:thematicShape |
| routeStartNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| routeEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| lengthAttribute | uro:LengthAttribute | 長さ情報 | uro:lengthAttribute |
| innerDiamiter | Measure | 内径 | uro:innerDiamiter |
| outerDiamiter | Measure | 外径 | uro:outerDiamiter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |

### uro:Waterway

航路

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| tranDmAttribute | uro:DmAttributeProperty | 図式属性 | uro:tranDmAttribute |
| class | Code | 分類 | tran:class |
| function | Code | 機能 | tran:function |
| usage | Code | 用途 | tran:usage |
| trafficArea | tran:TrafficArea | 交通領域 | tran:trafficArea |
| auxiliaryTrafficArea | tran:AuxiliaryTrafficArea | 交通補助領域 | tran:auxiliaryTrafficArea |
| tranDataQualityAttribute | uro:TransportationDataQualityAttribute | データ品質 | uro:tranDataQualityAttribute |
| tranFacilityAttribute | uro:FacilityAttributeProperty | 施設詳細属性 | uro:tranFacilityAttribute |
| tranFacilityIdAttribute | uro:FacilityIdAttributeProperty | 施設識別属性 | uro:tranFacilityIdAttribute |
| tranFacilityTypeAttribute | uro:FacilityTypeAttribute | 施設属性 | uro:tranFacilityTypeAttribute |
| waterwayDetailAttribute | uro:WaterwayDetailAttribute | 航路属性 | uro:waterwayDetailAttribute |

### urf:Agreement


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| applicableArea | Measure |  | urf:applicableArea |
| expiration | Date |  | urf:expiration |

### urf:AircraftNoiseControlZone

航空機騒音障害防止地区又は航空機騒音障害防止特別地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:AreaClassification

区域区分

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| population | Integer | 人口 | urf:population |

### urf:CollectiveFacilitiesForReconstruction

一団地の復興拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| housingFacilities | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| supecificBusinessFacilities | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| publicFacilities | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilityFacilities | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |

### urf:CollectiveFacilitiesForReconstructionAndRevitalization

一団地の復興再生拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| housingFacilities | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| supecificBusinessFacilities | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| publicFacilities | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilityFacilities | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |

### urf:CollectiveFacilitiesForTsunamiDisasterPrevention

一団地の津波防災拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| housingFacilities | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| supecificBusinessFacilities | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| publicFacilities | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilityFacilities | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |

### urf:CollectiveGovernmentAndPublicOfficeFacilities

一団地の官公庁施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| buildingCoverageRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| floorAreaRate | Double | 容積率の限度 | urf:floorAreaRate |
| publicFacilitiesAllocationPolicy | String | 公益的施設、住宅及び公共施設の配置方針 | urf:publicFacilitiesAllocationPolicy |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:CollectiveHousingFacilities

一団地の住宅施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| buildingCoverageRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| floorAreaRate | Double | 容積率の限度 | urf:floorAreaRate |
| numberOfLowRiseHousing | Integer | 低層住宅の予定戸数 | urf:numberOfLowRiseHousing |
| numberOfMiddleRiseHousing | Integer | 中層住宅の予定戸数 | urf:numberOfMiddleRiseHousing |
| numberOfHighRiseHousing | Integer | 高層住宅の予定戸数 | urf:numberOfHighRiseHousing |
| totalNumberOfHousing | Integer | 住宅予定数の合計 | urf:totalNumberOfHousing |
| publicFacilitiesAllocationPolicy | String | 住宅及び公共施設の配置方針 | urf:publicFacilitiesAllocationPolicy |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:CollectiveUrbanDisasterPreventionFacilities

一団地の都市安全確保拠点施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| specificUtilityAndPublicFacilities | String | 特定公益施設及び公共施設の位置及び規模 | urf:specificUtilityAndPublicFacilities |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低 | urf:minimumBuildingHeight |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |

### urf:ConservationZoneForClustersOfTraditionalStructures

伝統的建造物群保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:DisasterPreventionBlockImprovementProject

防災街区整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| disasterPreventionPublicFacilityAllocation | String | 防災公共施設の配置及び規模 | urf:disasterPreventionPublicFacilityAllocation |
| otherPublicFacilityAllocation | String | その他の公共施設の配置及び規模 | urf:otherPublicFacilityAllocation |
| developmentPlan | String | 防災施設建築物の整備に関する計画 | urf:developmentPlan |

### urf:DisasterPreventionBlockImprovementZonePlan

防災街区整備地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 方針 | urf:policy |
| districtDevelopmentPlan | urf:DistrictDevelopmentPlanProperty | 特定建築物地区整備計画及び防災街区整備地区整備計画 | urf:districtDevelopmentPlan |
| promotionDistrict | urf:PromotionDistrict | nan | urf:promotionDistrict |
| zonalDisasterPreventionFacilitiesAllocation | String | 地区防災施設の区域。 | urf:zonalDisasterPreventionFacilitiesAllocation |
| specifiedZonalDisasterPreventionFacilitiesAllocation | String | 特定地区防災施設の区域。 | urf:specifiedZonalDisasterPreventionFacilitiesAllocation |
| zonalDisasterPreventionFacilities | urf:ZonalDisasterPreventionFacility | 地区防災施設の区域及び特定地区防災施設 | urf:zonalDisasterPreventionFacilities |

### urf:DistributionBusinessPark

流通業務団地

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| distributionBusinessPark | String | 流通業務施設の敷地の位置及び規模 | urf:distributionBusinessPark |
| publicAndUtilityFacilities | String | 公共施設及び公益施設の位置及び規模 | urf:publicAndUtilityFacilities |
| buildingCoverageRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| floorAreaRate | Double | 容積率の限度 | urf:floorAreaRate |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:DistributionBusinessZone

流通業務地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| guidelinePublicationDate | Date | 基本方針が定められた日 | urf:guidelinePublicationDate |

### urf:District

地区（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| buildingRestrictions | String | 建築物に関する制限 | urf:buildingRestrictions |
| useRestrictions | String | 建築物の用途の制限 | urf:useRestrictions |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| minimumBuildingCoverageRate | Double | 建ぺい率の最低限度 | urf:minimumBuildingCoverageRate |
| minimumSiteArea | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| minimumBuildingArea | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| minimumGroundHeight | Measure |  | urf:minimumGroundHeight |
| setbackSize | String | 壁面の後退距離 | urf:setbackSize |
| structurePlacementRestrictions | String |  | urf:structurePlacementRestrictions |
| maximumBuildingHeight | Measure | 建築物の高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 建築物の高さの最低限度 | urf:minimumBuildingHeight |
| minimumFloorHeight | Measure | 床面の高さの最低限度 | urf:minimumFloorHeight |
| buildingDesignRestriction | String | 建築物の形態及び意匠にかかる制限 | urf:buildingDesignRestriction |
| minimumGreeningRate | Double | 最低限度の緑被率 | urf:minimumGreeningRate |
| fenceGuideline | String | 垣およびさくの構造にかかる制限 | urf:fenceGuideline |
| restrictionsForFireProtection | String | 防火上の必要な制限 | urf:restrictionsForFireProtection |
| restrictionsForNoiseProtection | String | 御盤上又は遮音上必要な制限 | urf:restrictionsForNoiseProtection |
| minimumFrontageRate | Double | 間口率の最低限度 | urf:minimumFrontageRate |

### urf:DistrictDevelopmentPlan

地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号 | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 地区 | urf:district |

### urf:DistrictFacility

地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |

### urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan

防災街区整備地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 防災地区 | urf:district |

### urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict

歴史的風致維持向上地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 地区 | urf:district |

### urf:DistrictPlan

地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| districtDevelopmentPlan | urf:DistrictDevelopmentPlanProperty | 地区整備計画 | urf:districtDevelopmentPlan |
| promotionDistrict | urf:PromotionDistrict | 促進区 | urf:promotionDistrict |
| facilityAllocation | String | 施設の配置及び方針 | urf:facilityAllocation |
| landUsePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |

### urf:DistrictsAndZones

地域地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:EducationalAndCulturalFacility

教育文化施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |

### urf:ExceptionalFloorAreaRateDistrict

特例容積率適用地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| buildingHeightLimits | Measure | 建築物の高さの最高限度 | urf:buildingHeightLimits |

### urf:FirePreventionDistrict

防火地域又は準防火地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 防火地域又は準防火地域の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:FireProtectionFacility

防火施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:FloodPreventionFacility

防水施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:GlobalHubCityDevelopmentProject


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| implementationBody | String |  | urf:implementationBody |
| implementationPeriod | String |  | urf:implementationPeriod |
| plan | String |  | urf:plan |

### urf:GreenSpaceConservationDistrict

緑地保全地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:HeightControlDistrict

高度地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |

### urf:HighLevelUseDistrict

高度利用地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| minimumBuildingArea | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |

### urf:HighRiseResidentialAttractionDistrict

高層住居誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| floorAreaRate | Double | 容積率 | urf:floorAreaRate |
| maximumBuildingCoverageRate | Double | 建蔽率 | urf:maximumBuildingCoverageRate |
| minimumSiteArea | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |

### urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan

歴史的風致維持向上地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 方針 | urf:policy |
| districtDevelopmentPlan | urf:DistrictDevelopmentPlanProperty | 地区整備計画 | urf:districtDevelopmentPlan |
| promotionDistrict | urf:PromotionDistrict | 促進区 | urf:promotionDistrict |
| landUsePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |

### urf:HousingControlArea

居住調整地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:IndustrialParkDevelopmentProject

工業団地造成事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| publicFacilityAllocation | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| residentialLandUsePlan | String | 宅地の利用計画 | urf:residentialLandUsePlan |

### urf:LandReadjustmentProject

土地区画整理事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| publicFacilityAllocation | String | 公共施設の配置 | urf:publicFacilityAllocation |
| buildingLotDevelopment | String | 宅地の整備に関する事項 | urf:buildingLotDevelopment |

### urf:LandReadjustmentPromotionArea

土地区画整理促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| developmentPolicy | String | 住宅市街地としての開発方針 | urf:developmentPolicy |
| publicFacilitiesPlans | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |

### urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment

拠点業務市街地整備土地区画整理促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| developmentPolicy | String | 開発の方針 | urf:developmentPolicy |
| publicFacilitiesPlans | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |

### urf:LandscapeZone

景観地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| buildingDesignRestriction | String | 建築物の形態にかかる制限 | urf:buildingDesignRestriction |
| maximumBuildingHeight | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |
| minimumSiteArea | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |

### urf:MarketsSlaughterhousesCrematoria

市場、と畜場、火葬場（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |

### urf:MedicalFacility

医療施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |

### urf:NewHousingAndUrbanDevelopmentProject

新住宅市街地開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| housing | String | 住区 | urf:housing |
| publicFacilityAllocation | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| residentialLandUsePlan | String | 宅地の利用計画 | urf:residentialLandUsePlan |

### urf:NewUrbanInfrastructureProject

新都市基盤整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| landForCentralPublicFacilities | String | 公共施設の用に供するべき土地の区域 | urf:landForCentralPublicFacilities |
| districtsAllocation | String | 開発誘導地区の配置及び規模 | urf:districtsAllocation |
| landUsePlan | String | 土地の利用計画 | urf:landUsePlan |

### urf:OpenSpaceForPublicUse

公共空地（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| parkAttribute | urf:ParkAttribute | 公園属性 | urf:parkAttribute |

### urf:ParkingPlaceDevelopmentZone

駐車場整備地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:PortZone

臨港地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 分区 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| floorAreaRate | Double | 容積率 | urf:floorAreaRate |

### urf:PrivateUrbanRenewalProjectPlan


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| developer | String |  | urf:developer |
| plan | String |  | urf:plan |

### urf:ProductiveGreenZone

生産緑地地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| zoneNumber | String | 生産緑地区番号 | urf:zoneNumber |
| specification | Code | 特定生産緑地指定の有無 | urf:specification |

### urf:ProjectPromotionArea

促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| developmentPolicy | String | 開発の方針 | urf:developmentPolicy |
| publicFacilitiesPlans | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |

### urf:PromotionDistrict


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:QuasiUrbanPlanningArea

準都市計画区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積（全域） | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| population | Integer | 準都市計画区域内の総人口 | urf:population |
| cityArea | Measure | 準都市計画区域面積（市区町村内） | urf:cityArea |
| cityPopulation | Integer | 準都市計画区域内の人口（市区町村内） | urf:cityPopulation |

### urf:Regulation


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:ResidenceAttractionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:ResidentialBlockConstructionProject

住宅街区整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| publicFacilityAllocation | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| developmentPlan | String | 施設住宅の建設に関する計画 | urf:developmentPlan |
| siteArea | Measure | 敷地面積 | urf:siteArea |
| totalFloorArea | Measure | 延床面積 | urf:totalFloorArea |

### urf:ResidentialBlockConstructionPromotionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| developmentPolicy | String |  | urf:developmentPolicy |
| publicFacilitiesPlans | String |  | urf:publicFacilitiesPlans |

### urf:ResidentialEnvironmentImprovementDistrict

居住環境向上用途誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| useToBeInduced | String | 誘導すべき用途 | urf:useToBeInduced |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| maximumBuildingHeight | String | 高さの最高限度 | urf:maximumBuildingHeight |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |

### urf:RoadsideDistrictFacility

沿道地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |

### urf:RoadsideDistrictImprovementPlan

沿道地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 沿道地区 | urf:district |
| roadsideDistrictFacilitiesAllocation | String | 沿道地区施設の配置及び規模 | urf:roadsideDistrictFacilitiesAllocation |

### urf:RoadsideDistrictPlan

沿道地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| districtDevelopmentPlan | urf:DistrictDevelopmentPlanProperty | 沿道地区整備計画 | urf:districtDevelopmentPlan |
| promotionDistrict | urf:PromotionDistrict | 沿道開発等促進区 | urf:promotionDistrict |
| facilitiesAllocation | String |  | urf:facilitiesAllocation |
| landUsePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |

### urf:RuralDistrictFacility

集落地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |

### urf:RuralDistrictImprovementPlan

集落地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 集落地区 | urf:district |
| ruralDistrictFacilitiesAllocation | String | 集落地区施設の配置及び規模 | urf:ruralDistrictFacilitiesAllocation |

### urf:RuralDistrictPlan

集落地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| districtDevelopmentPlan | urf:DistrictDevelopmentPlanProperty | 集落地区整備計画 | urf:districtDevelopmentPlan |
| promotionDistrict | urf:PromotionDistrict | 促進区 | urf:promotionDistrict |

### urf:SandControlFacility

防砂施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:ScenicDistrict

風致地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 風致地区の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| buildingCoverageRate | Double | 建蔽率 | urf:buildingCoverageRate |
| buildingHeightLimits | Measure | 高さの規制 | urf:buildingHeightLimits |
| wallSetbackDistanceWithRoad | Measure | 壁面から敷地境界までの距離（道路に接する部分） | urf:wallSetbackDistanceWithRoad |
| wallSetbackDistanceWithAdjoiningLand | Measure | 壁面から敷地境界までの距離（道路に接しない部分） | urf:wallSetbackDistanceWithAdjoiningLand |

### urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities

一団地の官公庁施設の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForCollectiveHousingFacilities

一団地の住宅施設の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForDistributionBusinessPark

流通業務団地の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForIndustrialParkDevelopmentProjects

工業団地造成事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects

新住宅市街地開発事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForNewUrbanInfrastructureProjects

新都市基盤整備事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:ScheduledAreaForUrbanDevelopmentProject

予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 予定区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:SedimentDisasterProneArea

土砂災害警戒区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日 | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号 | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 所在地 | urf:location |
| disasterType | Code | 災害種別 | urf:disasterType |
| areaType | Code | 区域区分 | urf:areaType |
| zoneNumber | String | 区域番号 | urf:zoneNumber |
| zoneName | String | 区域名 | urf:zoneName |
| status | Code | 特別警戒未指定フラグ | urf:status |

### urf:SnowProtectionFacility

防雪施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:SocialWelfareFacility

社会福祉施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |

### urf:SpecialGreenSpaceConservationDistrict

特別緑地保全地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| requirement | Code | 指定の要件 | urf:requirement |

### urf:SpecialUrbanRenaissanceDistrict

都市再生特別地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| useToBeInduced | String | 誘導すべき用途 | urf:useToBeInduced |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maximumBuildingCoverageRate | Double | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| minimumBuildingArea | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| maximumBuildingHeight | String | 高さの最高限度 | urf:maximumBuildingHeight |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |

### urf:SpecialUseAttractionDistrict

特定用途誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| useToBeInduced | String | 誘導すべき用途 | urf:useToBeInduced |
| maximumFloorAreaRate | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minimumFloorAreaRate | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| minimumBuildingArea | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| maximumBuildingHeight | String | 高さの最高限度 | urf:maximumBuildingHeight |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |

### urf:SpecialUseDistrict

特別用途地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 特別用途地区の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| buildingRestrictions | String | 建築物の制限 | urf:buildingRestrictions |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |

### urf:SpecialUseRestrictionDistrict

特定用途制限地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| buildingRestrictions | String | 建築物の制限 | urf:buildingRestrictions |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |

### urf:SpecialZoneForPreservationOfHistoricalLandscape

歴史的風土特別保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

### urf:SpecifiedBlock

特定街区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| floorAreaRate | Double | 容積率 | urf:floorAreaRate |
| maximumBuildingHeight | Measure | 建築物の高さの最高限度 | urf:maximumBuildingHeight |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |

### urf:SpecifiedBuildingZoneImprovementPlan

特定建築物地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地区整備計画の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| districtFacilitiesAllocation | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| buildingRestrictions | String | 建築物等の制限 | urf:buildingRestrictions |
| urbanGreenSpaceConservation | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| activityRestrictionInFarmland | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| landuseRestrictions | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| districtFacility | urf:DistrictFacilityProperty | 地区施設 | urf:districtFacility |
| district | urf:District | 防災地区 | urf:district |

### urf:SpecifiedDisasterPreventionBlockImprovementZone

特定防災街区整備地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| minimumSiteArea | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| setbackSize | String | 外壁の後退距離 | urf:setbackSize |
| minimumFrontageRate | Double | 間口率の最低限度 | urf:minimumFrontageRate |
| minimumBuildingHeight | Measure | 高さの最低限度 | urf:minimumBuildingHeight |

### urf:SpecifiedUrgentUrbanRenewalArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| developmentPolicy | String |  | urf:developmentPolicy |
| privateProject | urf:PrivateUrbanRenewalProjectPlan |  | urf:privateProject |
| specifiedArea | urf:SpecifiedUrgentUrbanRenewalArea |  | urf:specifiedArea |
| specialDistrict | urf:SpecialUrbanRenaissanceDistrict |  | urf:specialDistrict |
| developmentProject | urf:GlobalHubCityDevelopmentProject |  | urf:developmentProject |

### urf:SupplyFacility

供給施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| waterWorksAttribute | urf:WaterWorksAttribute | 水道属性 | urf:waterWorksAttribute |

### urf:TelecommunicationFacility

電気通信施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:ThreeDimensionalExtent

立体的な範囲

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 機能 | urf:function |
| usage | Code | 対象となる都市施設の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| minimumDistance | Measure | 離隔距離の最小限度 | urf:minimumDistance |
| maximumLoad | Measure | 載荷重の最大限度 | urf:maximumLoad |

### urf:TideFacility

防潮施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:TrafficFacility

交通施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| startLocation | String | 起点 | urf:startLocation |
| endLocation | String | 終点 | urf:endLocation |
| viaLocations | String | 経過地 | urf:viaLocations |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| urbanRoadAttribute | urf:UrbanRoadAttribute | 道路属性 | urf:urbanRoadAttribute |
| urbanRapidTransitRailroadAttribute | urf:UrbanRapidTransitRailroadAttribute | 都市高速鉄道属性 | urf:urbanRapidTransitRailroadAttribute |
| parkingPlaceAttribute | urf:ParkingPlaceAttribute | 駐車場属性 | urf:parkingPlaceAttribute |
| vehicleTerminalAttribute | urf:VehicleTerminalAttribute | 自動車ターミナル属性 | urf:vehicleTerminalAttribute |

### urf:TreatmentFacility

処理施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| sewerSystemsAttribute | urf:SewerSystemAttribute |  | urf:sewerSystemsAttribute |

### urf:TreePlantingDistrict

緑化地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| minimumGreeningRate | Double |  | urf:minimumGreeningRate |

### urf:UnclassifiedBlankArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:UnclassifiedUseDistrict


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:UnusedLandUsePromotionArea

遊休土地転換利用促進地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |

### urf:UrbanDevelopmentProject

市街地開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |

### urf:UrbanDisasterRecoveryPromotionArea

被災市街地復興推進地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| expirationDate | Date | 期間満了の日 | urf:expirationDate |
| emergencyRecoveryPolicy | String | 市街地の整備改善の方針 | urf:emergencyRecoveryPolicy |
| plannedProjectType | Code | 事業の種類 | urf:plannedProjectType |

### urf:UrbanFacility

都市施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |

### urf:UrbanFacilityStipulatedByCabinetOrder

政令で定める都市施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:UrbanFunctionAttractionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |

### urf:UrbanPlanningArea

都市計画区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaClassification | Code | 区域区分の決定の有無 | urf:areaClassification |
| reasonForAreaClassification | String | 区域区分を決定する理由又はしない理由 | urf:reasonForAreaClassification |
| policyForAreaClassification | String | 区域区分の決定方針 | urf:policyForAreaClassification |
| purposeForUrbanPlan | String | 目標 | urf:purposeForUrbanPlan |
| policyForUrbanPlanDecision | String | 土地利用、都市施設の整備及び市街地開発事業に関する主要な都市計画の決定の方針 | urf:policyForUrbanPlanDecision |
| population | Integer | 都市計画区域内の総人口 | urf:population |
| cityArea | Measure | 都市計画区域面積（市区町村内） | urf:cityArea |
| cityPopulation | Integer | 都市計画区域内の人口（市区町村内） | urf:cityPopulation |

### urf:UrbanRedevelopmentProject

市街地再開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| publicFacilityAllocation | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| developmentPlan | String | 建築物及び建築敷地の整備計画 | urf:developmentPlan |
| housingTarget | String | 整備されるべき住宅の個数その他住宅建設の目標 | urf:housingTarget |
| siteArea | Measure | 住宅の敷地面積 | urf:siteArea |
| totalFloorArea | Measure | 住宅の延べ床面積 | urf:totalFloorArea |
| numberOfHousing | Integer | 住宅の個数 | urf:numberOfHousing |

### urf:UrbanRedevelopmentPromotionArea

市街地再開発促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| developmentPolicy | String | 開発の方針 | urf:developmentPolicy |
| publicFacilitiesPlans | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |
| publicFacilities | String | 公共施設の配置及び規模 | urf:publicFacilities |
| unitArea | String | 整備区の単位 | urf:unitArea |

### urf:UrbanRenewalProject

市街地改造事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 市街地開発事業の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| scheduledExecutor | String | 施行予定者 | urf:scheduledExecutor |
| storeysAboveGround | NonNegativeInteger | 建築物の地上階数 | urf:storeysAboveGround |
| storeysBelowGround | NonNegativeInteger | 建築物の地下階数 | urf:storeysBelowGround |
| setbackSize | String | 壁面の位置の限度 | urf:setbackSize |
| floorAreaRate | Double | 容積率の限度 | urf:floorAreaRate |
| buildingUsage | String | 主な用途 | urf:buildingUsage |
| siteArea | Measure | 建築敷地面積 | urf:siteArea |

### urf:UrgentUrbanRenewalArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| developmentPolicy | String |  | urf:developmentPolicy |
| privateProject | urf:PrivateUrbanRenewalProjectPlan |  | urf:privateProject |
| specifiedArea | urf:SpecifiedUrgentUrbanRenewalArea |  | urf:specifiedArea |
| specialDistrict | urf:SpecialUrbanRenaissanceDistrict |  | urf:specialDistrict |

### urf:UseDistrict

用途地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 用途地域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |
| floorAreaRate | Double | 容積率 | urf:floorAreaRate |
| minimumSiteArea | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| buildingCoverageRate | Double | 建蔽率 | urf:buildingCoverageRate |
| wallSetbackDistance | String | 外壁の後退距離 | urf:wallSetbackDistance |
| buildingHeightLimits | Measure | 建築物の高さの限度 | urf:buildingHeightLimits |
| buildingRestrictions | String | 建築物の制限 | urf:buildingRestrictions |
| otherRestrictions | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |
| setbackRestrictions | String | 建築物の各部分の高さの制限 | urf:setbackRestrictions |
| frontRoadRestrictions | String | 道路斜線制限 | urf:frontRoadRestrictions |
| adjacentLandRestrictions | String | 隣地斜線制限 | urf:adjacentLandRestrictions |
| northDirectionRestrictions | String | 北側斜線制限 | urf:northDirectionRestrictions |
| shadeRegulation | String | 日影規制 | urf:shadeRegulation |

### urf:Waterway


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String |  | gml:description |
| name | Code |  | gml:name |
| creationDate | Date |  | core:creationDate |
| terminationDate | Date |  | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | Code |  | urf:function |
| usage | Code |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFromType | Code |  | urf:validFromType |
| enactmentFiscalYear | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToType | Code |  | urf:validToType |
| expirationFiscalYear | String |  | urf:expirationFiscalYear |
| legalGrounds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notificationNumber | String |  | urf:notificationNumber |
| finalNotificationDate | Date |  | urf:finalNotificationDate |
| finalNotificationNumber | String |  | urf:finalNotificationNumber |
| urbanPlanType | Code |  | urf:urbanPlanType |
| areaClassificationType | Code |  | urf:areaClassificationType |
| nominalArea | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| boundary | urf:Boundary |  | urf:boundary |
| location | String |  | urf:location |
| number | String |  | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent |  | urf:threeDimensionalExtent |
| startLocation | String |  | urf:startLocation |
| endLocation | String |  | urf:endLocation |
| structure | Code |  | urf:structure |
| length | Measure |  | urf:length |
| width | Measure |  | urf:width |

### urf:WindProtectionFacility

防風施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 都市施設の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| number | String | 番号 | urf:number |
| threeDimensionalExtent | urf:ThreeDimensionalExtent | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |

### urf:ZonalDisasterPreventionFacility

地区防災施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 機能 | urf:function |
| usage | Code | 地区防災施設の用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| facilityType | Code | 地区防災施設の種類 | urf:facilityType |

### urf:Zone

区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 区域の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日 | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号 | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String |  | urf:location |

### urf:ZoneForPreservationOfHistoricalLandscape

第一種歴史的風土保存地区又は第二種歴史的風土保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | String | 説明 | gml:description |
| name | Code | 名称 | gml:name |
| creationDate | Date | 作成日 | core:creationDate |
| terminationDate | Date | 消滅日 | core:terminationDate |
| genericAttribute | gen:genericAttribute | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | Code | 地域地区の種類 | urf:function |
| usage | Code | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFromType | Code | 効力を生じる日の区分 | urf:validFromType |
| enactmentFiscalYear | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToType | Code | 効力を失う日の区分 | urf:validToType |
| expirationFiscalYear | String | 廃止年度 | urf:expirationFiscalYear |
| legalGrounds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notificationNumber | String | 告示番号（当初） | urf:notificationNumber |
| finalNotificationDate | Date | 告示日（最終） | urf:finalNotificationDate |
| finalNotificationNumber | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbanPlanType | Code | 都市計画区域 | urf:urbanPlanType |
| areaClassificationType | Code | 区域区分 | urf:areaClassificationType |
| nominalArea | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| boundary | urf:Boundary | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| areaInTotal | Measure | 面積（合計） | urf:areaInTotal |

## プロパティ (Property stereotype)

### bldg:_BoundarySurfaceProperty

以下のいずれかの型の値をとる：

- <a href='#bldgceilingsurface'>bldg:CeilingSurface</a>
- <a href='#bldgclosuresurface'>bldg:ClosureSurface</a>
- <a href='#bldgfloorsurface'>bldg:FloorSurface</a>
- <a href='#bldggroundsurface'>bldg:GroundSurface</a>
- <a href='#bldginteriorwallsurface'>bldg:InteriorWallSurface</a>
- <a href='#bldgouterceilingsurface'>bldg:OuterCeilingSurface</a>
- <a href='#bldgouterfloorsurface'>bldg:OuterFloorSurface</a>
- <a href='#bldgroofsurface'>bldg:RoofSurface</a>
- <a href='#bldgwallsurface'>bldg:WallSurface</a>

### bldg:_OpeningProperty

以下のいずれかの型の値をとる：

- <a href='#bldgdoor'>bldg:Door</a>
- <a href='#bldgwindow'>bldg:Window</a>

### brid:_BoundarySurfaceProperty

以下のいずれかの型の値をとる：

- <a href='#bridceilingsurface'>brid:CeilingSurface</a>
- <a href='#bridclosuresurface'>brid:ClosureSurface</a>
- <a href='#bridfloorsurface'>brid:FloorSurface</a>
- <a href='#bridgroundsurface'>brid:GroundSurface</a>
- <a href='#bridinteriorwallsurface'>brid:InteriorWallSurface</a>
- <a href='#bridouterceilingsurface'>brid:OuterCeilingSurface</a>
- <a href='#bridouterfloorsurface'>brid:OuterFloorSurface</a>
- <a href='#bridroofsurface'>brid:RoofSurface</a>
- <a href='#bridwallsurface'>brid:WallSurface</a>

### brid:_OpeningProperty

以下のいずれかの型の値をとる：

- <a href='#briddoor'>brid:Door</a>
- <a href='#bridwindow'>brid:Window</a>

### tun:_BoundarySurfaceProperty

以下のいずれかの型の値をとる：

- <a href='#tunceilingsurface'>tun:CeilingSurface</a>
- <a href='#tunclosuresurface'>tun:ClosureSurface</a>
- <a href='#tunfloorsurface'>tun:FloorSurface</a>
- <a href='#tungroundsurface'>tun:GroundSurface</a>
- <a href='#tuninteriorwallsurface'>tun:InteriorWallSurface</a>
- <a href='#tunouterceilingsurface'>tun:OuterCeilingSurface</a>
- <a href='#tunouterfloorsurface'>tun:OuterFloorSurface</a>
- <a href='#tunroofsurface'>tun:RoofSurface</a>
- <a href='#tunwallsurface'>tun:WallSurface</a>

### tun:_OpeningProperty

以下のいずれかの型の値をとる：

- <a href='#tundoor'>tun:Door</a>
- <a href='#tunwindow'>tun:Window</a>

### wtr:_WaterBoundarySurfaceProperty

以下のいずれかの型の値をとる：

- <a href='#wtrwaterclosuresurface'>wtr:WaterClosureSurface</a>
- <a href='#wtrwatergroundsurface'>wtr:WaterGroundSurface</a>
- <a href='#wtrwatersurface'>wtr:WaterSurface</a>

### dem:_ReliefComponentProperty

以下のいずれかの型の値をとる：

- <a href='#dembreaklinerelief'>dem:BreaklineRelief</a>
- <a href='#demmasspointrelief'>dem:MassPointRelief</a>
- <a href='#demrasterrelief'>dem:RasterRelief</a>
- <a href='#demtinrelief'>dem:TINRelief</a>

### uro:BuildingDisasterRiskAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urobuildinghightideriskattribute'>uro:BuildingHighTideRiskAttribute</a>
- <a href='#urobuildinginlandfloodingriskattribute'>uro:BuildingInlandFloodingRiskAttribute</a>
- <a href='#urobuildinglandslideriskattribute'>uro:BuildingLandSlideRiskAttribute</a>
- <a href='#urobuildingriverfloodingriskattribute'>uro:BuildingRiverFloodingRiskAttribute</a>
- <a href='#urobuildingtsunamiriskattribute'>uro:BuildingTsunamiRiskAttribute</a>

### uro:ConstructionStructureAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#uroconstructionstructureattribute'>uro:ConstructionStructureAttribute</a>
- <a href='#urodamattribute'>uro:DamAttribute</a>
- <a href='#uroembankmentattribute'>uro:EmbankmentAttribute</a>

### uro:ControlPointType

以下のいずれかの型の値をとる：

- <a href='#urocircularcurvetype'>uro:CircularCurveType</a>
- <a href='#urotransitioncurvetype'>uro:TransitionCurveType</a>
- <a href='#uroslopetype'>uro:SlopeType</a>
- <a href='#uroverticalcurvetype'>uro:VerticalCurveType</a>

### uro:DisasterRiskAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urohightideriskattribute'>uro:HighTideRiskAttribute</a>
- <a href='#uroinlandfloodingriskattribute'>uro:InlandFloodingRiskAttribute</a>
- <a href='#urolandslideriskattribute'>uro:LandSlideRiskAttribute</a>
- <a href='#uroriverfloodingriskattribute'>uro:RiverFloodingRiskAttribute</a>
- <a href='#urotsunamiriskattribute'>uro:TsunamiRiskAttribute</a>

### uro:DmAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urodmannotation'>uro:DmAnnotation</a>
- <a href='#urodmgeometricattribute'>uro:DmGeometricAttribute</a>

### uro:FacilityAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urocargohandlingfacility'>uro:CargoHandlingFacility</a>
- <a href='#urocyberportmarinaandpbs'>uro:CyberportMarinaAndPBS</a>
- <a href='#urofishingportcapacity'>uro:FishingPortCapacity</a>
- <a href='#urofishingportfacility'>uro:FishingPortFacility</a>
- <a href='#uroharborfacility'>uro:HarborFacility</a>
- <a href='#uromaintenancehistoryattribute'>uro:MaintenanceHistoryAttribute</a>
- <a href='#uromooringfacility'>uro:MooringFacility</a>
- <a href='#uronavigationassistancefacility'>uro:NavigationAssistanceFacility</a>
- <a href='#uroportenvironmentalimprovementfacility'>uro:PortEnvironmentalImprovementFacility</a>
- <a href='#uroportmanagementfacility'>uro:PortManagementFacility</a>
- <a href='#uroportpassengerfacility'>uro:PortPassengerFacility</a>
- <a href='#uroportpollutioncontrolfacility'>uro:PortPollutionControlFacility</a>
- <a href='#uroportprotectivefacility'>uro:PortProtectiveFacility</a>
- <a href='#uroportstoragefacility'>uro:PortStorageFacility</a>
- <a href='#uroporttransportationfacility'>uro:PortTransportationFacility</a>
- <a href='#uroportwastetreatmentfacility'>uro:PortWasteTreatmentFacility</a>
- <a href='#uroportwelfarefacility'>uro:PortWelfareFacility</a>
- <a href='#uroshipservicefacility'>uro:ShipServiceFacility</a>

### uro:FacilityIdAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urofacilityidattribute'>uro:FacilityIdAttribute</a>
- <a href='#uroriverfacilityidattribute'>uro:RiverFacilityIdAttribute</a>

### uro:IfcAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#uroifcbuilding'>uro:IfcBuilding</a>
- <a href='#uroifcbuildingelement'>uro:IfcBuildingElement</a>
- <a href='#uroifcbuildingstorey'>uro:IfcBuildingStorey</a>
- <a href='#uroifcclassificationreference'>uro:IfcClassificationReference</a>
- <a href='#uroifccoordinatereferencesystem'>uro:IfcCoordinateReferenceSystem</a>
- <a href='#uroifccurtainwall'>uro:IfcCurtainWall</a>
- <a href='#uroifcdoor'>uro:IfcDoor</a>
- <a href='#uroifcfurnishingelement'>uro:IfcFurnishingElement</a>
- <a href='#uroifcgroup'>uro:IfcGroup</a>
- <a href='#uroifcmapconversion'>uro:IfcMapConversion</a>
- <a href='#uroifcopeningelement'>uro:IfcOpeningElement</a>
- <a href='#uroifcproject'>uro:IfcProject</a>
- <a href='#uroifcprojectedcrs'>uro:IfcProjectedCRS</a>
- <a href='#uroifcpsetbuildingcommon'>uro:IfcPsetBuildingCommon</a>
- <a href='#uroifcpsetdoorcommon'>uro:IfcPsetDoorCommon</a>
- <a href='#uroifcpsetopeningelementcommon'>uro:IfcPsetOpeningElementCommon</a>
- <a href='#uroifcpsetsitecommon'>uro:IfcPsetSiteCommon</a>
- <a href='#uroifcpsetspacecommon'>uro:IfcPsetSpaceCommon</a>
- <a href='#uroifcpsetwindowcommon'>uro:IfcPsetWindowCommon</a>
- <a href='#uroifcroof'>uro:IfcRoof</a>
- <a href='#uroifcsite'>uro:IfcSite</a>
- <a href='#uroifcspace'>uro:IfcSpace</a>
- <a href='#uroifcspacebasequantity'>uro:IfcSpaceBaseQuantity</a>
- <a href='#uroifcwall'>uro:IfcWall</a>
- <a href='#uroifcwallstandardcase'>uro:IfcWallStandardCase</a>
- <a href='#uroifcwindow'>uro:IfcWindow</a>
- <a href='#uroifczone'>uro:IfcZone</a>

### uro:IfcCoordinateReferenceSystemProperty

以下のいずれかの型の値をとる：

- <a href='#uroifccoordinatereferencesystem'>uro:IfcCoordinateReferenceSystem</a>
- <a href='#uroifcprojectedcrs'>uro:IfcProjectedCRS</a>

### uro:IfcCoordinateReferenceSystemSelectType

以下のいずれかの型の値をとる：

- <a href='#uroifccoordinatereferencesystemproperty'>uro:IfcCoordinateReferenceSystemProperty</a>
- <a href='#uroifcgeometricrepresentationcontext'>uro:IfcGeometricRepresentationContext</a>

### uro:IndoorAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#uroindoorfacilityattribute'>uro:IndoorFacilityAttribute</a>
- <a href='#uroindoorfurnishingattribute'>uro:IndoorFurnishingAttribute</a>
- <a href='#uroindoorpublictagattribute'>uro:IndoorPublicTagAttribute</a>
- <a href='#uroindoorspaceattribute'>uro:IndoorSpaceAttribute</a>
- <a href='#uroindoorstoreyattribute'>uro:IndoorStoreyAttribute</a>
- <a href='#uroindoortacatiletileattribute'>uro:IndoorTacatileTileAttribute</a>
- <a href='#uroindooruserdefinedattribute'>uro:IndoorUserDefinedAttribute</a>
- <a href='#uroindoorzoneattribute'>uro:IndoorZoneAttribute</a>

### uro:SquareUrbanPlanAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urosquareurbanplanattribute'>uro:SquareUrbanPlanAttribute</a>
- <a href='#urostationsquareattribute'>uro:StationSquareAttribute</a>
- <a href='#uroterminalattribute'>uro:TerminalAttribute</a>

### uro:WaterBodyFloodingRiskAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urowaterbodyhightideriskattribute'>uro:WaterBodyHighTideRiskAttribute</a>
- <a href='#urowaterbodyinlandfloodingriskattribute'>uro:WaterBodyInlandFloodingRiskAttribute</a>
- <a href='#urowaterbodyriverfloodingriskattribute'>uro:WaterBodyRiverFloodingRiskAttribute</a>
- <a href='#urowaterbodytsunamiriskattribute'>uro:WaterBodyTsunamiRiskAttribute</a>

### uro:_BoundarySurfaceProperty

以下のいずれかの型の値をとる：

- <a href='#uroclosuresurface'>uro:ClosureSurface</a>
- <a href='#urogroundsurface'>uro:GroundSurface</a>
- <a href='#uroouterceilingsurface'>uro:OuterCeilingSurface</a>
- <a href='#uroouterfloorsurface'>uro:OuterFloorSurface</a>
- <a href='#uroroofsurface'>uro:RoofSurface</a>
- <a href='#urowallsurface'>uro:WallSurface</a>

### urf:DistrictDevelopmentPlanProperty

以下のいずれかの型の値をとる：

- <a href='#urfdistrictdevelopmentplan'>urf:DistrictDevelopmentPlan</a>
- <a href='#urfdistrictimprovementplanfordisasterpreventionblockimprovementzoneplan'>urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan</a>
- <a href='#urfdistrictimprovementplanforhistoricscenerymaintenanceandimprovementdistrict'>urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict</a>
- <a href='#urfroadsidedistrictimprovementplan'>urf:RoadsideDistrictImprovementPlan</a>
- <a href='#urfruraldistrictimprovementplan'>urf:RuralDistrictImprovementPlan</a>
- <a href='#urfspecifiedbuildingzoneimprovementplan'>urf:SpecifiedBuildingZoneImprovementPlan</a>

### urf:DistrictFacilityProperty

以下のいずれかの型の値をとる：

- <a href='#urfdistrictfacility'>urf:DistrictFacility</a>
- <a href='#urfroadsidedistrictfacility'>urf:RoadsideDistrictFacility</a>
- <a href='#urfruraldistrictfacility'>urf:RuralDistrictFacility</a>

## データ (Data stereotype)

### gml:CoverageFunction


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| MappingRule | String |  | gml:MappingRule |
| GridFunction | gml:GridFunction |  | gml:GridFunction |

### gml:GridEnvelope


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|

### gml:GridFunction


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|

### gml:RectifiedGrid


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| limits | gml:GridEnvelope |  | gml:limits |
| axisName | String |  | gml:axisName |
| origin | Point |  | gml:origin |
| offsetVector | Point |  | gml:offsetVector |

### gml:RectifiedGridCoverage


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| rectifiedGridDomain | gml:RectifiedGridDomain |  | gml:rectifiedGridDomain |
| coverageFunction | gml:CoverageFunction |  | gml:coverageFunction |

### gml:RectifiedGridDomain


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| RectifiedGrid | gml:RectifiedGrid |  | gml:RectifiedGrid |

### core:Address


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|

### gen:genericAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|

### uro:BridgeFunctionalAttribute

橋梁機能属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| directionType | Code | 進行方向区分 | uro:directionType |
| userType | Code | 利用者区分 | uro:userType |

### uro:BridgeStructureAttribute

橋梁構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| material | Code | 材質 | uro:material |
| bridgeType | Code | 種類 | uro:bridgeType |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| area | Measure | 上部工面積 | uro:area |
| weightRestriction | Measure | 荷重制限 | uro:weightRestriction |
| heightRestriction | Measure | 高さ制限 | uro:heightRestriction |
| widthRestriction | Measure | 幅制限 | uro:widthRestriction |
| underGirderHeight | Measure | 桁下高さ制限 | uro:underGirderHeight |
| slopeType | Code | 昇降形式 | uro:slopeType |
| escalator | Boolean | エスカレータ有無 | uro:escalator |

### uro:BuildingDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code |  | uro:srcScale |
| geometrySrcDesc | Code |  | uro:geometrySrcDesc |
| thematicSrcDesc | Code |  | uro:thematicSrcDesc |
| appearanceSrcDesc | Code |  | uro:appearanceSrcDesc |
| lod1HeightType | Code |  | uro:lod1HeightType |
| lodType | Code |  | uro:lodType |

### uro:BuildingDetailAttribute

建物利用現況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| serialNumberOfBuildingCertification | String | 建築確認申請番号 | uro:serialNumberOfBuildingCertification |
| siteArea | Measure | 敷地面積 | uro:siteArea |
| totalFloorArea | Measure | 延床面積 | uro:totalFloorArea |
| buildingFootprintArea | Measure | 建築面積 | uro:buildingFootprintArea |
| buildingRoofEdgeArea | Measure | 図形面積 | uro:buildingRoofEdgeArea |
| developmentArea | Measure | 開発面積 | uro:developmentArea |
| buildingStructureType | Code | 構造種別 | uro:buildingStructureType |
| buildingStructureOrgType | Code | 構造種別（独自） | uro:buildingStructureOrgType |
| fireproofStructureType | Code | 耐火構造種別 | uro:fireproofStructureType |
| implementingBody | String | 事業主体 | uro:implementingBody |
| urbanPlanType | Code | 都市計画区域 | uro:urbanPlanType |
| areaClassificationType | Code | 区域区分 | uro:areaClassificationType |
| districtsAndZonesType | Code | 地域地区 | uro:districtsAndZonesType |
| landUseType | Code | 土地利用区分 | uro:landUseType |
| reference | String | 図面対象番号 | uro:reference |
| majorUsage | Code | 建物利用現況（大分類） | uro:majorUsage |
| majorUsage2 | Code | 建物利用現況（大分類2） | uro:majorUsage2 |
| orgUsage | Code | 建物利用現況（中分類） | uro:orgUsage |
| orgUsage2 | Code | 建物利用現況（小分類） | uro:orgUsage2 |
| detailedUsage | Code | 建物利用現況（詳細分類） | uro:detailedUsage |
| detailedUsage2 | Code | 建物利用現況（詳細分類2） | uro:detailedUsage2 |
| detailedUsage3 | Code | 建物利用現況（詳細分類3） | uro:detailedUsage3 |
| groundFloorUsage | Code | 1階用途 | uro:groundFloorUsage |
| secondFloorUsage | Code | 2階（以上）用途 | uro:secondFloorUsage |
| thirdFloorUsage | Code | 3階（以上）用途 | uro:thirdFloorUsage |
| basementUsage | Code | 地下用途 | uro:basementUsage |
| basementFirstUsage | Code | 地下1階用途 | uro:basementFirstUsage |
| basementSecondUsage | Code | 地下2階用途 | uro:basementSecondUsage |
| vacancy | Code | 空き家区分 | uro:vacancy |
| buildingCoverageRate | Double | 建蔽率 | uro:buildingCoverageRate |
| floorAreaRate | Double | 容積率 | uro:floorAreaRate |
| specifiedBuildingCoverageRate | Double | 指定建蔽率 | uro:specifiedBuildingCoverageRate |
| specifiedFloorAreaRate | Double | 指定容積率 | uro:specifiedFloorAreaRate |
| standardFloorAreaRate | Double | 基準容積率 | uro:standardFloorAreaRate |
| buildingHeight | Measure | 建築物の高さ | uro:buildingHeight |
| eaveHeight | Measure | 軒の高さ | uro:eaveHeight |
| note | String | 備考 | uro:note |
| surveyYear | String | 調査年 | uro:surveyYear |

### uro:BuildingHighTideRiskAttribute

高潮浸水リスク

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 説明 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |

### uro:BuildingIDAttribute

建物ID属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildingID | String | 建物ID | uro:buildingID |
| branchID | Integer | 建物ID枝番 | uro:branchID |
| partID | Integer | 建築物部分ID | uro:partID |
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |

### uro:BuildingInlandFloodingRiskAttribute

内水浸水リスク

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 説明 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |

### uro:BuildingLandSlideRiskAttribute

土砂災害リスク

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 現象区分 | uro:description |
| areaType | Code | 区域区分 | uro:areaType |

### uro:BuildingRiverFloodingRiskAttribute

洪水浸水リスク

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 指定河川名称 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |
| adminType | Code | 指定機関区分 | uro:adminType |
| scale | Code | 浸水規模 | uro:scale |
| duration | Measure | 継続時間 | uro:duration |

### uro:BuildingTsunamiRiskAttribute

津波浸水リスク

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 説明 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |

### uro:CargoHandlingFacility

荷さばき施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| mooringFacility | String | 係留施設名。 | uro:mooringFacility |
| liftableLoad | Measure | 荷役能力_吊り上げ荷重 | uro:liftableLoad |
| ability | Integer | 荷役能力_1時間あたりの能力 | uro:ability |
| packingName | Code | 荷姿名 | uro:packingName |
| acquisitionYear | String | 取得年度 | uro:acquisitionYear |
| innerTotalFloorArea | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| innerOfSiteArea | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| outerOfTotalFloorArea | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outerSiteArea | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |

### uro:CircularCurveType

円曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| radius | Measure | 半径 | uro:radius |
| intersection | Double | 交角 | uro:intersection |
| cutLength | Measure | 切線長 | uro:cutLength |
| curveLength | Measure | 曲線長 | uro:curveLength |

### uro:CityFurnitureDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code |  | uro:srcScale |
| geometrySrcDesc | Code |  | uro:geometrySrcDesc |
| thematicSrcDesc | Code |  | uro:thematicSrcDesc |
| appearanceSrcDesc | Code |  | uro:appearanceSrcDesc |
| lodType | Code |  | uro:lodType |

### uro:CityFurnitureDetailAttribute

都市設備詳細属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityType | Code | 施設詳細区分 | uro:facilityType |
| description | String |  | uro:description |

### uro:ConstructionBaseAttribute

構造物基本属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| adminType | Code | 管理者区分 | uro:adminType |
| administorator | String | 管理者名称 | uro:administorator |
| adminOffice | String | 管理事務所所在地 | uro:adminOffice |
| operatorType | Code | 運用者区分 | uro:operatorType |
| installerType | Code | 設置者区分 | uro:installerType |
| installer | String | 設置者名称 | uro:installer |
| structureOrdinance | String | 適用構造令 | uro:structureOrdinance |
| specification | String | 適用示方書 | uro:specification |
| kana | String | ふりがな | uro:kana |
| constructionStartYear | String | 建設開始年度 | uro:constructionStartYear |
| completionYear | String | 完成年度 | uro:completionYear |
| facilityAge | Integer | 施設年数 | uro:facilityAge |
| update | Date | 更新年月日 | uro:update |
| purpose | Code | 目的 | uro:purpose |

### uro:ConstructionDataQualityAttribute

データ品質

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code | 地図情報レベル | uro:srcScale |
| geometrySrcDesc | Code | 幾何属性作成方法 | uro:geometrySrcDesc |
| thematicSrcDesc | Code | 主題属性作成方法 | uro:thematicSrcDesc |
| appearanceSrcDesc | Code | テクスチャ作成方法 | uro:appearanceSrcDesc |
| dataAcquisition | String | データ取得方法 | uro:dataAcquisition |
| photoScale | Integer | 写真縮尺 | uro:photoScale |
| lod1HeightType | Code | LOD1高さ | uro:lod1HeightType |
| lodType | Code | 詳細LOD | uro:lodType |

### uro:ConstructionEvent

イベント

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| event | Code | 種類 | uro:event |
| dateOfEvent | Date | 日付 | uro:dateOfEvent |
| description | String | 説明 | uro:description |

### uro:ConstructionRiskAssessmentAttribute

構造物リスク評価属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| surveyYear | String | 点検実施年度 | uro:surveyYear |
| riskType | Code | 判定区分 | uro:riskType |
| status | Code | 対応状況 | uro:status |
| referenceDate | Date | 更新時点 | uro:referenceDate |

### uro:ConstructionStructureAttribute

構造物構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structureType | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 体積 | uro:volume |

### uro:ControlPoint

線形変化点

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| startPost | String | 開始キロ程 | uro:startPost |
| endPost | String | 終了キロ程 | uro:endPost |
| function | Code | 変化点種類 | uro:function |
| parameter | uro:ControlPointType | パラメータ | uro:parameter |
| startPoint | Point | 開始位置 | uro:startPoint |
| endPoint | Point | 終了位置 | uro:endPoint |

### uro:CyberportMarinaAndPBS

マリーナ/PBS

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| geologicalType | Code | 海底の地質名 | uro:geologicalType |
| obstructingStructures | String | 構造物による制限－構造物名 | uro:obstructingStructures |
| mainPartLength | Measure | 延長－取付部を除く延長 | uro:mainPartLength |
| totalLength | Measure |  | uro:totalLength |
| waveDissipatorLength | Measure | 消波工延長 | uro:waveDissipatorLength |
| facilityWidth | Measure | 施設の幅 | uro:facilityWidth |
| apronWidth | Measure | エプロン幅 | uro:apronWidth |
| restrictionStructure | String | 構造物による制限 | uro:restrictionStructure |
| plannedDepth | Measure | 計画上の水深 | uro:plannedDepth |
| currentDepth | Measure | 現在の水深 | uro:currentDepth |
| innerTotalFloorArea | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| innerOfSiteArea | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| outerOfTotalFloorArea | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outerSiteArea | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| ceilingHeight | Measure | 天端高 | uro:ceilingHeight |
| gravityResistant | Measure | 耐重力 | uro:gravityResistant |
| form | Code | 形態 | uro:form |
| areaType | Code | 内外区分 | uro:areaType |
| mainVessels | Code | 主要利用船舶の種類 | uro:mainVessels |
| isDredged | Boolean | 浚渫の有無 | uro:isDredged |
| mooringPostWeight | Measure | 附帯設備－係船柱の重さ | uro:mooringPostWeight |
| numberOfMooringPosts | Integer | 附帯設備－係船柱の個数 | uro:numberOfMooringPosts |
| resistantMaterial | Integer | 附帯設備－防げん材 | uro:resistantMaterial |
| lighting | Integer | 附帯設備－照明設備 | uro:lighting |
| stairs | Integer | 附帯設備－階段等 | uro:stairs |
| lifesaving | String | 附帯設備－救設備の名称 | uro:lifesaving |
| lifesavingNumber | Integer | 附帯設備－救命設備の数 | uro:lifesavingNumber |
| bumper | Measure | 附帯設備－車止め | uro:bumper |
| numberOfVehicleBoardings | Integer | 附帯設備－車両乗降設備－基数 | uro:numberOfVehicleBoardings |
| vehicleBoardingWidth | Measure | 附帯設備－車両乗降設備－幅員 | uro:vehicleBoardingWidth |
| shipType | String | 対象船舶－船型(D/W) | uro:shipType |
| numberOfSeats | Integer | 対象船舶－船席数 | uro:numberOfSeats |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| storageCapacity | Integer | 保管容量－値 | uro:storageCapacity |
| storageCapacityUnit | Code | 保管容量－単位 | uro:storageCapacityUnit |
| structureType | Code | 構造形式 | uro:structureType |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:DamAttribute

ダム属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structureType | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 堤頂長 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 堤体積 | uro:volume |
| damCode | Code | ダムコード | uro:damCode |
| totalWaterStorage | Measure | 総貯水量 | uro:totalWaterStorage |

### uro:DmAnnotation

DM注記情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dmCode | Code | 分類コード | uro:dmCode |
| meshCode | Code | 図郭番号 | uro:meshCode |
| dmElement | uro:DmElement | 要素情報 | uro:dmElement |
| geometryType | Code | レコードタイプ | uro:geometryType |
| shapeType | Code | 図形区分 | uro:shapeType |
| label | String | 注記文字列 | uro:label |
| isVertical | Boolean | 文字方向 | uro:isVertical |
| size | Integer | 字大 | uro:size |
| orientation | Integer | 表示角度 | uro:orientation |
| linewidth | Integer | 線号 | uro:linewidth |
| spacing | Integer | 字隔 | uro:spacing |

### uro:DmElement

要素情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| locationType | Code | 地域分類 | uro:locationType |
| infoType | Code | 情報分類 | uro:infoType |
| elementKey | String | 要素識別番号 | uro:elementKey |
| hierarchyLevel | String | 階層レベル | uro:hierarchyLevel |
| dataType | Code | 実データ区分 | uro:dataType |
| annotationType | Code | 注記区分 | uro:annotationType |
| precisionType | Code | 精度区分 | uro:precisionType |
| dislocationType | Code | 転位区分 | uro:dislocationType |
| breakType | Code | 間断区分 | uro:breakType |
| attributeValue | String | 属性数値 | uro:attributeValue |
| attributeType | Code | 属性区分 | uro:attributeType |
| attributeValueType | String | 属性データ書式 | uro:attributeValueType |
| creationDate | String | 取得年月 | uro:creationDate |
| updateDate | String | 更新年月 | uro:updateDate |
| terminationDate | String | 削除年月 | uro:terminationDate |
| freeSpace | String | 空き領域 | uro:freeSpace |

### uro:DmGeometricAttribute

DM図形情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dmCode | Code | 分類コード | uro:dmCode |
| meshCode | Code | 図郭番号 | uro:meshCode |
| dmElement | uro:DmElement | 要素情報 | uro:dmElement |
| geometryType | Code | レコードタイプ | uro:geometryType |
| mapLevel | Code | 地図情報レベル | uro:mapLevel |
| shapeType | Code | 図形区分 | uro:shapeType |
| visibility | Boolean | 可視性 | uro:visibility |
| is3d | Boolean | 3D区分 | uro:is3d |
| isInstallation | Boolean | 付属図形区分 | uro:isInstallation |
| isEdited | Boolean | 編集フラグ | uro:isEdited |
| isSupplementarySymbol | Boolean | 補助記号フラグ | uro:isSupplementarySymbol |
| angle | Double | 回転角度 | uro:angle |
| elevation | Measure | 標高 | uro:elevation |

### uro:Elevation

標高

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| elevationReference | Code | 計測位置 | uro:elevationReference |
| elevationValue | Point | 標高 | uro:elevationValue |

### uro:EmbankmentAttribute

堤防属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structureType | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 体積 | uro:volume |
| mainPartLength | Measure | 機能保有延長 | uro:mainPartLength |
| ceilingHeight | Measure | 天端高 | uro:ceilingHeight |
| waveDissipatorLength | Measure | 消波工延長 | uro:waveDissipatorLength |

### uro:FacilityIdAttribute

施設識別属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| partId | String |  | uro:partId |
| branchId | String |  | uro:branchId |
| prefecture | Code |  | uro:prefecture |
| city | Code |  | uro:city |
| route | String |  | uro:route |
| startPost | String |  | uro:startPost |
| endPost | String |  | uro:endPost |
| startLat | Double |  | uro:startLat |
| startLong | Double |  | uro:startLong |
| alternativeName | String |  | uro:alternativeName |

### uro:FacilityTypeAttribute

施設属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 区分 | uro:class |
| function | Code | 用途 | uro:function |

### uro:FishingPortCapacity

漁港施設能力

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| capacity | String | 能力 | uro:capacity |
| weightCapacity | Measure | 能力_耐重量 | uro:weightCapacity |
| hullForm | Integer | 能力_係船能力_船型 | uro:hullForm |
| shipNumber | Integer | 能力_係船能力_隻数 | uro:shipNumber |
| waterDepth-2m | Measure | 能力_水深別内訳_2ｍ未満の面積 | uro:waterDepth-2m |
| waterDepth2-3m | Measure | 能力_水深別内訳_2～3ｍ未満の面積 | uro:waterDepth2-3m |
| waterDepth3-6m | Measure | 能力_水深別内訳_3～6ｍ未満の面積 | uro:waterDepth3-6m |
| waterDepth6-m | Measure | 能力_水深別内訳_6ｍ以上の面積 | uro:waterDepth6-m |
| heightAboveAWL | Measure | 能力_種類_灯台_平均水面上の高さ | uro:heightAboveAWL |
| heightOnFoundations | Measure | 能力_種類_灯台_基礎上の高さ | uro:heightOnFoundations |
| luminousRange | Measure | 能力_光音電波の到達距離 | uro:luminousRange |
| luminousColor | String | 能力_灯色 | uro:luminousColor |
| candlePower | Integer | 能力_燭光数 | uro:candlePower |
| lightType | String | 能力_灯質の種類 | uro:lightType |
| period | String | 能力_灯質の周期 | uro:period |
| maximumGroundingWeight | Integer | 能力_入きょ又は上架できる最大船舶の総重量 | uro:maximumGroundingWeight |
| handleablePower | Integer | 能力_取り扱いできる機関の馬力数 | uro:handleablePower |
| maximumWaterSupply | Integer | 能力_最大給水能力 | uro:maximumWaterSupply |
| maximumRefueling | String | 能力_最大給油能力 | uro:maximumRefueling |
| people | Integer | 能力_最大収容可能人数 | uro:people |
| other | String | 能力_その他 | uro:other |

### uro:FishingPortFacility

漁港施設概要

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| facilityDetailsType | Code | 施設種類 | uro:facilityDetailsType |
| portName | String | 漁港名称 | uro:portName |
| portType | Code | 漁港の種類 | uro:portType |
| address | String | 所在地 | uro:address |
| designatedArea | String | 区域 | uro:designatedArea |
| designation | String | 漁港の指定 | uro:designation |
| designatedAdministrator | String | 漁港管理者の指定 | uro:designatedAdministrator |
| referenceNumber | String | 漁港の平面図対象番号 | uro:referenceNumber |
| grantType | Code | 施設区分名 | uro:grantType |
| administrator | String | 所有者の名称 | uro:administrator |
| facilityManager | String | 管理者の名称 | uro:facilityManager |
| structureType | Code | 構造_様式又は形式 | uro:structureType |
| mainMaterial | Code | 構造_主要用材 | uro:mainMaterial |
| otherStructure | String | 構造_その他の構造 | uro:otherStructure |
| length | Measure | 規模_延長 | uro:length |
| width | Measure | 規模_幅員 | uro:width |
| ceilingHeight | Measure | 規模_天端高 | uro:ceilingHeight |
| depth | Measure | 規模_水深 | uro:depth |
| area | Measure | 規模_面積 | uro:area |
| otherSizeDescription | String | 規模_その他の規模数量 | uro:otherSizeDescription |
| dateOfConstructionOrAcquisition | Date | 建設又は取得の年月日 | uro:dateOfConstructionOrAcquisition |
| cost | Integer | 建設又は取得の価格 | uro:cost |
| note | String | 備考 | uro:note |

### uro:HarborFacility

水域施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| geologicalType | Code | 地質名 | uro:geologicalType |
| obstructingStructures | String | 構造物による制限－構造物名 | uro:obstructingStructures |
| structuralLimitations | Measure | 構造物による制限 | uro:structuralLimitations |
| length | Measure | 延長 | uro:length |
| minimumWidth | Measure | 幅員－最小 | uro:minimumWidth |
| maximumWidth | Measure | 幅員－最大 | uro:maximumWidth |
| plannedDepth | Measure | 水深－計画上の水深 | uro:plannedDepth |
| currentDepth | Measure | 水深－現在の水深 | uro:currentDepth |
| isDredged | Boolean | 浚渫の有無 | uro:isDredged |
| areaType | Code | 内外区分 | uro:areaType |
| innerArea | Measure | 面積_防波堤等の内側 | uro:innerArea |
| outerArea | Measure | 面積_防波堤等の外側 | uro:outerArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:Height

高さ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| highReference | Code | 計測位置（高） | uro:highReference |
| lowReference | Code | 計測位置（低） | uro:lowReference |
| status | String | 計測方法 | uro:status |
| value | Measure | 値 | uro:value |

### uro:HighTideRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |

### uro:IfcAxis2Placement3D

IFCローカル座標系変換情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| location | Point | 原点 | uro:location |
| axis | String | Z軸ベクトル | uro:axis |
| refDirection | String | X軸ベクトル | uro:refDirection |

### uro:IfcBuilding

IFC建物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| compositionType | String | 構成型 | uro:compositionType |
| elevationOfRefHeight | Measure | 基準海抜高度 | uro:elevationOfRefHeight |
| elevationOfTerrain | Measure | 地盤最小海抜高度 | uro:elevationOfTerrain |
| buildingAddress | core:Address | 建物住所 | uro:buildingAddress |

### uro:IfcBuildingElement

IFC建築部材

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |

### uro:IfcBuildingStorey


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| description | String |  | uro:description |
| objectType | String |  | uro:objectType |
| longName | String |  | uro:longName |
| compositionType | String |  | uro:compositionType |
| elevation | Measure |  | uro:elevation |

### uro:IfcClassification

IFC分類諸元

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | String | ソース | uro:source |
| edition | String | 版 | uro:edition |
| editionDate | Date | 日付 | uro:editionDate |
| name | String | 分類ラベル | uro:name |

### uro:IfcClassificationReference

IFC分類諸元

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| location | URI |  | uro:location |
| itemReference | Code |  | uro:itemReference |
| name | String |  | uro:name |
| referenceSource | uro:IfcClassification |  | uro:referenceSource |

### uro:IfcCoordinateReferenceSystem

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| geodeticDatum | String | 測地原子 | uro:geodeticDatum |
| verticalDatum | String | 垂直原子 | uro:verticalDatum |

### uro:IfcCurtainWall

IFCカーテンウォール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |

### uro:IfcDoor

IFC扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |
| overallHeight | Measure | 全長 | uro:overallHeight |
| overallWidth | Measure | 全幅 | uro:overallWidth |

### uro:IfcFurnishingElement

IFC設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |

### uro:IfcGeometricRepresentationContext


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| contextIdentifier | String |  | uro:contextIdentifier |
| contextType | String |  | uro:contextType |
| coordinateSpaceDimension | Integer |  | uro:coordinateSpaceDimension |
| precision | Double |  | uro:precision |
| worldCoordinateSystem | uro:IfcAxis2Placement3D |  | uro:worldCoordinateSystem |
| trueNorth | String |  | uro:trueNorth |

### uro:IfcGroup


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| description | String |  | uro:description |
| objectType | String |  | uro:objectType |

### uro:IfcMapConversion

IFC座標変換

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| sourceCRS | uro:IfcCoordinateReferenceSystemSelectType |  | uro:sourceCRS |
| targetCRS | uro:IfcCoordinateReferenceSystemProperty |  | uro:targetCRS |
| eastings | Measure |  | uro:eastings |
| northings | Measure |  | uro:northings |
| orthogonalHeight | Measure |  | uro:orthogonalHeight |
| xAxisAbscissa | Double |  | uro:xAxisAbscissa |
| xAxisOrdinate | Double | 終点北座標 | uro:xAxisOrdinate |
| scale | Double | スケール | uro:scale |

### uro:IfcOpeningElement

IFC開口部

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| nominalArea | Measure | 公称面積 | uro:nominalArea |
| nominalVolume | Measure | 公称体積 | uro:nominalVolume |

### uro:IfcProject

IFCプロジェクト

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| phase | String | プロジェクトの状態 | uro:phase |
| representationContexts | uro:IfcGeometricRepresentationContext | 形状表現 | uro:representationContexts |
| unitsInContext | uro:IfcUnit | 単位系 | uro:unitsInContext |

### uro:IfcProjectedCRS

IFC投影座標参照系

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| geodeticDatum | String | 測地原子 | uro:geodeticDatum |
| verticalDatum | String | 垂直原子 | uro:verticalDatum |
| mapUnit | String | 軸単位 | uro:mapUnit |
| mapProjection | String | 投影座標系名称 | uro:mapProjection |
| mapZone | String | ゾーン番号 | uro:mapZone |

### uro:IfcPsetBuildingCommon

IFC建築物共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildingId | String | 建物ID | uro:buildingId |
| isPermanentId | Boolean | 識別子の永続性 | uro:isPermanentId |
| mainFireUse | String | 主な防災用途 | uro:mainFireUse |
| ancillaryFireUse | String | 付属的な防災用途 | uro:ancillaryFireUse |
| sprinklerProtection | Boolean | スプリンクラー有無 | uro:sprinklerProtection |
| sprinklerProtectionAutomatic | Boolean | 自動スプリンクラー有無 | uro:sprinklerProtectionAutomatic |
| occupancyType | Code | 入居者タイプ | uro:occupancyType |
| grossPlannedArea | Measure | 計画総面積 | uro:grossPlannedArea |
| numberOfStoreys | Integer | 階数 | uro:numberOfStoreys |
| yearOfConstruction | NonNegativeInteger | 建設年 | uro:yearOfConstruction |
| isLandmarked | Boolean | 歴史的建造物区分 | uro:isLandmarked |

### uro:IfcPsetDoorCommon

IFC扉共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| acousticRating | String | 遮音等級 | uro:acousticRating |
| firerating | String | 耐火等級 | uro:firerating |
| securityRating | String | 防犯等級 | uro:securityRating |
| isExternal | Boolean | 外部区分 | uro:isExternal |
| infiltration | Double | 隙間風流量 | uro:infiltration |
| thermalTransmittance | Double | 熱貫流率 | uro:thermalTransmittance |
| glazingAreaFraction | Double | ガラス領域比率 | uro:glazingAreaFraction |
| handicapAccessible | Boolean | ハンディキャップアクセス区分 | uro:handicapAccessible |
| fireExit | Boolean | 非常口区分 | uro:fireExit |
| selfClosing | Boolean | 自動ドア閉機能区分 | uro:selfClosing |
| smokeStop | Boolean | 防煙機能区分 | uro:smokeStop |

### uro:IfcPsetOpeningElementCommon

IFC開口部共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照用ID | uro:reference |
| purpose | String | 目的 | uro:purpose |
| fireExit | Boolean | 非常口区分 | uro:fireExit |
| protectedOpening | Boolean | 保護区分 | uro:protectedOpening |
| parallelJambs | Boolean | 平行区分 | uro:parallelJambs |

### uro:IfcPsetSiteCommon

IFC敷地共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildableArea | Measure | 使用面積 | uro:buildableArea |
| totalArea | Measure | 総面積 | uro:totalArea |
| buildingHeightLimit | Measure | 最大高さ | uro:buildingHeightLimit |

### uro:IfcPsetSpaceCommon

IFC空間共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| category | String | カテゴリ | uro:category |
| floorCovering | String | 床仕上げ | uro:floorCovering |
| wallCovering | String | 壁仕上げ | uro:wallCovering |
| ceilingCovering | String | 天井仕上げ | uro:ceilingCovering |
| skirtingBoard | String | 幅木 | uro:skirtingBoard |
| grossPlannedArea | Measure | 計画グロス面積 | uro:grossPlannedArea |
| netPlannedArea | Measure | 計画ネット面積 | uro:netPlannedArea |
| publiclyAccessible | Boolean | 公共アクセス区分 | uro:publiclyAccessible |
| handicapAccessible | Boolean | ハンディキャップアクセス区分 | uro:handicapAccessible |
| concealedFlooring | Boolean | 隠ぺい床スペース区分 | uro:concealedFlooring |
| concealedCeiling | Boolean | 隠ぺい天井スペース区分 | uro:concealedCeiling |

### uro:IfcPsetWindowCommon

IFC窓共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| acousticRating | String | 遮音等級 | uro:acousticRating |
| fireRating | String | 耐火等級 | uro:fireRating |
| securityRating | String | 防犯等級 | uro:securityRating |
| isExternal | Boolean | 外部区分 | uro:isExternal |
| infiltration | Double | 隙間風流量 | uro:infiltration |
| thermalTransmittance | Double | 熱貫流率 | uro:thermalTransmittance |
| glazingAreaFraction | Double | ガラス領域比率 | uro:glazingAreaFraction |
| smokeStop | Boolean | 防煙機能区分 | uro:smokeStop |

### uro:IfcRoof

IFC屋根

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |

### uro:IfcSite


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| description | String |  | uro:description |
| objectType | String |  | uro:objectType |
| longName | String |  | uro:longName |
| compositionType | String |  | uro:compositionType |
| refLongitude | Double |  | uro:refLongitude |
| refLatitude | Double |  | uro:refLatitude |
| refElevation | Measure |  | uro:refElevation |
| landTitleNumber | String |  | uro:landTitleNumber |
| siteAddress | core:Address |  | uro:siteAddress |

### uro:IfcSpace

IFC空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| compositionType | String | 構成型 | uro:compositionType |
| interiorOrExteriorSpace | String | 内外区分 | uro:interiorOrExteriorSpace |
| elevationWithFlooring | Measure | 床高さ | uro:elevationWithFlooring |

### uro:IfcSpaceBaseQuantity

IFC空間基本数値情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| nominalHeight | Measure | スラブ上端から上階スラブ下端までの高さ | uro:nominalHeight |
| clearHeight | Measure | 床面から天井面の高さ | uro:clearHeight |
| finishCeilingHeight | Measure | 天井高 | uro:finishCeilingHeight |
| grossPerimeter | Measure | 床レベルでの総周辺長 | uro:grossPerimeter |
| netPerimeter | Measure | 正味周囲長 | uro:netPerimeter |
| grossCeilingArea | Measure | 天井面積 | uro:grossCeilingArea |
| grossFloorArea | Measure | 延面積 | uro:grossFloorArea |
| netCeilingArea | Measure | 正味天井面積 | uro:netCeilingArea |
| netFloorArea | Measure | 正味延面積 | uro:netFloorArea |
| grossWallArea | Measure | 壁面積 | uro:grossWallArea |
| netWallArea | Measure | 正味壁面積 | uro:netWallArea |
| grossVolume | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |

### uro:IfcUnit

IFC単位

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dimensions | Integer | 次元数 | uro:dimensions |
| unitType | String | 単位種類 | uro:unitType |
| perfix | String |  | uro:perfix |
| name | String | 名称 | uro:name |

### uro:IfcWall

IFC壁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |
| nominalLength | Measure | 長さ | uro:nominalLength |
| nominalWidth | Measure | 幅 | uro:nominalWidth |
| nominalHeight | Measure | 高さ | uro:nominalHeight |
| grossFootPrintArea | Measure | フットプリント面積 | uro:grossFootPrintArea |
| netFootPrintArea | Measure | 正味フットプリント面積 | uro:netFootPrintArea |
| grossSideArea | Measure | 側面面積 | uro:grossSideArea |
| netSideArea | Measure | 正味側面面積 | uro:netSideArea |
| grossSideAreaLeft | Measure | 左側側面面積 | uro:grossSideAreaLeft |
| netSideAreaLeft | Measure | 左側正味側面面積 | uro:netSideAreaLeft |
| grossSideAreaRight | Measure | 右側側面面積 | uro:grossSideAreaRight |
| netSideAreaRight | Measure | 右側正味側面面積 | uro:netSideAreaRight |
| grossVolume | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |

### uro:IfcWallStandardCase

IFC標準壁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer | 蹴上数 | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |
| nominalLength | Measure | 長さ | uro:nominalLength |
| nominalWidth | Measure | 幅 | uro:nominalWidth |
| nominalHeight | Measure | 高さ | uro:nominalHeight |
| grossFootPrintArea | Measure | フットプリント面積 | uro:grossFootPrintArea |
| netFootPrintArea | Measure | 正味フットプリント面積 | uro:netFootPrintArea |
| grossSideArea | Measure | 側面面積 | uro:grossSideArea |
| netSideArea | Measure | 正味側面面積 | uro:netSideArea |
| grossSideAreaLeft | Measure | 左側側面面積 | uro:grossSideAreaLeft |
| netSideAreaLeft | Measure | 左側正味側面面積 | uro:netSideAreaLeft |
| grossSideAreaRight | Measure | 右側側面面積 | uro:grossSideAreaRight |
| netSideAreaRight | Measure | 右側正味側面面積 | uro:netSideAreaRight |
| grossVolume | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |

### uro:IfcWindow

IFC窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| description | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementType | Code | 建築部材区分 | uro:elementType |
| predefinedType | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| numberOfRiser | Integer |  | uro:numberOfRiser |
| numberOfTreads | Integer | 踏面数 | uro:numberOfTreads |
| riserHeight | Measure | 蹴上高さ | uro:riserHeight |
| treadLength | Measure | 奥行長さ | uro:treadLength |
| operationType | String | 輸送設備区分 | uro:operationType |
| capacityByWeight | Measure | 許容積載量 | uro:capacityByWeight |
| capacityByNumber | Integer | 許容定員数 | uro:capacityByNumber |
| overallHeight | Measure | 全長 | uro:overallHeight |
| overallWidth | Measure | 全幅 | uro:overallWidth |

### uro:IfcZone


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| description | String |  | uro:description |
| objectType | String |  | uro:objectType |

### uro:IndoorFacilityAttribute

屋内施設属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| weekdayHours | String | 施設の営業時間（平日） | uro:weekdayHours |
| weekendHours | String | 施設の営業時間（土日祝祭日） | uro:weekendHours |
| phone | String | 電話番号 | uro:phone |
| website | String | ウェブサイトアドレス | uro:website |

### uro:IndoorFurnishingAttribute

屋内設置物属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |

### uro:IndoorPublicTagAttribute

屋内タグ属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| ucode | String |  | uro:ucode |

### uro:IndoorSpaceAttribute

屋内空間属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |
| isRestricted | Boolean | 制限有無 | uro:isRestricted |
| suite | String | 注記ラベル | uro:suite |
| isPublic | Boolean | 公開可否 | uro:isPublic |
| tollType | Code | 有料区分 | uro:tollType |

### uro:IndoorStoreyAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| category | Boolean |  | uro:category |
| ordinal | Double |  | uro:ordinal |

### uro:IndoorTacatileTileAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| startNode | String |  | uro:startNode |
| endNode | String |  | uro:endNode |
| category | Code |  | uro:category |
| roof | String |  | uro:roof |
| floorId | String |  | uro:floorId |

### uro:IndoorUserDefinedAttribute

屋内利用者定義属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| name | String | 名称 | uro:name |
| nominalValue | uro:UserDefinedValue | 値 | uro:nominalValue |
| description | String | 説明 | uro:description |
| unit | String | 単位 | uro:unit |

### uro:IndoorZoneAttribute

屋内任意空間属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |

### uro:InlandFloodingRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |

### uro:KeyValuePairAttribute

拡張属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| key | Code | （コードに該当する説明を使用する） | uro:key |
| codeValue | Code | 値 | uro:codeValue |

### uro:LandSlideRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | nan | uro:description |
| areaType | Code | nan | uro:areaType |

### uro:LandUseDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code |  | uro:srcScale |
| geometrySrcDesc | Code |  | uro:geometrySrcDesc |
| thematicSrcDesc | Code |  | uro:thematicSrcDesc |

### uro:LandUseDetailAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| orgLandUse | Code |  | uro:orgLandUse |
| nominalArea | Measure |  | uro:nominalArea |
| ownerType | Code |  | uro:ownerType |
| owner | String |  | uro:owner |
| areaInSquareMeter | Measure |  | uro:areaInSquareMeter |
| areaInHa | Measure |  | uro:areaInHa |
| buildingCoverageRate | Double |  | uro:buildingCoverageRate |
| floorAreaRate | Double |  | uro:floorAreaRate |
| specifiedBuildingCoverageRate | Double |  | uro:specifiedBuildingCoverageRate |
| specifiedFloorAreaRate | Double |  | uro:specifiedFloorAreaRate |
| standardFloorAreaRate | Double |  | uro:standardFloorAreaRate |
| urbanPlanType | Code |  | uro:urbanPlanType |
| areaClassificationType | Code |  | uro:areaClassificationType |
| districtsAndZonesType | Code |  | uro:districtsAndZonesType |
| prefecture | Code |  | uro:prefecture |
| city | Code |  | uro:city |
| reference | String |  | uro:reference |
| note | String |  | uro:note |
| surveyYear | String |  | uro:surveyYear |

### uro:LargeCustomerFacilityAttribute

大規模小売店舗等の立地状況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 分類 | uro:class |
| name | String | 施設名称 | uro:name |
| capacity | Integer | 収容数 | uro:capacity |
| owner | String | 施設主 | uro:owner |
| totalFloorArea | Measure | 延床面積 | uro:totalFloorArea |
| totalStoreFloorArea | Measure | 店舗床面積 | uro:totalStoreFloorArea |
| inauguralDate | Date | 開業日（開校日） | uro:inauguralDate |
| yearOpened | String |  | uro:yearOpened |
| yearClosed | String |  | uro:yearClosed |
| keyTenants | String | 核テナント | uro:keyTenants |
| availability | Boolean | 利用可能性 | uro:availability |
| urbanPlanType | Code | 都市計画区域 | uro:urbanPlanType |
| areaClassificationType | Code | 区域区分 | uro:areaClassificationType |
| districtsAndZonesType | Code | 地域地区 | uro:districtsAndZonesType |
| landUseType | Code | 土地利用区分 | uro:landUseType |
| reference | String | 図面対象番号 | uro:reference |
| note | String | 備考 | uro:note |
| surveyYear | String | 調査年 | uro:surveyYear |

### uro:LengthAttribute

長さ情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| length | Measure | 長さ情報 | uro:length |
| mesureType | Code | 計測方法 | uro:mesureType |
| phaseType | Code | 計測段階 | uro:phaseType |

### uro:MaintenanceHistoryAttribute

工事・点検情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| maintenanceType | Code | 点検・工事種類 | uro:maintenanceType |
| maintenanceFiscalYear | String | 実施年度 | uro:maintenanceFiscalYear |
| maintenanceYear | String | 実施年度 | uro:maintenanceYear |
| maintenanceDate | Date | 実施日 | uro:maintenanceDate |
| status | String | 実施状況 | uro:status |
| description | String | 実施内容 | uro:description |

### uro:MooringFacility

係留施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| mainPartLength | Measure | 延長－取付部を除く延長 | uro:mainPartLength |
| totalLength | Measure | 延長－取付部を含む延長 | uro:totalLength |
| facilityWidth | Measure | 施設の幅 | uro:facilityWidth |
| apronWidth | Measure | エプロン幅 | uro:apronWidth |
| plannedDepth | Measure | 水深－計画上の水深 | uro:plannedDepth |
| currentDepth | Measure | 水深－現在の水深 | uro:currentDepth |
| area | Measure | 面積 | uro:area |
| ceilingHeight | Measure | 天端高 | uro:ceilingHeight |
| gravityResistant | Measure | 耐重力 | uro:gravityResistant |
| form | Code | 形態 | uro:form |
| mainVessels | Code | 主要利用船舶の種類 | uro:mainVessels |
| mooringPostWeight | Measure | 附帯設備－係船柱の重さ | uro:mooringPostWeight |
| numberOfMooringPosts | Integer | 附帯設備－係船柱の数 | uro:numberOfMooringPosts |
| resistantMaterial | Integer | 附帯設備－防げん材 | uro:resistantMaterial |
| lighting | Integer | 附帯設備－照明設備 | uro:lighting |
| stairs | Integer | 附帯設備－階段等 | uro:stairs |
| lifesavingAppliances | String | 附帯設備－救命設備の名称 | uro:lifesavingAppliances |
| numberOfLifesavingAppliances | Integer | 附帯設備－救命設備の数 | uro:numberOfLifesavingAppliances |
| bumper | Measure | 附帯設備－車止め | uro:bumper |
| numberOfVehicleBoardings | Integer | 附帯設備－車両乗降設備－基数 | uro:numberOfVehicleBoardings |
| vehicleBoardingWidth | Measure | 附帯設備－車両乗降設備－幅員 | uro:vehicleBoardingWidth |
| shipType | String | 対象船舶－船型（D／W） | uro:shipType |
| numberOfSeats | Integer | 対象船舶－船席数 | uro:numberOfSeats |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| structureType | Code | 構造形式 | uro:structureType |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:NavigationAssistanceFacility

航行補助施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | String | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:Occupancy

占有状況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| interval | Code | 期間 | uro:interval |
| numberofOccupants | Integer | 数 | uro:numberofOccupants |
| occupantType | Code | 種類 | uro:occupantType |

### uro:OffsetDepth

オフセットデプス情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| pos | Point | 計測位置 | uro:pos |
| offset | Measure | オフセット量 | uro:offset |
| depth | Measure | 土被り量 | uro:depth |
| minDepth | Measure | 最小土被り量 | uro:minDepth |
| maxDepth | Measure | 最大土被り量 | uro:maxDepth |

### uro:PortEnvironmentalImprovementFacility

環境整備施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| usage | String | 用途等 | uro:usage |
| length | Measure | 延長 | uro:length |
| area | Measure | 面積 | uro:area |
| totalFoorArea | Measure | 総床面積 | uro:totalFoorArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortManagementFacility

港湾管理施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| totalFloorArea | Measure | 総床面積 | uro:totalFloorArea |
| numberOfShipTypes | Integer | 船型数量 | uro:numberOfShipTypes |
| unitOfShipType | Code | 船型単位 | uro:unitOfShipType |
| loadingCapacity | Integer | 積載量 | uro:loadingCapacity |
| acquisitionYear | String | 取得年度 | uro:acquisitionYear |
| usage | String | 用途 | uro:usage |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortPassengerFacility

旅客施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalFloorArea | Measure | 総床面積 | uro:totalFloorArea |
| acquisitionYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |

### uro:PortPollutionControlFacility

公害防止施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| crossSectionalArea | Measure | 断面積 | uro:crossSectionalArea |
| area | Measure | 面積 | uro:area |
| height | Measure | 高さ | uro:height |
| mainMaterial | Code | 主要用材コードリスト | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortProtectiveFacility

外郭施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| structureType | Code | 構造形式 | uro:structureType |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortStorageFacility

保管施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| innerTotalFloorArea | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| innerOfSiteArea | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| outerOfTotalFloorArea | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outerSiteArea | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| storageCapacity | Integer | 保管容量－値 | uro:storageCapacity |
| storageCapacityUnit | Code | 保管容量－単位 | uro:storageCapacityUnit |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |

### uro:PortTransportationFacility

臨港交通施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| structureType | Code | 構造形式 | uro:structureType |
| startingPoint | String | 起終点 | uro:startingPoint |
| length | Measure | 規模_延長 | uro:length |
| area | Measure | 規模_面積 | uro:area |
| beddingWidth | Measure | 規模_道路敷幅 | uro:beddingWidth |
| numberOfLanes | Integer | 規模_車線数 | uro:numberOfLanes |
| parkingLotCapacityOfBus | Integer | 規模_駐車場収容台数_バス | uro:parkingLotCapacityOfBus |
| parkingLotCapacityOfCars | Integer | 規模_駐車場収容台数_乗用車 | uro:parkingLotCapacityOfCars |
| routeType | Code | 規模_単線・複線区分 | uro:routeType |
| heightToDigit | Measure | 規模_桁下高 | uro:heightToDigit |
| heightLimit | Measure | 規模_制限高 | uro:heightLimit |
| minimumWidth | Measure | 規模_車道幅員 | uro:minimumWidth |
| minimumDepth | Measure | 規模_最小水深 | uro:minimumDepth |
| numberOfAircraftParkingSpaces | Integer | 規模_駐機数 | uro:numberOfAircraftParkingSpaces |
| pavementType | Code | 舗装形態/塗装形態 | uro:pavementType |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortWasteTreatmentFacility

廃棄物処理施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| structureType | Code | 構造形式 | uro:structureType |
| perimeter | Measure | 延長_外周建設延長 | uro:perimeter |
| mainPartLength | Measure | 延長_機能保有延長 | uro:mainPartLength |
| innerShoreLength | Measure |  | uro:innerShoreLength |
| ceilingHeight | Measure | 天端高 | uro:ceilingHeight |
| waveDissipatorLength | Measure | 消波工延長 | uro:waveDissipatorLength |
| mainMaterial | Code | 主要用材 | uro:mainMaterial |
| wasteType | Code | 廃棄物の種類 | uro:wasteType |
| plannedDisposalArea | Measure | 計画処分面積 | uro:plannedDisposalArea |
| plannedDisposalAmount | Integer | 計画処分量 | uro:plannedDisposalAmount |
| receivingCapacity | Integer | 受入容量 | uro:receivingCapacity |
| shipType | String | 船型 | uro:shipType |
| unitOfReceivingCapacity | Code | 受入容量単位 | uro:unitOfReceivingCapacity |
| acquisitionYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |

### uro:PortWelfareFacility

厚生施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| totalFloorArea | Measure | 面積_防波堤等の外側 | uro:totalFloorArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |

### uro:RailwayRouteAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| operatorType | Code | 鉄道事業者区分 | uro:operatorType |
| operator | String | 鉄道事業者名 | uro:operator |
| alternativeName | String | 路線別称 | uro:alternativeName |
| railwayType | Code | 鉄道区分 | uro:railwayType |
| startStation | String | 起点駅名 | uro:startStation |
| endStation | String | 終点駅名 | uro:endStation |

### uro:RailwayTrackAttribute

軌道中心線線形情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeName | String | 路線名称 | uro:routeName |
| directionType | Code | 進行方向区分 | uro:directionType |
| trackType | Code | 軌道の種類 | uro:trackType |
| startPost | String | 開始キロ程 | uro:startPost |
| endPost | String | 終了キロ程 | uro:endPost |
| alignmentType | Code | 線形種別 | uro:alignmentType |
| controlPoint | uro:ControlPoint | 線形変化点 | uro:controlPoint |

### uro:RealEstateIDAttribute

不動産ID

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| realEstateIDOfBuilding | String | 建築物不動産ID | uro:realEstateIDOfBuilding |
| numberOfBuildingUnitOwnership | Integer | 区分所有数 | uro:numberOfBuildingUnitOwnership |
| realEstateIDOfBuildingUnitOwnership | String | 区分不動産ID | uro:realEstateIDOfBuildingUnitOwnership |
| numberOfRealEstateIDOfLand | Integer | 土地不動産ID数 | uro:numberOfRealEstateIDOfLand |
| realEstateIDOfLand | String | 土地不動産ID数 | uro:realEstateIDOfLand |
| matchingScore | Integer | マッチングスコア | uro:matchingScore |

### uro:RiverFacilityIdAttribute

河川管理施設識別属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| partId | String |  | uro:partId |
| branchId | String |  | uro:branchId |
| prefecture | Code |  | uro:prefecture |
| city | Code |  | uro:city |
| route | String |  | uro:route |
| startPost | String |  | uro:startPost |
| endPost | String |  | uro:endPost |
| startLat | Double |  | uro:startLat |
| startLong | Double |  | uro:startLong |
| alternativeName | String |  | uro:alternativeName |
| riverCode | Code |  | uro:riverCode |
| riverName | String |  | uro:riverName |
| sideType | Code |  | uro:sideType |
| leftPost | Measure |  | uro:leftPost |
| leftDistance | Measure |  | uro:leftDistance |
| rightPost | Measure |  | uro:rightPost |
| rightDistance | Measure |  | uro:rightDistance |
| leftStartPost | Measure |  | uro:leftStartPost |
| leftStartDistance | Measure |  | uro:leftStartDistance |
| leftEndPost | Measure |  | uro:leftEndPost |
| lefEndDistance | Measure |  | uro:lefEndDistance |
| rightStartPost | Measure |  | uro:rightStartPost |
| rightStartDistance | Measure |  | uro:rightStartDistance |
| rightEndPost | Measure |  | uro:rightEndPost |
| rightEndDistance | Measure |  | uro:rightEndDistance |

### uro:RiverFloodingRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |
| adminType | Code | nan | uro:adminType |
| scale | Code | nan | uro:scale |
| duration | Measure | nan | uro:duration |

### uro:RoadStructureAttribute

道路構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| widthType | Code | 幅員区分 | uro:widthType |
| width | Measure | 幅員 | uro:width |
| numberOfLanes | Integer | 車線数 | uro:numberOfLanes |
| sectionType | Code | 区間種別 | uro:sectionType |

### uro:RoadType


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| creationDate | Date |  | uro:creationDate |
| isTemporary | Boolean |  | uro:isTemporary |
| roadType | Code |  | uro:roadType |
| widthType | Code |  | uro:widthType |
| isTollRoad | Boolean |  | uro:isTollRoad |
| separator | Measure |  | uro:separator |
| isHighWay | Boolean |  | uro:isHighWay |

### uro:RoomDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code |  | uro:srcScale |
| geometrySrcDesc | Code |  | uro:geometrySrcDesc |
| thematicSrcDesc | Code |  | uro:thematicSrcDesc |
| appearanceSrcDesc | Code |  | uro:appearanceSrcDesc |
| lodType | Code |  | uro:lodType |

### uro:ShipServiceFacility

船舶役務用施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| portFacilityDetailsType | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| isDesignated | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradationLevel | Integer | 性能低下度 | uro:degradationLevel |
| shipType | String | 対象船舶－船型（D／W） | uro:shipType |
| supplyAbility | Integer | 供給能力容量 | uro:supplyAbility |
| supplyAbilityUnit | Code | 供給能力単位 | uro:supplyAbilityUnit |
| mooringPlace | String | 補給を受ける船舶の係留場所 | uro:mooringPlace |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅 | uro:width |
| area | Measure | 面積 | uro:area |
| acquisitionYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |

### uro:SlopeType

勾配変化情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| angle | Double | 勾配角度 | uro:angle |
| elevation | Measure | 標高 | uro:elevation |

### uro:SquareUrbanPlanAttribute

広場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbanPlanningAreaName | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | String | 施行者名 | uro:enforcer |
| dateOfDecision | Date | 決定日 | uro:dateOfDecision |
| dateOfRevision | Date | 変更日 | uro:dateOfRevision |
| areaPlanned | Measure | 計画面積 | uro:areaPlanned |
| areaInService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| areaImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| areaCompleted | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| projectStartDate | Date | 事業開始年月日 | uro:projectStartDate |
| projectEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| isCompleted | Boolean | 完成区分 | uro:isCompleted |
| isAuthorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |

### uro:StationSquareAttribute

駅前広場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbanPlanningAreaName | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | String | 施行者名 | uro:enforcer |
| dateOfDecision | Date | 決定日 | uro:dateOfDecision |
| dateOfRevision | Date | 変更日 | uro:dateOfRevision |
| areaPlanned | Measure | 計画面積 | uro:areaPlanned |
| areaInService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| areaImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| areaCompleted | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| projectStartDate | Date | 事業開始年月日 | uro:projectStartDate |
| projectEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| isCompleted | Boolean | 完成区分 | uro:isCompleted |
| isAuthorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |
| station | String | 駅名 | uro:station |
| route | String | 路線名 | uro:route |
| railwayType | Code | 鉄道種別 | uro:railwayType |

### uro:TerminalAttribute

自動車ターミナル属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbanPlanningAreaName | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | String | 施行者名 | uro:enforcer |
| dateOfDecision | Date | 決定日 | uro:dateOfDecision |
| dateOfRevision | Date | 変更日 | uro:dateOfRevision |
| areaPlanned | Measure | 計画面積 | uro:areaPlanned |
| areaInService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| areaImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| areaCompleted | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| projectStartDate | Date | 事業開始年月日 | uro:projectStartDate |
| projectEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| isCompleted | Boolean | 完成区分 | uro:isCompleted |
| isAuthorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |
| terminalType | Code | ターミナル区分 | uro:terminalType |
| structure | String | ターミナル区分構造 | uro:structure |
| numberOfBerthsPlanned | Integer | 計画バース数 | uro:numberOfBerthsPlanned |
| numberOfBerthsInService | Integer | 供用バース数 | uro:numberOfBerthsInService |
| userType | Code |  | uro:userType |

### uro:ThematicShape

主題図形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| horizontalType | Code | 水平区分 | uro:horizontalType |
| heightType | Code | 高さ区分 | uro:heightType |

### uro:TrackAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| alternativeName | String | 別名 | uro:alternativeName |
| adminType | Code | 管理区分 | uro:adminType |
| relativeLevel | Integer | 階層準 | uro:relativeLevel |
| widthType | Code | 幅員区分 | uro:widthType |
| structureType | Code | 構造区分 | uro:structureType |
| isTollRoad | Boolean | 有料区分 | uro:isTollRoad |
| separator | Measure | 分離帯区分 | uro:separator |

### uro:TrafficAreaStructureAttribute

道路構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| numberOfLanes | Integer | 車線数 | uro:numberOfLanes |

### uro:TrafficVolumeAttribute

交通量属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| sectionID | String | 交通調査基本区間番号 | uro:sectionID |
| routeName | String | nan | uro:routeName |
| weekday12hourTrafficVolume | Integer | 平日12時間交通量 | uro:weekday12hourTrafficVolume |
| weekday24hourTrafficVolume | Integer | 平日24時間交通量 | uro:weekday24hourTrafficVolume |
| largeVehicleRate | Double | 大型車混入率 | uro:largeVehicleRate |
| congestionRate | Double | 混雑度 | uro:congestionRate |
| averageTravelSpeedInCongestion | Double | 混雑時平均旅行速度 | uro:averageTravelSpeedInCongestion |
| averageInboundTravelSpeedInCongestion | Double | 混雑時平均旅行速度（上り） | uro:averageInboundTravelSpeedInCongestion |
| averageOutboundTravelSpeedInCongestion | Double | 混雑時平均旅行速度（下り） | uro:averageOutboundTravelSpeedInCongestion |
| averageInboundTravelSpeedNotCongestion | Double | 非混雑時平均旅行速度（上り） | uro:averageInboundTravelSpeedNotCongestion |
| averageOutboundTravelSpeedNotCongestion | Double | 非混雑時平均旅行速度（下り） | uro:averageOutboundTravelSpeedNotCongestion |
| observationPointName | String | 観測地点名 | uro:observationPointName |
| reference | String | 参照情報 | uro:reference |
| surveyYear | String | 調査年 | uro:surveyYear |

### uro:TransitionCurveType

緩和曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| intersection | Measure | 交角 | uro:intersection |
| distance | Measure | 移動距離 | uro:distance |
| curveLength | Measure | 曲線長 | uro:curveLength |

### uro:TransportationDataQualityAttribute

データ品質

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code | 地図情報レベル | uro:srcScale |
| geometrySrcDesc | Code | 幾何属性作成方法 | uro:geometrySrcDesc |
| thematicSrcDesc | Code | 主題属性作成方法 | uro:thematicSrcDesc |
| appearanceSrcDesc | Code | テクスチャ作成方法 | uro:appearanceSrcDesc |
| lodType | Code | 詳細LOD | uro:lodType |

### uro:TsunamiRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |

### uro:TunnelFunctionalAttribute

トンネル機能属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| directionType | Code | 進行方向区分 | uro:directionType |
| userType | Code | 利用者区分 | uro:userType |

### uro:TunnelStructureAttribute

トンネル構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| tunnelType | Code | 種類 | uro:tunnelType |
| tunnelSubtype | Code | 詳細種類 | uro:tunnelSubtype |
| mouthType | Code | 坑口区分 | uro:mouthType |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| area | Measure | 面積 | uro:area |
| innerHeight | Measure | 内空高 | uro:innerHeight |
| effectiveHeight | Measure | 有効高 | uro:effectiveHeight |
| slopeType | Code | 昇降形式 | uro:slopeType |

### uro:UserDefinedValue


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| stringValue | String |  | uro:stringValue |
| intValue | Integer |  | uro:intValue |
| doubleValue | Double |  | uro:doubleValue |
| codeValue | Code |  | uro:codeValue |
| dateValue | Date |  | uro:dateValue |
| uriValue | URI |  | uro:uriValue |
| measuredValue | Measure |  | uro:measuredValue |

### uro:VegetationDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | Code |  | uro:srcScale |
| geometrySrcDesc | Code |  | uro:geometrySrcDesc |
| thematicSrcDesc | Code |  | uro:thematicSrcDesc |
| appearanceSrcDesc | Code |  | uro:appearanceSrcDesc |

### uro:VerticalCurveType

縦曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| length | Measure | 長さ | uro:length |
| verticalDistance | Measure | 縦距 | uro:verticalDistance |

### uro:WaterBodyDetailAttribute

水部詳細属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| kana | String | フリガナ | uro:kana |
| waterSystemCode | Code | 水系域コード | uro:waterSystemCode |
| riverCode | Code | 河川コード | uro:riverCode |
| adminType | Code | 区間種別 | uro:adminType |
| flowDirection | Boolean | 流下方向区分 | uro:flowDirection |
| maximumDepth | Measure | 最大水深 | uro:maximumDepth |
| waterSurfaceElevation | Measure | 水面標高 | uro:waterSurfaceElevation |
| area | Measure | 面積 | uro:area |
| measurementYearMonth | String | 測量年月 | uro:measurementYearMonth |
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |

### uro:WaterBodyHighTideRiskAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code |  | uro:description |
| rank | Code |  | uro:rank |
| rankOrg | Code |  | uro:rankOrg |
| depth | Measure |  | uro:depth |

### uro:WaterBodyInlandFloodingRiskAttribute

内水浸水想定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 設定等名称 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自分類） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |

### uro:WaterBodyRiverFloodingRiskAttribute

洪水浸水想定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code | 指定河川名称 | uro:description |
| rank | Code | 浸水ランク | uro:rank |
| rankOrg | Code | 浸水ランク（独自分類） | uro:rankOrg |
| depth | Measure | 浸水深 | uro:depth |
| adminType | Code | 指定機関 | uro:adminType |
| scale | Code | 規模 | uro:scale |
| duration | Measure | 浸水継続時間 | uro:duration |

### uro:WaterBodyTsunamiRiskAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| description | Code |  | uro:description |
| rank | Code |  | uro:rank |
| rankOrg | Code |  | uro:rankOrg |
| depth | Measure |  | uro:depth |

### uro:WaterwayDetailAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeId | String | 航路番号 | uro:routeId |
| routeDirection | Code | 進行方向 | uro:routeDirection |
| minimumWidth | Measure | 最小幅員 | uro:minimumWidth |
| maximumWidth | Measure | 最大幅員 | uro:maximumWidth |
| length | Measure | 航路延長 | uro:length |
| navigation | String | 航法 | uro:navigation |
| plannedDepth | Measure | 計画水深 | uro:plannedDepth |
| speedLimit | Measure | 速力制限 | uro:speedLimit |
| targetShipType | String | 対象船型 | uro:targetShipType |

### urf:Boundary

境界

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 分類 | urf:class |
| function | Code | 境界線の確定根拠 | urf:function |
| usage | Code | 区域の種類 | urf:usage |
| offset | Measure | オフセット値 | urf:offset |
| offsetDirection | String | オフセット値の方向 | urf:offsetDirection |

### urf:ParkAttribute

公園属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parkTypeNumber | Code | 区分 | urf:parkTypeNumber |
| parkSizeNumber | Code | 規模 | urf:parkSizeNumber |
| parkSerialNumber | String | 一連番号 | urf:parkSerialNumber |

### urf:ParkingPlaceAttribute

駐車場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| storeysAboveGround | NonNegativeInteger | 地上階数 | urf:storeysAboveGround |
| storeysBelowGround | NonNegativeInteger | 地下階数 | urf:storeysBelowGround |

### urf:SewerSystemAttribute

下水道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| startLocation | String | 起点 | urf:startLocation |
| endLocation | String | 終点 | urf:endLocation |
| systemType | Code | 種別 | urf:systemType |
| drainageArea | String | 排水区域 | urf:drainageArea |

### urf:StructureDetails

構造詳細

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| startLocation | String | 起点 | urf:startLocation |
| endLocation | String | 終点 | urf:endLocation |
| viaLocations | String | 経過地 | urf:viaLocations |
| length | Measure | 延長 | urf:length |
| structureType | Code | 構造の形式 | urf:structureType |
| minimumWidth | Measure | 最小幅員 | urf:minimumWidth |
| maximumWidth | Measure | 最大幅員 | urf:maximumWidth |
| standardWidth | Measure | 標準幅員 | urf:standardWidth |
| crossType | Code | 交差種別 | urf:crossType |

### urf:UrbanRapidTransitRailroadAttribute

都市高速鉄道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structureType | Code | 構造種別 | urf:structureType |
| crossType | Code | 交差種別 | urf:crossType |
| structuralDetails | urf:StructureDetails |  | urf:structuralDetails |

### urf:UrbanRoadAttribute

道路属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeTypeNumber | Code | 区分 | urf:routeTypeNumber |
| routeSizeNumber | Code | 規模 | urf:routeSizeNumber |
| routeSerialNumber | String | 一連番号 | urf:routeSerialNumber |
| roadType | Code | 道路の種類 | urf:roadType |
| numberOfLanes | Integer | 車線数 | urf:numberOfLanes |
| roadStructure | String | 道路の構造 | urf:roadStructure |
| structureType | Code | 構造種別 | urf:structureType |
| crossType | Code | 交差種別 | urf:crossType |
| trafficPlazas | Code | 交通広場の有無 | urf:trafficPlazas |
| structuralDetails | urf:StructureDetails | 構造の内訳 | urf:structuralDetails |

### urf:VehicleTerminalAttribute

自動車ターミナル属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| terminalType | Code | 自動車ターミナルの種類 | urf:terminalType |

### urf:WaterWorksAttribute

水道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| startLocation | String | 起点 | urf:startLocation |
| endLocation | String | 終点 | urf:endLocation |

