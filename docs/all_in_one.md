## bldg:Building

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | NonNegativeInteger |
| yearOfDemolition | NonNegativeInteger |
| roofType | Code |
| measuredHeight | Measure |
| storeysAboveGround | NonNegativeInteger |
| storeysBelowGround | NonNegativeInteger |
| storeyHeightsAboveGround | String |
| storeyHeightsBelowGround | String |
| outerBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| interiorBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) |
| interiorRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) |
| consistsOfBuildingPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |
| bldgDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| bldgFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| bldgFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| bldgFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| bldgRealEstateIDAttribute | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) |
| buildingDataQualityAttribute | JSON (<code><a href="#urobuildingdataqualityattribute">uro:BuildingDataQualityAttribute</a></code>) |
| buildingDetailAttribute | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) |
| buildingDisasterRiskAttribute | JSON (<code><a href="#urobuildingdisasterriskattributeproperty">uro:BuildingDisasterRiskAttributeProperty</a>[]</code>) |
| buildingIDAttribute | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a></code>) |
| ifcBuildingAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBuildingAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| keyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |
| largeCustomerFacilityAttribute | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) |

## gen:genericAttribute

| field | type |
|-------|------|

## bldg:BuildingInstallation

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) |
| ifcBuildingInstallationAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |

## bldg:CeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:Door

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| ifcOpeningAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorOpeningAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |

## uro:IfcBuilding

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| longName | String |
| compositionType | String |
| elevationOfRefHeight | Measure |
| elevationOfTerrain | Measure |
| buildingAddress | JSON (<code><a href="#coreaddress">core:Address</a></code>) |

## core:Address

| field | type |
|-------|------|

## uro:IfcBuildingElement

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |

## uro:IfcBuildingStorey

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| longName | String |
| compositionType | String |
| elevation | Measure |

## uro:IfcClassificationReference

| field | type |
|-------|------|
| location | URI |
| itemReference | Code |
| name | String |
| referenceSource | JSON (<code><a href="#uroifcclassification">uro:IfcClassification</a></code>) |

## uro:IfcClassification

| field | type |
|-------|------|
| source | String |
| edition | String |
| editionDate | Date |
| name | String |

## uro:IfcCoordinateReferenceSystem

| field | type |
|-------|------|
| name | String |
| description | String |
| geodeticDatum | String |
| verticalDatum | String |

## uro:IfcCurtainWall

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |

## uro:IfcDoor

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |
| overallHeight | Measure |
| overallWidth | Measure |

## uro:IfcFurnishingElement

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |

## uro:IfcGroup

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |

## uro:IfcMapConversion

| field | type |
|-------|------|
| sourceCRS | JSON (<code><a href="#uroifccoordinatereferencesystemselecttype">uro:IfcCoordinateReferenceSystemSelectType</a></code>) |
| targetCRS | JSON (<code><a href="#uroifccoordinatereferencesystemproperty">uro:IfcCoordinateReferenceSystemProperty</a></code>) |
| eastings | Measure |
| northings | Measure |
| orthogonalHeight | Measure |
| xAxisAbscissa | Double |
| xAxisOrdinate | Double |
| scale | Double |

## uro:IfcProjectedCRS

| field | type |
|-------|------|
| name | String |
| description | String |
| geodeticDatum | String |
| verticalDatum | String |
| mapUnit | String |
| mapProjection | String |
| mapZone | String |

## uro:IfcCoordinateReferenceSystemProperty

以下のいずれかの型をとります：

- uro:IfcCoordinateReferenceSystem
- uro:IfcProjectedCRS

## uro:IfcGeometricRepresentationContext

| field | type |
|-------|------|
| contextIdentifier | String |
| contextType | String |
| coordinateSpaceDimension | Integer |
| precision | Double |
| worldCoordinateSystem | JSON (<code><a href="#uroifcaxis2placement3d">uro:IfcAxis2Placement3D</a></code>) |
| trueNorth | String |

## uro:IfcAxis2Placement3D

| field | type |
|-------|------|
| location | Point |
| axis | String |
| refDirection | String |

## uro:IfcCoordinateReferenceSystemSelectType

以下のいずれかの型をとります：

- uro:IfcCoordinateReferenceSystemProperty
- uro:IfcGeometricRepresentationContext

## uro:IfcOpeningElement

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| nominalArea | Measure |
| nominalVolume | Measure |

## uro:IfcProject

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| longName | String |
| phase | String |
| representationContexts | JSON (<code><a href="#uroifcgeometricrepresentationcontext">uro:IfcGeometricRepresentationContext</a></code>) |
| unitsInContext | JSON (<code><a href="#uroifcunit">uro:IfcUnit</a>[]</code>) |

## uro:IfcUnit

| field | type |
|-------|------|
| dimensions | Integer |
| unitType | String |
| perfix | String |
| name | String |

## uro:IfcPsetBuildingCommon

| field | type |
|-------|------|
| buildingId | String |
| isPermanentId | Boolean |
| mainFireUse | String |
| ancillaryFireUse | String |
| sprinklerProtection | Boolean |
| sprinklerProtectionAutomatic | Boolean |
| occupancyType | Code |
| grossPlannedArea | Measure |
| numberOfStoreys | Integer |
| yearOfConstruction | NonNegativeInteger |
| isLandmarked | Boolean |

## uro:IfcPsetDoorCommon

| field | type |
|-------|------|
| reference | String |
| acousticRating | String |
| firerating | String |
| securityRating | String |
| isExternal | Boolean |
| infiltration | Double |
| thermalTransmittance | Double |
| glazingAreaFraction | Double |
| handicapAccessible | Boolean |
| fireExit | Boolean |
| selfClosing | Boolean |
| smokeStop | Boolean |

## uro:IfcPsetOpeningElementCommon

| field | type |
|-------|------|
| reference | String |
| purpose | String |
| fireExit | Boolean |
| protectedOpening | Boolean |
| parallelJambs | Boolean |

## uro:IfcPsetSiteCommon

| field | type |
|-------|------|
| buildableArea | Measure |
| totalArea | Measure |
| buildingHeightLimit | Measure |

## uro:IfcPsetSpaceCommon

| field | type |
|-------|------|
| reference | String |
| category | String |
| floorCovering | String |
| wallCovering | String |
| ceilingCovering | String |
| skirtingBoard | String |
| grossPlannedArea | Measure |
| netPlannedArea | Measure |
| publiclyAccessible | Boolean |
| handicapAccessible | Boolean |
| concealedFlooring | Boolean |
| concealedCeiling | Boolean |

## uro:IfcPsetWindowCommon

| field | type |
|-------|------|
| reference | String |
| acousticRating | String |
| fireRating | String |
| securityRating | String |
| isExternal | Boolean |
| infiltration | Double |
| thermalTransmittance | Double |
| glazingAreaFraction | Double |
| smokeStop | Boolean |

## uro:IfcRoof

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |

## uro:IfcSite

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| longName | String |
| compositionType | String |
| refLongitude | Double |
| refLatitude | Double |
| refElevation | Measure |
| landTitleNumber | String |
| siteAddress | JSON (<code><a href="#coreaddress">core:Address</a></code>) |

## uro:IfcSpace

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| longName | String |
| compositionType | String |
| interiorOrExteriorSpace | String |
| elevationWithFlooring | Measure |

## uro:IfcSpaceBaseQuantity

| field | type |
|-------|------|
| nominalHeight | Measure |
| clearHeight | Measure |
| finishCeilingHeight | Measure |
| grossPerimeter | Measure |
| netPerimeter | Measure |
| grossCeilingArea | Measure |
| grossFloorArea | Measure |
| netCeilingArea | Measure |
| netFloorArea | Measure |
| grossWallArea | Measure |
| netWallArea | Measure |
| grossVolume | Measure |
| netVolume | Measure |

## uro:IfcWall

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |
| nominalLength | Measure |
| nominalWidth | Measure |
| nominalHeight | Measure |
| grossFootPrintArea | Measure |
| netFootPrintArea | Measure |
| grossSideArea | Measure |
| netSideArea | Measure |
| grossSideAreaLeft | Measure |
| netSideAreaLeft | Measure |
| grossSideAreaRight | Measure |
| netSideAreaRight | Measure |
| grossVolume | Measure |
| netVolume | Measure |

## uro:IfcWallStandardCase

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |
| nominalLength | Measure |
| nominalWidth | Measure |
| nominalHeight | Measure |
| grossFootPrintArea | Measure |
| netFootPrintArea | Measure |
| grossSideArea | Measure |
| netSideArea | Measure |
| grossSideAreaLeft | Measure |
| netSideAreaLeft | Measure |
| grossSideAreaRight | Measure |
| netSideAreaRight | Measure |
| grossVolume | Measure |
| netVolume | Measure |

## uro:IfcWindow

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |
| tag | String |
| elementType | Code |
| predefinedType | Code |
| shapeType | Code |
| numberOfRiser | Integer |
| numberOfTreads | Integer |
| riserHeight | Measure |
| treadLength | Measure |
| operationType | String |
| capacityByWeight | Measure |
| capacityByNumber | Integer |
| overallHeight | Measure |
| overallWidth | Measure |

## uro:IfcZone

| field | type |
|-------|------|
| globalId | String |
| name | String |
| description | String |
| objectType | String |

## uro:IfcAttributeProperty

以下のいずれかの型をとります：

- uro:IfcBuilding
- uro:IfcBuildingElement
- uro:IfcBuildingStorey
- uro:IfcClassificationReference
- uro:IfcCoordinateReferenceSystem
- uro:IfcCurtainWall
- uro:IfcDoor
- uro:IfcFurnishingElement
- uro:IfcGroup
- uro:IfcMapConversion
- uro:IfcOpeningElement
- uro:IfcProject
- uro:IfcProjectedCRS
- uro:IfcPsetBuildingCommon
- uro:IfcPsetDoorCommon
- uro:IfcPsetOpeningElementCommon
- uro:IfcPsetSiteCommon
- uro:IfcPsetSpaceCommon
- uro:IfcPsetWindowCommon
- uro:IfcRoof
- uro:IfcSite
- uro:IfcSpace
- uro:IfcSpaceBaseQuantity
- uro:IfcWall
- uro:IfcWallStandardCase
- uro:IfcWindow
- uro:IfcZone

## uro:IndoorFacilityAttribute

| field | type |
|-------|------|
| source | Code |
| weekdayHours | String |
| weekendHours | String |
| phone | String |
| website | String |

## uro:IndoorFurnishingAttribute

| field | type |
|-------|------|
| source | Code |
| floorId | String |

## uro:IndoorPublicTagAttribute

| field | type |
|-------|------|
| source | Code |
| ucode | String |

## uro:IndoorSpaceAttribute

| field | type |
|-------|------|
| source | Code |
| floorId | String |
| isRestricted | Boolean |
| suite | String |
| isPublic | Boolean |
| tollType | Code |

## uro:IndoorStoreyAttribute

