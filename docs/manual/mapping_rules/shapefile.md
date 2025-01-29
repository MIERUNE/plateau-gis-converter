## 地物 (Feature stereotype)

### bldg:Building

建築物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | bldg:usage |
| yrConstrtn | String | 建築年 | bldg:yearOfConstruction |
| yrDemolitn | String | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measurHgt | Measure | 計測高さ | bldg:measuredHeight |
| strysAbvG | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| strysBlwG | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| stryHtAbvG | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| stryHtBlwG | String |  | bldg:storeyHeightsBelowGround |
| outBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 建物付属物 | bldg:outerBuildingInstallation |
| intBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | bldg:boundedBy |
| intrirRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) | 部屋 | bldg:interiorRoom |
| bldgPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) | 建物部品 | bldg:consistsOfBuildingPart |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) | 住所 | bldg:address |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:buildingDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク | uro:buildingDisasterRiskAttribute |
| dmAttr | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:bldgDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:bldgFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:bldgFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:bldgFacilityTypeAttribute |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) | 拡張属性 | uro:keyValuePairAttribute |
| ReIDAttr | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) | 不動産ID | uro:bldgRealEstateIDAttribute |
| bldgUC | JSON (<code><a href="#urobuildingusecaseattribute">uro:BuildingUsecaseAttribute</a>[]</code>) |  | uro:bldgUsecaseAttribute |
| detail | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) | 建物利用現況 | uro:buildingDetailAttribute |
| IDAttr | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a>[]</code>) | 建物識別情報 | uro:buildingIDAttribute |
| ifcBldg | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBuildingAttribute |
| indoorBldg | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorBuildingAttribute |
| lgCustFacl | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:BuildingFurniture


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | bldg:usage |
| ifcBldFrn | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |  | uro:ifcBuildingFurnitureAttribute |
| indoorFrn | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorFutnitureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:BuildingInstallation

建築物付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | bldg:usage |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | bldg:boundedBy |
| ifcBldInst | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBuildingInstallationAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:BuildingPart

建築物部品

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | bldg:usage |
| yrConstrtn | String | 建築年 | bldg:yearOfConstruction |
| yrDemolitn | String | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measurHgt | Measure | 計測高さ | bldg:measuredHeight |
| strysAbvG | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| strysBlwG | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| stryHtAbvG | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| stryHtBlwG | String |  | bldg:storeyHeightsBelowGround |
| outBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 建物付属物 | bldg:outerBuildingInstallation |
| intBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | bldg:boundedBy |
| intrirRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) | 部屋 | bldg:interiorRoom |
| bldgPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) | ー | bldg:consistsOfBuildingPart |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) | 住所 | bldg:address |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | ー | uro:buildingDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | ー | uro:buildingDisasterRiskAttribute |
| dmAttr | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:bldgDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:bldgFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:bldgFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:bldgFacilityTypeAttribute |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) | ー | uro:keyValuePairAttribute |
| ReIDAttr | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) |  | uro:bldgRealEstateIDAttribute |
| bldgUC | JSON (<code><a href="#urobuildingusecaseattribute">uro:BuildingUsecaseAttribute</a>[]</code>) |  | uro:bldgUsecaseAttribute |
| detail | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) | 建物利用現況 | uro:buildingDetailAttribute |
| IDAttr | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a>[]</code>) | 建物識別属性 | uro:buildingIDAttribute |
| ifcBldg | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBuildingAttribute |
| indoorBldg | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内属性 | uro:indoorBuildingAttribute |
| lgCustFacl | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| ifcOpening | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcOpeningAttribute |
| indoorOpng | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorOpeningAttribute |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |  | bldg:address |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:GroundSurface


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |  | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |  | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:OuterCeilingSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:OuterFloorSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:Room

部屋

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | bldg:usage |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | bldg:boundedBy |
| intFurn | JSON (<code><a href="#bldgbuildingfurniture">bldg:BuildingFurniture</a>[]</code>) | 家具 | bldg:interiorFurniture |
| roomInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 屋内付属物 | bldg:roomInstallation |
| ifcRoom | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcRoomAttribute |
| indoorRoom | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorRoomAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) | 開口部 | bldg:opening |
| ifcSurface | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBoundarySurfaceAttribute |
| indoorSurf | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorBoundarySurfaceAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### bldg:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| ifcOpening | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcOpeningAttribute |
| indoorOpng | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorOpeningAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:AuxiliaryTrafficArea

交通補助領域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| surfaceMat | Code | 舗装種類 | tran:surfaceMaterial |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:Railway

鉄道

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| tfcArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) | 交通領域 | tran:trafficArea |
| auxTfcArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) | 交通補助領域 | tran:auxiliaryTrafficArea |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tranDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tranDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tranFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tranFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tranFacilityTypeAttribute |
| tranKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tranKeyValuePairAttribute |
| tranUC | JSON (<code><a href="#urotrafficobjectusecaseattribute">uro:TrafficObjectUsecaseAttribute</a>[]</code>) |  | uro:tranUsecaseAttribute |
| route | JSON (<code><a href="#urorailwayrouteattribute">uro:RailwayRouteAttribute</a>[]</code>) | 鉄道路線属性 | uro:railwayRouteAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:Road

道路

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| tfcArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) | 交通領域 | tran:trafficArea |
| auxTfcArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) | 交通補助領域 | tran:auxiliaryTrafficArea |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tranDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tranDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tranFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tranFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tranFacilityTypeAttribute |
| tranKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tranKeyValuePairAttribute |
| tranUC | JSON (<code><a href="#urotrafficobjectusecaseattribute">uro:TrafficObjectUsecaseAttribute</a>[]</code>) |  | uro:tranUsecaseAttribute |
| roadStatus | JSON (<code><a href="#uroroadtype">uro:RoadType</a>[]</code>) |  | uro:roadStatus |
| roadStruct | JSON (<code><a href="#uroroadstructureattribute">uro:RoadStructureAttribute</a>[]</code>) | 道路構造属性 | uro:roadStructureAttribute |
| tfcVolume | JSON (<code><a href="#urotrafficvolumeattribute">uro:TrafficVolumeAttribute</a>[]</code>) | 交通量属性 | uro:trafficVolumeAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:Square

広場

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| tfcArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) | 交通領域 | tran:trafficArea |
| auxTfcArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) | 交通補助領域 | tran:auxiliaryTrafficArea |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tranDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tranDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tranFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tranFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tranFacilityTypeAttribute |
| tranKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tranKeyValuePairAttribute |
| tranUC | JSON (<code><a href="#urotrafficobjectusecaseattribute">uro:TrafficObjectUsecaseAttribute</a>[]</code>) |  | uro:tranUsecaseAttribute |
| sqUrbanPln | JSON (<code><a href="#urosquareurbanplanattributeproperty">uro:SquareUrbanPlanAttributeProperty</a>[]</code>) | 都市計画施設現況属性 | uro:squareUrbanPlanAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:Track

徒歩道

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| tfcArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) | 交通領域 | tran:trafficArea |
| auxTfcArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) | 交通補助領域 | tran:auxiliaryTrafficArea |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tranDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tranDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tranFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tranFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tranFacilityTypeAttribute |
| tranKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tranKeyValuePairAttribute |
| tranUC | JSON (<code><a href="#urotrafficobjectusecaseattribute">uro:TrafficObjectUsecaseAttribute</a>[]</code>) |  | uro:tranUsecaseAttribute |
| track | JSON (<code><a href="#urotrackattribute">uro:TrackAttribute</a>[]</code>) | 徒歩道属性 | uro:trackAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tran:TrafficArea

交通領域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| surfaceMat | Code | 舗装種類 | tran:surfaceMaterial |
| track | JSON (<code><a href="#urorailwaytrackattribute">uro:RailwayTrackAttribute</a>[]</code>) | 軌道中心線線形情報 | uro:railwayTrackAttribute |
| tfcArStruc | JSON (<code><a href="#urotrafficareastructureattribute">uro:TrafficAreaStructureAttribute</a>[]</code>) | 道路構造属性 | uro:trafficAreaStructureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### luse:LandUse

土地利用

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 土地利用区分 | luse:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | luse:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | luse:usage |
| ifcLandUse | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |  | uro:ifcLandUseAttribute |
| detail | JSON (<code><a href="#urolandusedetailattribute">uro:LandUseDetailAttribute</a>[]</code>) |  | uro:landUseDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:luseDataQualityAttribute |
| luseDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:luseDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:luseFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:luseFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:luseFacilityTypeAttribute |
| luseKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:luseKeyValuePairAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:Bridge

橋梁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| yrConstrtn | String | 建設年 | brid:yearOfConstruction |
| yrDemolitn | String | 解体年 | brid:yearOfDemolition |
| isMovable | Boolean | 可動橋区分 | brid:isMovable |
| ouBridCons | JSON (<code><a href="#bridbridgeconstructionelement">brid:BridgeConstructionElement</a>[]</code>) | 橋梁部材 | brid:outerBridgeConstruction |
| ouBridInst | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) | 橋梁付属物 | brid:outerBridgeInstallation |
| intBrdInst | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) | 橋梁内部付属物 | brid:interiorBridgeInstallation |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | brid:boundedBy |
| intBrdRoom | JSON (<code><a href="#bridbridgeroom">brid:BridgeRoom</a>[]</code>) | 橋梁内部 | brid:interiorBridgeRoom |
| bridgePart | JSON (<code><a href="#bridbridgepart">brid:BridgePart</a>[]</code>) | 橋梁部分 | brid:consistsOfBridgePart |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) | 住所 | brid:address |
| Base | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a>[]</code>) | 構造物基本属性 | uro:bridBaseAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:bridDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク属性 | uro:bridDisasterRiskAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:bridDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:bridFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:bridFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:bridFacilityTypeAttribute |
| Functional | JSON (<code><a href="#urobridgefunctionalattribute">uro:BridgeFunctionalAttribute</a>[]</code>) | 橋梁機能属性 | uro:bridFunctionalAttribute |
| bridKeyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:bridKeyValuePairAttribute |
| RiskAssess | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a>[]</code>) | 構造物リスク評価属性 | uro:bridRiskAssessmentAttribute |
| Struct | JSON (<code><a href="#urobridgestructureattribute">uro:BridgeStructureAttribute</a>[]</code>) | 橋梁構造属性 | uro:bridStructureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:BridgeConstructionElement

橋梁建設部材

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | brid:boundedBy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:BridgeFurniture

設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:BridgeInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | brid:boundedBy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:BridgePart

橋梁部分

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| yrConstrtn | String | 建設年 | brid:yearOfConstruction |
| yrDemolitn | String | 解体年 | brid:yearOfDemolition |
| isMovable | Boolean | 可動橋区分 | brid:isMovable |
| ouBridCons | JSON (<code><a href="#bridbridgeconstructionelement">brid:BridgeConstructionElement</a>[]</code>) | 橋梁部材 | brid:outerBridgeConstruction |
| ouBridInst | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) | 橋梁付属物 | brid:outerBridgeInstallation |
| intBrdInst | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) | 橋梁内部付属物 | brid:interiorBridgeInstallation |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | brid:boundedBy |
| intBrdRoom | JSON (<code><a href="#bridbridgeroom">brid:BridgeRoom</a>[]</code>) | 橋梁内部 | brid:interiorBridgeRoom |
| bridgePart | JSON (<code><a href="#bridbridgepart">brid:BridgePart</a>[]</code>) | 橋梁部分 | brid:consistsOfBridgePart |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) | 住所 | brid:address |
| Base | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a>[]</code>) | 構造物基本属性 | uro:bridBaseAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:bridDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク属性 | uro:bridDisasterRiskAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:bridDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:bridFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:bridFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:bridFacilityTypeAttribute |
| Functional | JSON (<code><a href="#urobridgefunctionalattribute">uro:BridgeFunctionalAttribute</a>[]</code>) | 機能属性 | uro:bridFunctionalAttribute |
| bridKeyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:bridKeyValuePairAttribute |
| RiskAssess | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a>[]</code>) | 構造物リスク評価属性 | uro:bridRiskAssessmentAttribute |
| Struct | JSON (<code><a href="#urobridgestructureattribute">uro:BridgeStructureAttribute</a>[]</code>) | 構造属性 | uro:bridStructureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:BridgeRoom

内部空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | brid:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | brid:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | brid:usage |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | brid:boundedBy |
| intFurn | JSON (<code><a href="#bridbridgefurniture">brid:BridgeFurniture</a>[]</code>) | 設置物 | brid:interiorFurniture |
| roomInst | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) | 内部付属物 | brid:bridgeRoomInstallation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |  | brid:address |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) | 開口部 | brid:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### brid:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:CeilingSurface

天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:Door

扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:FloorSurface

床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:HollowSpace

内部空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tun:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tun:usage |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | tun:boundedBy |
| intFurn | JSON (<code><a href="#tuntunnelfurniture">tun:TunnelFurniture</a>[]</code>) | 設置物 | tun:interiorFurniture |
| hollwInstn | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) | 内部付属物 | tun:hollowSpaceInstallation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:InteriorWallSurface

内壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:Tunnel

トンネル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tun:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tun:usage |
| yrConstrtn | String | 建設年 | tun:yearOfConstruction |
| yrDemolitn | String | 解体年 | tun:yearOfDemolition |
| outTunInst | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) | トンネル付属物 | tun:outerTunnelInstallation |
| intTunInst | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) | トンネル内部付属物 | tun:interiorTunnelInstallation |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | tun:boundedBy |
| intHllwSpc | JSON (<code><a href="#tunhollowspace">tun:HollowSpace</a>[]</code>) | トンネル内部空間 | tun:interiorHollowSpace |
| tunPart | JSON (<code><a href="#tuntunnelpart">tun:TunnelPart</a>[]</code>) | トンネル部分 | tun:consistsOfTunnelPart |
| base | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a>[]</code>) | 構造物基本属性 | uro:tunBaseAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tunDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク属性 | uro:tunDisasterRiskAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tunDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tunFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tunFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tunFacilityTypeAttribute |
| Functional | JSON (<code><a href="#urotunnelfunctionalattribute">uro:TunnelFunctionalAttribute</a>[]</code>) | トンネル機能属性 | uro:tunFunctionalAttribute |
| tunKeyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tunKeyValuePairAttribute |
| RiskAssess | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a>[]</code>) | 構造物リスク評価属性 | uro:tunRiskAssessmentAttribute |
| structure | JSON (<code><a href="#urotunnelstructureattribute">uro:TunnelStructureAttribute</a>[]</code>) | トンネル構造属性 | uro:tunStructureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:TunnelFurniture

設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tun:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tun:usage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:TunnelInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tun:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tun:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tun:usage |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | tun:boundedBy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:TunnelPart


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | tun:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | tun:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | tun:usage |
| yrConstrtn | String |  | tun:yearOfConstruction |
| yrDemolitn | String |  | tun:yearOfDemolition |
| outTunInst | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |  | tun:outerTunnelInstallation |
| intTunInst | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |  | tun:interiorTunnelInstallation |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) |  | tun:boundedBy |
| intHllwSpc | JSON (<code><a href="#tunhollowspace">tun:HollowSpace</a>[]</code>) |  | tun:interiorHollowSpace |
| tunPart | JSON (<code><a href="#tuntunnelpart">tun:TunnelPart</a>[]</code>) |  | tun:consistsOfTunnelPart |
| base | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a>[]</code>) |  | uro:tunBaseAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:tunDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |  | uro:tunDisasterRiskAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |  | uro:tunDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |  | uro:tunFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |  | uro:tunFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |  | uro:tunFacilityTypeAttribute |
| Functional | JSON (<code><a href="#urotunnelfunctionalattribute">uro:TunnelFunctionalAttribute</a>[]</code>) |  | uro:tunFunctionalAttribute |
| tunKeyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tunKeyValuePairAttribute |
| RiskAssess | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a>[]</code>) |  | uro:tunRiskAssessmentAttribute |
| structure | JSON (<code><a href="#urotunnelstructureattribute">uro:TunnelStructureAttribute</a>[]</code>) |  | uro:tunStructureAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) | 開口部 | tun:opening |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### tun:Window

窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### frn:CityFurniture

都市設備

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### veg:PlantCover

植被

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:vegetationDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:vegDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:vegFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:vegFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:vegFacilityTypeAttribute |
| vegKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:vegKeyValuePairAttribute |
| class | Code | 分類 | veg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | veg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | veg:usage |
| avgHeight | Measure | 平均高 | veg:averageHeight |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### veg:SolitaryVegetationObject

単独木

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:vegetationDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:vegDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:vegFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:vegFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:vegFacilityTypeAttribute |
| vegKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:vegKeyValuePairAttribute |
| class | Code | 分類 | veg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | veg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | veg:usage |
| species | Code | 樹種 | veg:species |
| height | Measure | 樹高 | veg:height |
| trunkDiam | Measure |  | veg:trunkDiameter |
| crownDia | Measure |  | veg:crownDiameter |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### wtr:WaterBody

洪水浸水想定区域、津波浸水想定、高潮浸水想定区域、内水浸水想定区域、海、潮汐水域、水路、河川/小川、湖、滝、湿地・沼地、浸水域、貯水池、不明

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | wtr:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | wtr:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | wtr:usage |
| boundedBy | JSON (<code><a href="#wtr-waterboundarysurfaceproperty">wtr:_WaterBoundarySurfaceProperty</a>[]</code>) |  | wtr:boundedBy |
| floodRisk | JSON (<code><a href="#urofloodingriskattributeproperty">uro:FloodingRiskAttributeProperty</a>[]</code>) | 災害リスク | uro:floodingRiskAttribute |
| bodyDetail | JSON (<code><a href="#urowaterbodydetailattribute">uro:WaterBodyDetailAttribute</a>[]</code>) | 水部詳細属性 | uro:waterBodyDetailAttribute |
| wtrDQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:wtrDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:wtrDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:wtrFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:wtrFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:wtrFacilityTypeAttribute |
| wtrKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:wtrKeyValuePairAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### wtr:WaterClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### wtr:WaterGroundSurface

水底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### wtr:WaterSurface

水面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| waterLevel | Code |  | wtr:waterLevel |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### dem:BreaklineRelief

点群地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:demDmAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### dem:MassPointRelief

点群地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:demDmAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### dem:RasterRelief


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger |  | dem:lod |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |  | uro:demDmAttribute |
| grid | JSON (<code><a href="#gmlrectifiedgridcoverage">gml:RectifiedGridCoverage</a></code>) |  | dem:grid |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### dem:ReliefFeature

地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| relefCmpnt | JSON (<code><a href="#dem-reliefcomponentproperty">dem:_ReliefComponentProperty</a>[]</code>) | 地形構成要素 | dem:reliefComponent |
| demDQual | JSON (<code><a href="#urodemdataqualityattribute">uro:demDataQualityAttribute</a></code>) |  | uro:demDataQualityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### dem:TINRelief

TIN地形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| lod | NonNegativeInteger | lod | dem:lod |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:demDmAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### grp:CityObjectGroup


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | grp:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | grp:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | grp:usage |
| grpMember | JSON (<code><a href="#grp-cityobjectorref">grp:_CityObjectOrRef</a>[]</code>) |  | grp:groupMember |
| parent | JSON (<code><a href="#grp-cityobjectorref">grp:_CityObjectOrRef</a></code>) |  | grp:parent |
| FYPublicat | JSON (<code><a href="#string">String</a>[]</code>) |  | uro:fiscalYearOfPublication |
| ifcBldStry | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |  | uro:ifcBuildingStoreyAttribute |
| indoorStry | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |  | uro:indoorStoreyAttribute |
| language | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:language |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### grp:_CityObjectOrRef


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| href | String |  | @xlink:href |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gen:GenericCityObject


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | gen:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | gen:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | gen:usage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Appurtenance

ユーティリティ設備

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| prevLink | JSON (<code><a href="#string">String</a>[]</code>) | 前のリンク | uro:previousLink |
| nextLink | JSON (<code><a href="#string">String</a>[]</code>) | 次のリンク | uro:nextLink |
| rotatAngle | Double | 回転角度 | uro:rotationAngle |
| appurteTy | Code | 設備区分 | uro:appurtenanceType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Cable

ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ClosureSurface

閉鎖面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ConstructionInstallation

付属物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | uro:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | uro:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | uro:usage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Duct

トラフ、洞道、鞘管、CAB、情報BOX

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| width | Measure | 外側幅 | uro:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ElectricityCable

電力ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:GroundSurface

底面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Handhole

ハンドホール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| containrTy | Code | 構造物種類 | uro:containerType |
| innerDiameterLong | Measure |  | uro:innerDiameterLong |
| outerDiameterLong | Measure |  | uro:outerDiameterLong |
| innerDiameterShort | Measure |  | uro:innerDiameterShort |
| outerDiameterShort | Measure |  | uro:outerDiameterShort |
| depth | Measure | 深さ | uro:depth |
| appurte | JSON (<code><a href="#string">String</a>[]</code>) | 識別子 | uro:appurtenance |
| rotatAngle | Double | 回転角度 | uro:rotationAngle |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Manhole

マンホール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| containrTy | Code | 構造物種類 | uro:containerType |
| innerDiameterLong | Measure |  | uro:innerDiameterLong |
| outerDiameterLong | Measure |  | uro:outerDiameterLong |
| innerDiameterShort | Measure |  | uro:innerDiameterShort |
| outerDiameterShort | Measure |  | uro:outerDiameterShort |
| depth | Measure | 深さ | uro:depth |
| appurte | JSON (<code><a href="#string">String</a>[]</code>) | 識別子 | uro:appurtenance |
| rotatAngle | Double | 回転角度 | uro:rotationAngle |
| elevation | Measure |  | uro:elevation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:OilGasChemicalsPipe


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |  | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |  | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |  | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |  | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |  | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code |  | uro:occupierType |
| occupierNm | Code |  | uro:occupierName |
| year | String |  | uro:year |
| yearType | Code |  | uro:yearType |
| admin | Code |  | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |  | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |  | uro:thematicShape |
| rtStrtNode | String |  | uro:routeStartNode |
| startNode | String |  | uro:startNode |
| rtEndNode | String |  | uro:routeEndNode |
| endNode | String |  | uro:endNode |
| depth | Measure |  | uro:depth |
| minDepth | Measure |  | uro:minDepth |
| maxDepth | Measure |  | uro:maxDepth |
| maxWidth | Measure |  | uro:maxWidth |
| offset | Measure |  | uro:offset |
| material | Code |  | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |  | uro:lengthAttribute |
| innerDiameter | Measure |  | uro:innerDiameter |
| outerDiameter | Measure |  | uro:outerDiameter |
| sleeveType | Code |  | uro:sleeveType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:OtherConstruction

その他の構造物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| cndtnCnstr | String | 稼働状況 | uro:conditionOfConstruction |
| dateCnstr | Date | 完成年月日 | uro:dateOfConstruction |
| dateDemol | Date | 解体年月日 | uro:dateOfDemolition |
| cnstrEvent | JSON (<code><a href="#uroconstructionevent">uro:ConstructionEvent</a>[]</code>) | 建設イベント | uro:constructionEvent |
| elevation | JSON (<code><a href="#uroelevation">uro:Elevation</a>[]</code>) | 標高 | uro:elevation |
| height | JSON (<code><a href="#uroheight">uro:Height</a>[]</code>) | 高さ | uro:height |
| occupancy | JSON (<code><a href="#urooccupancy">uro:Occupancy</a>[]</code>) | 占有状況 | uro:occupancy |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:consFacilityTypeAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:consFacilityIdAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:consFacilityAttribute |
| Base | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) | 構造物基本情報 | uro:consBaseAttribute |
| structure | JSON (<code><a href="#uroconstructionstructureattributeproperty">uro:ConstructionStructureAttributeProperty</a></code>) | 構造属性 | uro:consStructureAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク属性 | uro:consDisasterRiskAttribute |
| dmAttr | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:consDmAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | 品質属性 | uro:consDataQualityAttribute |
| boundedBy | JSON (<code><a href="#uro-boundarysurfaceproperty">uro:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | uro:boundedBy |
| installatn | JSON (<code><a href="#uroconstructioninstallation">uro:ConstructionInstallation</a>[]</code>) | 付属物 | uro:constructionInstallation |
| class | Code | 分類 | uro:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | uro:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | uro:usage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:OuterCeilingSurface

屋外床面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:OuterFloorSurface

屋外天井面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Pipe

管きょ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| innerDiameter | Measure |  | uro:innerDiameter |
| outerDiameter | Measure |  | uro:outerDiameter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RoofSurface

屋根面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:SewerPipe

下水道管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| innerDiameter | Measure |  | uro:innerDiameter |
| outerDiameter | Measure |  | uro:outerDiameter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |
| slope | Measure | 勾配 | uro:slope |
| invertElevationUpstream | Measure |  | uro:invertElevationUpstream |
| invertElevationDownstream | Measure |  | uro:invertElevationDownstream |
| flowDir | Boolean |  | uro:flowDirection |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TelecommunicationsCable

通信ケーブル

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| columns | Integer | 列数 | uro:columns |
| rows | Integer | 段数 | uro:rows |
| cables | Integer | 条数 | uro:cables |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ThermalPipe

熱配管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| innerDiameter | Measure |  | uro:innerDiameter |
| outerDiameter | Measure |  | uro:outerDiameter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:UndergroundBuilding

地下街

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | bldg:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | bldg:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | bldg:usage |
| yrConstrtn | String | 建築年 | bldg:yearOfConstruction |
| yrDemolitn | String | 解体年 | bldg:yearOfDemolition |
| roofType | Code | 屋根の種別 | bldg:roofType |
| measurHgt | Measure | 計測高さ | bldg:measuredHeight |
| strysAbvG | NonNegativeInteger | 地上階数 | bldg:storeysAboveGround |
| strysBlwG | NonNegativeInteger | 地下階数 | bldg:storeysBelowGround |
| stryHtAbvG | String | 地下階高さリスト | bldg:storeyHeightsAboveGround |
| stryHtBlwG | String |  | bldg:storeyHeightsBelowGround |
| outBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 建物付属物 | bldg:outerBuildingInstallation |
| intBldInst | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) | 屋内付属物 | bldg:interiorBuildingInstallation |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) | 境界面 | bldg:boundedBy |
| intrirRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) | 部屋 | bldg:interiorRoom |
| bldgPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) | 建物部品 | bldg:consistsOfBuildingPart |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) | 住所 | bldg:address |
| bldgDQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:bldgDataQualityAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:buildingDataQualityAttribute |
| disastRisk | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) | 災害リスク | uro:buildingDisasterRiskAttribute |
| dmAttr | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:bldgDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:bldgFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:bldgFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:bldgFacilityTypeAttribute |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) | 拡張属性 | uro:keyValuePairAttribute |
| ReIDAttr | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) |  | uro:bldgRealEstateIDAttribute |
| bldgUC | JSON (<code><a href="#urobuildingusecaseattribute">uro:BuildingUsecaseAttribute</a>[]</code>) |  | uro:bldgUsecaseAttribute |
| detail | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) | 建物利用現況 | uro:buildingDetailAttribute |
| IDAttr | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a>[]</code>) | 建物識別情報 | uro:buildingIDAttribute |
| ifcBldg | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) | IFC属性 | uro:ifcBuildingAttribute |
| indoorBldg | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) | 屋内ナビゲーション属性 | uro:indoorBuildingAttribute |
| lgCustFacl | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) | 大規模小売店舗等の立地状況 | uro:largeCustomerFacilityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:WallSurface

外壁面

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:WaterPipe

水道管

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | frn:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | frn:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | frn:usage |
| detail | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) | 都市設備詳細属性 | uro:cityFurnitureDetailAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:cityFurnitureDataQualityAttribute |
| frnDm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:frnDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:frnFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設属性 | uro:frnFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設識別属性 | uro:frnFacilityTypeAttribute |
| frnKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:frnKeyValuePairAttribute |
| occupierTy | Code | 事業者種類 | uro:occupierType |
| occupierNm | Code | 事業者名 | uro:occupierName |
| year | String | 埋設年度 | uro:year |
| yearType | Code | 埋設年度の確からしさ | uro:yearType |
| admin | Code | 主管事業者 | uro:administrator |
| offsetDpth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) | オフセットデプス情報 | uro:offsetDepth |
| thmShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) | 主題図形 | uro:thematicShape |
| rtStrtNode | String | 路線開始ノード | uro:routeStartNode |
| startNode | String | 開始ノード | uro:startNode |
| rtEndNode | String | 路線終了ノード | uro:routeEndNode |
| endNode | String | 終了ノード | uro:endNode |
| depth | Measure | 土被り深さ | uro:depth |
| minDepth | Measure | 最小土被り深さ | uro:minDepth |
| maxDepth | Measure | 最大土被り深さ | uro:maxDepth |
| maxWidth | Measure | 最大幅 | uro:maxWidth |
| offset | Measure | オフセット量 | uro:offset |
| material | Code | 管きょ材質 | uro:material |
| len | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) | 長さ情報 | uro:lengthAttribute |
| innerDiameter | Measure |  | uro:innerDiameter |
| outerDiameter | Measure |  | uro:outerDiameter |
| sleeveType | Code | 被覆区分 | uro:sleeveType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Waterway

航路

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | tran:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | tran:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | tran:usage |
| tfcArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) | 交通領域 | tran:trafficArea |
| auxTfcArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) | 交通補助領域 | tran:auxiliaryTrafficArea |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) | データ品質 | uro:tranDataQualityAttribute |
| dm | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) | 図式属性 | uro:tranDmAttribute |
| facility | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) | 施設詳細属性 | uro:tranFacilityAttribute |
| facilityId | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) | 施設識別属性 | uro:tranFacilityIdAttribute |
| facilityTy | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) | 施設属性 | uro:tranFacilityTypeAttribute |
| tranKVPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | uro:tranKeyValuePairAttribute |
| tranUC | JSON (<code><a href="#urotrafficobjectusecaseattribute">uro:TrafficObjectUsecaseAttribute</a>[]</code>) |  | uro:tranUsecaseAttribute |
| detail | JSON (<code><a href="#urowaterwaydetailattribute">uro:WaterwayDetailAttribute</a></code>) | 航路属性 | uro:waterwayDetailAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:demDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | uro:DataQualityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:Agreement


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| applicaAr | Measure |  | urf:applicableArea |
| expiration | Date |  | urf:expiration |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:AircraftNoiseControlZone

航空機騒音障害防止地区又は航空機騒音障害防止特別地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:AreaClassification

区域区分

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| population | Integer | 人口 | urf:population |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveFacilitiesForReconstruction

一団地の復興拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| housFacl | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| spBusiFacl | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| pubFaclty | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilFaclty | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveFacilitiesForReconstructionAndRevitalization

一団地の復興再生拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| housFacl | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| spBusiFacl | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| pubFaclty | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilFaclty | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveFacilitiesForTsunamiDisasterPrevention

一団地の津波防災拠点市街地形成施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| housFacl | String | 住宅施設の位置及び規模 | urf:housingFacilities |
| spBusiFacl | String | 特定業務施設の位置及び規模 | urf:supecificBusinessFacilities |
| pubFaclty | String | 公共施設の位置及び規模 | urf:publicFacilities |
| utilFaclty | String | 公益的施設の位置及び伊保 | urf:utilityFacilities |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveGovernmentAndPublicOfficeFacilities

一団地の官公庁施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| CovRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| flrArRate | Double | 容積率の限度 | urf:floorAreaRate |
| pubFclAPol | String | 公益的施設、住宅及び公共施設の配置方針 | urf:publicFacilitiesAllocationPolicy |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveHousingFacilities

