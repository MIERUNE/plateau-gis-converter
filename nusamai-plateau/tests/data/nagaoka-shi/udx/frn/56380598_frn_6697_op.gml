<?xml version="1.0" encoding="UTF-8"?>
<core:CityModel xmlns:brid="http://www.opengis.net/citygml/bridge/2.0" xmlns:tran="http://www.opengis.net/citygml/transportation/2.0" xmlns:frn="http://www.opengis.net/citygml/cityfurniture/2.0" xmlns:wtr="http://www.opengis.net/citygml/waterbody/2.0" xmlns:sch="http://www.ascc.net/xml/schematron" xmlns:veg="http://www.opengis.net/citygml/vegetation/2.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:tun="http://www.opengis.net/citygml/tunnel/2.0" xmlns:tex="http://www.opengis.net/citygml/texturedsurface/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:app="http://www.opengis.net/citygml/appearance/2.0" xmlns:gen="http://www.opengis.net/citygml/generics/2.0" xmlns:dem="http://www.opengis.net/citygml/relief/2.0" xmlns:luse="http://www.opengis.net/citygml/landuse/2.0" xmlns:uro="https://www.geospatial.jp/iur/uro/3.1" xmlns:xAL="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:smil20="http://www.w3.org/2001/SMIL20/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:smil20lang="http://www.w3.org/2001/SMIL20/Language" xmlns:pbase="http://www.opengis.net/citygml/profiles/base/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:grp="http://www.opengis.net/citygml/cityobjectgroup/2.0" xsi:schemaLocation="https://www.geospatial.jp/iur/uro/3.1 ../../schemas/iur/uro/3.1/urbanObject.xsd http://www.opengis.net/citygml/2.0 http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd http://www.opengis.net/citygml/landuse/2.0 http://schemas.opengis.net/citygml/landuse/2.0/landUse.xsd http://www.opengis.net/citygml/building/2.0 http://schemas.opengis.net/citygml/building/2.0/building.xsd http://www.opengis.net/citygml/transportation/2.0 http://schemas.opengis.net/citygml/transportation/2.0/transportation.xsd http://www.opengis.net/citygml/generics/2.0 http://schemas.opengis.net/citygml/generics/2.0/generics.xsd http://www.opengis.net/citygml/cityobjectgroup/2.0 http://schemas.opengis.net/citygml/cityobjectgroup/2.0/cityObjectGroup.xsd http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/base/gml.xsd http://www.opengis.net/citygml/cityfurniture/2.0 http://schemas.opengis.net/citygml/cityfurniture/2.0/cityFurniture.xsd http://www.opengis.net/citygml/vegetation/2.0 http://schemas.opengis.net/citygml/vegetation/2.0/vegetation.xsd http://www.opengis.net/citygml/appearance/2.0 http://schemas.opengis.net/citygml/appearance/2.0/appearance.xsd">
	<gml:boundedBy>
		<gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
			<gml:lowerCorner>37.41632839720099 138.73697501715893 0</gml:lowerCorner>
			<gml:upperCorner>37.41638315398499 138.73704103366492 0</gml:upperCorner>
		</gml:Envelope>
	</gml:boundedBy>
	<core:cityObjectMember>
		<frn:CityFurniture gml:id="frn_1cc6e096-caab-45e1-a195-6bd5fbe6ee8c">
			<core:creationDate>2024-03-22</core:creationDate>
			<frn:function codeSpace="../../codelists/CityFurniture_function.xml">9000</frn:function>
			<uro:frnDmAttribute>
				<uro:DmGeometricAttribute>
					<uro:dmCode codeSpace="../../codelists/Common_dmCode.xml">8142</uro:dmCode>
					<uro:geometryType codeSpace="../../codelists/Common_geometryType.xml">E2</uro:geometryType>
					<uro:mapLevel codeSpace="../../codelists/Common_MapLevel.xml">2500</uro:mapLevel>
					<uro:shapeType codeSpace="../../codelists/Common_shapeType.xml">0</uro:shapeType>
				<uro:lod0Geometry>
					<gml:MultiCurve>
						<gml:curveMembers>
							<gml:Curve>
								<gml:segments>
									<gml:LineStringSegment>
										<gml:posList srsDimension="3">37.41632839720099 138.73697501715893 0 37.41638315398499 138.73704103366492 0</gml:posList>
									</gml:LineStringSegment>
								</gml:segments>
							</gml:Curve>
						</gml:curveMembers>
					</gml:MultiCurve>
				</uro:lod0Geometry>
				</uro:DmGeometricAttribute>
			</uro:frnDmAttribute>
			<uro:frnFacilityAttribute>
				<uro:ParkFacilityLongevityPlanAttribute>
					<uro:facilityId>KAN0144</uro:facilityId>
					<uro:parkCode codeSpace="../../codelists/Common_parkCode.xml">13</uro:parkCode>
					<uro:parkName codeSpace="../../codelists/Common_parkName.xml">13</uro:parkName>
					<uro:parkType codeSpace="../../codelists/Common_parkType.xml">7</uro:parkType>
					<uro:facilityName codeSpace="../../codelists/Common_parkFacilityName.xml">8010</uro:facilityName>
					<uro:facilityNameOptional>YF-27</uro:facilityNameOptional>
					<uro:specificFacilityName>鋼製ゲート</uro:specificFacilityName>
					<uro:numberOfFacilities>
						<uro:NumberOfFacilities>
							<uro:quantity>1</uro:quantity>
							<uro:quantityUnit codeSpace="../../codelists/Common_unitOfNumberOfParkFacilities.xml">7</uro:quantityUnit>
						</uro:NumberOfFacilities>
					</uro:numberOfFacilities>
					<uro:size>1基</uro:size>
					<uro:mainMaterial codeSpace="../../codelists/Common_parkFacilityMainMaterial.xml">15</uro:mainMaterial>
					<uro:mainMaterialOptional>金属製</uro:mainMaterialOptional>
					<uro:installationYear>2007</uro:installationYear>
					<uro:disposalLimitPeriod>18</uro:disposalLimitPeriod>
					<uro:expectedUsagePeriod>44</uro:expectedUsagePeriod>
					<uro:parkHealthAssessment>
						<uro:ParkHealthAssessment>
							<uro:assessmentFiscalYear>2012</uro:assessmentFiscalYear>
							<uro:condition codeSpace="../../codelists/Common_parkHealthAssessmentCondition.xml">1</uro:condition>
							<uro:urgency codeSpace="../../codelists/Common_parkHealthAssessmentUrgency.xml">1</uro:urgency>
						</uro:ParkHealthAssessment>
					</uro:parkHealthAssessment>
					<uro:managementType codeSpace="../../codelists/Common_parkFacilityManagementType.xml">2</uro:managementType>
				</uro:ParkFacilityLongevityPlanAttribute>
			</uro:frnFacilityAttribute>
			<uro:frnFacilityTypeAttribute>
				<uro:FacilityTypeAttribute>
					<uro:class codeSpace="../../codelists/FacilityTypeAttribute_class.xml">07</uro:class>
					<uro:function codeSpace="../../codelists/FacilityTypeAttribute_function.xml">0708</uro:function>
				</uro:FacilityTypeAttribute>
			</uro:frnFacilityTypeAttribute>
			<uro:frnDataQualityAttribute>
				<uro:DataQualityAttribute>
					<uro:geometrySrcDescLod0 codeSpace="../../codelists/DataQualityAttribute_geometrySrcDesc.xml">000</uro:geometrySrcDescLod0>
					<uro:geometrySrcDescLod1 codeSpace="../../codelists/DataQualityAttribute_geometrySrcDesc.xml">999</uro:geometrySrcDescLod1>
					<uro:geometrySrcDescLod2 codeSpace="../../codelists/DataQualityAttribute_geometrySrcDesc.xml">999</uro:geometrySrcDescLod2>
					<uro:geometrySrcDescLod3 codeSpace="../../codelists/DataQualityAttribute_geometrySrcDesc.xml">999</uro:geometrySrcDescLod3>
					<uro:thematicSrcDesc codeSpace="../../codelists/DataQualityAttribute_thematicSrcDesc.xml">300</uro:thematicSrcDesc>
					<uro:thematicSrcDesc codeSpace="../../codelists/DataQualityAttribute_thematicSrcDesc.xml">700</uro:thematicSrcDesc>
					<uro:appearanceSrcDescLod2 codeSpace="../../codelists/DataQualityAttribute_appearanceSrcDesc.xml">99</uro:appearanceSrcDescLod2>
					<uro:appearanceSrcDescLod3 codeSpace="../../codelists/DataQualityAttribute_appearanceSrcDesc.xml">99</uro:appearanceSrcDescLod3>
					<uro:publicSurveyDataQualityAttribute>
						<uro:PublicSurveyDataQualityAttribute>
							<uro:srcScaleLod0 codeSpace="../../codelists/PublicSurveyDataQualityAttribute_srcScale.xml">1</uro:srcScaleLod0>
							<uro:publicSurveySrcDescLod0 codeSpace="../../codelists/PublicSurveyDataQualityAttribute_publicSurveySrcDesc.xml">003</uro:publicSurveySrcDescLod0>
						</uro:PublicSurveyDataQualityAttribute>
					</uro:publicSurveyDataQualityAttribute>
				</uro:DataQualityAttribute>
			</uro:frnDataQualityAttribute>
		</frn:CityFurniture>
	</core:cityObjectMember>
</core:CityModel>