| field | type |
|-------|------|
| source | Code |
| category | Boolean |
| ordinal | Double |

## uro:IndoorTacatileTileAttribute

| field | type |
|-------|------|
| source | Code |
| startNode | String |
| endNode | String |
| category | Code |
| roof | String |
| floorId | String |

## uro:IndoorUserDefinedAttribute

| field | type |
|-------|------|
| source | Code |
| name | String |
| nominalValue | JSON (<code><a href="#urouserdefinedvalue">uro:UserDefinedValue</a></code>) |
| description | String |
| unit | String |

## uro:UserDefinedValue

| field | type |
|-------|------|
| stringValue | String |
| intValue | Integer |
| doubleValue | Double |
| codeValue | Code |
| dateValue | Date |
| uriValue | URI |
| measuredValue | Measure |

## uro:IndoorZoneAttribute

| field | type |
|-------|------|
| source | Code |
| floorId | String |

## uro:IndoorAttributeProperty

以下のいずれかの型をとります：

- uro:IndoorFacilityAttribute
- uro:IndoorFurnishingAttribute
- uro:IndoorPublicTagAttribute
- uro:IndoorSpaceAttribute
- uro:IndoorStoreyAttribute
- uro:IndoorTacatileTileAttribute
- uro:IndoorUserDefinedAttribute
- uro:IndoorZoneAttribute

## bldg:Window

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| ifcOpeningAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorOpeningAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:_OpeningProperty

以下のいずれかの型をとります：

- bldg:Door
- bldg:Window

## bldg:ClosureSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:FloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:GroundSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:InteriorWallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:OuterCeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:OuterFloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:RoofSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:WallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#bldg-openingproperty">bldg:_OpeningProperty</a>[]</code>) |
| ifcBoundarySurfaceAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBoundarySurfaceAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## bldg:_BoundarySurfaceProperty

以下のいずれかの型をとります：

- bldg:CeilingSurface
- bldg:ClosureSurface
- bldg:FloorSurface
- bldg:GroundSurface
- bldg:InteriorWallSurface
- bldg:OuterCeilingSurface
- bldg:OuterFloorSurface
- bldg:RoofSurface
- bldg:WallSurface

## bldg:Room

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) |
| interiorFurniture | JSON (<code><a href="#bldgbuildingfurniture">bldg:BuildingFurniture</a>[]</code>) |
| roomInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| ifcRoomAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorRoomAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| roomDataQualityAttribute | JSON (<code><a href="#uroroomdataqualityattribute">uro:RoomDataQualityAttribute</a></code>) |

## bldg:BuildingFurniture

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| ifcBuildingFurnitureAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorFutnitureAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |

## uro:RoomDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| lodType | Code |

## bldg:BuildingPart

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| roofType | Code |
| measuredHeight | Measure |
| storeysAboveGround | NonNegativeInteger |
| storeysBelowGround | NonNegativeInteger |
| storeyHeightsAboveGround | String |
| storeyHeightsBelowGround | String |
| outerBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| interiorBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) |
| interiorRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) |
| consistsOfBuildingPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |
| bldgDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| bldgFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| bldgFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| bldgFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| bldgRealEstateIDAttribute | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) |
| buildingDataQualityAttribute | JSON (<code><a href="#urobuildingdataqualityattribute">uro:BuildingDataQualityAttribute</a></code>) |
| buildingDetailAttribute | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) |
| buildingDisasterRiskAttribute | JSON (<code><a href="#urobuildingdisasterriskattributeproperty">uro:BuildingDisasterRiskAttributeProperty</a>[]</code>) |
| buildingIDAttribute | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a></code>) |
| ifcBuildingAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBuildingAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| keyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |
| largeCustomerFacilityAttribute | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) |

## uro:DmAnnotation

| field | type |
|-------|------|
| dmCode | Code |
| meshCode | Code |
| dmElement | JSON (<code><a href="#urodmelement">uro:DmElement</a></code>) |
| geometryType | Code |
| shapeType | Code |
| label | String |
| isVertical | Boolean |
| size | Integer |
| orientation | Integer |
| linewidth | Integer |
| spacing | Integer |

## uro:DmElement

| field | type |
|-------|------|
| locationType | Code |
| infoType | Code |
| elementKey | String |
| hierarchyLevel | String |
| dataType | Code |
| annotationType | Code |
| precisionType | Code |
| dislocationType | Code |
| breakType | Code |
| attributeValue | String |
| attributeType | Code |
| attributeValueType | String |
| creationDate | String |
| updateDate | String |
| terminationDate | String |
| freeSpace | String |

## uro:DmGeometricAttribute

| field | type |
|-------|------|
| dmCode | Code |
| meshCode | Code |
| dmElement | JSON (<code><a href="#urodmelement">uro:DmElement</a></code>) |
| geometryType | Code |
| mapLevel | Code |
| shapeType | Code |
| visibility | Boolean |
| is3d | Boolean |
| isInstallation | Boolean |
| isEdited | Boolean |
| isSupplementarySymbol | Boolean |
| angle | Double |
| elevation | Measure |

## uro:DmAttributeProperty

以下のいずれかの型をとります：

- uro:DmAnnotation
- uro:DmGeometricAttribute

## uro:CargoHandlingFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| mainCargo | Code |
| mooringFacility | String |
| liftableLoad | Measure |
| ability | Integer |
| packingName | Code |
| acquisitionYear | String |
| innerTotalFloorArea | Measure |
| innerOfSiteArea | Measure |
| outerOfTotalFloorArea | Measure |
| outerSiteArea | Measure |
| mainMaterial | Code |
| totalCost | Integer |
| note | String |

## uro:CyberportMarinaAndPBS

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| geologicalType | Code |
| obstructingStructures | String |
| mainPartLength | Measure |
| totalLength | Measure |
| waveDissipatorLength | Measure |
| facilityWidth | Measure |
| apronWidth | Measure |
| restrictionStructure | String |
| plannedDepth | Measure |
| currentDepth | Measure |
| innerTotalFloorArea | Measure |
| innerOfSiteArea | Measure |
| outerOfTotalFloorArea | Measure |
| outerSiteArea | Measure |
| ceilingHeight | Measure |
| gravityResistant | Measure |
| form | Code |
| areaType | Code |
| mainVessels | Code |
| isDredged | Boolean |
| mooringPostWeight | Measure |
| numberOfMooringPosts | Integer |
| resistantMaterial | Integer |
| lighting | Integer |
| stairs | Integer |
| lifesaving | String |
| lifesavingNumber | Integer |
| bumper | Measure |
| numberOfVehicleBoardings | Integer |
| vehicleBoardingWidth | Measure |
| shipType | String |
| numberOfSeats | Integer |
| mainCargo | Code |
| storageCapacity | Integer |
| storageCapacityUnit | Code |
| structureType | Code |
| mainMaterial | Code |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:FishingPortCapacity

| field | type |
|-------|------|
| facilityId | String |
| capacity | String |
| weightCapacity | Measure |
| hullForm | Integer |
| shipNumber | Integer |
| waterDepth-2m | Measure |
| waterDepth2-3m | Measure |
| waterDepth3-6m | Measure |
| waterDepth6-m | Measure |
| heightAboveAWL | Measure |
| heightOnFoundations | Measure |
| luminousRange | Measure |
| luminousColor | String |
| candlePower | Integer |
| lightType | String |
| period | String |
| maximumGroundingWeight | Integer |
| handleablePower | Integer |
| maximumWaterSupply | Integer |
| maximumRefueling | String |
| people | Integer |
| other | String |

## uro:FishingPortFacility

| field | type |
|-------|------|
| facilityId | String |
| facilityDetailsType | Code |
| portName | String |
| portType | Code |
| address | String |
| designatedArea | String |
| designation | JSON (<code><a href="#string">String</a>[]</code>) |
| designatedAdministrator | JSON (<code><a href="#string">String</a>[]</code>) |
| referenceNumber | JSON (<code><a href="#string">String</a>[]</code>) |
| grantType | Code |
| administrator | String |
| facilityManager | String |
| structureType | Code |
| mainMaterial | Code |
| otherStructure | String |
| length | Measure |
| width | Measure |
| ceilingHeight | Measure |
| depth | Measure |
| area | Measure |
| otherSizeDescription | String |
| dateOfConstructionOrAcquisition | Date |
| cost | Integer |
| note | String |

## uro:HarborFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| geologicalType | Code |
| obstructingStructures | String |
| structuralLimitations | Measure |
| length | Measure |
| minimumWidth | Measure |
| maximumWidth | Measure |
| plannedDepth | Measure |
| currentDepth | Measure |
| isDredged | Boolean |
| areaType | Code |
| innerArea | Measure |
| outerArea | Measure |
| totalCost | Integer |
| subsidy | Integer |
| note | JSON (<code><a href="#string">String</a>[]</code>) |

## uro:MaintenanceHistoryAttribute

| field | type |
|-------|------|
| facilityId | String |
| maintenanceType | Code |
| maintenanceFiscalYear | String |
| maintenanceYear | String |
| maintenanceDate | Date |
| status | String |
| description | String |

## uro:MooringFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| mainPartLength | Measure |
| totalLength | Measure |
| facilityWidth | Measure |
| apronWidth | Measure |
| plannedDepth | Measure |
| currentDepth | Measure |
| area | Measure |
| ceilingHeight | Measure |
| gravityResistant | Measure |
| form | Code |
| mainVessels | Code |
| mooringPostWeight | Measure |
| numberOfMooringPosts | Integer |
| resistantMaterial | Integer |
| lighting | Integer |
| stairs | Integer |
| lifesavingAppliances | String |
| numberOfLifesavingAppliances | Integer |
| bumper | Measure |
| numberOfVehicleBoardings | Integer |
| vehicleBoardingWidth | Measure |
| shipType | String |
| numberOfSeats | Integer |
| mainCargo | Code |
| structureType | Code |
| mainMaterial | Code |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:NavigationAssistanceFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| totalCost | Integer |
| subsidy | String |
| note | String |

## uro:PortEnvironmentalImprovementFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| usage | String |
| length | Measure |
| area | Measure |
| totalFoorArea | Measure |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:PortManagementFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| totalFloorArea | Measure |
| numberOfShipTypes | Integer |
| unitOfShipType | Code |
| loadingCapacity | Integer |
| acquisitionYear | String |
| usage | String |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:PortPassengerFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| length | Measure |
| width | Measure |
| mainMaterial | Code |
| totalFloorArea | Measure |
| acquisitionYear | String |
| totalCost | Integer |
| note | String |

## uro:PortPollutionControlFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| length | Measure |
| width | Measure |
| crossSectionalArea | Measure |
| area | Measure |
| height | Measure |
| mainMaterial | Code |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:PortProtectiveFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| structureType | Code |
| mainMaterial | Code |
| totalCost | Integer |
| subsidy | Integer |
| note | JSON (<code><a href="#string">String</a>[]</code>) |