一団地の住宅施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| CovRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| flrArRate | Double | 容積率の限度 | urf:floorAreaRate |
| #LRHousing | Integer | 低層住宅の予定戸数 | urf:numberOfLowRiseHousing |
| #MRHousing | Integer | 中層住宅の予定戸数 | urf:numberOfMiddleRiseHousing |
| #HRHousing | Integer | 高層住宅の予定戸数 | urf:numberOfHighRiseHousing |
| total#Hous | Integer | 住宅予定数の合計 | urf:totalNumberOfHousing |
| pubFclAPol | String | 住宅及び公共施設の配置方針 | urf:publicFacilitiesAllocationPolicy |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:CollectiveUrbanDisasterPreventionFacilities

一団地の都市安全確保拠点施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| spUtilPubF | String | 特定公益施設及び公共施設の位置及び規模 | urf:specificUtilityAndPublicFacilities |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低 | urf:minimumBuildingHeight |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ConservationZoneForClustersOfTraditionalStructures

伝統的建造物群保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DisasterPreventionBlockImprovementProject

防災街区整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| diPrPbFlAl | String | 防災公共施設の配置及び規模 | urf:disasterPreventionPublicFacilityAllocation |
| othPbFcAlc | String | その他の公共施設の配置及び規模 | urf:otherPublicFacilityAllocation |
| devPlan | String | 防災施設建築物の整備に関する計画 | urf:developmentPlan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DisasterPreventionBlockImprovementZonePlan

防災街区整備地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 方針 | urf:policy |
| distDevPln | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) | 特定建築物地区整備計画及び防災街区整備地区整備計画 | urf:districtDevelopmentPlan |
| promDistr | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) | nan | urf:promotionDistrict |
| zDiPrFcAlc | String | 地区防災施設の区域。 | urf:zonalDisasterPreventionFacilitiesAllocation |
| spZoDiPrFA | String | 特定地区防災施設の区域。 | urf:specifiedZonalDisasterPreventionFacilitiesAllocation |
| zoneDPFacl | JSON (<code><a href="#urfzonaldisasterpreventionfacility">urf:ZonalDisasterPreventionFacility</a>[]</code>) | 地区防災施設の区域及び特定地区防災施設 | urf:zonalDisasterPreventionFacilities |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistributionBusinessPark

流通業務団地

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| dstrBusiPk | String | 流通業務施設の敷地の位置及び規模 | urf:distributionBusinessPark |
| pubUtilFcl | String | 公共施設及び公益施設の位置及び規模 | urf:publicAndUtilityFacilities |
| CovRate | Double | 建ぺい率の限度 | urf:buildingCoverageRate |
| flrArRate | Double | 容積率の限度 | urf:floorAreaRate |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistributionBusinessZone

流通業務地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| gdlnPubDte | Date | 基本方針が定められた日 | urf:guidelinePublicationDate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:District

地区（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| Rstr | String | 建築物に関する制限 | urf:buildingRestrictions |
| useRestrct | String | 建築物の用途の制限 | urf:useRestrictions |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建ぺい率の最高限度 | urf:maximumBuildingCoverageRate |
| minBCR | Double | 建ぺい率の最低限度 | urf:minimumBuildingCoverageRate |
| minSiteAr | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| minBldgAr | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| minGndHgt | Measure |  | urf:minimumGroundHeight |
| setbckSize | String | 壁面の後退距離 | urf:setbackSize |
| strPlcRstr | String |  | urf:structurePlacementRestrictions |
| maxBldgHgt | Measure | 建築物の高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 建築物の高さの最低限度 | urf:minimumBuildingHeight |
| minFlrHgt | Measure | 床面の高さの最低限度 | urf:minimumFloorHeight |
| DesignRstr | String | 建築物の形態及び意匠にかかる制限 | urf:buildingDesignRestriction |
| minGreenRt | Double | 最低限度の緑被率 | urf:minimumGreeningRate |
| fenceGdlne | String | 垣およびさくの構造にかかる制限 | urf:fenceGuideline |
| rsnFirePr | String | 防火上の必要な制限 | urf:restrictionsForFireProtection |
| rsnNoisePr | String | 御盤上又は遮音上必要な制限 | urf:restrictionsForNoiseProtection |
| minFrntgRt | Double | 間口率の最低限度 | urf:minimumFrontageRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictDevelopmentPlan

地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号 | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 地区 | urf:district |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictFacility

地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan

防災街区整備地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 防災地区 | urf:district |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict

歴史的風致維持向上地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 地区 | urf:district |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictPlan

地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| distDevPln | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) | 地区整備計画 | urf:districtDevelopmentPlan |
| promDistr | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) | 促進区 | urf:promotionDistrict |
| faclAlloc | String | 施設の配置及び方針 | urf:facilityAllocation |
| lusePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:DistrictsAndZones

地域地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:EducationalAndCulturalFacility

教育文化施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ExceptionalFloorAreaRateDistrict

特例容積率適用地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| HeightLim | Measure | 建築物の高さの最高限度 | urf:buildingHeightLimits |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:FirePreventionDistrict

防火地域又は準防火地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 防火地域又は準防火地域の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:FireProtectionFacility

防火施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:FloodPreventionFacility

防水施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:GlobalHubCityDevelopmentProject


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| implBody | String |  | urf:implementationBody |
| implPeriod | String |  | urf:implementationPeriod |
| plan | String |  | urf:plan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:GreenSpaceConservationDistrict

緑地保全地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:HeightControlDistrict

高度地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:HighLevelUseDistrict

高度利用地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| maxFAR | JSON (<code><a href="#double">Double</a>[]</code>) | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | JSON (<code><a href="#double">Double</a>[]</code>) | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | JSON (<code><a href="#double">Double</a>[]</code>) | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| minBldgAr | JSON (<code><a href="#measure">Measure</a>[]</code>) | 建築面積の最低限度 | urf:minimumBuildingArea |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:HighRiseResidentialAttractionDistrict

高層住居誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| flrArRate | Double | 容積率 | urf:floorAreaRate |
| maxBCR | Double | 建蔽率 | urf:maximumBuildingCoverageRate |
| minSiteAr | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan

歴史的風致維持向上地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 方針 | urf:policy |
| distDevPln | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) | 地区整備計画 | urf:districtDevelopmentPlan |
| promDistr | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) | 促進区 | urf:promotionDistrict |
| lusePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:HousingControlArea

居住調整地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:IndustrialParkDevelopmentProject

工業団地造成事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| pubFaclAlc | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| residLuseP | String | 宅地の利用計画 | urf:residentialLandUsePlan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:LandReadjustmentProject

土地区画整理事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| pubFaclAlc | String | 公共施設の配置 | urf:publicFacilityAllocation |
| LotDev | String | 宅地の整備に関する事項 | urf:buildingLotDevelopment |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:LandReadjustmentPromotionArea

土地区画整理促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String | 住宅市街地としての開発方針 | urf:developmentPolicy |
| pubFclPln | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment

拠点業務市街地整備土地区画整理促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String | 開発の方針 | urf:developmentPolicy |
| pubFclPln | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:LandscapeZone

景観地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| DesignRstr | String | 建築物の形態にかかる制限 | urf:buildingDesignRestriction |
| maxBldgHgt | Measure | 高さの最高限度 | urf:maximumBuildingHeight |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| minSiteAr | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:MarketsSlaughterhousesCrematoria

市場、と畜場、火葬場（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:MedicalFacility

医療施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:NewHousingAndUrbanDevelopmentProject

新住宅市街地開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| housing | String | 住区 | urf:housing |
| pubFaclAlc | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| residLuseP | String | 宅地の利用計画 | urf:residentialLandUsePlan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:NewUrbanInfrastructureProject

新都市基盤整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| ldCenPbFcl | String | 公共施設の用に供するべき土地の区域 | urf:landForCentralPublicFacilities |
| distrAlloc | String | 開発誘導地区の配置及び規模 | urf:districtsAllocation |
| lusePlan | String | 土地の利用計画 | urf:landUsePlan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:OpenSpaceForPublicUse

公共空地（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| park | JSON (<code><a href="#urfparkattribute">urf:ParkAttribute</a></code>) | 公園属性 | urf:parkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ParkingPlaceDevelopmentZone

駐車場整備地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:PortZone

臨港地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 分区 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| flrArRate | Double | 容積率 | urf:floorAreaRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:PrivateUrbanRenewalProjectPlan


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| developer | String |  | urf:developer |
| plan | String |  | urf:plan |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ProductiveGreenZone

生産緑地地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| zone# | String | 生産緑地区番号 | urf:zoneNumber |
| specificat | Code | 特定生産緑地指定の有無 | urf:specification |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ProjectPromotionArea

促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String | 開発の方針 | urf:developmentPolicy |
| pubFclPln | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:PromotionDistrict


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:QuasiUrbanPlanningArea

準都市計画区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積（全域） | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| population | Integer | 準都市計画区域内の総人口 | urf:population |
| cityArea | Measure | 準都市計画区域面積（市区町村内） | urf:cityArea |
| cityPop | Integer | 準都市計画区域内の人口（市区町村内） | urf:cityPopulation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:Regulation


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ResidenceAttractionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ResidentialBlockConstructionProject

住宅街区整備事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| pubFaclAlc | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| devPlan | String | 施設住宅の建設に関する計画 | urf:developmentPlan |
| siteArea | Measure | 敷地面積 | urf:siteArea |
| totalFlrAr | Measure | 延床面積 | urf:totalFloorArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ResidentialBlockConstructionPromotionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String |  | urf:developmentPolicy |
| pubFclPln | String |  | urf:publicFacilitiesPlans |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ResidentialEnvironmentImprovementDistrict

居住環境向上用途誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| useBeInduc | String | 誘導すべき用途 | urf:useToBeInduced |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| maxBCR | Double | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| maxBldgHgt | String | 高さの最高限度 | urf:maximumBuildingHeight |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| otherRstr | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RoadsideDistrictFacility

沿道地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RoadsideDistrictImprovementPlan

沿道地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 沿道地区 | urf:district |
| rsDFaclAlc | String | 沿道地区施設の配置及び規模 | urf:roadsideDistrictFacilitiesAllocation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RoadsideDistrictPlan

沿道地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| distDevPln | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) | 沿道地区整備計画 | urf:districtDevelopmentPlan |
| promDistr | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) | 沿道開発等促進区 | urf:promotionDistrict |
| faclAlloc | String |  | urf:facilitiesAllocation |
| lusePolicy | String | 土地利用に関する基本方針 | urf:landUsePolicy |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RuralDistrictFacility

集落地区施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RuralDistrictImprovementPlan

集落地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 集落地区 | urf:district |
| rurDFclAlc | String | 集落地区施設の配置及び規模 | urf:ruralDistrictFacilitiesAllocation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:RuralDistrictPlan

集落地区計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| objectives | String | 地区計画の目標 | urf:objectives |
| policy | String | 区域の整備、開発及び保全に関する方針 | urf:policy |
| distDevPln | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) | 集落地区整備計画 | urf:districtDevelopmentPlan |
| promDistr | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) | 促進区 | urf:promotionDistrict |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SandControlFacility

防砂施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScenicDistrict

風致地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 風致地区の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| CovRate | Double | 建蔽率 | urf:buildingCoverageRate |
| HeightLim | Measure | 高さの規制 | urf:buildingHeightLimits |
| wallSbDRd | Measure | 壁面から敷地境界までの距離（道路に接する部分） | urf:wallSetbackDistanceWithRoad |
| wallSbDAdj | Measure | 壁面から敷地境界までの距離（道路に接しない部分） | urf:wallSetbackDistanceWithAdjoiningLand |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities

一団地の官公庁施設の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForCollectiveHousingFacilities

一団地の住宅施設の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForDistributionBusinessPark

流通業務団地の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForIndustrialParkDevelopmentProjects

工業団地造成事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects

新住宅市街地開発事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForNewUrbanInfrastructureProjects

新都市基盤整備事業の予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ScheduledAreaForUrbanDevelopmentProject

予定区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 予定区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SedimentDisasterProneArea

土砂災害警戒区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日 | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号 | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 所在地 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| disasterTy | Code | 災害種別 | urf:disasterType |
| areaType | Code | 区域区分 | urf:areaType |
| zone# | String | 区域番号 | urf:zoneNumber |
| zoneName | String | 区域名 | urf:zoneName |
| status | Code | 特別警戒未指定フラグ | urf:status |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SnowProtectionFacility

防雪施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SocialWelfareFacility

社会福祉施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialGreenSpaceConservationDistrict

特別緑地保全地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| requiremnt | Code | 指定の要件 | urf:requirement |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialUrbanRenaissanceDistrict

都市再生特別地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| useBeInduc | String | 誘導すべき用途 | urf:useToBeInduced |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| maxBCR | Double | 建蔽率の最高限度 | urf:maximumBuildingCoverageRate |
| minBldgAr | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| maxBldgHgt | String | 高さの最高限度 | urf:maximumBuildingHeight |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| otherRstr | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialUseAttractionDistrict

特定用途誘導地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| useBeInduc | String | 誘導すべき用途 | urf:useToBeInduced |
| maxFAR | Double | 容積率の最高限度 | urf:maximumFloorAreaRate |
| minFAR | Double | 容積率の最低限度 | urf:minimumFloorAreaRate |
| minBldgAr | Measure | 建築面積の最低限度 | urf:minimumBuildingArea |
| maxBldgHgt | String | 高さの最高限度 | urf:maximumBuildingHeight |
| otherRstr | String | 建築物の敷地、構造又は建築設備に対する制限 | urf:otherRestrictions |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialUseDistrict

特別用途地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 特別用途地区の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| Rstr | String | 建築物の制限 | urf:buildingRestrictions |
| otherRstr | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialUseRestrictionDistrict

特定用途制限地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| Rstr | String | 建築物の制限 | urf:buildingRestrictions |
| otherRstr | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecialZoneForPreservationOfHistoricalLandscape

歴史的風土特別保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecifiedBlock

特定街区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| flrArRate | Double | 容積率 | urf:floorAreaRate |
| maxBldgHgt | Measure | 建築物の高さの最高限度 | urf:maximumBuildingHeight |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecifiedBuildingZoneImprovementPlan

特定建築物地区整備計画

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地区整備計画の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| dstrFclAlc | String | 地区施設の配置及び規模 | urf:districtFacilitiesAllocation |
| Rstr | String | 建築物等の制限 | urf:buildingRestrictions |
| urbGrenCvs | String | 樹林地、草地等の保全に関する事項 | urf:urbanGreenSpaceConservation |
| actRstrFrm | String | 農地での行為の制限 | urf:activityRestrictionInFarmland |
| luseRestri | String | 土地の利用に関する事項 | urf:landuseRestrictions |
| distrFacl | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) | 地区施設 | urf:districtFacility |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) | 防災地区 | urf:district |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecifiedDisasterPreventionBlockImprovementZone

特定防災街区整備地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| minSiteAr | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| setbckSize | String | 外壁の後退距離 | urf:setbackSize |
| minFrntgRt | Double | 間口率の最低限度 | urf:minimumFrontageRate |
| minBldHgt | Measure | 高さの最低限度 | urf:minimumBuildingHeight |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SpecifiedUrgentUrbanRenewalArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String |  | urf:developmentPolicy |
| privProj | JSON (<code><a href="#urfprivateurbanrenewalprojectplan">urf:PrivateUrbanRenewalProjectPlan</a>[]</code>) |  | urf:privateProject |
| specifArea | JSON (<code><a href="#urfspecifiedurgenturbanrenewalarea">urf:SpecifiedUrgentUrbanRenewalArea</a>[]</code>) |  | urf:specifiedArea |
| specDistr | JSON (<code><a href="#urfspecialurbanrenaissancedistrict">urf:SpecialUrbanRenaissanceDistrict</a>[]</code>) |  | urf:specialDistrict |
| devProj | JSON (<code><a href="#urfglobalhubcitydevelopmentproject">urf:GlobalHubCityDevelopmentProject</a>[]</code>) |  | urf:developmentProject |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SupplyFacility

供給施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| wtrWorks | JSON (<code><a href="#urfwaterworksattribute">urf:WaterWorksAttribute</a></code>) | 水道属性 | urf:waterWorksAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:TelecommunicationFacility

電気通信施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ThreeDimensionalExtent

立体的な範囲

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 対象となる都市施設の種類 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| minDist | Measure | 離隔距離の最小限度 | urf:minimumDistance |
| maxLoad | Measure | 載荷重の最大限度 | urf:maximumLoad |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:TideFacility

防潮施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:TrafficFacility

交通施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| stLoc | String | 起点 | urf:startLocation |
| endLoc | String | 終点 | urf:endLocation |
| viaLocs | String | 経過地 | urf:viaLocations |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| urbanRoad | JSON (<code><a href="#urfurbanroadattribute">urf:UrbanRoadAttribute</a></code>) | 道路属性 | urf:urbanRoadAttribute |
| urRpdTtRrd | JSON (<code><a href="#urfurbanrapidtransitrailroadattribute">urf:UrbanRapidTransitRailroadAttribute</a></code>) | 都市高速鉄道属性 | urf:urbanRapidTransitRailroadAttribute |
| parkPlace | JSON (<code><a href="#urfparkingplaceattribute">urf:ParkingPlaceAttribute</a></code>) | 駐車場属性 | urf:parkingPlaceAttribute |
| vehclTerm | JSON (<code><a href="#urfvehicleterminalattribute">urf:VehicleTerminalAttribute</a></code>) | 自動車ターミナル属性 | urf:vehicleTerminalAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:TreatmentFacility

処理施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| sewerSys | JSON (<code><a href="#urfsewersystemattribute">urf:SewerSystemAttribute</a></code>) |  | urf:sewerSystemsAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:TreePlantingDistrict

緑化地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| minGreenRt | Double |  | urf:minimumGreeningRate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UnclassifiedBlankArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UnclassifiedUseDistrict


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UnusedLandUsePromotionArea

遊休土地転換利用促進地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanDevelopmentProject

市街地開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanDisasterRecoveryPromotionArea

被災市街地復興推進地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| exprDate | Date | 期間満了の日 | urf:expirationDate |
| emgRcovPol | String | 市街地の整備改善の方針 | urf:emergencyRecoveryPolicy |
| plnProjTy | Code | 事業の種類 | urf:plannedProjectType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanFacility

都市施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanFacilityStipulatedByCabinetOrder

政令で定める都市施設（urf:functionの値により決定）

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanFunctionAttractionArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanPlanningArea

都市計画区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| arCls | Code | 区域区分の決定の有無 | urf:areaClassification |
| reasAreaCl | String | 区域区分を決定する理由又はしない理由 | urf:reasonForAreaClassification |
| polArCls | String | 区域区分の決定方針 | urf:policyForAreaClassification |
| purpUrbPln | String | 目標 | urf:purposeForUrbanPlan |
| polUrPlDec | String | 土地利用、都市施設の整備及び市街地開発事業に関する主要な都市計画の決定の方針 | urf:policyForUrbanPlanDecision |
| population | Integer | 都市計画区域内の総人口 | urf:population |
| cityArea | Measure | 都市計画区域面積（市区町村内） | urf:cityArea |
| cityPop | Integer | 都市計画区域内の人口（市区町村内） | urf:cityPopulation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanRedevelopmentProject

市街地再開発事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| pubFaclAlc | String | 公共施設の配置及び規模 | urf:publicFacilityAllocation |
| devPlan | String | 建築物及び建築敷地の整備計画 | urf:developmentPlan |
| housTarget | String | 整備されるべき住宅の個数その他住宅建設の目標 | urf:housingTarget |
| siteArea | Measure | 住宅の敷地面積 | urf:siteArea |
| totalFlrAr | Measure | 住宅の延べ床面積 | urf:totalFloorArea |
| #Housing | Integer | 住宅の個数 | urf:numberOfHousing |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanRedevelopmentPromotionArea

市街地再開発促進区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String | 開発の方針 | urf:developmentPolicy |
| pubFclPln | String | 公共施設に関する都市計画 | urf:publicFacilitiesPlans |
| pubFaclty | String | 公共施設の配置及び規模 | urf:publicFacilities |
| unitArea | String | 整備区の単位 | urf:unitArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanRenewalProject

市街地改造事業

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 市街地開発事業の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| schdExectr | String | 施行予定者 | urf:scheduledExecutor |
| strysAbvG | NonNegativeInteger | 建築物の地上階数 | urf:storeysAboveGround |
| strysBlwG | NonNegativeInteger | 建築物の地下階数 | urf:storeysBelowGround |
| setbckSize | String | 壁面の位置の限度 | urf:setbackSize |
| flrArRate | Double | 容積率の限度 | urf:floorAreaRate |
| Usage | String | 主な用途 | urf:buildingUsage |
| siteArea | Measure | 建築敷地面積 | urf:siteArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrgentUrbanRenewalArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| devPol | String |  | urf:developmentPolicy |
| privProj | JSON (<code><a href="#urfprivateurbanrenewalprojectplan">urf:PrivateUrbanRenewalProjectPlan</a>[]</code>) |  | urf:privateProject |
| specifArea | JSON (<code><a href="#urfspecifiedurgenturbanrenewalarea">urf:SpecifiedUrgentUrbanRenewalArea</a>[]</code>) |  | urf:specifiedArea |
| specDistr | JSON (<code><a href="#urfspecialurbanrenaissancedistrict">urf:SpecialUrbanRenaissanceDistrict</a>[]</code>) |  | urf:specialDistrict |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UseDistrict

用途地域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 用途地域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| flrArRate | Double | 容積率 | urf:floorAreaRate |
| minSiteAr | Measure | 敷地面積の最低限度 | urf:minimumSiteArea |
| CovRate | Double | 建蔽率 | urf:buildingCoverageRate |
| wallSbD | String | 外壁の後退距離 | urf:wallSetbackDistance |
| HeightLim | Measure | 建築物の高さの限度 | urf:buildingHeightLimits |
| Rstr | String | 建築物の制限 | urf:buildingRestrictions |
| otherRstr | String | 建築物の敷地、構造又は建築設備に関する制限 | urf:otherRestrictions |
| setbckRstr | String | 建築物の各部分の高さの制限 | urf:setbackRestrictions |
| frntRdRstr | String | 道路斜線制限 | urf:frontRoadRestrictions |
| adjLndRstr | String | 隣地斜線制限 | urf:adjacentLandRestrictions |
| northDRstr | String | 北側斜線制限 | urf:northDirectionRestrictions |
| shadeRegul | String | 日影規制 | urf:shadeRegulation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:Waterway


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String |  | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |  | gml:name |
| creatDate | Date |  | core:creationDate |
| termDate | Date |  | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code |  | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |  | urf:usage |
| validFrom | Date |  | urf:validFrom |
| validFrmTy | Code |  | urf:validFromType |
| enactFY | String |  | urf:enactmentFiscalYear |
| validTo | Date |  | urf:validTo |
| validToTy | Code |  | urf:validToType |
| exprFY | String |  | urf:expirationFiscalYear |
| legalGnds | String |  | urf:legalGrounds |
| custodian | String |  | urf:custodian |
| notif# | String |  | urf:notificationNumber |
| finNotifDa | Date |  | urf:finalNotificationDate |
| finNotif# | String |  | urf:finalNotificationNumber |
| urbPlanTy | Code |  | urf:urbanPlanType |
| arClsTy | Code |  | urf:areaClassificationType |
| nominalAr | Measure |  | urf:nominalArea |
| prefecture | Code |  | urf:prefecture |
| city | Code |  | urf:city |
| reference | URI |  | urf:reference |
| reason | String |  | urf:reason |
| note | String |  | urf:note |
| surveyYear | String |  | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |  | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String |  | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |  | urf:threeDimensionalExtent |
| stLoc | String |  | urf:startLocation |
| endLoc | String |  | urf:endLocation |
| structure | Code |  | urf:structure |
| length | Measure |  | urf:length |
| width | Measure |  | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:WindProtectionFacility

防風施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 都市施設の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| number | String | 番号 | urf:number |
| 3DExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) | 立体的な範囲 | urf:threeDimensionalExtent |
| length | Measure | 延長 | urf:length |
| width | Measure | 幅員 | urf:width |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ZonalDisasterPreventionFacility

地区防災施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 機能 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 地区防災施設の用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| facilityTy | Code | 地区防災施設の種類 | urf:facilityType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:Zone

区域

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日 | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号 | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 総面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String |  | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ZoneForPreservationOfHistoricalLandscape

第一種歴史的風土保存地区又は第二種歴史的風土保存地区

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | String | 説明 | gml:description |
| name | JSON (<code><a href="#code">Code</a>[]</code>) | 名称 | gml:name |
| creatDate | Date | 作成日 | core:creationDate |
| termDate | Date | 消滅日 | core:terminationDate |
| generic | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) | 汎用属性 | gen:genericAttribute |
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区の種類 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | urf:usage |
| validFrom | Date | 効力を生じる日(当初の決定年月日） | urf:validFrom |
| validFrmTy | Code | 効力を生じる日の区分 | urf:validFromType |
| enactFY | String | 決定年度 | urf:enactmentFiscalYear |
| validTo | Date | 効力を失う日 | urf:validTo |
| validToTy | Code | 効力を失う日の区分 | urf:validToType |
| exprFY | String | 廃止年度 | urf:expirationFiscalYear |
| legalGnds | String | 法的背景 | urf:legalGrounds |
| custodian | String | 決定者 | urf:custodian |
| notif# | String | 告示番号（当初） | urf:notificationNumber |
| finNotifDa | Date | 告示日（最終） | urf:finalNotificationDate |
| finNotif# | String | 告示番号（最終） | urf:finalNotificationNumber |
| urbPlanTy | Code | 都市計画区域 | urf:urbanPlanType |
| arClsTy | Code | 区域区分 | urf:areaClassificationType |
| nominalAr | Measure | 面積 | urf:nominalArea |
| prefecture | Code | 都道府県名 | urf:prefecture |
| city | Code | 市区町村名 | urf:city |
| reference | URI | 参考 | urf:reference |
| reason | String | 決定の事由 | urf:reason |
| note | String | 備考 | urf:note |
| surveyYear | String | 調査年度 | urf:surveyYear |
| keyValPair | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |  | urf:keyValuePairAttribute |
| dataQual | JSON (<code><a href="#urodataqualityattribute">uro:DataQualityAttribute</a></code>) |  | urf:dataQualityAttribute |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) | 境界 | urf:boundary |
| location | String | 位置 | urf:location |
| urbPark | JSON (<code><a href="#urfurbanparkattribute">urf:UrbanParkAttribute</a></code>) |  | urf:urbanParkAttribute |
| areaTotal | Measure | 面積（合計） | urf:areaInTotal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

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
- <a href='#uroreservoirfloodingriskattribute'>uro:ReservoirFloodingRiskAttribute</a>
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
- <a href='#uroparkfacilitylongevityplanattribute'>uro:ParkFacilityLongevityPlanAttribute</a>
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

### uro:FloodingRiskAttributeProperty

以下のいずれかの型の値をとる：

- <a href='#urohightideriskattribute'>uro:HighTideRiskAttribute</a>
- <a href='#uroinlandfloodingriskattribute'>uro:InlandFloodingRiskAttribute</a>
- <a href='#uroreservoirfloodingriskattribute'>uro:ReservoirFloodingRiskAttribute</a>
- <a href='#uroriverfloodingriskattribute'>uro:RiverFloodingRiskAttribute</a>
- <a href='#urotsunamiriskattribute'>uro:TsunamiRiskAttribute</a>

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
| mappngRule | String |  | gml:MappingRule |
| gridFunct | JSON (<code><a href="#gmlgridfunction">gml:GridFunction</a></code>) |  | gml:GridFunction |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gml:GridEnvelope


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gml:GridFunction


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gml:RectifiedGrid


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| limits | JSON (<code><a href="#gmlgridenvelope">gml:GridEnvelope</a></code>) |  | gml:limits |
| axisName | JSON (<code><a href="#string">String</a>[]</code>) |  | gml:axisName |
| origin | Point |  | gml:origin |
| offsetVec | JSON (<code><a href="#point">Point</a>[]</code>) |  | gml:offsetVector |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gml:RectifiedGridCoverage


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| rectGrdDom | JSON (<code><a href="#gmlrectifiedgriddomain">gml:RectifiedGridDomain</a></code>) |  | gml:rectifiedGridDomain |
| covFunct | JSON (<code><a href="#gmlcoveragefunction">gml:CoverageFunction</a></code>) |  | gml:coverageFunction |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gml:RectifiedGridDomain


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| rectifGrid | JSON (<code><a href="#gmlrectifiedgrid">gml:RectifiedGrid</a></code>) |  | gml:RectifiedGrid |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### core:Address


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| xalAddress | JSON (<code><a href="#xaladdressdetails">xAL:AddressDetails</a>[]</code>) |  | core:xalAddress |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### gen:genericAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BridgeFunctionalAttribute

橋梁機能属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| directnTy | Code | 進行方向区分 | uro:directionType |
| userType | Code | 利用者区分 | uro:userType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BridgeStructureAttribute

橋梁構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| material | Code | 材質 | uro:material |
| bridgeType | Code | 種類 | uro:bridgeType |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| area | Measure | 上部工面積 | uro:area |
| weightRstr | Measure | 荷重制限 | uro:weightRestriction |
| heightRstr | Measure | 高さ制限 | uro:heightRestriction |
| widthRstr | Measure | 幅制限 | uro:widthRestriction |
| unGirdrHgt | Measure | 桁下高さ制限 | uro:underGirderHeight |
| slopeType | Code | 昇降形式 | uro:slopeType |
| escalator | Boolean | エスカレータ有無 | uro:escalator |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BuildingDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:srcScale |
| geomSrcDsc | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc |
| SrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDesc |
| lod1HgtTy | Code |  | uro:lod1HeightType |
| lodType | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:lodType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BuildingDetailAttribute

建物利用現況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| ser#BldCrt | String | 建築確認申請番号 | uro:serialNumberOfBuildingCertification |
| siteArea | Measure | 敷地面積 | uro:siteArea |
| totalFlrAr | Measure | 延床面積 | uro:totalFloorArea |
| FootprAr | Measure | 建築面積 | uro:buildingFootprintArea |
| RoofEdgeAr | Measure | 図形面積 | uro:buildingRoofEdgeArea |
| devAr | Measure | 開発面積 | uro:developmentArea |
| StructreTy | Code | 構造種別 | uro:buildingStructureType |
| StrctOrgTy | Code | 構造種別（独自） | uro:buildingStructureOrgType |
| fireprfSTy | Code | 耐火構造種別 | uro:fireproofStructureType |
| impleBody | String | 事業主体 | uro:implementingBody |
| urbPlanTy | Code | 都市計画区域 | uro:urbanPlanType |
| arClsTy | Code | 区域区分 | uro:areaClassificationType |
| distrZonTy | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区 | uro:districtsAndZonesType |
| luseType | Code | 土地利用区分 | uro:landUseType |
| reference | String | 図面対象番号 | uro:reference |
| mjrUsage | Code | 建物利用現況（大分類） | uro:majorUsage |
| mjrUsage2 | Code | 建物利用現況（大分類2） | uro:majorUsage2 |
| orgUsage | Code | 建物利用現況（中分類） | uro:orgUsage |
| orgUsage2 | Code | 建物利用現況（小分類） | uro:orgUsage2 |
| dtlUsage | Code | 建物利用現況（詳細分類） | uro:detailedUsage |
| dtlUsage2 | Code | 建物利用現況（詳細分類2） | uro:detailedUsage2 |
| dtlUsage3 | Code | 建物利用現況（詳細分類3） | uro:detailedUsage3 |
| gFloorUsge | Code | 1階用途 | uro:groundFloorUsage |
| 2FloorUsag | Code | 2階（以上）用途 | uro:secondFloorUsage |
| 3rdFlrUsag | Code | 3階（以上）用途 | uro:thirdFloorUsage |
| baseUsage | Code | 地下用途 | uro:basementUsage |
| base1Usage | Code | 地下1階用途 | uro:basementFirstUsage |
| base2Usage | Code | 地下2階用途 | uro:basementSecondUsage |
| vacancy | Code | 空き家区分 | uro:vacancy |
| CovRate | Double | 建蔽率 | uro:buildingCoverageRate |
| flrArRate | Double | 容積率 | uro:floorAreaRate |
| specfidBCR | Double | 指定建蔽率 | uro:specifiedBuildingCoverageRate |
| specifdFAR | Double | 指定容積率 | uro:specifiedFloorAreaRate |
| stdFAR | Double | 基準容積率 | uro:standardFloorAreaRate |
| Height | Measure | 建築物の高さ | uro:buildingHeight |
| eaveHeight | Measure | 軒の高さ | uro:eaveHeight |
| note | String | 備考 | uro:note |
| surveyYear | String | 調査年 | uro:surveyYear |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BuildingIDAttribute