## uro:PortStorageFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| innerTotalFloorArea | Measure |
| innerOfSiteArea | Measure |
| outerOfTotalFloorArea | Measure |
| outerSiteArea | Measure |
| mainCargo | Code |
| storageCapacity | Integer |
| storageCapacityUnit | Code |
| mainMaterial | Code |
| totalCost | Integer |
| note | String |

## uro:PortTransportationFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| structureType | Code |
| startingPoint | String |
| length | Measure |
| area | Measure |
| beddingWidth | Measure |
| numberOfLanes | Integer |
| parkingLotCapacityOfBus | Integer |
| parkingLotCapacityOfCars | Integer |
| routeType | Code |
| heightToDigit | Measure |
| heightLimit | Measure |
| minimumWidth | Measure |
| minimumDepth | Measure |
| numberOfAircraftParkingSpaces | Integer |
| pavementType | Code |
| mainCargo | Code |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:PortWasteTreatmentFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| structureType | Code |
| perimeter | Measure |
| mainPartLength | Measure |
| innerShoreLength | Measure |
| ceilingHeight | Measure |
| waveDissipatorLength | Measure |
| mainMaterial | Code |
| wasteType | Code |
| plannedDisposalArea | Measure |
| plannedDisposalAmount | Integer |
| receivingCapacity | Integer |
| shipType | String |
| unitOfReceivingCapacity | Code |
| acquisitionYear | String |
| totalCost | Integer |
| subsidy | Integer |
| note | String |

## uro:PortWelfareFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| totalFloorArea | Measure |
| totalCost | Integer |
| note | String |

## uro:ShipServiceFacility

| field | type |
|-------|------|
| facilityId | String |
| portFacilityDetailsType | Code |
| portName | String |
| portStatus | Code |
| district | String |
| grantType | Code |
| isDesignated | Boolean |
| degradationLevel | Integer |
| shipType | String |
| supplyAbility | Integer |
| supplyAbilityUnit | Code |
| mooringPlace | String |
| length | Measure |
| width | Measure |
| area | Measure |
| acquisitionYear | String |
| totalCost | Integer |
| note | String |

## uro:FacilityAttributeProperty

以下のいずれかの型をとります：

- uro:CargoHandlingFacility
- uro:CyberportMarinaAndPBS
- uro:FishingPortCapacity
- uro:FishingPortFacility
- uro:HarborFacility
- uro:MaintenanceHistoryAttribute
- uro:MooringFacility
- uro:NavigationAssistanceFacility
- uro:PortEnvironmentalImprovementFacility
- uro:PortManagementFacility
- uro:PortPassengerFacility
- uro:PortPollutionControlFacility
- uro:PortProtectiveFacility
- uro:PortStorageFacility
- uro:PortTransportationFacility
- uro:PortWasteTreatmentFacility
- uro:PortWelfareFacility
- uro:ShipServiceFacility

## uro:FacilityIdAttribute

| field | type |
|-------|------|
| id | String |
| partId | String |
| branchId | String |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) |
| city | JSON (<code><a href="#code">Code</a>[]</code>) |
| route | String |
| startPost | String |
| endPost | String |
| startLat | Double |
| startLong | Double |
| alternativeName | JSON (<code><a href="#string">String</a>[]</code>) |

## uro:RiverFacilityIdAttribute

| field | type |
|-------|------|
| id | String |
| partId | String |
| branchId | String |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) |
| city | JSON (<code><a href="#code">Code</a>[]</code>) |
| route | String |
| startPost | String |
| endPost | String |
| startLat | Double |
| startLong | Double |
| alternativeName | JSON (<code><a href="#string">String</a>[]</code>) |
| riverCode | Code |
| riverName | String |
| sideType | Code |
| leftPost | Measure |
| leftDistance | Measure |
| rightPost | Measure |
| rightDistance | Measure |
| leftStartPost | Measure |
| leftStartDistance | Measure |
| leftEndPost | Measure |
| lefEndDistance | Measure |
| rightStartPost | Measure |
| rightStartDistance | Measure |
| rightEndPost | Measure |
| rightEndDistance | Measure |

## uro:FacilityIdAttributeProperty

以下のいずれかの型をとります：

- uro:FacilityIdAttribute
- uro:RiverFacilityIdAttribute

## uro:FacilityTypeAttribute

| field | type |
|-------|------|
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:RealEstateIDAttribute

| field | type |
|-------|------|
| realEstateIDOfBuilding | String |
| numberOfBuildingUnitOwnership | Integer |
| realEstateIDOfBuildingUnitOwnership | JSON (<code><a href="#string">String</a>[]</code>) |
| numberOfRealEstateIDOfLand | Integer |
| realEstateIDOfLand | JSON (<code><a href="#string">String</a>[]</code>) |
| matchingScore | Integer |

## uro:BuildingDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| lod1HeightType | Code |
| lodType | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:BuildingDetailAttribute

| field | type |
|-------|------|
| serialNumberOfBuildingCertification | String |
| siteArea | Measure |
| totalFloorArea | Measure |
| buildingFootprintArea | Measure |
| buildingRoofEdgeArea | Measure |
| developmentArea | Measure |
| buildingStructureType | Code |
| buildingStructureOrgType | Code |
| fireproofStructureType | Code |
| implementingBody | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| districtsAndZonesType | JSON (<code><a href="#code">Code</a>[]</code>) |
| landUseType | Code |
| reference | String |
| majorUsage | Code |
| majorUsage2 | Code |
| orgUsage | Code |
| orgUsage2 | Code |
| detailedUsage | Code |
| detailedUsage2 | Code |
| detailedUsage3 | Code |
| groundFloorUsage | Code |
| secondFloorUsage | Code |
| thirdFloorUsage | Code |
| basementUsage | Code |
| basementFirstUsage | Code |
| basementSecondUsage | Code |
| vacancy | Code |
| buildingCoverageRate | Double |
| floorAreaRate | Double |
| specifiedBuildingCoverageRate | Double |
| specifiedFloorAreaRate | Double |
| standardFloorAreaRate | Double |
| buildingHeight | Measure |
| eaveHeight | Measure |
| note | String |
| surveyYear | String |

## uro:BuildingHighTideRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:BuildingInlandFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:BuildingLandSlideRiskAttribute

| field | type |
|-------|------|
| description | Code |
| areaType | Code |

## uro:BuildingRiverFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |
| adminType | Code |
| scale | Code |
| duration | Measure |

## uro:BuildingTsunamiRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:BuildingDisasterRiskAttributeProperty

以下のいずれかの型をとります：

- uro:BuildingHighTideRiskAttribute
- uro:BuildingInlandFloodingRiskAttribute
- uro:BuildingLandSlideRiskAttribute
- uro:BuildingRiverFloodingRiskAttribute
- uro:BuildingTsunamiRiskAttribute

## uro:BuildingIDAttribute

| field | type |
|-------|------|
| buildingID | String |
| branchID | Integer |
| partID | Integer |
| prefecture | Code |
| city | Code |

## uro:KeyValuePairAttribute

| field | type |
|-------|------|
| key | Code |
| codeValue | Code |

## uro:LargeCustomerFacilityAttribute

| field | type |
|-------|------|
| class | Code |
| name | String |
| capacity | Integer |
| owner | String |
| totalFloorArea | Measure |
| totalStoreFloorArea | Measure |
| inauguralDate | Date |
| yearOpened | String |
| yearClosed | String |
| keyTenants | String |
| availability | Boolean |
| urbanPlanType | Code |
| areaClassificationType | Code |
| districtsAndZonesType | JSON (<code><a href="#code">Code</a>[]</code>) |
| landUseType | Code |
| reference | String |
| note | String |
| surveyYear | String |

## tran:Road

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| trafficArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) |
| auxiliaryTrafficArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) |
| tranDataQualityAttribute | JSON (<code><a href="#urotransportationdataqualityattribute">uro:TransportationDataQualityAttribute</a></code>) |
| tranFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tranFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tranFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| roadStatus | JSON (<code><a href="#uroroadtype">uro:RoadType</a>[]</code>) |
| roadStructureAttribute | JSON (<code><a href="#uroroadstructureattribute">uro:RoadStructureAttribute</a>[]</code>) |
| trafficVolumeAttribute | JSON (<code><a href="#urotrafficvolumeattribute">uro:TrafficVolumeAttribute</a></code>) |

## tran:TrafficArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| surfaceMaterial | Code |
| railwayTrackAttribute | JSON (<code><a href="#urorailwaytrackattribute">uro:RailwayTrackAttribute</a>[]</code>) |
| trafficAreaStructureAttribute | JSON (<code><a href="#urotrafficareastructureattribute">uro:TrafficAreaStructureAttribute</a></code>) |

## uro:RailwayTrackAttribute

| field | type |
|-------|------|
| routeName | String |
| directionType | Code |
| trackType | Code |
| startPost | String |
| endPost | String |
| alignmentType | Code |
| controlPoint | JSON (<code><a href="#urocontrolpoint">uro:ControlPoint</a>[]</code>) |

## uro:ControlPoint

| field | type |
|-------|------|
| startPost | String |
| endPost | String |
| function | Code |
| parameter | JSON (<code><a href="#urocontrolpointtype">uro:ControlPointType</a></code>) |
| startPoint | Point |
| endPoint | Point |

## uro:CircularCurveType

| field | type |
|-------|------|
| radius | Measure |
| intersection | Double |
| cutLength | Measure |
| curveLength | Measure |

## uro:TransitionCurveType

| field | type |
|-------|------|
| intersection | Measure |
| distance | Measure |
| curveLength | Measure |

## uro:SlopeType

| field | type |
|-------|------|
| angle | Double |
| elevation | Measure |

## uro:VerticalCurveType

| field | type |
|-------|------|
| length | Measure |
| verticalDistance | Measure |

## uro:ControlPointType

以下のいずれかの型をとります：

- uro:CircularCurveType
- uro:TransitionCurveType
- uro:SlopeType
- uro:VerticalCurveType

## uro:TrafficAreaStructureAttribute

| field | type |
|-------|------|
| numberOfLanes | Integer |

## tran:AuxiliaryTrafficArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| surfaceMaterial | Code |

## uro:TransportationDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| lodType | Code |

## uro:RoadType

| field | type |
|-------|------|
| id | String |
| creationDate | Date |
| isTemporary | Boolean |
| roadType | Code |
| widthType | Code |
| isTollRoad | Boolean |
| separator | Measure |
| isHighWay | Boolean |

## uro:RoadStructureAttribute

| field | type |
|-------|------|
| widthType | Code |
| width | Measure |
| numberOfLanes | Integer |
| sectionType | Code |

## uro:TrafficVolumeAttribute

| field | type |
|-------|------|
| sectionID | String |
| routeName | String |
| weekday12hourTrafficVolume | Integer |
| weekday24hourTrafficVolume | Integer |
| largeVehicleRate | Double |
| congestionRate | Double |
| averageTravelSpeedInCongestion | Double |
| averageInboundTravelSpeedInCongestion | Double |
| averageOutboundTravelSpeedInCongestion | Double |
| averageInboundTravelSpeedNotCongestion | Double |
| averageOutboundTravelSpeedNotCongestion | Double |
| observationPointName | String |
| reference | String |
| surveyYear | String |