建物ID属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildingID | String | 建物ID | uro:buildingID |
| branchID | Integer | 建物ID枝番 | uro:branchID |
| partID | Integer | 建築物部分ID | uro:partID |
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:BuildingUsecaseAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| isTemporal | Code |  | uro:isTemporal |
| floorHeight | Measure |  | uro:floorHeight |
| isGroundFloorOpen | Boolean |  | uro:isGroundFloorOpen |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:CargoHandlingFacility

荷さばき施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| moorFacl | String | 係留施設名。 | uro:mooringFacility |
| liftaLoad | Measure | 荷役能力_吊り上げ荷重 | uro:liftableLoad |
| ability | Integer | 荷役能力_1時間あたりの能力 | uro:ability |
| packngName | Code | 荷姿名 | uro:packingName |
| acqYear | String | 取得年度 | uro:acquisitionYear |
| inTotFlrAr | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| inSiteAr | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| oTotFlrAr | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outSiteAr | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:CircularCurveType

円曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| radius | Measure | 半径 | uro:radius |
| int | Double | 交角 | uro:intersection |
| cutLength | Measure | 切線長 | uro:cutLength |
| curveLen | Measure | 曲線長 | uro:curveLength |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:CityFurnitureDetailAttribute

都市設備詳細属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityTy | Code | 施設詳細区分 | uro:facilityType |
| desc | String |  | uro:description |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ConstructionBaseAttribute

構造物基本属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| adminType | Code | 管理者区分 | uro:adminType |
| admin | String | 管理者名称 | uro:administorator |
| adminOffic | String | 管理事務所所在地 | uro:adminOffice |
| operatorTy | Code | 運用者区分 | uro:operatorType |
| installrTy | Code | 設置者区分 | uro:installerType |
| installer | String | 設置者名称 | uro:installer |
| structOrd | String | 適用構造令 | uro:structureOrdinance |
| specificat | String | 適用示方書 | uro:specification |
| kana | String | ふりがな | uro:kana |
| cnstrStYr | String | 建設開始年度 | uro:constructionStartYear |
| completYr | String | 完成年度 | uro:completionYear |
| faclAge | Integer | 施設年数 | uro:facilityAge |
| update | Date | 更新年月日 | uro:update |
| purpose | Code | 目的 | uro:purpose |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ConstructionEvent

イベント

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| event | Code | 種類 | uro:event |
| dateEvent | Date | 日付 | uro:dateOfEvent |
| desc | String | 説明 | uro:description |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ConstructionRiskAssessmentAttribute

構造物リスク評価属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| surveyYear | String | 点検実施年度 | uro:surveyYear |
| riskType | Code | 判定区分 | uro:riskType |
| status | Code | 対応状況 | uro:status |
| refDate | Date | 更新時点 | uro:referenceDate |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ConstructionStructureAttribute

構造物構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structutTy | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 体積 | uro:volume |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ControlPoint

線形変化点

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| startPost | String | 開始キロ程 | uro:startPost |
| endPost | String | 終了キロ程 | uro:endPost |
| function | Code | 変化点種類 | uro:function |
| parameter | JSON (<code><a href="#urocontrolpointtype">uro:ControlPointType</a></code>) | パラメータ | uro:parameter |
| startPoint | Point | 開始位置 | uro:startPoint |
| endPoint | Point | 終了位置 | uro:endPoint |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:CountermeasuresCost


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| cost | Integer |  | uro:cost |
| costUnit | String |  | uro:costUnit |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:CyberportMarinaAndPBS

マリーナ/PBS

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| geolTy | Code | 海底の地質名 | uro:geologicalType |
| obsStructr | String | 構造物による制限－構造物名 | uro:obstructingStructures |
| maiPartLen | Measure | 延長－取付部を除く延長 | uro:mainPartLength |
| totalLen | Measure |  | uro:totalLength |
| wavDsptLen | Measure | 消波工延長 | uro:waveDissipatorLength |
| faclWidth | Measure | 施設の幅 | uro:facilityWidth |
| apronWidth | Measure | エプロン幅 | uro:apronWidth |
| rsnStrctre | String | 構造物による制限 | uro:restrictionStructure |
| plnDpth | Measure | 計画上の水深 | uro:plannedDepth |
| currDpth | Measure | 現在の水深 | uro:currentDepth |
| inTotFlrAr | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| inSiteAr | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| oTotFlrAr | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outSiteAr | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| ceilHgt | Measure | 天端高 | uro:ceilingHeight |
| gravResis | Measure | 耐重力 | uro:gravityResistant |
| form | Code | 形態 | uro:form |
| areaType | Code | 内外区分 | uro:areaType |
| maiVessels | Code | 主要利用船舶の種類 | uro:mainVessels |
| isDredged | Boolean | 浚渫の有無 | uro:isDredged |
| moorPstWgt | Measure | 附帯設備－係船柱の重さ | uro:mooringPostWeight |
| #MoorPosts | Integer | 附帯設備－係船柱の個数 | uro:numberOfMooringPosts |
| resisMatl | Integer | 附帯設備－防げん材 | uro:resistantMaterial |
| lighting | Integer | 附帯設備－照明設備 | uro:lighting |
| stairs | Integer | 附帯設備－階段等 | uro:stairs |
| lifesaving | String | 附帯設備－救設備の名称 | uro:lifesaving |
| lifesav# | Integer | 附帯設備－救命設備の数 | uro:lifesavingNumber |
| bumper | Measure | 附帯設備－車止め | uro:bumper |
| #VehclBrdn | Integer | 附帯設備－車両乗降設備－基数 | uro:numberOfVehicleBoardings |
| vehcBrdWth | Measure | 附帯設備－車両乗降設備－幅員 | uro:vehicleBoardingWidth |
| shipType | String | 対象船舶－船型(D/W) | uro:shipType |
| #Seats | Integer | 対象船舶－船席数 | uro:numberOfSeats |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| storageCap | Integer | 保管容量－値 | uro:storageCapacity |
| stoCapUnit | Code | 保管容量－単位 | uro:storageCapacityUnit |
| structutTy | Code | 構造形式 | uro:structureType |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:DamAttribute

ダム属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structutTy | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 堤頂長 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 堤体積 | uro:volume |
| damCode | Code | ダムコード | uro:damCode |
| tWtrStorag | Measure | 総貯水量 | uro:totalWaterStorage |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:DataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:srcScale |
| geomSrcDsc | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc |
| SrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDesc |
| geomSrcDL0 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDescLod0 |
| geomSrcDL1 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDescLod1 |
| geomSrcDL2 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDescLod2 |
| geomSrcDL3 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDescLod3 |
| geomSrcDL4 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDescLod4 |
| geomSrcD0 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc0 |
| geomSrcD1 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc1 |
| geomSrcD2 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc2 |
| geomSrcD3 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc3 |
| geomSrcD4 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:geometrySrcDesc4 |
| thmSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:thematicSrcDesc |
| appSDLod0 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDescLod0 |
| appSDLod1 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDescLod1 |
| appSDLod2 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDescLod2 |
| appSDLod3 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDescLod3 |
| appSDLod4 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:appearanceSrcDescLod4 |
| lodType | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:lodType |
| lod1HgtTy | Code |  | uro:lod1HeightType |
| tranDtaAcq | String |  | uro:tranDataAcquisition |
| pubSvDQual | JSON (<code><a href="#uropublicsurveydataqualityattribute">uro:PublicSurveyDataQualityAttribute</a></code>) |  | uro:publicSurveyDataQualityAttribute |
| bldgDQual | JSON (<code><a href="#urobuildingdataqualityattribute">uro:BuildingDataQualityAttribute</a></code>) |  | uro:BuildingDataQualityAttribute |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:DmAnnotation

DM注記情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dmCode | Code | 分類コード | uro:dmCode |
| meshCode | Code | 図郭番号 | uro:meshCode |
| dmElement | JSON (<code><a href="#urodmelement">uro:DmElement</a></code>) | 要素情報 | uro:dmElement |
| geometryTy | Code | レコードタイプ | uro:geometryType |
| shapeType | Code | 図形区分 | uro:shapeType |
| label | String | 注記文字列 | uro:label |
| isVertical | Boolean | 文字方向 | uro:isVertical |
| size | Integer | 字大 | uro:size |
| orientatin | Integer | 表示角度 | uro:orientation |
| linewidth | Integer | 線号 | uro:linewidth |
| spacing | Integer | 字隔 | uro:spacing |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:DmElement

要素情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| locationTy | Code | 地域分類 | uro:locationType |
| infoType | Code | 情報分類 | uro:infoType |
| elementKey | String | 要素識別番号 | uro:elementKey |
| hirarchyLv | String | 階層レベル | uro:hierarchyLevel |
| dataType | Code | 実データ区分 | uro:dataType |
| annotTy | Code | 注記区分 | uro:annotationType |
| precisonTy | Code | 精度区分 | uro:precisionType |
| dislocatTy | Code | 転位区分 | uro:dislocationType |
| breakType | Code | 間断区分 | uro:breakType |
| attrVae | String | 属性数値 | uro:attributeValue |
| attrTy | Code | 属性区分 | uro:attributeType |
| valueTy | String | 属性データ書式 | uro:attributeValueType |
| creatDate | String | 取得年月 | uro:creationDate |
| updateDate | String | 更新年月 | uro:updateDate |
| termDate | String | 削除年月 | uro:terminationDate |
| freeSpace | String | 空き領域 | uro:freeSpace |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:DmGeometricAttribute

DM図形情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dmCode | Code | 分類コード | uro:dmCode |
| meshCode | Code | 図郭番号 | uro:meshCode |
| dmElement | JSON (<code><a href="#urodmelement">uro:DmElement</a></code>) | 要素情報 | uro:dmElement |
| geometryTy | Code | レコードタイプ | uro:geometryType |
| mapLevel | Code | 地図情報レベル | uro:mapLevel |
| shapeType | Code | 図形区分 | uro:shapeType |
| visibility | Boolean | 可視性 | uro:visibility |
| is3d | Boolean | 3D区分 | uro:is3d |
| installatn | Boolean | 付属図形区分 | uro:isInstallation |
| isEdited | Boolean | 編集フラグ | uro:isEdited |
| supplSymbl | Boolean | 補助記号フラグ | uro:isSupplementarySymbol |
| angle | Double | 回転角度 | uro:angle |
| elevation | Measure | 標高 | uro:elevation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Elevation

標高

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| elvRef | Code | 計測位置 | uro:elevationReference |
| elvValue | Point | 標高 | uro:elevationValue |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:EmbankmentAttribute

堤防属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structutTy | Code | 構造種別 | uro:structureType |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| depth | Measure | 水深 | uro:depth |
| volume | Measure | 体積 | uro:volume |
| maiPartLen | Measure | 機能保有延長 | uro:mainPartLength |
| ceilHgt | Measure | 天端高 | uro:ceilingHeight |
| wavDsptLen | Measure | 消波工延長 | uro:waveDissipatorLength |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:FacilityIdAttribute

施設識別属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| partId | String |  | uro:partId |
| branchId | String |  | uro:branchId |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:prefecture |
| city | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:city |
| route | String |  | uro:route |
| startPost | String |  | uro:startPost |
| endPost | String |  | uro:endPost |
| startLat | Double |  | uro:startLat |
| startLong | Double |  | uro:startLong |
| altNm | JSON (<code><a href="#string">String</a>[]</code>) |  | uro:alternativeName |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:FacilityTypeAttribute

施設属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 区分 | uro:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 用途 | uro:function |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:FishingPortCapacity

漁港施設能力

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| capacity | String | 能力 | uro:capacity |
| weightCap | Measure | 能力_耐重量 | uro:weightCapacity |
| hullForm | Integer | 能力_係船能力_船型 | uro:hullForm |
| shipNumber | Integer | 能力_係船能力_隻数 | uro:shipNumber |
| depth2m | Measure | 能力_水深別内訳_2ｍ未満の面積 | uro:waterDepth-2m |
| depth23m | Measure | 能力_水深別内訳_2～3ｍ未満の面積 | uro:waterDepth2-3m |
| depth36m | Measure | 能力_水深別内訳_3～6ｍ未満の面積 | uro:waterDepth3-6m |
| depth6m | Measure | 能力_水深別内訳_6ｍ以上の面積 | uro:waterDepth6-m |
| hgtAbvAWL | Measure | 能力_種類_灯台_平均水面上の高さ | uro:heightAboveAWL |
| heightFndn | Measure | 能力_種類_灯台_基礎上の高さ | uro:heightOnFoundations |
| luminRange | Measure | 能力_光音電波の到達距離 | uro:luminousRange |
| luminColor | String | 能力_灯色 | uro:luminousColor |
| candlePow | Integer | 能力_燭光数 | uro:candlePower |
| lightType | String | 能力_灯質の種類 | uro:lightType |
| period | String | 能力_灯質の周期 | uro:period |
| maxGndWgt | Integer | 能力_入きょ又は上架できる最大船舶の総重量 | uro:maximumGroundingWeight |
| handlabPow | Integer | 能力_取り扱いできる機関の馬力数 | uro:handleablePower |
| maxWtrSupl | Integer | 能力_最大給水能力 | uro:maximumWaterSupply |
| maxRefuel | String | 能力_最大給油能力 | uro:maximumRefueling |
| people | Integer | 能力_最大収容可能人数 | uro:people |
| other | String | 能力_その他 | uro:other |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:FishingPortFacility

漁港施設概要

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| faclDtlTy | Code | 施設種類 | uro:facilityDetailsType |
| portName | String | 漁港名称 | uro:portName |
| portType | Code | 漁港の種類 | uro:portType |
| address | String | 所在地 | uro:address |
| desigArea | String | 区域 | uro:designatedArea |
| desig | JSON (<code><a href="#string">String</a>[]</code>) | 漁港の指定 | uro:designation |
| desigAdmin | JSON (<code><a href="#string">String</a>[]</code>) | 漁港管理者の指定 | uro:designatedAdministrator |
| reference# | JSON (<code><a href="#string">String</a>[]</code>) | 漁港の平面図対象番号 | uro:referenceNumber |
| grantType | Code | 施設区分名 | uro:grantType |
| admin | String | 所有者の名称 | uro:administrator |
| faclMngr | String | 管理者の名称 | uro:facilityManager |
| structutTy | Code | 構造_様式又は形式 | uro:structureType |
| mainMat | Code | 構造_主要用材 | uro:mainMaterial |
| othStruct | String | 構造_その他の構造 | uro:otherStructure |
| length | Measure | 規模_延長 | uro:length |
| width | Measure | 規模_幅員 | uro:width |
| ceilHgt | Measure | 規模_天端高 | uro:ceilingHeight |
| depth | Measure | 規模_水深 | uro:depth |
| area | Measure | 規模_面積 | uro:area |
| othSizeDsc | String | 規模_その他の規模数量 | uro:otherSizeDescription |
| daCnstrAcq | Date | 建設又は取得の年月日 | uro:dateOfConstructionOrAcquisition |
| cost | Integer | 建設又は取得の価格 | uro:cost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:HarborFacility

水域施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| geolTy | Code | 地質名 | uro:geologicalType |
| obsStructr | String | 構造物による制限－構造物名 | uro:obstructingStructures |
| structLim | Measure | 構造物による制限 | uro:structuralLimitations |
| length | Measure | 延長 | uro:length |
| minWidth | Measure | 幅員－最小 | uro:minimumWidth |
| maxWidth | Measure | 幅員－最大 | uro:maximumWidth |
| plnDpth | Measure | 水深－計画上の水深 | uro:plannedDepth |
| currDpth | Measure | 水深－現在の水深 | uro:currentDepth |
| isDredged | Boolean | 浚渫の有無 | uro:isDredged |
| areaType | Code | 内外区分 | uro:areaType |
| innerArea | Measure | 面積_防波堤等の内側 | uro:innerArea |
| outerArea | Measure | 面積_防波堤等の外側 | uro:outerArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | JSON (<code><a href="#string">String</a>[]</code>) | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Height

高さ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| highRef | Code | 計測位置（高） | uro:highReference |
| lowRef | Code | 計測位置（低） | uro:lowReference |
| status | String | 計測方法 | uro:status |
| value | Measure | 値 | uro:value |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:HighTideRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcAxis2Placement3D

IFCローカル座標系変換情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| location | Point | 原点 | uro:location |
| axis | String | Z軸ベクトル | uro:axis |
| refDir | String | X軸ベクトル | uro:refDirection |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcBuilding

IFC建物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| compositTy | String | 構成型 | uro:compositionType |
| elvRefHgt | Measure | 基準海抜高度 | uro:elevationOfRefHeight |
| elvTerrain | Measure | 地盤最小海抜高度 | uro:elevationOfTerrain |
| Address | JSON (<code><a href="#coreaddress">core:Address</a></code>) | 建物住所 | uro:buildingAddress |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcBuildingElement

IFC建築部材

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcBuildingStorey


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| desc | String |  | uro:description |
| objectType | String |  | uro:objectType |
| longName | String |  | uro:longName |
| compositTy | String |  | uro:compositionType |
| elevation | Measure |  | uro:elevation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcClassification

IFC分類諸元

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | String | ソース | uro:source |
| edition | String | 版 | uro:edition |
| editDate | Date | 日付 | uro:editionDate |
| name | String | 分類ラベル | uro:name |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcClassificationReference

IFC分類諸元

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| location | URI |  | uro:location |
| itemRef | Code |  | uro:itemReference |
| name | String |  | uro:name |
| refSource | JSON (<code><a href="#uroifcclassification">uro:IfcClassification</a></code>) |  | uro:referenceSource |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcCoordinateReferenceSystem

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| geodDatum | String | 測地原子 | uro:geodeticDatum |
| vertDatum | String | 垂直原子 | uro:verticalDatum |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcCurtainWall

IFCカーテンウォール

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcDoor

IFC扉

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| overallHgt | Measure | 全長 | uro:overallHeight |
| overallWth | Measure | 全幅 | uro:overallWidth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcFurnishingElement

IFC設置物

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcGeometricRepresentationContext


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| contextId | String |  | uro:contextIdentifier |
| contextTy | String |  | uro:contextType |
| coordSpDim | Integer |  | uro:coordinateSpaceDimension |
| precision | Double |  | uro:precision |
| wldCordSys | JSON (<code><a href="#uroifcaxis2placement3d">uro:IfcAxis2Placement3D</a></code>) |  | uro:worldCoordinateSystem |
| trueNorth | String |  | uro:trueNorth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcGroup


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| desc | String |  | uro:description |
| objectType | String |  | uro:objectType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcMapConversion

IFC座標変換

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| sourceCRS | JSON (<code><a href="#uroifccoordinatereferencesystemselecttype">uro:IfcCoordinateReferenceSystemSelectType</a></code>) |  | uro:sourceCRS |
| targetCRS | JSON (<code><a href="#uroifccoordinatereferencesystemproperty">uro:IfcCoordinateReferenceSystemProperty</a></code>) |  | uro:targetCRS |
| eastings | Measure |  | uro:eastings |
| northings | Measure |  | uro:northings |
| orthoHeigt | Measure |  | uro:orthogonalHeight |
| xAxisAbsc | Double |  | uro:xAxisAbscissa |
| xAxisOrd | Double | 終点北座標 | uro:xAxisOrdinate |
| scale | Double | スケール | uro:scale |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcOpeningElement

IFC開口部

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| nominalAr | Measure | 公称面積 | uro:nominalArea |
| nominalVol | Measure | 公称体積 | uro:nominalVolume |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcProject

IFCプロジェクト

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| phase | String | プロジェクトの状態 | uro:phase |
| reprCtx | JSON (<code><a href="#uroifcgeometricrepresentationcontext">uro:IfcGeometricRepresentationContext</a></code>) | 形状表現 | uro:representationContexts |
| unitsCtx | JSON (<code><a href="#uroifcunit">uro:IfcUnit</a>[]</code>) | 単位系 | uro:unitsInContext |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcProjectedCRS

IFC投影座標参照系

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| geodDatum | String | 測地原子 | uro:geodeticDatum |
| vertDatum | String | 垂直原子 | uro:verticalDatum |
| mapUnit | String | 軸単位 | uro:mapUnit |
| mapProject | String | 投影座標系名称 | uro:mapProjection |
| mapZone | String | ゾーン番号 | uro:mapZone |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetBuildingCommon

IFC建築物共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildingId | String | 建物ID | uro:buildingId |
| permnentId | Boolean | 識別子の永続性 | uro:isPermanentId |
| maiFireUse | String | 主な防災用途 | uro:mainFireUse |
| ancFireUse | String | 付属的な防災用途 | uro:ancillaryFireUse |
| sprnklPrtc | Boolean | スプリンクラー有無 | uro:sprinklerProtection |
| sprnklPrAu | Boolean | 自動スプリンクラー有無 | uro:sprinklerProtectionAutomatic |
| occupanTy | Code | 入居者タイプ | uro:occupancyType |
| grossPlnAr | Measure | 計画総面積 | uro:grossPlannedArea |
| #Storeys | Integer | 階数 | uro:numberOfStoreys |
| yrConstrtn | NonNegativeInteger | 建設年 | uro:yearOfConstruction |
| landmarked | Boolean | 歴史的建造物区分 | uro:isLandmarked |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetDoorCommon

IFC扉共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| acousRati | String | 遮音等級 | uro:acousticRating |
| firerating | String | 耐火等級 | uro:firerating |
| secrtyRtng | String | 防犯等級 | uro:securityRating |
| isExternal | Boolean | 外部区分 | uro:isExternal |
| infiltrat | Double | 隙間風流量 | uro:infiltration |
| thermalTx | Double | 熱貫流率 | uro:thermalTransmittance |
| glazArFrct | Double | ガラス領域比率 | uro:glazingAreaFraction |
| handcpAccs | Boolean | ハンディキャップアクセス区分 | uro:handicapAccessible |
| fireExit | Boolean | 非常口区分 | uro:fireExit |
| selfClosng | Boolean | 自動ドア閉機能区分 | uro:selfClosing |
| smokeStop | Boolean | 防煙機能区分 | uro:smokeStop |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetOpeningElementCommon

IFC開口部共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照用ID | uro:reference |
| purpose | String | 目的 | uro:purpose |
| fireExit | Boolean | 非常口区分 | uro:fireExit |
| protectOpn | Boolean | 保護区分 | uro:protectedOpening |
| paraJambs | Boolean | 平行区分 | uro:parallelJambs |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetSiteCommon

IFC敷地共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| buildaAr | Measure | 使用面積 | uro:buildableArea |
| totalArea | Measure | 総面積 | uro:totalArea |
| HeightLim | Measure | 最大高さ | uro:buildingHeightLimit |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetSpaceCommon

IFC空間共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| category | String | カテゴリ | uro:category |
| flrCover | String | 床仕上げ | uro:floorCovering |
| wallCover | String | 壁仕上げ | uro:wallCovering |
| ceilCover | String | 天井仕上げ | uro:ceilingCovering |
| skirtBrd | String | 幅木 | uro:skirtingBoard |
| grossPlnAr | Measure | 計画グロス面積 | uro:grossPlannedArea |
| netPlnAr | Measure | 計画ネット面積 | uro:netPlannedArea |
| pubAccess | Boolean | 公共アクセス区分 | uro:publiclyAccessible |
| handcpAccs | Boolean | ハンディキャップアクセス区分 | uro:handicapAccessible |
| cnclFloor | Boolean | 隠ぺい床スペース区分 | uro:concealedFlooring |
| cnclCeil | Boolean | 隠ぺい天井スペース区分 | uro:concealedCeiling |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcPsetWindowCommon

IFC窓共通属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reference | String | 参照記号 | uro:reference |
| acousRati | String | 遮音等級 | uro:acousticRating |
| fireRating | String | 耐火等級 | uro:fireRating |
| secrtyRtng | String | 防犯等級 | uro:securityRating |
| isExternal | Boolean | 外部区分 | uro:isExternal |
| infiltrat | Double | 隙間風流量 | uro:infiltration |
| thermalTx | Double | 熱貫流率 | uro:thermalTransmittance |
| glazArFrct | Double | ガラス領域比率 | uro:glazingAreaFraction |
| smokeStop | Boolean | 防煙機能区分 | uro:smokeStop |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcRoof

IFC屋根

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcSite


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| desc | String |  | uro:description |
| objectType | String |  | uro:objectType |
| longName | String |  | uro:longName |
| compositTy | String |  | uro:compositionType |
| refLng | Double |  | uro:refLongitude |
| refLat | Double |  | uro:refLatitude |
| refElevat | Measure |  | uro:refElevation |
| landTitle# | String |  | uro:landTitleNumber |
| siteAddr | JSON (<code><a href="#coreaddress">core:Address</a></code>) |  | uro:siteAddress |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcSpace

IFC空間

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| longName | String | 名称定義 | uro:longName |
| compositTy | String | 構成型 | uro:compositionType |
| intExtSpce | String | 内外区分 | uro:interiorOrExteriorSpace |
| elvFloor | Measure | 床高さ | uro:elevationWithFlooring |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcSpaceBaseQuantity

IFC空間基本数値情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| nominalHgt | Measure | スラブ上端から上階スラブ下端までの高さ | uro:nominalHeight |
| clearHgt | Measure | 床面から天井面の高さ | uro:clearHeight |
| finCeilHgt | Measure | 天井高 | uro:finishCeilingHeight |
| grossPerim | Measure | 床レベルでの総周辺長 | uro:grossPerimeter |
| netPerim | Measure | 正味周囲長 | uro:netPerimeter |
| gCeilAr | Measure | 天井面積 | uro:grossCeilingArea |
| grossFlrAr | Measure | 延面積 | uro:grossFloorArea |
| netCeilAr | Measure | 正味天井面積 | uro:netCeilingArea |
| netFlrAr | Measure | 正味延面積 | uro:netFloorArea |
| gWallAr | Measure | 壁面積 | uro:grossWallArea |
| netWallAr | Measure | 正味壁面積 | uro:netWallArea |
| grossVol | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcUnit

IFC単位

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| dimensions | Integer | 次元数 | uro:dimensions |
| unitType | String | 単位種類 | uro:unitType |
| perfix | String |  | uro:perfix |
| name | String | 名称 | uro:name |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcWall

IFC壁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| nominalLen | Measure | 長さ | uro:nominalLength |
| nominalWth | Measure | 幅 | uro:nominalWidth |
| nominalHgt | Measure | 高さ | uro:nominalHeight |
| gFootPAr | Measure | フットプリント面積 | uro:grossFootPrintArea |
| netFPrntAr | Measure | 正味フットプリント面積 | uro:netFootPrintArea |
| gSideAr | Measure | 側面面積 | uro:grossSideArea |
| netSideAr | Measure | 正味側面面積 | uro:netSideArea |
| gSideArL | Measure | 左側側面面積 | uro:grossSideAreaLeft |
| netSideArL | Measure | 左側正味側面面積 | uro:netSideAreaLeft |
| gSideArR | Measure | 右側側面面積 | uro:grossSideAreaRight |
| netSideArR | Measure | 右側正味側面面積 | uro:netSideAreaRight |
| grossVol | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcWallStandardCase

IFC標準壁

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer | 蹴上数 | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| nominalLen | Measure | 長さ | uro:nominalLength |
| nominalWth | Measure | 幅 | uro:nominalWidth |
| nominalHgt | Measure | 高さ | uro:nominalHeight |
| gFootPAr | Measure | フットプリント面積 | uro:grossFootPrintArea |
| netFPrntAr | Measure | 正味フットプリント面積 | uro:netFootPrintArea |
| gSideAr | Measure | 側面面積 | uro:grossSideArea |
| netSideAr | Measure | 正味側面面積 | uro:netSideArea |
| gSideArL | Measure | 左側側面面積 | uro:grossSideAreaLeft |
| netSideArL | Measure | 左側正味側面面積 | uro:netSideAreaLeft |
| gSideArR | Measure | 右側側面面積 | uro:grossSideAreaRight |
| netSideArR | Measure | 右側正味側面面積 | uro:netSideAreaRight |
| grossVol | Measure | 体積 | uro:grossVolume |
| netVolume | Measure | 正味体積 | uro:netVolume |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcWindow

IFC窓

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String | 識別子 | uro:globalId |
| name | String | 名称 | uro:name |
| desc | String | 説明 | uro:description |
| objectType | String | オブジェクトタイプ | uro:objectType |
| tag | String | 識別番号 | uro:tag |
| elementTy | Code | 建築部材区分 | uro:elementType |
| predefTy | Code | 詳細区分 | uro:predefinedType |
| shapeType | Code | 形状区分 | uro:shapeType |
| #Riser | Integer |  | uro:numberOfRiser |
| #Treads | Integer | 踏面数 | uro:numberOfTreads |
| riserHgt | Measure | 蹴上高さ | uro:riserHeight |
| treadLen | Measure | 奥行長さ | uro:treadLength |
| operatTy | String | 輸送設備区分 | uro:operationType |
| capByWgt | Measure | 許容積載量 | uro:capacityByWeight |
| capBy# | Integer | 許容定員数 | uro:capacityByNumber |
| overallHgt | Measure | 全長 | uro:overallHeight |
| overallWth | Measure | 全幅 | uro:overallWidth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IfcZone


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| globalId | String |  | uro:globalId |
| name | String |  | uro:name |
| desc | String |  | uro:description |
| objectType | String |  | uro:objectType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorFacilityAttribute

屋内施設属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| wkdayHours | String | 施設の営業時間（平日） | uro:weekdayHours |
| wkendHours | String | 施設の営業時間（土日祝祭日） | uro:weekendHours |
| phone | String | 電話番号 | uro:phone |
| website | String | ウェブサイトアドレス | uro:website |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorFurnishingAttribute

屋内設置物属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorPublicTagAttribute

屋内タグ属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| ucode | String |  | uro:ucode |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorSpaceAttribute

屋内空間属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |
| restricted | Boolean | 制限有無 | uro:isRestricted |
| suite | String | 注記ラベル | uro:suite |
| isPublic | Boolean | 公開可否 | uro:isPublic |
| tollType | Code | 有料区分 | uro:tollType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorStoreyAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| category | Boolean |  | uro:category |
| ordinal | Double |  | uro:ordinal |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorTacatileTileAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code |  | uro:source |
| startNode | String |  | uro:startNode |
| endNode | String |  | uro:endNode |
| category | Code |  | uro:category |
| roof | String |  | uro:roof |
| floorId | String |  | uro:floorId |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorUserDefinedAttribute

屋内利用者定義属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| name | String | 名称 | uro:name |
| nominalVal | JSON (<code><a href="#urouserdefinedvalue">uro:UserDefinedValue</a></code>) | 値 | uro:nominalValue |
| desc | String | 説明 | uro:description |
| unit | String | 単位 | uro:unit |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:IndoorZoneAttribute

屋内任意空間属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| source | Code | 原典 | uro:source |
| floorId | String | 階層ID | uro:floorId |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:InlandFloodingRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:KeyValuePairAttribute

拡張属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| key | Code | （コードに該当する説明を使用する） | uro:key |
| codeValue | Code | 値 | uro:codeValue |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:LandSlideRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code | nan | uro:description |
| areaType | Code | nan | uro:areaType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:LandUseDetailAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| orgLandUse | Code |  | uro:orgLandUse |
| nominalAr | Measure |  | uro:nominalArea |
| ownerType | Code |  | uro:ownerType |
| owner | String |  | uro:owner |
| areaSqMetr | Measure |  | uro:areaInSquareMeter |
| areaInHa | Measure |  | uro:areaInHa |
| CovRate | Double |  | uro:buildingCoverageRate |
| flrArRate | Double |  | uro:floorAreaRate |
| specfidBCR | Double |  | uro:specifiedBuildingCoverageRate |
| specifdFAR | Double |  | uro:specifiedFloorAreaRate |
| stdFAR | Double |  | uro:standardFloorAreaRate |
| urbPlanTy | Code |  | uro:urbanPlanType |
| arClsTy | Code |  | uro:areaClassificationType |
| distrZonTy | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:districtsAndZonesType |
| prefecture | Code |  | uro:prefecture |
| city | Code |  | uro:city |
| reference | String |  | uro:reference |
| note | String |  | uro:note |
| surveyYear | String |  | uro:surveyYear |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:LargeCustomerFacilityAttribute

大規模小売店舗等の立地状況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 分類 | uro:class |
| name | String | 施設名称 | uro:name |
| capacity | Integer | 収容数 | uro:capacity |
| owner | String | 施設主 | uro:owner |
| totalFlrAr | Measure | 延床面積 | uro:totalFloorArea |
| tStorFlrAr | Measure | 店舗床面積 | uro:totalStoreFloorArea |
| inaugDate | Date | 開業日（開校日） | uro:inauguralDate |
| yearOpened | String |  | uro:yearOpened |
| yearClosed | String |  | uro:yearClosed |
| keyTenants | String | 核テナント | uro:keyTenants |
| availablty | Boolean | 利用可能性 | uro:availability |
| urbPlanTy | Code | 都市計画区域 | uro:urbanPlanType |
| arClsTy | Code | 区域区分 | uro:areaClassificationType |
| distrZonTy | JSON (<code><a href="#code">Code</a>[]</code>) | 地域地区 | uro:districtsAndZonesType |
| luseType | Code | 土地利用区分 | uro:landUseType |
| reference | String | 図面対象番号 | uro:reference |
| note | String | 備考 | uro:note |
| surveyYear | String | 調査年 | uro:surveyYear |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:LengthAttribute

長さ情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| length | Measure | 長さ情報 | uro:length |
| mesureType | Code | 計測方法 | uro:mesureType |
| phaseType | Code | 計測段階 | uro:phaseType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:LongevityMeasures


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| fiscalYearForCountermeasures | String |  | uro:fiscalYearForCountermeasures |
| countermeasuresCost | JSON (<code><a href="#urocountermeasurescost">uro:CountermeasuresCost</a></code>) |  | uro:countermeasuresCost |
| desc | String |  | uro:description |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:MaintenanceHistoryAttribute

工事・点検情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| maintTy | Code | 点検・工事種類 | uro:maintenanceType |
| maintFY | String | 実施年度 | uro:maintenanceFiscalYear |
| maintYear | String | 実施年度 | uro:maintenanceYear |
| maintDate | Date | 実施日 | uro:maintenanceDate |
| status | String | 実施状況 | uro:status |
| desc | String | 実施内容 | uro:description |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:MooringFacility

係留施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| maiPartLen | Measure | 延長－取付部を除く延長 | uro:mainPartLength |
| totalLen | Measure | 延長－取付部を含む延長 | uro:totalLength |
| faclWidth | Measure | 施設の幅 | uro:facilityWidth |
| apronWidth | Measure | エプロン幅 | uro:apronWidth |
| plnDpth | Measure | 水深－計画上の水深 | uro:plannedDepth |
| currDpth | Measure | 水深－現在の水深 | uro:currentDepth |
| area | Measure | 面積 | uro:area |
| ceilHgt | Measure | 天端高 | uro:ceilingHeight |
| gravResis | Measure | 耐重力 | uro:gravityResistant |
| form | Code | 形態 | uro:form |
| maiVessels | Code | 主要利用船舶の種類 | uro:mainVessels |
| moorPstWgt | Measure | 附帯設備－係船柱の重さ | uro:mooringPostWeight |
| #MoorPosts | Integer | 附帯設備－係船柱の数 | uro:numberOfMooringPosts |
| resisMatl | Integer | 附帯設備－防げん材 | uro:resistantMaterial |
| lighting | Integer | 附帯設備－照明設備 | uro:lighting |
| stairs | Integer | 附帯設備－階段等 | uro:stairs |
| lifesvAppl | String | 附帯設備－救命設備の名称 | uro:lifesavingAppliances |
| #lifesvApl | Integer | 附帯設備－救命設備の数 | uro:numberOfLifesavingAppliances |
| bumper | Measure | 附帯設備－車止め | uro:bumper |
| #VehclBrdn | Integer | 附帯設備－車両乗降設備－基数 | uro:numberOfVehicleBoardings |
| vehcBrdWth | Measure | 附帯設備－車両乗降設備－幅員 | uro:vehicleBoardingWidth |
| shipType | String | 対象船舶－船型（D／W） | uro:shipType |
| #Seats | Integer | 対象船舶－船席数 | uro:numberOfSeats |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| structutTy | Code | 構造形式 | uro:structureType |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:NavigationAssistanceFacility

航行補助施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | String | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:NumberOfFacilities


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| quantity | Integer |  | uro:quantity |
| quantityUnit | Code |  | uro:quantityUnit |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:Occupancy

占有状況

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| interval | Code | 期間 | uro:interval |
| #Occupants | Integer | 数 | uro:numberofOccupants |
| occupantTy | Code | 種類 | uro:occupantType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:OffsetDepth

オフセットデプス情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| pos | Point | 計測位置 | uro:pos |
| offset | Measure | オフセット量 | uro:offset |
| depth | Measure | 土被り量 | uro:depth |
| minDepth | Measure | 最小土被り量 | uro:minDepth |
| maxDepth | Measure | 最大土被り量 | uro:maxDepth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ParkFacilityLongevityPlanAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String |  | uro:facilityId |
| parkCode | Code |  | uro:parkCode |
| parkName | Code |  | uro:parkName |
| parkType | Code |  | uro:parkType |
| facilityName | Code |  | uro:facilityName |
| facilityNameOptional | String |  | uro:facilityNameOptional |
| specificFacilityName | String |  | uro:specificFacilityName |
| numberOfFacilities | JSON (<code><a href="#uronumberoffacilities">uro:NumberOfFacilities</a></code>) |  | uro:numberOfFacilities |
| size | String |  | uro:size |
| mainMat | Code |  | uro:mainMaterial |
| mainMaterialOptional | String |  | uro:mainMaterialOptional |
| installationYear | String |  | uro:installationYear |
| disposalLimitPeriod | Integer |  | uro:disposalLimitPeriod |
| expectedUsagePeriod | Integer |  | uro:expectedUsagePeriod |
| repairsBeforeParkHealthAssessment | JSON (<code><a href="#urorepairsbeforeparkhealthassessment">uro:RepairsBeforeParkHealthAssessment</a></code>) |  | uro:repairsBeforeParkHealthAssessment |
| parkHealthAssessment | JSON (<code><a href="#uroparkhealthassessment">uro:ParkHealthAssessment</a>[]</code>) |  | uro:parkHealthAssessment |
| managementType | Code |  | uro:managementType |
| expectedRenewalYearWithMeasures | String |  | uro:expectedRenewalYearWithMeasures |
| longevityMeasures | JSON (<code><a href="#urolongevitymeasures">uro:LongevityMeasures</a>[]</code>) |  | uro:longevityMeasures |
| noteForLongevity | String |  | uro:noteForLongevity |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ParkHealthAssessment


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| assessmentFiscalYear | String |  | uro:assessmentFiscalYear |
| deteriorationStatus | String |  | uro:deteriorationStatus |
| condition | Code |  | uro:condition |
| urgency | Code |  | uro:urgency |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortEnvironmentalImprovementFacility

環境整備施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| usage | String | 用途等 | uro:usage |
| length | Measure | 延長 | uro:length |
| area | Measure | 面積 | uro:area |
| totFloorAr | Measure | 総床面積 | uro:totalFoorArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortManagementFacility

港湾管理施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| totalFlrAr | Measure | 総床面積 | uro:totalFloorArea |
| #ShipTypes | Integer | 船型数量 | uro:numberOfShipTypes |
| unitShipTy | Code | 船型単位 | uro:unitOfShipType |
| loadingCap | Integer | 積載量 | uro:loadingCapacity |
| acqYear | String | 取得年度 | uro:acquisitionYear |
| usage | String | 用途 | uro:usage |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortPassengerFacility

旅客施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalFlrAr | Measure | 総床面積 | uro:totalFloorArea |
| acqYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortPollutionControlFacility

公害防止施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| length | Measure | 延長 | uro:length |
| width | Measure | 幅員 | uro:width |
| crssSectAr | Measure | 断面積 | uro:crossSectionalArea |
| area | Measure | 面積 | uro:area |
| height | Measure | 高さ | uro:height |
| mainMat | Code | 主要用材コードリスト | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortProtectiveFacility

外郭施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| structutTy | Code | 構造形式 | uro:structureType |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | JSON (<code><a href="#string">String</a>[]</code>) | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortStorageFacility

保管施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| inTotFlrAr | Measure | 臨港地区内－総床面積 | uro:innerTotalFloorArea |
| inSiteAr | Measure | 臨港地区内－敷地面積 | uro:innerOfSiteArea |
| oTotFlrAr | Measure | 臨港地区外－総床面積 | uro:outerOfTotalFloorArea |
| outSiteAr | Measure | 臨港地区外－敷地面積 | uro:outerSiteArea |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| storageCap | Integer | 保管容量－値 | uro:storageCapacity |
| stoCapUnit | Code | 保管容量－単位 | uro:storageCapacityUnit |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortTransportationFacility

臨港交通施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| structutTy | Code | 構造形式 | uro:structureType |
| startPoint | String | 起終点 | uro:startingPoint |
| length | Measure | 規模_延長 | uro:length |
| area | Measure | 規模_面積 | uro:area |
| beddWth | Measure | 規模_道路敷幅 | uro:beddingWidth |
| #Lanes | Integer | 規模_車線数 | uro:numberOfLanes |
| parkCapBus | Integer | 規模_駐車場収容台数_バス | uro:parkingLotCapacityOfBus |
| parkCapCar | Integer | 規模_駐車場収容台数_乗用車 | uro:parkingLotCapacityOfCars |
| routeType | Code | 規模_単線・複線区分 | uro:routeType |
| hgtToDigit | Measure | 規模_桁下高 | uro:heightToDigit |
| heightLim | Measure | 規模_制限高 | uro:heightLimit |
| minWidth | Measure | 規模_車道幅員 | uro:minimumWidth |
| minDpth | Measure | 規模_最小水深 | uro:minimumDepth |
| #aircftPrk | Integer | 規模_駐機数 | uro:numberOfAircraftParkingSpaces |
| pavementTy | Code | 舗装形態/塗装形態 | uro:pavementType |
| mainCargo | Code | 主要取扱貨物名 | uro:mainCargo |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortWasteTreatmentFacility

廃棄物処理施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| structutTy | Code | 構造形式 | uro:structureType |
| perimeter | Measure | 延長_外周建設延長 | uro:perimeter |
| maiPartLen | Measure | 延長_機能保有延長 | uro:mainPartLength |
| inShoreLen | Measure |  | uro:innerShoreLength |
| ceilHgt | Measure | 天端高 | uro:ceilingHeight |
| wavDsptLen | Measure | 消波工延長 | uro:waveDissipatorLength |
| mainMat | Code | 主要用材 | uro:mainMaterial |
| wasteType | Code | 廃棄物の種類 | uro:wasteType |
| plnDispoAr | Measure | 計画処分面積 | uro:plannedDisposalArea |
| plnDispAmt | Integer | 計画処分量 | uro:plannedDisposalAmount |
| recvCap | Integer | 受入容量 | uro:receivingCapacity |
| shipType | String | 船型 | uro:shipType |
| unitRcvCap | Code | 受入容量単位 | uro:unitOfReceivingCapacity |
| acqYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| subsidy | Integer | 事業費－補助金額 | uro:subsidy |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PortWelfareFacility

厚生施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| totalFlrAr | Measure | 面積_防波堤等の外側 | uro:totalFloorArea |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:PublicSurveyDataQualityAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| srcScaleLod0 | Code |  | uro:srcScaleLod0 |
| srcScaleLod1 | Code |  | uro:srcScaleLod1 |
| srcScaleLod2 | Code |  | uro:srcScaleLod2 |
| srcScaleLod3 | Code |  | uro:srcScaleLod3 |
| srcScaleLod4 | Code |  | uro:srcScaleLod4 |
| publicSurveySrcDescLod0 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:publicSurveySrcDescLod0 |
| publicSurveySrcDescLod1 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:publicSurveySrcDescLod1 |
| publicSurveySrcDescLod2 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:publicSurveySrcDescLod2 |
| publicSurveySrcDescLod3 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:publicSurveySrcDescLod3 |
| publicSurveySrcDescLod4 | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:publicSurveySrcDescLod4 |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RailwayRouteAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| operatorTy | Code | 鉄道事業者区分 | uro:operatorType |
| operator | String | 鉄道事業者名 | uro:operator |
| altNm | JSON (<code><a href="#string">String</a>[]</code>) | 路線別称 | uro:alternativeName |
| type | Code | 鉄道区分 | uro:railwayType |
| stStation | String | 起点駅名 | uro:startStation |
| endStation | String | 終点駅名 | uro:endStation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RailwayTrackAttribute

軌道中心線線形情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeName | String | 路線名称 | uro:routeName |
| directnTy | Code | 進行方向区分 | uro:directionType |
| trackType | Code | 軌道の種類 | uro:trackType |
| startPost | String | 開始キロ程 | uro:startPost |
| endPost | String | 終了キロ程 | uro:endPost |
| alignmntTy | Code | 線形種別 | uro:alignmentType |
| ctrlPoint | JSON (<code><a href="#urocontrolpoint">uro:ControlPoint</a>[]</code>) | 線形変化点 | uro:controlPoint |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RealEstateIDAttribute

不動産ID

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| reIdBldg | String | 建築物不動産ID | uro:realEstateIDOfBuilding |
| #bldUnitOw | Integer | 区分所有数 | uro:numberOfBuildingUnitOwnership |
| reIdBldOwn | JSON (<code><a href="#string">String</a>[]</code>) | 区分不動産ID | uro:realEstateIDOfBuildingUnitOwnership |
| #reIdLand | Integer | 土地不動産ID数 | uro:numberOfRealEstateIDOfLand |
| reIdLand | JSON (<code><a href="#string">String</a>[]</code>) | 土地不動産ID数 | uro:realEstateIDOfLand |
| matchScore | Integer | マッチングスコア | uro:matchingScore |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RepairsBeforeParkHealthAssessment


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| repair | Code |  | uro:repair |
| repairFiscalYear | String |  | uro:repairFiscalYear |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ReservoirFloodingRiskAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code |  | uro:description |
| rank | Code |  | uro:rank |
| rankOrg | Code |  | uro:rankOrg |
| depth | Measure |  | uro:depth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RiverFacilityIdAttribute