## tran:Railway

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| trafficArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) |
| auxiliaryTrafficArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) |
| tranDataQualityAttribute | JSON (<code><a href="#urotransportationdataqualityattribute">uro:TransportationDataQualityAttribute</a></code>) |
| tranFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tranFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tranFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| railwayRouteAttribute | JSON (<code><a href="#urorailwayrouteattribute">uro:RailwayRouteAttribute</a></code>) |

## uro:RailwayRouteAttribute

| field | type |
|-------|------|
| operatorType | Code |
| operator | String |
| alternativeName | JSON (<code><a href="#string">String</a>[]</code>) |
| railwayType | Code |
| startStation | String |
| endStation | String |

## tran:Track

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| trafficArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) |
| auxiliaryTrafficArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) |
| tranDataQualityAttribute | JSON (<code><a href="#urotransportationdataqualityattribute">uro:TransportationDataQualityAttribute</a></code>) |
| tranFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tranFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tranFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| trackAttribute | JSON (<code><a href="#urotrackattribute">uro:TrackAttribute</a></code>) |

## uro:TrackAttribute

| field | type |
|-------|------|
| alternativeName | JSON (<code><a href="#string">String</a>[]</code>) |
| adminType | Code |
| relativeLevel | Integer |
| widthType | Code |
| structureType | Code |
| isTollRoad | Boolean |
| separator | Measure |

## tran:Square

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| trafficArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) |
| auxiliaryTrafficArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) |
| tranDataQualityAttribute | JSON (<code><a href="#urotransportationdataqualityattribute">uro:TransportationDataQualityAttribute</a></code>) |
| tranFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tranFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tranFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| squareUrbanPlanAttribute | JSON (<code><a href="#urosquareurbanplanattributeproperty">uro:SquareUrbanPlanAttributeProperty</a></code>) |

## uro:SquareUrbanPlanAttribute

| field | type |
|-------|------|
| prefecture | Code |
| city | Code |
| urbanPlanningAreaName | String |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) |
| dateOfDecision | Date |
| dateOfRevision | JSON (<code><a href="#date">Date</a>[]</code>) |
| areaPlanned | Measure |
| areaInService | Measure |
| remarks | String |
| status | Code |
| areaImproved | Measure |
| areaCompleted | Measure |
| projectStartDate | Date |
| projectEndDate | Date |
| isCompleted | Boolean |
| isAuthorized | Boolean |
| purpose | String |
| note | String |

## uro:StationSquareAttribute

| field | type |
|-------|------|
| prefecture | Code |
| city | Code |
| urbanPlanningAreaName | String |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) |
| dateOfDecision | Date |
| dateOfRevision | JSON (<code><a href="#date">Date</a>[]</code>) |
| areaPlanned | Measure |
| areaInService | Measure |
| remarks | String |
| status | Code |
| areaImproved | Measure |
| areaCompleted | Measure |
| projectStartDate | Date |
| projectEndDate | Date |
| isCompleted | Boolean |
| isAuthorized | Boolean |
| purpose | String |
| note | String |
| station | JSON (<code><a href="#string">String</a>[]</code>) |
| route | JSON (<code><a href="#string">String</a>[]</code>) |
| railwayType | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:TerminalAttribute

| field | type |
|-------|------|
| prefecture | Code |
| city | Code |
| urbanPlanningAreaName | String |
| enforcer | JSON (<code><a href="#string">String</a>[]</code>) |
| dateOfDecision | Date |
| dateOfRevision | JSON (<code><a href="#date">Date</a>[]</code>) |
| areaPlanned | Measure |
| areaInService | Measure |
| remarks | String |
| status | Code |
| areaImproved | Measure |
| areaCompleted | Measure |
| projectStartDate | Date |
| projectEndDate | Date |
| isCompleted | Boolean |
| isAuthorized | Boolean |
| purpose | String |
| note | String |
| terminalType | Code |
| structure | String |
| numberOfBerthsPlanned | Integer |
| numberOfBerthsInService | Integer |
| userType | Code |

## uro:SquareUrbanPlanAttributeProperty

以下のいずれかの型をとります：

- uro:SquareUrbanPlanAttribute
- uro:StationSquareAttribute
- uro:TerminalAttribute

## brid:Bridge

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| isMovable | Boolean |
| outerBridgeConstruction | JSON (<code><a href="#bridbridgeconstructionelement">brid:BridgeConstructionElement</a>[]</code>) |
| outerBridgeInstallation | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) |
| interiorBridgeInstallation | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) |
| interiorBridgeRoom | JSON (<code><a href="#bridbridgeroom">brid:BridgeRoom</a>[]</code>) |
| consistsOfBridgePart | JSON (<code><a href="#bridbridgepart">brid:BridgePart</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |
| bridBaseAttribute | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) |
| bridDataQualityAttribute | JSON (<code><a href="#uroconstructiondataqualityattribute">uro:ConstructionDataQualityAttribute</a></code>) |
| bridDisasterRiskAttribute | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |
| bridDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| bridFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| bridFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| bridFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| bridFunctionalAttribute | JSON (<code><a href="#urobridgefunctionalattribute">uro:BridgeFunctionalAttribute</a></code>) |
| bridRiskAssessmentAttribute | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a></code>) |
| bridStructureAttribute | JSON (<code><a href="#urobridgestructureattribute">uro:BridgeStructureAttribute</a></code>) |

## brid:BridgeConstructionElement

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) |

## brid:CeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:Door

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |

## brid:Window

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## brid:_OpeningProperty

以下のいずれかの型をとります：

- brid:Door
- brid:Window

## brid:ClosureSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:FloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:GroundSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:InteriorWallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:OuterCeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:OuterFloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:RoofSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:WallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#brid-openingproperty">brid:_OpeningProperty</a>[]</code>) |

## brid:_BoundarySurfaceProperty

以下のいずれかの型をとります：

- brid:CeilingSurface
- brid:ClosureSurface
- brid:FloorSurface
- brid:GroundSurface
- brid:InteriorWallSurface
- brid:OuterCeilingSurface
- brid:OuterFloorSurface
- brid:RoofSurface
- brid:WallSurface

## brid:BridgeInstallation

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) |

## brid:BridgeRoom

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) |
| interiorFurniture | JSON (<code><a href="#bridbridgefurniture">brid:BridgeFurniture</a>[]</code>) |
| bridgeRoomInstallation | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) |

## brid:BridgeFurniture

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |

## brid:BridgePart

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| isMovable | Boolean |
| outerBridgeConstruction | JSON (<code><a href="#bridbridgeconstructionelement">brid:BridgeConstructionElement</a>[]</code>) |
| outerBridgeInstallation | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) |
| interiorBridgeInstallation | JSON (<code><a href="#bridbridgeinstallation">brid:BridgeInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#brid-boundarysurfaceproperty">brid:_BoundarySurfaceProperty</a>[]</code>) |
| interiorBridgeRoom | JSON (<code><a href="#bridbridgeroom">brid:BridgeRoom</a>[]</code>) |
| consistsOfBridgePart | JSON (<code><a href="#bridbridgepart">brid:BridgePart</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |
| bridBaseAttribute | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) |
| bridDataQualityAttribute | JSON (<code><a href="#uroconstructiondataqualityattribute">uro:ConstructionDataQualityAttribute</a></code>) |
| bridDisasterRiskAttribute | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |
| bridDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| bridFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| bridFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| bridFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| bridFunctionalAttribute | JSON (<code><a href="#urobridgefunctionalattribute">uro:BridgeFunctionalAttribute</a></code>) |
| bridRiskAssessmentAttribute | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a></code>) |
| bridStructureAttribute | JSON (<code><a href="#urobridgestructureattribute">uro:BridgeStructureAttribute</a></code>) |

## uro:ConstructionBaseAttribute

| field | type |
|-------|------|
| adminType | Code |
| administorator | String |
| adminOffice | String |
| operatorType | Code |
| installerType | Code |
| installer | String |
| structureOrdinance | String |
| specification | String |
| kana | String |
| constructionStartYear | String |
| completionYear | String |
| facilityAge | Integer |
| update | Date |
| purpose | Code |

## uro:ConstructionDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| dataAcquisition | String |
| photoScale | Integer |
| lod1HeightType | Code |
| lodType | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:HighTideRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:InlandFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:LandSlideRiskAttribute

| field | type |
|-------|------|
| description | Code |
| areaType | Code |

## uro:RiverFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |
| adminType | Code |
| scale | Code |
| duration | Measure |

## uro:TsunamiRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:DisasterRiskAttributeProperty

以下のいずれかの型をとります：

- uro:HighTideRiskAttribute
- uro:InlandFloodingRiskAttribute
- uro:LandSlideRiskAttribute
- uro:RiverFloodingRiskAttribute
- uro:TsunamiRiskAttribute

## uro:BridgeFunctionalAttribute

| field | type |
|-------|------|
| directionType | Code |
| userType | Code |

## uro:ConstructionRiskAssessmentAttribute

| field | type |
|-------|------|
| surveyYear | String |
| riskType | Code |
| status | Code |
| referenceDate | Date |

## uro:BridgeStructureAttribute

| field | type |
|-------|------|
| material | Code |
| bridgeType | Code |
| length | Measure |
| width | Measure |
| area | Measure |
| weightRestriction | Measure |
| heightRestriction | Measure |
| widthRestriction | Measure |
| underGirderHeight | Measure |
| slopeType | Code |
| escalator | Boolean |

## frn:CityFurniture

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |

## uro:CityFurnitureDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| lodType | Code |

## uro:CityFurnitureDetailAttribute

| field | type |
|-------|------|
| facilityType | Code |
| description | String |

## veg:SolitaryVegetationObject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| vegDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| vegFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| vegFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| vegFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| vegetationDataQualityAttribute | JSON (<code><a href="#urovegetationdataqualityattribute">uro:VegetationDataQualityAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| species | Code |
| height | Measure |
| trunkDiameter | Measure |
| crownDiameter | Measure |

## uro:VegetationDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| appearanceSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |

## veg:PlantCover

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| vegDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| vegFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| vegFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| vegFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| vegetationDataQualityAttribute | JSON (<code><a href="#urovegetationdataqualityattribute">uro:VegetationDataQualityAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| averageHeight | Measure |

## luse:LandUse

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| ifcLandUseAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| landUseDetailAttribute | JSON (<code><a href="#urolandusedetailattribute">uro:LandUseDetailAttribute</a></code>) |
| luseDataQualityAttribute | JSON (<code><a href="#urolandusedataqualityattribute">uro:LandUseDataQualityAttribute</a></code>) |
| luseDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| luseFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| luseFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| luseFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |

## uro:LandUseDetailAttribute

| field | type |
|-------|------|
| id | String |
| orgLandUse | Code |
| nominalArea | Measure |
| ownerType | Code |
| owner | String |
| areaInSquareMeter | Measure |
| areaInHa | Measure |
| buildingCoverageRate | Double |
| floorAreaRate | Double |
| specifiedBuildingCoverageRate | Double |
| specifiedFloorAreaRate | Double |
| standardFloorAreaRate | Double |
| urbanPlanType | Code |
| areaClassificationType | Code |
| districtsAndZonesType | JSON (<code><a href="#code">Code</a>[]</code>) |
| prefecture | Code |
| city | Code |
| reference | String |
| note | String |
| surveyYear | String |

## uro:LandUseDataQualityAttribute

| field | type |
|-------|------|
| srcScale | JSON (<code><a href="#code">Code</a>[]</code>) |
| geometrySrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |
| thematicSrcDesc | JSON (<code><a href="#code">Code</a>[]</code>) |

## tun:Tunnel

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| outerTunnelInstallation | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |
| interiorTunnelInstallation | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) |
| interiorHollowSpace | JSON (<code><a href="#tunhollowspace">tun:HollowSpace</a>[]</code>) |
| consistsOfTunnelPart | JSON (<code><a href="#tuntunnelpart">tun:TunnelPart</a>[]</code>) |
| tunBaseAttribute | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) |
| tunDataQualityAttribute | JSON (<code><a href="#uroconstructiondataqualityattribute">uro:ConstructionDataQualityAttribute</a></code>) |
| tunDisasterRiskAttribute | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |
| tunDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| tunFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tunFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tunFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| tunFunctionalAttribute | JSON (<code><a href="#urotunnelfunctionalattribute">uro:TunnelFunctionalAttribute</a></code>) |
| tunRiskAssessmentAttribute | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a></code>) |
| tunStructureAttribute | JSON (<code><a href="#urotunnelstructureattribute">uro:TunnelStructureAttribute</a></code>) |

## tun:TunnelInstallation

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) |

## tun:CeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:Door

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## tun:Window

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## tun:_OpeningProperty

以下のいずれかの型をとります：

- tun:Door
- tun:Window

## tun:ClosureSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:FloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:GroundSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:InteriorWallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:OuterCeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:OuterFloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:RoofSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:WallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| opening | JSON (<code><a href="#tun-openingproperty">tun:_OpeningProperty</a>[]</code>) |

## tun:_BoundarySurfaceProperty

以下のいずれかの型をとります：

- tun:CeilingSurface
- tun:ClosureSurface
- tun:FloorSurface
- tun:GroundSurface
- tun:InteriorWallSurface
- tun:OuterCeilingSurface
- tun:OuterFloorSurface
- tun:RoofSurface
- tun:WallSurface

## tun:HollowSpace

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) |
| interiorFurniture | JSON (<code><a href="#tuntunnelfurniture">tun:TunnelFurniture</a>[]</code>) |
| hollowSpaceInstallation | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |

## tun:TunnelFurniture

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |

## tun:TunnelPart

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| outerTunnelInstallation | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |
| interiorTunnelInstallation | JSON (<code><a href="#tuntunnelinstallation">tun:TunnelInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#tun-boundarysurfaceproperty">tun:_BoundarySurfaceProperty</a>[]</code>) |
| interiorHollowSpace | JSON (<code><a href="#tunhollowspace">tun:HollowSpace</a>[]</code>) |
| consistsOfTunnelPart | JSON (<code><a href="#tuntunnelpart">tun:TunnelPart</a>[]</code>) |
| tunBaseAttribute | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) |
| tunDataQualityAttribute | JSON (<code><a href="#uroconstructiondataqualityattribute">uro:ConstructionDataQualityAttribute</a></code>) |
| tunDisasterRiskAttribute | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |
| tunDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| tunFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tunFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tunFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| tunFunctionalAttribute | JSON (<code><a href="#urotunnelfunctionalattribute">uro:TunnelFunctionalAttribute</a></code>) |
| tunRiskAssessmentAttribute | JSON (<code><a href="#uroconstructionriskassessmentattribute">uro:ConstructionRiskAssessmentAttribute</a></code>) |
| tunStructureAttribute | JSON (<code><a href="#urotunnelstructureattribute">uro:TunnelStructureAttribute</a></code>) |

## uro:TunnelFunctionalAttribute

| field | type |
|-------|------|
| directionType | Code |
| userType | Code |

## uro:TunnelStructureAttribute

| field | type |
|-------|------|
| tunnelType | Code |
| tunnelSubtype | Code |
| mouthType | JSON (<code><a href="#code">Code</a>[]</code>) |
| length | Measure |
| width | Measure |
| area | Measure |
| innerHeight | Measure |
| effectiveHeight | Measure |
| slopeType | Code |

## dem:ReliefFeature

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| lod | NonNegativeInteger |
| reliefComponent | JSON (<code><a href="#dem-reliefcomponentproperty">dem:_ReliefComponentProperty</a>[]</code>) |

## dem:BreaklineRelief

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| lod | NonNegativeInteger |
| demDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |

## dem:MassPointRelief

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| lod | NonNegativeInteger |
| demDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |

## dem:RasterRelief

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| lod | NonNegativeInteger |
| demDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| grid | JSON (<code><a href="#gmlrectifiedgridcoverage">gml:RectifiedGridCoverage</a></code>) |

## gml:RectifiedGridCoverage

| field | type |
|-------|------|
| rectifiedGridDomain | JSON (<code><a href="#gmlrectifiedgriddomain">gml:RectifiedGridDomain</a></code>) |
| coverageFunction | JSON (<code><a href="#gmlcoveragefunction">gml:CoverageFunction</a></code>) |

## gml:RectifiedGridDomain

| field | type |
|-------|------|
| RectifiedGrid | JSON (<code><a href="#gmlrectifiedgrid">gml:RectifiedGrid</a></code>) |

## gml:RectifiedGrid

| field | type |
|-------|------|
| limits | JSON (<code><a href="#gmlgridenvelope">gml:GridEnvelope</a></code>) |
| axisName | JSON (<code><a href="#string">String</a>[]</code>) |
| origin | Point |
| offsetVector | JSON (<code><a href="#point">Point</a>[]</code>) |

## gml:GridEnvelope

| field | type |
|-------|------|

## gml:CoverageFunction

| field | type |
|-------|------|
| MappingRule | String |
| GridFunction | JSON (<code><a href="#gmlgridfunction">gml:GridFunction</a></code>) |

## gml:GridFunction

| field | type |
|-------|------|

## dem:TINRelief

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| lod | NonNegativeInteger |
| demDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |

## dem:_ReliefComponentProperty

以下のいずれかの型をとります：

- dem:BreaklineRelief
- dem:MassPointRelief
- dem:RasterRelief
- dem:TINRelief

## wtr:WaterBody

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| boundedBy | JSON (<code><a href="#wtr-waterboundarysurfaceproperty">wtr:_WaterBoundarySurfaceProperty</a>[]</code>) |
| floodingRiskAttribute | JSON (<code><a href="#urowaterbodyfloodingriskattributeproperty">uro:WaterBodyFloodingRiskAttributeProperty</a>[]</code>) |
| waterBodyDetailAttribute | JSON (<code><a href="#urowaterbodydetailattribute">uro:WaterBodyDetailAttribute</a></code>) |
| wtrDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| wtrFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| wtrFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| wtrFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |

## wtr:WaterClosureSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## wtr:WaterGroundSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## wtr:WaterSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| waterLevel | Code |

## wtr:_WaterBoundarySurfaceProperty

以下のいずれかの型をとります：

- wtr:WaterClosureSurface
- wtr:WaterGroundSurface
- wtr:WaterSurface

## uro:WaterBodyHighTideRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:WaterBodyInlandFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:WaterBodyRiverFloodingRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |
| adminType | Code |
| scale | Code |
| duration | Measure |

## uro:WaterBodyTsunamiRiskAttribute

| field | type |
|-------|------|
| description | Code |
| rank | Code |
| rankOrg | Code |
| depth | Measure |

## uro:WaterBodyFloodingRiskAttributeProperty

以下のいずれかの型をとります：

- uro:WaterBodyHighTideRiskAttribute
- uro:WaterBodyInlandFloodingRiskAttribute
- uro:WaterBodyRiverFloodingRiskAttribute
- uro:WaterBodyTsunamiRiskAttribute

## uro:WaterBodyDetailAttribute

| field | type |
|-------|------|
| kana | String |
| waterSystemCode | Code |
| riverCode | Code |
| adminType | Code |
| flowDirection | Boolean |
| maximumDepth | Measure |
| waterSurfaceElevation | Measure |
| area | Measure |
| measurementYearMonth | String |
| prefecture | JSON (<code><a href="#code">Code</a>[]</code>) |
| city | JSON (<code><a href="#code">Code</a>[]</code>) |

## gen:GenericCityObject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |

## grp:CityObjectGroup

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| groupMember | JSON (<code><a href="#grp-cityobjectorref">grp:_CityObjectOrRef</a>[]</code>) |
| parent | JSON (<code><a href="#grp-cityobjectorref">grp:_CityObjectOrRef</a></code>) |
| fiscalYearOfPublication | JSON (<code><a href="#string">String</a>[]</code>) |
| ifcBuildingStoreyAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorStoreyAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| language | JSON (<code><a href="#code">Code</a>[]</code>) |

## grp:_CityObjectOrRef

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| href | String |

## uro:Waterway

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| tranDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| trafficArea | JSON (<code><a href="#trantrafficarea">tran:TrafficArea</a>[]</code>) |
| auxiliaryTrafficArea | JSON (<code><a href="#tranauxiliarytrafficarea">tran:AuxiliaryTrafficArea</a>[]</code>) |
| tranDataQualityAttribute | JSON (<code><a href="#urotransportationdataqualityattribute">uro:TransportationDataQualityAttribute</a></code>) |
| tranFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| tranFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| tranFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| waterwayDetailAttribute | JSON (<code><a href="#urowaterwaydetailattribute">uro:WaterwayDetailAttribute</a></code>) |

## uro:WaterwayDetailAttribute

| field | type |
|-------|------|
| routeId | String |
| routeDirection | Code |
| minimumWidth | Measure |
| maximumWidth | Measure |
| length | Measure |
| navigation | String |
| plannedDepth | Measure |
| speedLimit | Measure |
| targetShipType | JSON (<code><a href="#string">String</a>[]</code>) |