河川管理施設識別属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| partId | String |  | uro:partId |
| branchId | String |  | uro:branchId |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:prefecture |
| city | JSON (<code><a href="#code">Code</a>[]</code>) |  | uro:city |
| route | String |  | uro:route |
| startPost | String |  | uro:startPost |
| endPost | String |  | uro:endPost |
| startLat | Double |  | uro:startLat |
| startLong | Double |  | uro:startLong |
| altNm | JSON (<code><a href="#string">String</a>[]</code>) |  | uro:alternativeName |
| riverCode | Code |  | uro:riverCode |
| riverName | String |  | uro:riverName |
| sideType | Code |  | uro:sideType |
| leftPost | Measure |  | uro:leftPost |
| leftDist | Measure |  | uro:leftDistance |
| rightPost | Measure |  | uro:rightPost |
| rightDist | Measure |  | uro:rightDistance |
| leftStPost | Measure |  | uro:leftStartPost |
| leftStDist | Measure |  | uro:leftStartDistance |
| leftEdPst | Measure |  | uro:leftEndPost |
| lefEndDist | Measure |  | uro:lefEndDistance |
| rightStPst | Measure |  | uro:rightStartPost |
| rightStDis | Measure |  | uro:rightStartDistance |
| rightEdPst | Measure |  | uro:rightEndPost |
| rightEdDis | Measure |  | uro:rightEndDistance |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RiverFloodingRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |
| adminType | Code | nan | uro:adminType |
| scale | Code | nan | uro:scale |
| duration | Measure | nan | uro:duration |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RoadStructureAttribute

道路構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| widthType | Code | 幅員区分 | uro:widthType |
| width | Measure | 幅員 | uro:width |
| #Lanes | Integer | 車線数 | uro:numberOfLanes |
| sectionTy | Code | 区間種別 | uro:sectionType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:RoadType


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| id | String |  | uro:id |
| creatDate | Date |  | uro:creationDate |
| temporary | Boolean |  | uro:isTemporary |
| roadType | Code |  | uro:roadType |
| widthType | Code |  | uro:widthType |
| isTollRoad | Boolean |  | uro:isTollRoad |
| separator | Measure |  | uro:separator |
| isHighWay | Boolean |  | uro:isHighWay |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ShipServiceFacility

船舶役務用施設

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| facilityId | String | 管理ID | uro:facilityId |
| detailTy | Code | 施設種類 | uro:portFacilityDetailsType |
| portName | String | 港湾名 | uro:portName |
| portStatus | Code | 港格 | uro:portStatus |
| district | String | 地区名 | uro:district |
| grantType | Code | 施設区分 | uro:grantType |
| desig | Boolean | 特定技術基準対象施設 | uro:isDesignated |
| degradatLv | Integer | 性能低下度 | uro:degradationLevel |
| shipType | String | 対象船舶－船型（D／W） | uro:shipType |
| suplAbilty | Integer | 供給能力容量 | uro:supplyAbility |
| suplAbUnit | Code | 供給能力単位 | uro:supplyAbilityUnit |
| moorPlace | String | 補給を受ける船舶の係留場所 | uro:mooringPlace |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅 | uro:width |
| area | Measure | 面積 | uro:area |
| acqYear | String | 取得年度 | uro:acquisitionYear |
| totalCost | Integer | 事業費－総額 | uro:totalCost |
| note | String | 備考 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:SlopeType

勾配変化情報

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| angle | Double | 勾配角度 | uro:angle |
| elevation | Measure | 標高 | uro:elevation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:SquareUrbanPlanAttribute

広場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbPlnArNm | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) | 施行者名 | uro:enforcer |
| dateDecis | Date | 決定日 | uro:dateOfDecision |
| dateRevis | JSON (<code><a href="#date">Date</a>[]</code>) | 変更日 | uro:dateOfRevision |
| areaPlannd | Measure | 計画面積 | uro:areaPlanned |
| arService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| arImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| arComplet | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| prjStDate | Date | 事業開始年月日 | uro:projectStartDate |
| prjEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| completed | Boolean | 完成区分 | uro:isCompleted |
| authorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:StationSquareAttribute

駅前広場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbPlnArNm | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) | 施行者名 | uro:enforcer |
| dateDecis | Date | 決定日 | uro:dateOfDecision |
| dateRevis | JSON (<code><a href="#date">Date</a>[]</code>) | 変更日 | uro:dateOfRevision |
| areaPlannd | Measure | 計画面積 | uro:areaPlanned |
| arService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| arImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| arComplet | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| prjStDate | Date | 事業開始年月日 | uro:projectStartDate |
| prjEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| completed | Boolean | 完成区分 | uro:isCompleted |
| authorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |
| station | JSON (<code><a href="#string">String</a>[]</code>) | 駅名 | uro:station |
| route | JSON (<code><a href="#string">String</a>[]</code>) | 路線名 | uro:route |
| type | JSON (<code><a href="#code">Code</a>[]</code>) | 鉄道種別 | uro:railwayType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TerminalAttribute

自動車ターミナル属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| prefecture | Code | 都道府県 | uro:prefecture |
| city | Code | 市区町村 | uro:city |
| urbPlnArNm | String | 都市計画区域名称 | uro:urbanPlanningAreaName |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) | 施行者名 | uro:enforcer |
| dateDecis | Date | 決定日 | uro:dateOfDecision |
| dateRevis | JSON (<code><a href="#date">Date</a>[]</code>) | 変更日 | uro:dateOfRevision |
| areaPlannd | Measure | 計画面積 | uro:areaPlanned |
| arService | Measure | 供用面積 | uro:areaInService |
| remarks | String | 摘要 | uro:remarks |
| status | Code | 進捗状況 | uro:status |
| arImproved | Measure | 改良済み面積又は延長 | uro:areaImproved |
| arComplet | Measure | 概成済み面積又は延長 | uro:areaCompleted |
| prjStDate | Date | 事業開始年月日 | uro:projectStartDate |
| prjEndDate | Date | 事業終了年月日 | uro:projectEndDate |
| completed | Boolean | 完成区分 | uro:isCompleted |
| authorized | Boolean | 認可区分 | uro:isAuthorized |
| purpose | String | 目的 | uro:purpose |
| note | String | その他特筆事項 | uro:note |
| terminalTy | Code | ターミナル区分 | uro:terminalType |
| structure | String | ターミナル区分構造 | uro:structure |
| #BerthsPln | Integer | 計画バース数 | uro:numberOfBerthsPlanned |
| #berthsSvc | Integer | 供用バース数 | uro:numberOfBerthsInService |
| userType | Code |  | uro:userType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:ThematicShape

主題図形

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| horizTy | Code | 水平区分 | uro:horizontalType |
| heightType | Code | 高さ区分 | uro:heightType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TrackAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| altNm | JSON (<code><a href="#string">String</a>[]</code>) | 別名 | uro:alternativeName |
| adminType | Code | 管理区分 | uro:adminType |
| relatLv | Integer | 階層準 | uro:relativeLevel |
| widthType | Code | 幅員区分 | uro:widthType |
| structutTy | Code | 構造区分 | uro:structureType |
| isTollRoad | Boolean | 有料区分 | uro:isTollRoad |
| separator | Measure | 分離帯区分 | uro:separator |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TrafficAreaStructureAttribute

道路構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| #Lanes | Integer | 車線数 | uro:numberOfLanes |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TrafficObjectUsecaseAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| minWidth | Measure |  | uro:minWidth |
| maxWidth | Measure |  | uro:maxWidth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TrafficVolumeAttribute

交通量属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| sectionID | String | 交通調査基本区間番号 | uro:sectionID |
| routeName | String | nan | uro:routeName |
| w12hTfcVol | Integer | 平日12時間交通量 | uro:weekday12hourTrafficVolume |
| w24hTfcVol | Integer | 平日24時間交通量 | uro:weekday24hourTrafficVolume |
| lgVehRate | Double | 大型車混入率 | uro:largeVehicleRate |
| cngestRate | Double | 混雑度 | uro:congestionRate |
| avgTSC | Double | 混雑時平均旅行速度 | uro:averageTravelSpeedInCongestion |
| avgInTSC | Double | 混雑時平均旅行速度（上り） | uro:averageInboundTravelSpeedInCongestion |
| avgOutTSC | Double | 混雑時平均旅行速度（下り） | uro:averageOutboundTravelSpeedInCongestion |
| avgInTSNC | Double | 非混雑時平均旅行速度（上り） | uro:averageInboundTravelSpeedNotCongestion |
| avgOutTSNC | Double | 非混雑時平均旅行速度（下り） | uro:averageOutboundTravelSpeedNotCongestion |
| obsPointNm | String | 観測地点名 | uro:observationPointName |
| reference | String | 参照情報 | uro:reference |
| surveyYear | String | 調査年 | uro:surveyYear |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TransitionCurveType

緩和曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| int | Measure | 交角 | uro:intersection |
| distance | Measure | 移動距離 | uro:distance |
| curveLen | Measure | 曲線長 | uro:curveLength |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TsunamiRiskAttribute

nan

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| desc | Code | nan | uro:description |
| rank | Code | nan | uro:rank |
| rankOrg | Code | nan | uro:rankOrg |
| depth | Measure | nan | uro:depth |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TunnelFunctionalAttribute

トンネル機能属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| directnTy | Code | 進行方向区分 | uro:directionType |
| userType | Code | 利用者区分 | uro:userType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:TunnelStructureAttribute

トンネル構造属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| tunnelType | Code | 種類 | uro:tunnelType |
| subtype | Code | 詳細種類 | uro:tunnelSubtype |
| mouthType | JSON (<code><a href="#code">Code</a>[]</code>) | 坑口区分 | uro:mouthType |
| length | Measure | 長さ | uro:length |
| width | Measure | 幅員 | uro:width |
| area | Measure | 面積 | uro:area |
| inHeight | Measure | 内空高 | uro:innerHeight |
| effectHgt | Measure | 有効高 | uro:effectiveHeight |
| slopeType | Code | 昇降形式 | uro:slopeType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:UserDefinedValue


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| strVal | String |  | uro:stringValue |
| intValue | Integer |  | uro:intValue |
| douVal | Double |  | uro:doubleValue |
| codeValue | Code |  | uro:codeValue |
| dateValue | Date |  | uro:dateValue |
| uriValue | URI |  | uro:uriValue |
| measurVal | Measure |  | uro:measuredValue |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:VerticalCurveType

縦曲線パラメータ

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| length | Measure | 長さ | uro:length |
| vertDist | Measure | 縦距 | uro:verticalDistance |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:WaterBodyDetailAttribute

水部詳細属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| kana | String | フリガナ | uro:kana |
| wtrSysCode | Code | 水系域コード | uro:waterSystemCode |
| riverCode | Code | 河川コード | uro:riverCode |
| adminType | Code | 区間種別 | uro:adminType |
| flowDir | Boolean | 流下方向区分 | uro:flowDirection |
| maxDepth | Measure | 最大水深 | uro:maximumDepth |
| wtrSurfElv | Measure | 水面標高 | uro:waterSurfaceElevation |
| area | Measure | 面積 | uro:area |
| measureYM | String | 測量年月 | uro:measurementYearMonth |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) | 都道府県 | uro:prefecture |
| city | JSON (<code><a href="#code">Code</a>[]</code>) | 市区町村 | uro:city |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### uro:WaterwayDetailAttribute

鉄道路線属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeId | String | 航路番号 | uro:routeId |
| routeDir | Code | 進行方向 | uro:routeDirection |
| minWidth | Measure | 最小幅員 | uro:minimumWidth |
| maxWidth | Measure | 最大幅員 | uro:maximumWidth |
| length | Measure | 航路延長 | uro:length |
| navigation | String | 航法 | uro:navigation |
| plnDpth | Measure | 計画水深 | uro:plannedDepth |
| speedLimit | Measure | 速力制限 | uro:speedLimit |
| tgtShipTy | JSON (<code><a href="#string">String</a>[]</code>) | 対象船型 | uro:targetShipType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:Boundary

境界

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| class | Code | 分類 | urf:class |
| function | JSON (<code><a href="#code">Code</a>[]</code>) | 境界線の確定根拠 | urf:function |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) | 区域の種類 | urf:usage |
| offset | Measure | オフセット値 | urf:offset |
| offsetDir | String | オフセット値の方向 | urf:offsetDirection |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:BreakdownOfNominalArea


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| breakdown | String |  | urf:breakdown |
| areaSqMetr | Measure |  | urf:areaInSquareMeter |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ParkAttribute

公園属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parkTy# | Code | 区分 | urf:parkTypeNumber |
| parkSize# | Code | 規模 | urf:parkSizeNumber |
| parkSer# | String | 一連番号 | urf:parkSerialNumber |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:ParkingPlaceAttribute

駐車場属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| strysAbvG | NonNegativeInteger | 地上階数 | urf:storeysAboveGround |
| strysBlwG | NonNegativeInteger | 地下階数 | urf:storeysBelowGround |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:SewerSystemAttribute

下水道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| stLoc | String | 起点 | urf:startLocation |
| endLoc | String | 終点 | urf:endLocation |
| systemType | Code | 種別 | urf:systemType |
| drainageAr | String | 排水区域 | urf:drainageArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:StructureDetails

構造詳細

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| stLoc | String | 起点 | urf:startLocation |
| endLoc | String | 終点 | urf:endLocation |
| viaLocs | String | 経過地 | urf:viaLocations |
| length | Measure | 延長 | urf:length |
| structutTy | Code | 構造の形式 | urf:structureType |
| minWidth | Measure | 最小幅員 | urf:minimumWidth |
| maxWidth | Measure | 最大幅員 | urf:maximumWidth |
| stdWidth | Measure | 標準幅員 | urf:standardWidth |
| crossType | Code | 交差種別 | urf:crossType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanParkAttribute


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| parkCode | Code |  | urf:parkCode |
| startFrom | Date |  | urf:startFrom |
| breakdownOfNominalArea | JSON (<code><a href="#urfbreakdownofnominalarea">urf:BreakdownOfNominalArea</a>[]</code>) |  | urf:breakdownOfNominalArea |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanRapidTransitRailroadAttribute

都市高速鉄道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| structutTy | Code | 構造種別 | urf:structureType |
| crossType | Code | 交差種別 | urf:crossType |
| structDtl | JSON (<code><a href="#urfstructuredetails">urf:StructureDetails</a>[]</code>) |  | urf:structuralDetails |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:UrbanRoadAttribute

道路属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| routeTy# | Code | 区分 | urf:routeTypeNumber |
| routeSize# | Code | 規模 | urf:routeSizeNumber |
| routeSer# | String | 一連番号 | urf:routeSerialNumber |
| roadType | Code | 道路の種類 | urf:roadType |
| #Lanes | Integer | 車線数 | urf:numberOfLanes |
| roadStruct | String | 道路の構造 | urf:roadStructure |
| structutTy | Code | 構造種別 | urf:structureType |
| crossType | Code | 交差種別 | urf:crossType |
| tfcPlazas | Code | 交通広場の有無 | urf:trafficPlazas |
| structDtl | JSON (<code><a href="#urfstructuredetails">urf:StructureDetails</a>[]</code>) | 構造の内訳 | urf:structuralDetails |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:VehicleTerminalAttribute

自動車ターミナル属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| terminalTy | Code | 自動車ターミナルの種類 | urf:terminalType |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### urf:WaterWorksAttribute

水道属性

| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| stLoc | String | 起点 | urf:startLocation |
| endLoc | String | 終点 | urf:endLocation |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### xAL:AddressDetails


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| Country | JSON (<code><a href="#xalcountry">xAL:Country</a></code>) |  | xAL:Country |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

### xAL:Country


| フィールド名 | 型 | 日本語名 | CityGML 属性名 |
|-----------|----|--------|---------------|
| CountryName | String |  | xAL:CountryName |
| Locality | String |  | xAL:Locality |
| parentId | String |  | parentId |
| parentType | String |  | parentType |