## uro:OtherConstruction

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| conditionOfConstruction | String |
| dateOfConstruction | Date |
| dateOfDemolition | Date |
| constructionEvent | JSON (<code><a href="#uroconstructionevent">uro:ConstructionEvent</a>[]</code>) |
| elevation | JSON (<code><a href="#uroelevation">uro:Elevation</a>[]</code>) |
| height | JSON (<code><a href="#uroheight">uro:Height</a>[]</code>) |
| occupancy | JSON (<code><a href="#urooccupancy">uro:Occupancy</a>[]</code>) |
| consFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| consFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| consFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| consBaseAttribute | JSON (<code><a href="#uroconstructionbaseattribute">uro:ConstructionBaseAttribute</a></code>) |
| consStructureAttribute | JSON (<code><a href="#uroconstructionstructureattributeproperty">uro:ConstructionStructureAttributeProperty</a></code>) |
| consDisasterRiskAttribute | JSON (<code><a href="#urodisasterriskattributeproperty">uro:DisasterRiskAttributeProperty</a>[]</code>) |
| consDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| consDataQualityAttribute | JSON (<code><a href="#uroconstructiondataqualityattribute">uro:ConstructionDataQualityAttribute</a></code>) |
| boundedBy | JSON (<code><a href="#uro-boundarysurfaceproperty">uro:_BoundarySurfaceProperty</a>[]</code>) |
| constructionInstallation | JSON (<code><a href="#uroconstructioninstallation">uro:ConstructionInstallation</a>[]</code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:ConstructionEvent

| field | type |
|-------|------|
| event | Code |
| dateOfEvent | Date |
| description | String |

## uro:Elevation

| field | type |
|-------|------|
| elevationReference | Code |
| elevationValue | Point |

## uro:Height

| field | type |
|-------|------|
| highReference | Code |
| lowReference | Code |
| status | String |
| value | Measure |

## uro:Occupancy

| field | type |
|-------|------|
| interval | Code |
| numberofOccupants | Integer |
| occupantType | Code |

## uro:ConstructionStructureAttribute

| field | type |
|-------|------|
| structureType | Code |
| length | Measure |
| width | Measure |
| depth | Measure |
| volume | Measure |

## uro:DamAttribute

| field | type |
|-------|------|
| structureType | Code |
| length | Measure |
| width | Measure |
| depth | Measure |
| volume | Measure |
| damCode | Code |
| totalWaterStorage | Measure |

## uro:EmbankmentAttribute

| field | type |
|-------|------|
| structureType | Code |
| length | Measure |
| width | Measure |
| depth | Measure |
| volume | Measure |
| mainPartLength | Measure |
| ceilingHeight | Measure |
| waveDissipatorLength | Measure |

## uro:ConstructionStructureAttributeProperty

以下のいずれかの型をとります：

- uro:ConstructionStructureAttribute
- uro:DamAttribute
- uro:EmbankmentAttribute

## uro:ClosureSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:GroundSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:OuterCeilingSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:OuterFloorSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:RoofSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:WallSurface

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |

## uro:_BoundarySurfaceProperty

以下のいずれかの型をとります：

- uro:ClosureSurface
- uro:GroundSurface
- uro:OuterCeilingSurface
- uro:OuterFloorSurface
- uro:RoofSurface
- uro:WallSurface

## uro:ConstructionInstallation

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |

## uro:UndergroundBuilding

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| yearOfConstruction | String |
| yearOfDemolition | String |
| roofType | Code |
| measuredHeight | Measure |
| storeysAboveGround | NonNegativeInteger |
| storeysBelowGround | NonNegativeInteger |
| storeyHeightsAboveGround | String |
| storeyHeightsBelowGround | String |
| outerBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| interiorBuildingInstallation | JSON (<code><a href="#bldgbuildinginstallation">bldg:BuildingInstallation</a>[]</code>) |
| boundedBy | JSON (<code><a href="#bldg-boundarysurfaceproperty">bldg:_BoundarySurfaceProperty</a>[]</code>) |
| interiorRoom | JSON (<code><a href="#bldgroom">bldg:Room</a>[]</code>) |
| consistsOfBuildingPart | JSON (<code><a href="#bldgbuildingpart">bldg:BuildingPart</a>[]</code>) |
| address | JSON (<code><a href="#coreaddress">core:Address</a>[]</code>) |
| bldgDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| bldgFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| bldgFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| bldgFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| bldgRealEstateIDAttribute | JSON (<code><a href="#urorealestateidattribute">uro:RealEstateIDAttribute</a></code>) |
| buildingDataQualityAttribute | JSON (<code><a href="#urobuildingdataqualityattribute">uro:BuildingDataQualityAttribute</a></code>) |
| buildingDetailAttribute | JSON (<code><a href="#urobuildingdetailattribute">uro:BuildingDetailAttribute</a>[]</code>) |
| buildingDisasterRiskAttribute | JSON (<code><a href="#urobuildingdisasterriskattributeproperty">uro:BuildingDisasterRiskAttributeProperty</a>[]</code>) |
| buildingIDAttribute | JSON (<code><a href="#urobuildingidattribute">uro:BuildingIDAttribute</a>[]</code>) |
| ifcBuildingAttribute | JSON (<code><a href="#uroifcattributeproperty">uro:IfcAttributeProperty</a>[]</code>) |
| indoorBuildingAttribute | JSON (<code><a href="#uroindoorattributeproperty">uro:IndoorAttributeProperty</a>[]</code>) |
| keyValuePairAttribute | JSON (<code><a href="#urokeyvaluepairattribute">uro:KeyValuePairAttribute</a>[]</code>) |
| largeCustomerFacilityAttribute | JSON (<code><a href="#urolargecustomerfacilityattribute">uro:LargeCustomerFacilityAttribute</a>[]</code>) |

## uro:Appurtenance

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| previousLink | JSON (<code><a href="#string">String</a>[]</code>) |
| nextLink | JSON (<code><a href="#string">String</a>[]</code>) |
| rotationAngle | Double |
| appurtenanceType | Code |

## uro:OffsetDepth

| field | type |
|-------|------|
| pos | Point |
| offset | Measure |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |

## uro:ThematicShape

| field | type |
|-------|------|
| horizontalType | Code |
| heightType | Code |

## uro:Cable

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| columns | Integer |
| rows | Integer |
| cables | Integer |

## uro:LengthAttribute

| field | type |
|-------|------|
| length | Measure |
| mesureType | Code |
| phaseType | Code |

## uro:Duct

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| width | Measure |

## uro:ElectricityCable

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| columns | Integer |
| rows | Integer |
| cables | Integer |

## uro:Handhole

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| containerType | Code |
| innerDiamiterLong | Measure |
| outerDiamiterLong | Measure |
| innerDiamiterShort | Measure |
| outerDiamiterShort | Measure |
| depth | Measure |
| appurtenance | JSON (<code><a href="#string">String</a>[]</code>) |
| rotationAngle | Double |

## uro:Manhole

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| containerType | Code |
| innerDiamiterLong | Measure |
| outerDiamiterLong | Measure |
| innerDiamiterShort | Measure |
| outerDiamiterShort | Measure |
| depth | Measure |
| appurtenance | JSON (<code><a href="#string">String</a>[]</code>) |
| rotationAngle | Double |

## uro:OilGasChemicalsPipe

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| innerDiamiter | Measure |
| outerDiamiter | Measure |
| sleeveType | Code |

## uro:Pipe

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| innerDiamiter | Measure |
| outerDiamiter | Measure |
| sleeveType | Code |

## uro:SewerPipe

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| innerDiamiter | Measure |
| outerDiamiter | Measure |
| sleeveType | Code |
| slope | Measure |

## uro:TelecommunicationsCable

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| columns | Integer |
| rows | Integer |
| cables | Integer |

## uro:ThermalPipe

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| innerDiamiter | Measure |
| outerDiamiter | Measure |
| sleeveType | Code |

## uro:WaterPipe

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| cityFurnitureDataQualityAttribute | JSON (<code><a href="#urocityfurnituredataqualityattribute">uro:CityFurnitureDataQualityAttribute</a></code>) |
| cityFurnitureDetailAttribute | JSON (<code><a href="#urocityfurnituredetailattribute">uro:CityFurnitureDetailAttribute</a>[]</code>) |
| frnDmAttribute | JSON (<code><a href="#urodmattributeproperty">uro:DmAttributeProperty</a>[]</code>) |
| frnFacilityAttribute | JSON (<code><a href="#urofacilityattributeproperty">uro:FacilityAttributeProperty</a>[]</code>) |
| frnFacilityIdAttribute | JSON (<code><a href="#urofacilityidattributeproperty">uro:FacilityIdAttributeProperty</a></code>) |
| frnFacilityTypeAttribute | JSON (<code><a href="#urofacilitytypeattribute">uro:FacilityTypeAttribute</a>[]</code>) |
| occupierType | Code |
| occupierName | Code |
| year | String |
| yearType | Code |
| administrator | Code |
| offsetDepth | JSON (<code><a href="#urooffsetdepth">uro:OffsetDepth</a>[]</code>) |
| thematicShape | JSON (<code><a href="#urothematicshape">uro:ThematicShape</a>[]</code>) |
| routeStartNode | String |
| startNode | String |
| routeEndNode | String |
| endNode | String |
| depth | Measure |
| minDepth | Measure |
| maxDepth | Measure |
| maxWidth | Measure |
| offset | Measure |
| material | Code |
| lengthAttribute | JSON (<code><a href="#urolengthattribute">uro:LengthAttribute</a>[]</code>) |
| innerDiamiter | Measure |
| outerDiamiter | Measure |
| sleeveType | Code |

## urf:Zone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:Boundary

| field | type |
|-------|------|
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| offset | Measure |
| offsetDirection | String |

## urf:Agreement

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| applicableArea | Measure |
| expiration | Date |

## urf:AircraftNoiseControlZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:AreaClassification

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| population | Integer |

## urf:CollectiveFacilitiesForReconstruction

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| housingFacilities | String |
| supecificBusinessFacilities | String |
| publicFacilities | String |
| utilityFacilities | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |

## urf:ThreeDimensionalExtent

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| minimumDistance | Measure |
| maximumLoad | Measure |

## urf:CollectiveFacilitiesForReconstructionAndRevitalization

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| housingFacilities | String |
| supecificBusinessFacilities | String |
| publicFacilities | String |
| utilityFacilities | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |

## urf:CollectiveFacilitiesForTsunamiDisasterPrevention

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| housingFacilities | String |
| supecificBusinessFacilities | String |
| publicFacilities | String |
| utilityFacilities | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |

## urf:CollectiveGovernmentAndPublicOfficeFacilities

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| buildingCoverageRate | Double |
| floorAreaRate | Double |
| publicFacilitiesAllocationPolicy | String |
| scheduledExecutor | String |

## urf:CollectiveHousingFacilities

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| buildingCoverageRate | Double |
| floorAreaRate | Double |
| numberOfLowRiseHousing | Integer |
| numberOfMiddleRiseHousing | Integer |
| numberOfHighRiseHousing | Integer |
| totalNumberOfHousing | Integer |
| publicFacilitiesAllocationPolicy | String |
| scheduledExecutor | String |

## urf:CollectiveUrbanDisasterPreventionFacilities

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| specificUtilityAndPublicFacilities | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |

## urf:ConservationZoneForClustersOfTraditionalStructures

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:DisasterPreventionBlockImprovementProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| disasterPreventionPublicFacilityAllocation | String |
| otherPublicFacilityAllocation | String |
| developmentPlan | String |

## urf:DisasterPreventionBlockImprovementZonePlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| objectives | String |
| policy | String |
| districtDevelopmentPlan | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) |
| promotionDistrict | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) |
| zonalDisasterPreventionFacilitiesAllocation | String |
| specifiedZonalDisasterPreventionFacilitiesAllocation | String |
| zonalDisasterPreventionFacilities | JSON (<code><a href="#urfzonaldisasterpreventionfacility">urf:ZonalDisasterPreventionFacility</a>[]</code>) |

## urf:DistrictDevelopmentPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |

## urf:DistrictFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:RoadsideDistrictFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:RuralDistrictFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:DistrictFacilityProperty

以下のいずれかの型をとります：

- urf:DistrictFacility
- urf:RoadsideDistrictFacility
- urf:RuralDistrictFacility

## urf:District

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| buildingRestrictions | String |
| useRestrictions | String |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |
| minimumBuildingCoverageRate | Double |
| minimumSiteArea | Measure |
| minimumBuildingArea | Measure |
| minimumGroundHeight | Measure |
| setbackSize | String |
| structurePlacementRestrictions | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| minimumFloorHeight | Measure |
| buildingDesignRestriction | String |
| minimumGreeningRate | Double |
| fenceGuideline | String |
| restrictionsForFireProtection | String |
| restrictionsForNoiseProtection | String |
| minimumFrontageRate | Double |

## urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |

## urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |

## urf:RoadsideDistrictImprovementPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |
| roadsideDistrictFacilitiesAllocation | String |

## urf:RuralDistrictImprovementPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |
| ruralDistrictFacilitiesAllocation | String |

## urf:SpecifiedBuildingZoneImprovementPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| districtFacilitiesAllocation | String |
| buildingRestrictions | String |
| urbanGreenSpaceConservation | String |
| activityRestrictionInFarmland | String |
| landuseRestrictions | String |
| districtFacility | JSON (<code><a href="#urfdistrictfacilityproperty">urf:DistrictFacilityProperty</a>[]</code>) |
| district | JSON (<code><a href="#urfdistrict">urf:District</a>[]</code>) |

## urf:DistrictDevelopmentPlanProperty

以下のいずれかの型をとります：

- urf:DistrictDevelopmentPlan
- urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan
- urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict
- urf:RoadsideDistrictImprovementPlan
- urf:RuralDistrictImprovementPlan
- urf:SpecifiedBuildingZoneImprovementPlan

## urf:PromotionDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:ZonalDisasterPreventionFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| facilityType | Code |

## urf:DistributionBusinessPark

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| distributionBusinessPark | String |
| publicAndUtilityFacilities | String |
| buildingCoverageRate | Double |
| floorAreaRate | Double |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| setbackSize | String |
| scheduledExecutor | String |

## urf:DistributionBusinessZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| guidelinePublicationDate | Date |

## urf:DistrictPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| objectives | String |
| policy | String |
| districtDevelopmentPlan | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) |
| promotionDistrict | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) |
| facilityAllocation | String |
| landUsePolicy | String |

## urf:DistrictsAndZones

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:EducationalAndCulturalFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |

## urf:ExceptionalFloorAreaRateDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| buildingHeightLimits | Measure |

## urf:FirePreventionDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:FireProtectionFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:FloodPreventionFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:GlobalHubCityDevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| implementationBody | String |
| implementationPeriod | String |
| plan | String |

## urf:GreenSpaceConservationDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:HeightControlDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |

## urf:HighLevelUseDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| maximumFloorAreaRate | JSON (<code><a href="#double">Double</a>[]</code>) |
| minimumFloorAreaRate | JSON (<code><a href="#double">Double</a>[]</code>) |
| maximumBuildingCoverageRate | JSON (<code><a href="#double">Double</a>[]</code>) |
| minimumBuildingArea | JSON (<code><a href="#measure">Measure</a>[]</code>) |
| setbackSize | String |

## urf:HighRiseResidentialAttractionDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| floorAreaRate | Double |
| maximumBuildingCoverageRate | Double |
| minimumSiteArea | Measure |

## urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| objectives | String |
| policy | String |
| districtDevelopmentPlan | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) |
| promotionDistrict | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) |
| landUsePolicy | String |

## urf:HousingControlArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:IndustrialParkDevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| publicFacilityAllocation | String |
| residentialLandUsePlan | String |

## urf:LandReadjustmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| publicFacilityAllocation | String |
| buildingLotDevelopment | String |

## urf:LandReadjustmentPromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| publicFacilitiesPlans | String |

## urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| publicFacilitiesPlans | String |

## urf:LandscapeZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| buildingDesignRestriction | String |
| maximumBuildingHeight | Measure |
| minimumBuildingHeight | Measure |
| setbackSize | String |
| minimumSiteArea | Measure |

## urf:MarketsSlaughterhousesCrematoria

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |

## urf:MedicalFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |

## urf:NewHousingAndUrbanDevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| housing | String |
| publicFacilityAllocation | String |
| residentialLandUsePlan | String |

## urf:NewUrbanInfrastructureProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| landForCentralPublicFacilities | String |
| districtsAllocation | String |
| landUsePlan | String |

## urf:OpenSpaceForPublicUse

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| parkAttribute | JSON (<code><a href="#urfparkattribute">urf:ParkAttribute</a></code>) |

## urf:ParkAttribute

| field | type |
|-------|------|
| parkTypeNumber | Code |
| parkSizeNumber | Code |
| parkSerialNumber | String |

## urf:ParkingPlaceDevelopmentZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:PortZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| floorAreaRate | Double |

## urf:PrivateUrbanRenewalProjectPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developer | String |
| plan | String |

## urf:ProductiveGreenZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| zoneNumber | String |
| specification | Code |

## urf:ProjectPromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| publicFacilitiesPlans | String |

## urf:QuasiUrbanPlanningArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| population | Integer |
| cityArea | Measure |
| cityPopulation | Integer |

## urf:Regulation

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:ResidenceAttractionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:ResidentialBlockConstructionProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| publicFacilityAllocation | String |
| developmentPlan | String |
| siteArea | Measure |
| totalFloorArea | Measure |

## urf:ResidentialBlockConstructionPromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| publicFacilitiesPlans | String |

## urf:ResidentialEnvironmentImprovementDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| useToBeInduced | String |
| maximumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |
| maximumBuildingHeight | String |
| setbackSize | String |
| otherRestrictions | String |

## urf:RoadsideDistrictPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| objectives | String |
| policy | String |
| districtDevelopmentPlan | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) |
| promotionDistrict | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) |
| facilitiesAllocation | String |
| landUsePolicy | String |

## urf:RuralDistrictPlan

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| objectives | String |
| policy | String |
| districtDevelopmentPlan | JSON (<code><a href="#urfdistrictdevelopmentplanproperty">urf:DistrictDevelopmentPlanProperty</a>[]</code>) |
| promotionDistrict | JSON (<code><a href="#urfpromotiondistrict">urf:PromotionDistrict</a>[]</code>) |

## urf:SandControlFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:ScenicDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| buildingCoverageRate | Double |
| buildingHeightLimits | Measure |
| wallSetbackDistanceWithRoad | Measure |
| wallSetbackDistanceWithAdjoiningLand | Measure |

## urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForCollectiveHousingFacilities

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForDistributionBusinessPark

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForIndustrialParkDevelopmentProjects

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForNewUrbanInfrastructureProjects

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:ScheduledAreaForUrbanDevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:SedimentDisasterProneArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| disasterType | Code |
| areaType | Code |
| zoneNumber | String |
| zoneName | String |
| status | Code |

## urf:SnowProtectionFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:SocialWelfareFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |

## urf:SpecialGreenSpaceConservationDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| requirement | Code |

## urf:SpecialUrbanRenaissanceDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| useToBeInduced | String |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| maximumBuildingCoverageRate | Double |
| minimumBuildingArea | Measure |
| maximumBuildingHeight | String |
| setbackSize | String |
| otherRestrictions | String |

## urf:SpecialUseAttractionDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| useToBeInduced | String |
| maximumFloorAreaRate | Double |
| minimumFloorAreaRate | Double |
| minimumBuildingArea | Measure |
| maximumBuildingHeight | String |
| otherRestrictions | String |

## urf:SpecialUseDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| buildingRestrictions | String |
| otherRestrictions | String |

## urf:SpecialUseRestrictionDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| buildingRestrictions | String |
| otherRestrictions | String |

## urf:SpecialZoneForPreservationOfHistoricalLandscape

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## urf:SpecifiedBlock

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| floorAreaRate | Double |
| maximumBuildingHeight | Measure |
| setbackSize | String |

## urf:SpecifiedDisasterPreventionBlockImprovementZone

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| minimumSiteArea | Measure |
| setbackSize | String |
| minimumFrontageRate | Double |
| minimumBuildingHeight | Measure |

## urf:SpecifiedUrgentUrbanRenewalArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| privateProject | JSON (<code><a href="#urfprivateurbanrenewalprojectplan">urf:PrivateUrbanRenewalProjectPlan</a>[]</code>) |
| specifiedArea | JSON (<code><a href="#urfspecifiedurgenturbanrenewalarea">urf:SpecifiedUrgentUrbanRenewalArea</a>[]</code>) |
| specialDistrict | JSON (<code><a href="#urfspecialurbanrenaissancedistrict">urf:SpecialUrbanRenaissanceDistrict</a>[]</code>) |
| developmentProject | JSON (<code><a href="#urfglobalhubcitydevelopmentproject">urf:GlobalHubCityDevelopmentProject</a>[]</code>) |

## urf:SupplyFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| waterWorksAttribute | JSON (<code><a href="#urfwaterworksattribute">urf:WaterWorksAttribute</a></code>) |

## urf:WaterWorksAttribute

| field | type |
|-------|------|
| startLocation | String |
| endLocation | String |

## urf:TelecommunicationFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:TideFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:TrafficFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| startLocation | String |
| endLocation | String |
| viaLocations | String |
| length | Measure |
| width | Measure |
| urbanRoadAttribute | JSON (<code><a href="#urfurbanroadattribute">urf:UrbanRoadAttribute</a></code>) |
| urbanRapidTransitRailroadAttribute | JSON (<code><a href="#urfurbanrapidtransitrailroadattribute">urf:UrbanRapidTransitRailroadAttribute</a></code>) |
| parkingPlaceAttribute | JSON (<code><a href="#urfparkingplaceattribute">urf:ParkingPlaceAttribute</a></code>) |
| vehicleTerminalAttribute | JSON (<code><a href="#urfvehicleterminalattribute">urf:VehicleTerminalAttribute</a></code>) |

## urf:UrbanRoadAttribute

| field | type |
|-------|------|
| routeTypeNumber | Code |
| routeSizeNumber | Code |
| routeSerialNumber | String |
| roadType | Code |
| numberOfLanes | Integer |
| roadStructure | String |
| structureType | Code |
| crossType | Code |
| trafficPlazas | Code |
| structuralDetails | JSON (<code><a href="#urfstructuredetails">urf:StructureDetails</a>[]</code>) |

## urf:StructureDetails

| field | type |
|-------|------|
| startLocation | String |
| endLocation | String |
| viaLocations | String |
| length | Measure |
| structureType | Code |
| minimumWidth | Measure |
| maximumWidth | Measure |
| standardWidth | Measure |
| crossType | Code |

## urf:UrbanRapidTransitRailroadAttribute

| field | type |
|-------|------|
| structureType | Code |
| crossType | Code |
| structuralDetails | JSON (<code><a href="#urfstructuredetails">urf:StructureDetails</a>[]</code>) |

## urf:ParkingPlaceAttribute

| field | type |
|-------|------|
| storeysAboveGround | NonNegativeInteger |
| storeysBelowGround | NonNegativeInteger |

## urf:VehicleTerminalAttribute

| field | type |
|-------|------|
| terminalType | Code |

## urf:TreatmentFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| sewerSystemsAttribute | JSON (<code><a href="#urfsewersystemattribute">urf:SewerSystemAttribute</a></code>) |

## urf:SewerSystemAttribute

| field | type |
|-------|------|
| startLocation | String |
| endLocation | String |
| systemType | Code |
| drainageArea | String |

## urf:TreePlantingDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| minimumGreeningRate | Double |

## urf:UnclassifiedBlankArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:UnclassifiedUseDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:UnusedLandUsePromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:UrbanDevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |

## urf:UrbanDisasterRecoveryPromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| expirationDate | Date |
| emergencyRecoveryPolicy | String |
| plannedProjectType | Code |

## urf:UrbanFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |

## urf:UrbanFacilityStipulatedByCabinetOrder

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:UrbanFunctionAttractionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |

## urf:UrbanPlanningArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaClassification | Code |
| reasonForAreaClassification | String |
| policyForAreaClassification | String |
| purposeForUrbanPlan | String |
| policyForUrbanPlanDecision | String |
| population | Integer |
| cityArea | Measure |
| cityPopulation | Integer |

## urf:UrbanRedevelopmentProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| publicFacilityAllocation | String |
| developmentPlan | String |
| housingTarget | String |
| siteArea | Measure |
| totalFloorArea | Measure |
| numberOfHousing | Integer |

## urf:UrbanRedevelopmentPromotionArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| publicFacilitiesPlans | String |
| publicFacilities | String |
| unitArea | String |

## urf:UrbanRenewalProject

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| scheduledExecutor | String |
| storeysAboveGround | NonNegativeInteger |
| storeysBelowGround | NonNegativeInteger |
| setbackSize | String |
| floorAreaRate | Double |
| buildingUsage | String |
| siteArea | Measure |

## urf:UrgentUrbanRenewalArea

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| developmentPolicy | String |
| privateProject | JSON (<code><a href="#urfprivateurbanrenewalprojectplan">urf:PrivateUrbanRenewalProjectPlan</a>[]</code>) |
| specifiedArea | JSON (<code><a href="#urfspecifiedurgenturbanrenewalarea">urf:SpecifiedUrgentUrbanRenewalArea</a>[]</code>) |
| specialDistrict | JSON (<code><a href="#urfspecialurbanrenaissancedistrict">urf:SpecialUrbanRenaissanceDistrict</a>[]</code>) |

## urf:UseDistrict

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |
| floorAreaRate | Double |
| minimumSiteArea | Measure |
| buildingCoverageRate | Double |
| wallSetbackDistance | String |
| buildingHeightLimits | Measure |
| buildingRestrictions | String |
| otherRestrictions | String |
| setbackRestrictions | String |
| frontRoadRestrictions | String |
| adjacentLandRestrictions | String |
| northDirectionRestrictions | String |
| shadeRegulation | String |

## urf:Waterway

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| startLocation | String |
| endLocation | String |
| structure | Code |
| length | Measure |
| width | Measure |

## urf:WindProtectionFacility

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| number | String |
| threeDimensionalExtent | JSON (<code><a href="#urfthreedimensionalextent">urf:ThreeDimensionalExtent</a>[]</code>) |
| length | Measure |
| width | Measure |

## urf:ZoneForPreservationOfHistoricalLandscape

| field | type |
|-------|------|
| description | String |
| name | JSON (<code><a href="#code">Code</a>[]</code>) |
| creationDate | Date |
| terminationDate | Date |
| genericAttribute | JSON (<code><a href="#gengenericattribute">gen:genericAttribute</a></code>) |
| class | Code |
| function | JSON (<code><a href="#code">Code</a>[]</code>) |
| usage | JSON (<code><a href="#code">Code</a>[]</code>) |
| validFrom | Date |
| validFromType | Code |
| enactmentFiscalYear | String |
| validTo | Date |
| validToType | Code |
| expirationFiscalYear | String |
| legalGrounds | String |
| custodian | String |
| notificationNumber | String |
| finalNotificationDate | Date |
| finalNotificationNumber | String |
| urbanPlanType | Code |
| areaClassificationType | Code |
| nominalArea | Measure |
| prefecture | Code |
| city | Code |
| reference | URI |
| reason | String |
| note | String |
| surveyYear | String |
| boundary | JSON (<code><a href="#urfboundary">urf:Boundary</a>[]</code>) |
| location | String |
| areaInTotal | Measure |

## _:TopLevelFeatureProperty

以下のいずれかの型をとります：

- bldg:Building
- tran:Road
- tran:Railway
- tran:Track
- tran:Square
- brid:Bridge
- frn:CityFurniture
- veg:SolitaryVegetationObject
- veg:PlantCover
- luse:LandUse
- tun:Tunnel
- dem:ReliefFeature
- wtr:WaterBody
- gen:GenericCityObject
- grp:CityObjectGroup
- uro:Waterway
- uro:OtherConstruction
- uro:UndergroundBuilding
- uro:Appurtenance
- uro:Cable
- uro:Duct
- uro:ElectricityCable
- uro:Handhole
- uro:Manhole
- uro:OilGasChemicalsPipe
- uro:Pipe
- uro:SewerPipe
- uro:TelecommunicationsCable
- uro:ThermalPipe
- uro:WaterPipe
- urf:Zone
- urf:Agreement
- urf:AircraftNoiseControlZone
- urf:AreaClassification
- urf:CollectiveFacilitiesForReconstruction
- urf:CollectiveFacilitiesForReconstructionAndRevitalization
- urf:CollectiveFacilitiesForTsunamiDisasterPrevention
- urf:CollectiveGovernmentAndPublicOfficeFacilities
- urf:CollectiveHousingFacilities
- urf:CollectiveUrbanDisasterPreventionFacilities
- urf:ConservationZoneForClustersOfTraditionalStructures
- urf:DisasterPreventionBlockImprovementProject
- urf:DisasterPreventionBlockImprovementZonePlan
- urf:DistributionBusinessPark
- urf:DistributionBusinessZone
- urf:District
- urf:DistrictDevelopmentPlan
- urf:DistrictFacility
- urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan
- urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict
- urf:DistrictPlan
- urf:DistrictsAndZones
- urf:EducationalAndCulturalFacility
- urf:ExceptionalFloorAreaRateDistrict
- urf:FirePreventionDistrict
- urf:FireProtectionFacility
- urf:FloodPreventionFacility
- urf:GlobalHubCityDevelopmentProject
- urf:GreenSpaceConservationDistrict
- urf:HeightControlDistrict
- urf:HighLevelUseDistrict
- urf:HighRiseResidentialAttractionDistrict
- urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan
- urf:HousingControlArea
- urf:IndustrialParkDevelopmentProject
- urf:LandReadjustmentProject
- urf:LandReadjustmentPromotionArea
- urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment
- urf:LandscapeZone
- urf:MarketsSlaughterhousesCrematoria
- urf:MedicalFacility
- urf:NewHousingAndUrbanDevelopmentProject
- urf:NewUrbanInfrastructureProject
- urf:OpenSpaceForPublicUse
- urf:ParkingPlaceDevelopmentZone
- urf:PortZone
- urf:PrivateUrbanRenewalProjectPlan
- urf:ProductiveGreenZone
- urf:ProjectPromotionArea
- urf:PromotionDistrict
- urf:QuasiUrbanPlanningArea
- urf:Regulation
- urf:ResidenceAttractionArea
- urf:ResidentialBlockConstructionProject
- urf:ResidentialBlockConstructionPromotionArea
- urf:ResidentialEnvironmentImprovementDistrict
- urf:RoadsideDistrictFacility
- urf:RoadsideDistrictImprovementPlan
- urf:RoadsideDistrictPlan
- urf:RuralDistrictFacility
- urf:RuralDistrictImprovementPlan
- urf:RuralDistrictPlan
- urf:SandControlFacility
- urf:ScenicDistrict
- urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities
- urf:ScheduledAreaForCollectiveHousingFacilities
- urf:ScheduledAreaForDistributionBusinessPark
- urf:ScheduledAreaForIndustrialParkDevelopmentProjects
- urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects
- urf:ScheduledAreaForNewUrbanInfrastructureProjects
- urf:ScheduledAreaForUrbanDevelopmentProject
- urf:SedimentDisasterProneArea
- urf:SnowProtectionFacility
- urf:SocialWelfareFacility
- urf:SpecialGreenSpaceConservationDistrict
- urf:SpecialUrbanRenaissanceDistrict
- urf:SpecialUseAttractionDistrict
- urf:SpecialUseDistrict
- urf:SpecialUseRestrictionDistrict
- urf:SpecialZoneForPreservationOfHistoricalLandscape
- urf:SpecifiedBlock
- urf:SpecifiedBuildingZoneImprovementPlan
- urf:SpecifiedDisasterPreventionBlockImprovementZone
- urf:SpecifiedUrgentUrbanRenewalArea
- urf:SupplyFacility
- urf:TelecommunicationFacility
- urf:TideFacility
- urf:TrafficFacility
- urf:TreatmentFacility
- urf:TreePlantingDistrict
- urf:UnclassifiedBlankArea
- urf:UnclassifiedUseDistrict
- urf:UnusedLandUsePromotionArea
- urf:UrbanDevelopmentProject
- urf:UrbanDisasterRecoveryPromotionArea
- urf:UrbanFacility
- urf:UrbanFacilityStipulatedByCabinetOrder
- urf:UrbanFunctionAttractionArea
- urf:UrbanPlanningArea
- urf:UrbanRedevelopmentProject
- urf:UrbanRedevelopmentPromotionArea
- urf:UrbanRenewalProject
- urf:UrgentUrbanRenewalArea
- urf:UseDistrict
- urf:Waterway
- urf:WindProtectionFacility
- urf:ZonalDisasterPreventionFacility
- urf:ZoneForPreservationOfHistoricalLandscape

