<?xml version="1.0" encoding="UTF-8"?>
<core:CityModel xmlns:grp="http://www.opengis.net/citygml/cityobjectgroup/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:smil20lang="http://www.w3.org/2001/SMIL20/Language" xmlns:pbase="http://www.opengis.net/citygml/profiles/base/2.0" xmlns:smil20="http://www.w3.org/2001/SMIL20/" xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:uro="https://www.geospatial.jp/iur/uro/2.0" xmlns:xAL="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" xmlns:luse="http://www.opengis.net/citygml/landuse/2.0" xmlns:urf="https://www.geospatial.jp/iur/urf/2.0" xmlns:app="http://www.opengis.net/citygml/appearance/2.0" xmlns:gen="http://www.opengis.net/citygml/generics/2.0" xmlns:dem="http://www.opengis.net/citygml/relief/2.0" xmlns:tex="http://www.opengis.net/citygml/texturedsurface/2.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:tun="http://www.opengis.net/citygml/tunnel/2.0" xmlns:veg="http://www.opengis.net/citygml/vegetation/2.0" xmlns:sch="http://www.ascc.net/xml/schematron" xmlns:frn="http://www.opengis.net/citygml/cityfurniture/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:tran="http://www.opengis.net/citygml/transportation/2.0" xmlns:wtr="http://www.opengis.net/citygml/waterbody/2.0" xmlns:brid="http://www.opengis.net/citygml/bridge/2.0" xsi:schemaLocation="https://www.geospatial.jp/iur/uro/2.0  ../../schemas/iur/uro/2.0/urbanObject.xsd  https://www.geospatial.jp/iur/urf/2.0  ../../schemas/iur/urf/2.0/urbanFunction.xsd  http://www.opengis.net/citygml/2.0  http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd  http://www.opengis.net/citygml/landuse/2.0  http://schemas.opengis.net/citygml/landuse/2.0/landUse.xsd http://www.opengis.net/citygml/building/2.0  http://schemas.opengis.net/citygml/building/2.0/building.xsd http://www.opengis.net/citygml/transportation/2.0  http://schemas.opengis.net/citygml/transportation/2.0/transportation.xsd http://www.opengis.net/citygml/generics/2.0  http://schemas.opengis.net/citygml/generics/2.0/generics.xsd http://www.opengis.net/citygml/waterbody/2.0  http://schemas.opengis.net/citygml/waterbody/2.0/waterBody.xsd http://www.opengis.net/citygml/relief/2.0  http://schemas.opengis.net/citygml/relief/2.0/relief.xsd http://www.opengis.net/citygml/cityobjectgroup/2.0  http://schemas.opengis.net/citygml/cityobjectgroup/2.0/cityObjectGroup.xsd http://www.opengis.net/gml  http://schemas.opengis.net/gml/3.1.1/base/gml.xsd http://www.opengis.net/citygml/appearance/2.0  http://schemas.opengis.net/citygml/appearance/2.0/appearance.xsd">
	<gml:boundedBy>
		<gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
			<gml:lowerCorner>42.93865811879179 141.43735604632477 97.942</gml:lowerCorner>
			<gml:upperCorner>42.94162525937725 141.4418085108974 119.054</gml:upperCorner>
		</gml:Envelope>
	</gml:boundedBy>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_e3cf1894-2973-4742-b301-3896f04afd99">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642328</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">2.9</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94147409013628 141.44132418475294 103.378 42.94148796554621 141.4413003542986 103.378 42.94143635007984 141.4412449098532 103.378 42.94142247468128 141.44126874029976 103.378 42.94147409013628 141.44132418475294 103.378</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94147409013628 141.44132418475294 100.378 42.94142247468128 141.44126874029976 100.378 42.94143635007984 141.4412449098532 100.378 42.94148796554621 141.4413003542986 100.378 42.94147409013628 141.44132418475294 100.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94147409013628 141.44132418475294 100.378 42.94148796554621 141.4413003542986 100.378 42.94148796554621 141.4413003542986 103.378 42.94147409013628 141.44132418475294 103.378 42.94147409013628 141.44132418475294 100.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94148796554621 141.4413003542986 100.378 42.94143635007984 141.4412449098532 100.378 42.94143635007984 141.4412449098532 103.378 42.94148796554621 141.4413003542986 103.378 42.94148796554621 141.4413003542986 100.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94143635007984 141.4412449098532 100.378 42.94142247468128 141.44126874029976 100.378 42.94142247468128 141.44126874029976 103.378 42.94143635007984 141.4412449098532 103.378 42.94143635007984 141.4412449098532 100.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94142247468128 141.44126874029976 100.378 42.94147409013628 141.44132418475294 100.378 42.94147409013628 141.44132418475294 103.378 42.94142247468128 141.44126874029976 103.378 42.94142247468128 141.44126874029976 100.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94147409013628 141.44132418475294 103.378 42.94148796554621 141.4413003542986 103.378 42.94143635007984 141.4412449098532 103.378 42.94142247468128 141.44126874029976 103.378 42.94147409013628 141.44132418475294 103.378</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636971</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.682</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">1.090</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_86497637-7b88-4200-a47a-d81121fe9a36">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642201</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">4.5</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94111966608341 141.44171945605513 101.03 42.941159723315955 141.4416552152691 101.03 42.9411369415842 141.44162892382838 101.03 42.941096974382035 141.44169316342973 101.03 42.94111966608341 141.44171945605513 101.03</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94111966608341 141.44171945605513 97.989 42.941096974382035 141.44169316342973 97.989 42.9411369415842 141.44162892382838 97.989 42.941159723315955 141.4416552152691 97.989 42.94111966608341 141.44171945605513 97.989</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94111966608341 141.44171945605513 97.989 42.941159723315955 141.4416552152691 97.989 42.941159723315955 141.4416552152691 101.03 42.94111966608341 141.44171945605513 101.03 42.94111966608341 141.44171945605513 97.989</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941159723315955 141.4416552152691 97.989 42.9411369415842 141.44162892382838 97.989 42.9411369415842 141.44162892382838 101.03 42.941159723315955 141.4416552152691 101.03 42.941159723315955 141.4416552152691 97.989</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9411369415842 141.44162892382838 97.989 42.941096974382035 141.44169316342973 97.989 42.941096974382035 141.44169316342973 101.03 42.9411369415842 141.44162892382838 101.03 42.9411369415842 141.44162892382838 97.989</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941096974382035 141.44169316342973 97.989 42.94111966608341 141.44171945605513 97.989 42.94111966608341 141.44171945605513 101.03 42.941096974382035 141.44169316342973 101.03 42.941096974382035 141.44169316342973 97.989</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94111966608341 141.44171945605513 101.03 42.941159723315955 141.4416552152691 101.03 42.9411369415842 141.44162892382838 101.03 42.941096974382035 141.44169316342973 101.03 42.94111966608341 141.44171945605513 101.03</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636888</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.773</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">3</uro:rankOrg>
					<uro:depth uom="m">3.235</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_e9cf094c-590b-4e4f-838a-29ddabe63025">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642205</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">21.8</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94083109488781 141.4412671684233 119.054 42.94085387835086 141.4412045016079 119.054 42.94082981726827 141.4411882747998 119.054 42.940807123829735 141.44125094041829 119.054 42.94083109488781 141.4412671684233 119.054</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94083109488781 141.4412671684233 98.823 42.940807123829735 141.44125094041829 98.823 42.94082981726827 141.4411882747998 98.823 42.94085387835086 141.4412045016079 98.823 42.94083109488781 141.4412671684233 98.823</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94083109488781 141.4412671684233 98.823 42.94085387835086 141.4412045016079 98.823 42.94085387835086 141.4412045016079 119.054 42.94083109488781 141.4412671684233 119.054 42.94083109488781 141.4412671684233 98.823</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94085387835086 141.4412045016079 98.823 42.94082981726827 141.4411882747998 98.823 42.94082981726827 141.4411882747998 119.054 42.94085387835086 141.4412045016079 119.054 42.94085387835086 141.4412045016079 98.823</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94082981726827 141.4411882747998 98.823 42.940807123829735 141.44125094041829 98.823 42.940807123829735 141.44125094041829 119.054 42.94082981726827 141.4411882747998 119.054 42.94082981726827 141.4411882747998 98.823</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940807123829735 141.44125094041829 98.823 42.94083109488781 141.4412671684233 98.823 42.94083109488781 141.4412671684233 119.054 42.940807123829735 141.44125094041829 119.054 42.940807123829735 141.44125094041829 98.823</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94083109488781 141.4412671684233 119.054 42.94085387835086 141.4412045016079 119.054 42.94082981726827 141.4411882747998 119.054 42.940807123829735 141.44125094041829 119.054 42.94083109488781 141.4412671684233 119.054</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636892</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.563</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.999</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_4a250485-9726-4978-99f4-ffdb18248b7d">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642257</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">4.5</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94126217295874 141.4418085108974 101.673 42.94129536044345 141.44175305969384 101.673 42.94122095654926 141.44167034337713 101.673 42.94118767908815 141.4417257957347 101.673 42.94126217295874 141.4418085108974 101.673</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94126217295874 141.4418085108974 97.967 42.94118767908815 141.4417257957347 97.967 42.94122095654926 141.44167034337713 97.967 42.94129536044345 141.44175305969384 97.967 42.94126217295874 141.4418085108974 97.967</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94126217295874 141.4418085108974 97.967 42.94129536044345 141.44175305969384 97.967 42.94129536044345 141.44175305969384 101.673 42.94126217295874 141.4418085108974 101.673 42.94126217295874 141.4418085108974 97.967</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94129536044345 141.44175305969384 97.967 42.94122095654926 141.44167034337713 97.967 42.94122095654926 141.44167034337713 101.673 42.94129536044345 141.44175305969384 101.673 42.94129536044345 141.44175305969384 97.967</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94122095654926 141.44167034337713 97.967 42.94118767908815 141.4417257957347 97.967 42.94118767908815 141.4417257957347 101.673 42.94122095654926 141.44167034337713 101.673 42.94122095654926 141.44167034337713 97.967</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94118767908815 141.4417257957347 97.967 42.94126217295874 141.4418085108974 97.967 42.94126217295874 141.4418085108974 101.673 42.94118767908815 141.4417257957347 101.673 42.94118767908815 141.4417257957347 97.967</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94126217295874 141.4418085108974 101.673 42.94129536044345 141.44175305969384 101.673 42.94122095654926 141.44167034337713 101.673 42.94118767908815 141.4417257957347 101.673 42.94126217295874 141.4418085108974 101.673</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636944</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.491</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.966</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_83b943ee-b7b3-4720-bf4f-7d7b89723d7c">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642327</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">2.7</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.941595221886736 141.44119099878384 103.267 42.94162525937725 141.4411357107184 103.267 42.941607035462305 141.44111744679097 103.267 42.94157699798047 141.44117273484866 103.267 42.941595221886736 141.44119099878384 103.267</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941595221886736 141.44119099878384 100.267 42.94157699798047 141.44117273484866 100.267 42.941607035462305 141.44111744679097 100.267 42.94162525937725 141.4411357107184 100.267 42.941595221886736 141.44119099878384 100.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941595221886736 141.44119099878384 100.267 42.94162525937725 141.4411357107184 100.267 42.94162525937725 141.4411357107184 103.267 42.941595221886736 141.44119099878384 103.267 42.941595221886736 141.44119099878384 100.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94162525937725 141.4411357107184 100.267 42.941607035462305 141.44111744679097 100.267 42.941607035462305 141.44111744679097 103.267 42.94162525937725 141.4411357107184 103.267 42.94162525937725 141.4411357107184 100.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941607035462305 141.44111744679097 100.267 42.94157699798047 141.44117273484866 100.267 42.94157699798047 141.44117273484866 103.267 42.941607035462305 141.44111744679097 103.267 42.941607035462305 141.44111744679097 100.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94157699798047 141.44117273484866 100.267 42.941595221886736 141.44119099878384 100.267 42.941595221886736 141.44119099878384 103.267 42.94157699798047 141.44117273484866 103.267 42.94157699798047 141.44117273484866 100.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941595221886736 141.44119099878384 103.267 42.94162525937725 141.4411357107184 103.267 42.941607035462305 141.44111744679097 103.267 42.94157699798047 141.44117273484866 103.267 42.941595221886736 141.44119099878384 103.267</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636970</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.558</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.920</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_045815ac-ed4f-4104-aca2-385ec0ff7737">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642256</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">4.9</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94135443505804 141.4417557178382 102.256 42.94139748622636 141.4416820026422 102.256 42.941362015760625 141.44164362383137 102.256 42.94131896548179 141.44171746153225 102.256 42.94135443505804 141.4417557178382 102.256</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94135443505804 141.4417557178382 98.091 42.94131896548179 141.44171746153225 98.091 42.941362015760625 141.44164362383137 98.091 42.94139748622636 141.4416820026422 98.091 42.94135443505804 141.4417557178382 98.091</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94135443505804 141.4417557178382 98.091 42.94139748622636 141.4416820026422 98.091 42.94139748622636 141.4416820026422 102.256 42.94135443505804 141.4417557178382 102.256 42.94135443505804 141.4417557178382 98.091</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94139748622636 141.4416820026422 98.091 42.941362015760625 141.44164362383137 98.091 42.941362015760625 141.44164362383137 102.256 42.94139748622636 141.4416820026422 102.256 42.94139748622636 141.4416820026422 98.091</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941362015760625 141.44164362383137 98.091 42.94131896548179 141.44171746153225 98.091 42.94131896548179 141.44171746153225 102.256 42.941362015760625 141.44164362383137 102.256 42.941362015760625 141.44164362383137 98.091</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94131896548179 141.44171746153225 98.091 42.94135443505804 141.4417557178382 98.091 42.94135443505804 141.4417557178382 102.256 42.94131896548179 141.44171746153225 102.256 42.94131896548179 141.44171746153225 98.091</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94135443505804 141.4417557178382 102.256 42.94139748622636 141.4416820026422 102.256 42.941362015760625 141.44164362383137 102.256 42.94131896548179 141.44171746153225 102.256 42.94135443505804 141.4417557178382 102.256</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636943</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.491</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.966</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_bfaf9ade-c8be-4b85-87d6-2c3073a5280c">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642195</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">5.4</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.93875786733414 141.43870149916597 108.126 42.93880615313624 141.43866802780934 108.126 42.93877275169289 141.4385787762347 108.126 42.938724465048566 141.43861212512616 108.126 42.93875786733414 141.43870149916597 108.126</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93875786733414 141.43870149916597 103.504 42.938724465048566 141.43861212512616 103.504 42.93877275169289 141.4385787762347 103.504 42.93880615313624 141.43866802780934 103.504 42.93875786733414 141.43870149916597 103.504</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93875786733414 141.43870149916597 103.504 42.93880615313624 141.43866802780934 103.504 42.93880615313624 141.43866802780934 108.126 42.93875786733414 141.43870149916597 108.126 42.93875786733414 141.43870149916597 103.504</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93880615313624 141.43866802780934 103.504 42.93877275169289 141.4385787762347 103.504 42.93877275169289 141.4385787762347 108.126 42.93880615313624 141.43866802780934 108.126 42.93880615313624 141.43866802780934 103.504</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93877275169289 141.4385787762347 103.504 42.938724465048566 141.43861212512616 103.504 42.938724465048566 141.43861212512616 108.126 42.93877275169289 141.4385787762347 108.126 42.93877275169289 141.4385787762347 103.504</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.938724465048566 141.43861212512616 103.504 42.93875786733414 141.43870149916597 103.504 42.93875786733414 141.43870149916597 108.126 42.938724465048566 141.43861212512616 108.126 42.938724465048566 141.43861212512616 103.504</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93875786733414 141.43870149916597 108.126 42.93880615313624 141.43866802780934 108.126 42.93877275169289 141.4385787762347 108.126 42.938724465048566 141.43861212512616 108.126 42.93875786733414 141.43870149916597 108.126</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636882</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.396</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.811</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_c3d63f19-6007-4f03-9269-9246c887e1f5">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642255</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">5.2</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.940729014234556 141.44152300224562 101.467 42.94074573558205 141.44149423362938 101.467 42.940670350848194 141.44141287903034 101.467 42.9406596770353 141.44143115333847 101.467 42.94070068007809 141.44147534070453 101.467 42.94069463255963 141.44148583500169 101.467 42.940729014234556 141.44152300224562 101.467</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940729014234556 141.44152300224562 98.199 42.94069463255963 141.44148583500169 98.199 42.94070068007809 141.44147534070453 98.199 42.9406596770353 141.44143115333847 98.199 42.940670350848194 141.44141287903034 98.199 42.94074573558205 141.44149423362938 98.199 42.940729014234556 141.44152300224562 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940729014234556 141.44152300224562 98.199 42.94074573558205 141.44149423362938 98.199 42.94074573558205 141.44149423362938 101.467 42.940729014234556 141.44152300224562 101.467 42.940729014234556 141.44152300224562 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94074573558205 141.44149423362938 98.199 42.940670350848194 141.44141287903034 98.199 42.940670350848194 141.44141287903034 101.467 42.94074573558205 141.44149423362938 101.467 42.94074573558205 141.44149423362938 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940670350848194 141.44141287903034 98.199 42.9406596770353 141.44143115333847 98.199 42.9406596770353 141.44143115333847 101.467 42.940670350848194 141.44141287903034 101.467 42.940670350848194 141.44141287903034 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9406596770353 141.44143115333847 98.199 42.94070068007809 141.44147534070453 98.199 42.94070068007809 141.44147534070453 101.467 42.9406596770353 141.44143115333847 101.467 42.9406596770353 141.44143115333847 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94070068007809 141.44147534070453 98.199 42.94069463255963 141.44148583500169 98.199 42.94069463255963 141.44148583500169 101.467 42.94070068007809 141.44147534070453 101.467 42.94070068007809 141.44147534070453 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94069463255963 141.44148583500169 98.199 42.940729014234556 141.44152300224562 98.199 42.940729014234556 141.44152300224562 101.467 42.94069463255963 141.44148583500169 101.467 42.94069463255963 141.44148583500169 98.199</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940729014234556 141.44152300224562 101.467 42.94074573558205 141.44149423362938 101.467 42.940670350848194 141.44141287903034 101.467 42.9406596770353 141.44143115333847 101.467 42.94070068007809 141.44147534070453 101.467 42.94069463255963 141.44148583500169 101.467 42.940729014234556 141.44152300224562 101.467</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636942</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.580</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">3</uro:rankOrg>
					<uro:depth uom="m">3.014</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_8816c897-f983-4726-a8f2-b5d5672f80af">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642316</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">4.1</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.9406216650457 141.4415810162781 101.887 42.940661989639885 141.44152914808424 101.887 42.94063124928932 141.44148494895094 101.887 42.940590924714954 141.4415368171472 101.887 42.9406216650457 141.4415810162781 101.887</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9406216650457 141.4415810162781 98.231 42.940590924714954 141.4415368171472 98.231 42.94063124928932 141.44148494895094 98.231 42.940661989639885 141.44152914808424 98.231 42.9406216650457 141.4415810162781 98.231</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9406216650457 141.4415810162781 98.231 42.940661989639885 141.44152914808424 98.231 42.940661989639885 141.44152914808424 101.887 42.9406216650457 141.4415810162781 101.887 42.9406216650457 141.4415810162781 98.231</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940661989639885 141.44152914808424 98.231 42.94063124928932 141.44148494895094 98.231 42.94063124928932 141.44148494895094 101.887 42.940661989639885 141.44152914808424 101.887 42.940661989639885 141.44152914808424 98.231</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94063124928932 141.44148494895094 98.231 42.940590924714954 141.4415368171472 98.231 42.940590924714954 141.4415368171472 101.887 42.94063124928932 141.44148494895094 101.887 42.94063124928932 141.44148494895094 98.231</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940590924714954 141.4415368171472 98.231 42.9406216650457 141.4415810162781 98.231 42.9406216650457 141.4415810162781 101.887 42.940590924714954 141.4415368171472 101.887 42.940590924714954 141.4415368171472 98.231</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9406216650457 141.4415810162781 101.887 42.940661989639885 141.44152914808424 101.887 42.94063124928932 141.44148494895094 101.887 42.940590924714954 141.4415368171472 101.887 42.9406216650457 141.4415810162781 101.887</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636959</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.580</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">3</uro:rankOrg>
					<uro:depth uom="m">3.014</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_e0582268-914c-483f-8ba5-8dd6afc7d6b6">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642216</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">3.2</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.941312524589506 141.44025842818291 103.602 42.941338956791654 141.4402153178997 103.602 42.94130564968754 141.44017740174337 103.602 42.941279217499506 141.44022051201912 103.602 42.941312524589506 141.44025842818291 103.602</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941312524589506 141.44025842818291 100.602 42.941279217499506 141.44022051201912 100.602 42.94130564968754 141.44017740174337 100.602 42.941338956791654 141.4402153178997 100.602 42.941312524589506 141.44025842818291 100.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941312524589506 141.44025842818291 100.602 42.941338956791654 141.4402153178997 100.602 42.941338956791654 141.4402153178997 103.602 42.941312524589506 141.44025842818291 103.602 42.941312524589506 141.44025842818291 100.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941338956791654 141.4402153178997 100.602 42.94130564968754 141.44017740174337 100.602 42.94130564968754 141.44017740174337 103.602 42.941338956791654 141.4402153178997 103.602 42.941338956791654 141.4402153178997 100.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94130564968754 141.44017740174337 100.602 42.941279217499506 141.44022051201912 100.602 42.941279217499506 141.44022051201912 103.602 42.94130564968754 141.44017740174337 103.602 42.94130564968754 141.44017740174337 100.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941279217499506 141.44022051201912 100.602 42.941312524589506 141.44025842818291 100.602 42.941312524589506 141.44025842818291 103.602 42.941279217499506 141.44022051201912 103.602 42.941279217499506 141.44022051201912 100.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941312524589506 141.44025842818291 103.602 42.941338956791654 141.4402153178997 103.602 42.94130564968754 141.44017740174337 103.602 42.941279217499506 141.44022051201912 103.602 42.941312524589506 141.44025842818291 103.602</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636903</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.658</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">1.002</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_237a25bf-b337-4a32-b17c-988a674c2ad1">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642204</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">7.4</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.9413432163982 141.4411233512482 106.35300000000001 42.941398379387984 141.44101835373897 106.35300000000001 42.941315436457025 141.44093758797467 106.35300000000001 42.941260273539974 141.4410425854128 106.35300000000001 42.9413432163982 141.4411233512482 106.35300000000001</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9413432163982 141.4411233512482 100.295 42.941260273539974 141.4410425854128 100.295 42.941315436457025 141.44093758797467 100.295 42.941398379387984 141.44101835373897 100.295 42.9413432163982 141.4411233512482 100.295</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9413432163982 141.4411233512482 100.295 42.941398379387984 141.44101835373897 100.295 42.941398379387984 141.44101835373897 106.35300000000001 42.9413432163982 141.4411233512482 106.35300000000001 42.9413432163982 141.4411233512482 100.295</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941398379387984 141.44101835373897 100.295 42.941315436457025 141.44093758797467 100.295 42.941315436457025 141.44093758797467 106.35300000000001 42.941398379387984 141.44101835373897 106.35300000000001 42.941398379387984 141.44101835373897 100.295</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941315436457025 141.44093758797467 100.295 42.941260273539974 141.4410425854128 100.295 42.941260273539974 141.4410425854128 106.35300000000001 42.941315436457025 141.44093758797467 106.35300000000001 42.941315436457025 141.44093758797467 100.295</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941260273539974 141.4410425854128 100.295 42.9413432163982 141.4411233512482 100.295 42.9413432163982 141.4411233512482 106.35300000000001 42.941260273539974 141.4410425854128 106.35300000000001 42.941260273539974 141.4410425854128 100.295</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9413432163982 141.4411233512482 106.35300000000001 42.941398379387984 141.44101835373897 106.35300000000001 42.941315436457025 141.44093758797467 106.35300000000001 42.941260273539974 141.4410425854128 106.35300000000001 42.9413432163982 141.4411233512482 106.35300000000001</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636891</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.700</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">1.103</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_80e5cea4-b1b0-4cd1-9c0d-5bc23e3f3a5b">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642214</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">8.6</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94124553318999 141.44011655586806 107.42 42.94133302637883 141.44000059599614 107.42 42.941277199173626 141.4399225394846 107.42 42.941203886797894 141.44001968855105 107.42 42.94123016318462 141.44005634886756 107.42 42.941215983310784 141.44007528219717 107.42 42.94124553318999 141.44011655586806 107.42</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94124553318999 141.44011655586806 100.563 42.941215983310784 141.44007528219717 100.563 42.94123016318462 141.44005634886756 100.563 42.941203886797894 141.44001968855105 100.563 42.941277199173626 141.4399225394846 100.563 42.94133302637883 141.44000059599614 100.563 42.94124553318999 141.44011655586806 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94124553318999 141.44011655586806 100.563 42.94133302637883 141.44000059599614 100.563 42.94133302637883 141.44000059599614 107.42 42.94124553318999 141.44011655586806 107.42 42.94124553318999 141.44011655586806 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94133302637883 141.44000059599614 100.563 42.941277199173626 141.4399225394846 100.563 42.941277199173626 141.4399225394846 107.42 42.94133302637883 141.44000059599614 107.42 42.94133302637883 141.44000059599614 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941277199173626 141.4399225394846 100.563 42.941203886797894 141.44001968855105 100.563 42.941203886797894 141.44001968855105 107.42 42.941277199173626 141.4399225394846 107.42 42.941277199173626 141.4399225394846 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941203886797894 141.44001968855105 100.563 42.94123016318462 141.44005634886756 100.563 42.94123016318462 141.44005634886756 107.42 42.941203886797894 141.44001968855105 107.42 42.941203886797894 141.44001968855105 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94123016318462 141.44005634886756 100.563 42.941215983310784 141.44007528219717 100.563 42.941215983310784 141.44007528219717 107.42 42.94123016318462 141.44005634886756 107.42 42.94123016318462 141.44005634886756 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941215983310784 141.44007528219717 100.563 42.94124553318999 141.44011655586806 100.563 42.94124553318999 141.44011655586806 107.42 42.941215983310784 141.44007528219717 107.42 42.941215983310784 141.44007528219717 100.563</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94124553318999 141.44011655586806 107.42 42.94133302637883 141.44000059599614 107.42 42.941277199173626 141.4399225394846 107.42 42.941203886797894 141.44001968855105 107.42 42.94123016318462 141.44005634886756 107.42 42.941215983310784 141.44007528219717 107.42 42.94124553318999 141.44011655586806 107.42</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636901</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.658</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">1.002</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_b01e9991-826a-4fa1-8d6a-e937461f85a4">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642330</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">2.9</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.9412387375539 141.4396270054673 104.232 42.94127524174373 141.43958241500172 104.232 42.9412508538035 141.43954548502518 104.232 42.94121434962794 141.43959007549452 104.232 42.9412387375539 141.4396270054673 104.232</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9412387375539 141.4396270054673 101.232 42.94121434962794 141.43959007549452 101.232 42.9412508538035 141.43954548502518 101.232 42.94127524174373 141.43958241500172 101.232 42.9412387375539 141.4396270054673 101.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9412387375539 141.4396270054673 101.232 42.94127524174373 141.43958241500172 101.232 42.94127524174373 141.43958241500172 104.232 42.9412387375539 141.4396270054673 104.232 42.9412387375539 141.4396270054673 101.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94127524174373 141.43958241500172 101.232 42.9412508538035 141.43954548502518 101.232 42.9412508538035 141.43954548502518 104.232 42.94127524174373 141.43958241500172 104.232 42.94127524174373 141.43958241500172 101.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9412508538035 141.43954548502518 101.232 42.94121434962794 141.43959007549452 101.232 42.94121434962794 141.43959007549452 104.232 42.9412508538035 141.43954548502518 104.232 42.9412508538035 141.43954548502518 101.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94121434962794 141.43959007549452 101.232 42.9412387375539 141.4396270054673 101.232 42.9412387375539 141.4396270054673 104.232 42.94121434962794 141.43959007549452 104.232 42.94121434962794 141.43959007549452 101.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9412387375539 141.4396270054673 104.232 42.94127524174373 141.43958241500172 104.232 42.9412508538035 141.43954548502518 104.232 42.94121434962794 141.43959007549452 104.232 42.9412387375539 141.4396270054673 104.232</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636973</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.065</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.394</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_5f1eb5a0-641d-4ced-88e8-871cc3756355">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642332</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94012806817203 141.43865532660632 105.878 42.940154718020665 141.4386176049457 105.878 42.940118825682454 141.4385706570851 105.878 42.94009217584911 141.43860837874362 105.878 42.94012806817203 141.43865532660632 105.878</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94012806817203 141.43865532660632 102.878 42.94009217584911 141.43860837874362 102.878 42.940118825682454 141.4385706570851 102.878 42.940154718020665 141.4386176049457 102.878 42.94012806817203 141.43865532660632 102.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94012806817203 141.43865532660632 102.878 42.940154718020665 141.4386176049457 102.878 42.940154718020665 141.4386176049457 105.878 42.94012806817203 141.43865532660632 105.878 42.94012806817203 141.43865532660632 102.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940154718020665 141.4386176049457 102.878 42.940118825682454 141.4385706570851 102.878 42.940118825682454 141.4385706570851 105.878 42.940154718020665 141.4386176049457 105.878 42.940154718020665 141.4386176049457 102.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940118825682454 141.4385706570851 102.878 42.94009217584911 141.43860837874362 102.878 42.94009217584911 141.43860837874362 105.878 42.940118825682454 141.4385706570851 105.878 42.940118825682454 141.4385706570851 102.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94009217584911 141.43860837874362 102.878 42.94012806817203 141.43865532660632 102.878 42.94012806817203 141.43865532660632 105.878 42.94009217584911 141.43860837874362 105.878 42.94009217584911 141.43860837874362 102.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94012806817203 141.43865532660632 105.878 42.940154718020665 141.4386176049457 105.878 42.940118825682454 141.4385706570851 105.878 42.94009217584911 141.43860837874362 105.878 42.94012806817203 141.43865532660632 105.878</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636975</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.062</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.288</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_9dce7eb8-f82c-4895-af73-50a42c52a78f">
			<gen:stringAttribute name="KeyCode">
				<gen:value>658646</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3003</bldg:class>
			<bldg:measuredHeight uom="m">3.8</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.93878408296403 141.43756177147557 108.837 42.93887458911485 141.43765505036868 108.837 42.93891681099823 141.43757901818125 108.837 42.93889858304416 141.4375602660346 108.837 42.93870034052956 141.43735604632477 108.837 42.93865811879179 141.4374320783937 108.837 42.93878408296403 141.43756177147557 108.837</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93878408296403 141.43756177147557 105.599 42.93865811879179 141.4374320783937 105.599 42.93870034052956 141.43735604632477 105.599 42.93889858304416 141.4375602660346 105.599 42.93891681099823 141.43757901818125 105.599 42.93887458911485 141.43765505036868 105.599 42.93878408296403 141.43756177147557 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93878408296403 141.43756177147557 105.599 42.93887458911485 141.43765505036868 105.599 42.93887458911485 141.43765505036868 108.837 42.93878408296403 141.43756177147557 108.837 42.93878408296403 141.43756177147557 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93887458911485 141.43765505036868 105.599 42.93891681099823 141.43757901818125 105.599 42.93891681099823 141.43757901818125 108.837 42.93887458911485 141.43765505036868 108.837 42.93887458911485 141.43765505036868 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93891681099823 141.43757901818125 105.599 42.93889858304416 141.4375602660346 105.599 42.93889858304416 141.4375602660346 108.837 42.93891681099823 141.43757901818125 108.837 42.93891681099823 141.43757901818125 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93889858304416 141.4375602660346 105.599 42.93870034052956 141.43735604632477 105.599 42.93870034052956 141.43735604632477 108.837 42.93889858304416 141.4375602660346 108.837 42.93889858304416 141.4375602660346 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93870034052956 141.43735604632477 105.599 42.93865811879179 141.4374320783937 105.599 42.93865811879179 141.4374320783937 108.837 42.93870034052956 141.43735604632477 108.837 42.93870034052956 141.43735604632477 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93865811879179 141.4374320783937 105.599 42.93878408296403 141.43756177147557 105.599 42.93878408296403 141.43756177147557 108.837 42.93865811879179 141.4374320783937 108.837 42.93865811879179 141.4374320783937 105.599</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93878408296403 141.43756177147557 108.837 42.93887458911485 141.43765505036868 108.837 42.93891681099823 141.43757901818125 108.837 42.93889858304416 141.4375602660346 108.837 42.93870034052956 141.43735604632477 108.837 42.93865811879179 141.4374320783937 108.837 42.93878408296403 141.43756177147557 108.837</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-646450</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.486</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.316</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_57f28fa4-964c-48d4-ada9-e9cb19d63aec">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642194</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">3.9</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.93882097535177 141.43866354472726 106.651 42.93884983018367 141.43864478671125 106.651 42.938828420544496 141.43858355896413 106.651 42.93879956572271 141.4386023170019 106.651 42.93882097535177 141.43866354472726 106.651</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93882097535177 141.43866354472726 103.566 42.93879956572271 141.4386023170019 103.566 42.938828420544496 141.43858355896413 103.566 42.93884983018367 141.43864478671125 103.566 42.93882097535177 141.43866354472726 103.566</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93882097535177 141.43866354472726 103.566 42.93884983018367 141.43864478671125 103.566 42.93884983018367 141.43864478671125 106.651 42.93882097535177 141.43866354472726 106.651 42.93882097535177 141.43866354472726 103.566</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93884983018367 141.43864478671125 103.566 42.938828420544496 141.43858355896413 103.566 42.938828420544496 141.43858355896413 106.651 42.93884983018367 141.43864478671125 106.651 42.93884983018367 141.43864478671125 103.566</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.938828420544496 141.43858355896413 103.566 42.93879956572271 141.4386023170019 103.566 42.93879956572271 141.4386023170019 106.651 42.938828420544496 141.43858355896413 106.651 42.938828420544496 141.43858355896413 103.566</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93879956572271 141.4386023170019 103.566 42.93882097535177 141.43866354472726 103.566 42.93882097535177 141.43866354472726 106.651 42.93879956572271 141.4386023170019 106.651 42.93879956572271 141.4386023170019 103.566</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93882097535177 141.43866354472726 106.651 42.93884983018367 141.43864478671125 106.651 42.938828420544496 141.43858355896413 106.651 42.93879956572271 141.4386023170019 106.651 42.93882097535177 141.43866354472726 106.651</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636881</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.396</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.811</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_cdfcce52-091c-479a-9a3a-e824d7cf756e">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642215</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">9.6</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94130734035923 141.44039095379853 106.393 42.94138268689366 141.44027588857762 106.393 42.94134155350891 141.4402260667361 106.393 42.94126629617311 141.440341008239 106.393 42.94130734035923 141.44039095379853 106.393</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94130734035923 141.44039095379853 100.592 42.94126629617311 141.440341008239 100.592 42.94134155350891 141.4402260667361 100.592 42.94138268689366 141.44027588857762 100.592 42.94130734035923 141.44039095379853 100.592</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94130734035923 141.44039095379853 100.592 42.94138268689366 141.44027588857762 100.592 42.94138268689366 141.44027588857762 106.393 42.94130734035923 141.44039095379853 106.393 42.94130734035923 141.44039095379853 100.592</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94138268689366 141.44027588857762 100.592 42.94134155350891 141.4402260667361 100.592 42.94134155350891 141.4402260667361 106.393 42.94138268689366 141.44027588857762 106.393 42.94138268689366 141.44027588857762 100.592</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94134155350891 141.4402260667361 100.592 42.94126629617311 141.440341008239 100.592 42.94126629617311 141.440341008239 106.393 42.94134155350891 141.4402260667361 106.393 42.94134155350891 141.4402260667361 100.592</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94126629617311 141.440341008239 100.592 42.94130734035923 141.44039095379853 100.592 42.94130734035923 141.44039095379853 106.393 42.94126629617311 141.440341008239 106.393 42.94126629617311 141.440341008239 100.592</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94130734035923 141.44039095379853 106.393 42.94138268689366 141.44027588857762 106.393 42.94134155350891 141.4402260667361 106.393 42.94126629617311 141.440341008239 106.393 42.94130734035923 141.44039095379853 106.393</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636902</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.208</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.526</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_de45f0ed-0554-4e2c-b41b-f983c8d28f4d">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642269</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">3.3</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94156921850408 141.44115470179497 103.173 42.941596437809785 141.44110827301512 103.173 42.9415437273422 141.44105075990976 103.173 42.94151641804364 141.44109718985317 103.173 42.94156921850408 141.44115470179497 103.173</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94156921850408 141.44115470179497 100.173 42.94151641804364 141.44109718985317 100.173 42.9415437273422 141.44105075990976 100.173 42.941596437809785 141.44110827301512 100.173 42.94156921850408 141.44115470179497 100.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94156921850408 141.44115470179497 100.173 42.941596437809785 141.44110827301512 100.173 42.941596437809785 141.44110827301512 103.173 42.94156921850408 141.44115470179497 103.173 42.94156921850408 141.44115470179497 100.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941596437809785 141.44110827301512 100.173 42.9415437273422 141.44105075990976 100.173 42.9415437273422 141.44105075990976 103.173 42.941596437809785 141.44110827301512 103.173 42.941596437809785 141.44110827301512 100.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.9415437273422 141.44105075990976 100.173 42.94151641804364 141.44109718985317 100.173 42.94151641804364 141.44109718985317 103.173 42.9415437273422 141.44105075990976 103.173 42.9415437273422 141.44105075990976 100.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94151641804364 141.44109718985317 100.173 42.94156921850408 141.44115470179497 100.173 42.94156921850408 141.44115470179497 103.173 42.94151641804364 141.44109718985317 103.173 42.94151641804364 141.44109718985317 100.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94156921850408 141.44115470179497 103.173 42.941596437809785 141.44110827301512 103.173 42.9415437273422 141.44105075990976 103.173 42.94151641804364 141.44109718985317 103.173 42.94156921850408 141.44115470179497 103.173</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636956</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.558</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.920</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_cdd56240-4194-4013-b88b-91fa5d69a996">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642254</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">18.2</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.940875756509946 141.44119208458807 114.86699999999999 42.94088562618026 141.44116193512548 114.86699999999999 42.940843856190185 141.44113662774728 114.86699999999999 42.94083398652635 141.4411667771933 114.86699999999999 42.940875756509946 141.44119208458807 114.86699999999999</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940875756509946 141.44119208458807 99.472 42.94083398652635 141.4411667771933 99.472 42.940843856190185 141.44113662774728 99.472 42.94088562618026 141.44116193512548 99.472 42.940875756509946 141.44119208458807 99.472</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940875756509946 141.44119208458807 99.472 42.94088562618026 141.44116193512548 99.472 42.94088562618026 141.44116193512548 114.86699999999999 42.940875756509946 141.44119208458807 114.86699999999999 42.940875756509946 141.44119208458807 99.472</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94088562618026 141.44116193512548 99.472 42.940843856190185 141.44113662774728 99.472 42.940843856190185 141.44113662774728 114.86699999999999 42.94088562618026 141.44116193512548 114.86699999999999 42.94088562618026 141.44116193512548 99.472</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940843856190185 141.44113662774728 99.472 42.94083398652635 141.4411667771933 99.472 42.94083398652635 141.4411667771933 114.86699999999999 42.940843856190185 141.44113662774728 114.86699999999999 42.940843856190185 141.44113662774728 99.472</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94083398652635 141.4411667771933 99.472 42.940875756509946 141.44119208458807 99.472 42.940875756509946 141.44119208458807 114.86699999999999 42.94083398652635 141.4411667771933 114.86699999999999 42.94083398652635 141.4411667771933 99.472</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940875756509946 141.44119208458807 114.86699999999999 42.94088562618026 141.44116193512548 114.86699999999999 42.940843856190185 141.44113662774728 114.86699999999999 42.94083398652635 141.4411667771933 114.86699999999999 42.940875756509946 141.44119208458807 114.86699999999999</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636941</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">1.626</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.045</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_4c44e65c-4f30-431a-998d-acf42a1437e2">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642203</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">7.2</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94141529799393 141.44100477615058 106.893 42.94146491371719 141.44091737325215 106.893 42.941384465176235 141.44083289889042 106.893 42.94133484951644 141.4409203017409 106.893 42.94141529799393 141.44100477615058 106.893</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94141529799393 141.44100477615058 100.269 42.94133484951644 141.4409203017409 100.269 42.941384465176235 141.44083289889042 100.269 42.94146491371719 141.44091737325215 100.269 42.94141529799393 141.44100477615058 100.269</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94141529799393 141.44100477615058 100.269 42.94146491371719 141.44091737325215 100.269 42.94146491371719 141.44091737325215 106.893 42.94141529799393 141.44100477615058 106.893 42.94141529799393 141.44100477615058 100.269</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94146491371719 141.44091737325215 100.269 42.941384465176235 141.44083289889042 100.269 42.941384465176235 141.44083289889042 106.893 42.94146491371719 141.44091737325215 106.893 42.94146491371719 141.44091737325215 100.269</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941384465176235 141.44083289889042 100.269 42.94133484951644 141.4409203017409 100.269 42.94133484951644 141.4409203017409 106.893 42.941384465176235 141.44083289889042 106.893 42.941384465176235 141.44083289889042 100.269</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94133484951644 141.4409203017409 100.269 42.94141529799393 141.44100477615058 100.269 42.94141529799393 141.44100477615058 106.893 42.94133484951644 141.4409203017409 106.893 42.94133484951644 141.4409203017409 100.269</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94141529799393 141.44100477615058 106.893 42.94146491371719 141.44091737325215 106.893 42.941384465176235 141.44083289889042 106.893 42.94133484951644 141.4409203017409 106.893 42.94141529799393 141.44100477615058 106.893</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636890</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.387</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">0.726</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_5e0a494e-20f6-422c-b2c0-86bd3c984b24">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642258</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">2.8</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94121203072322 141.44179593327848 100.964 42.941223145155746 141.44177630524862 100.964 42.94118931663442 141.44174096831904 100.964 42.94117820134275 141.44176047382336 100.964 42.94121203072322 141.44179593327848 100.964</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94121203072322 141.44179593327848 97.964 42.94117820134275 141.44176047382336 97.964 42.94118931663442 141.44174096831904 97.964 42.941223145155746 141.44177630524862 97.964 42.94121203072322 141.44179593327848 97.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94121203072322 141.44179593327848 97.964 42.941223145155746 141.44177630524862 97.964 42.941223145155746 141.44177630524862 100.964 42.94121203072322 141.44179593327848 100.964 42.94121203072322 141.44179593327848 97.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941223145155746 141.44177630524862 97.964 42.94118931663442 141.44174096831904 97.964 42.94118931663442 141.44174096831904 100.964 42.941223145155746 141.44177630524862 100.964 42.941223145155746 141.44177630524862 97.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94118931663442 141.44174096831904 97.964 42.94117820134275 141.44176047382336 97.964 42.94117820134275 141.44176047382336 100.964 42.94118931663442 141.44174096831904 100.964 42.94118931663442 141.44174096831904 97.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94117820134275 141.44176047382336 97.964 42.94121203072322 141.44179593327848 97.964 42.94121203072322 141.44179593327848 100.964 42.94117820134275 141.44176047382336 100.964 42.94117820134275 141.44176047382336 97.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94121203072322 141.44179593327848 100.964 42.941223145155746 141.44177630524862 100.964 42.94118931663442 141.44174096831904 100.964 42.94117820134275 141.44176047382336 100.964 42.94121203072322 141.44179593327848 100.964</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636945</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.773</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">3</uro:rankOrg>
					<uro:depth uom="m">3.235</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_a13a78be-5caa-4e44-af07-b79130b64448">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642335</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.941313528617606 141.43957345807127 104.19 42.941331047464224 141.4395429627368 104.19 42.941295307978976 141.4395048337902 104.19 42.94127778914235 141.43953532911755 104.19 42.941313528617606 141.43957345807127 104.19</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941313528617606 141.43957345807127 101.19 42.94127778914235 141.43953532911755 101.19 42.941295307978976 141.4395048337902 101.19 42.941331047464224 141.4395429627368 101.19 42.941313528617606 141.43957345807127 101.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941313528617606 141.43957345807127 101.19 42.941331047464224 141.4395429627368 101.19 42.941331047464224 141.4395429627368 104.19 42.941313528617606 141.43957345807127 104.19 42.941313528617606 141.43957345807127 101.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941331047464224 141.4395429627368 101.19 42.941295307978976 141.4395048337902 101.19 42.941295307978976 141.4395048337902 104.19 42.941331047464224 141.4395429627368 104.19 42.941331047464224 141.4395429627368 101.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941295307978976 141.4395048337902 101.19 42.94127778914235 141.43953532911755 101.19 42.94127778914235 141.43953532911755 104.19 42.941295307978976 141.4395048337902 104.19 42.941295307978976 141.4395048337902 101.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94127778914235 141.43953532911755 101.19 42.941313528617606 141.43957345807127 101.19 42.941313528617606 141.43957345807127 104.19 42.94127778914235 141.43953532911755 104.19 42.94127778914235 141.43953532911755 101.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941313528617606 141.43957345807127 104.19 42.941331047464224 141.4395429627368 104.19 42.941295307978976 141.4395048337902 104.19 42.94127778914235 141.43953532911755 104.19 42.941313528617606 141.43957345807127 104.19</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636978</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.065</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.394</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_85754aa2-9d6f-402e-8c43-ea37953f470e">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642253</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">8.2</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.94021498724567 141.43901748782716 108.73899999999999 42.94034133470104 141.4388076499884 108.73899999999999 42.94020558479323 141.43865614670213 108.73899999999999 42.940079147595576 141.43886598556287 108.73899999999999 42.94021498724567 141.43901748782716 108.73899999999999</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94021498724567 141.43901748782716 101.761 42.940079147595576 141.43886598556287 101.761 42.94020558479323 141.43865614670213 101.761 42.94034133470104 141.4388076499884 101.761 42.94021498724567 141.43901748782716 101.761</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94021498724567 141.43901748782716 101.761 42.94034133470104 141.4388076499884 101.761 42.94034133470104 141.4388076499884 108.73899999999999 42.94021498724567 141.43901748782716 108.73899999999999 42.94021498724567 141.43901748782716 101.761</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94034133470104 141.4388076499884 101.761 42.94020558479323 141.43865614670213 101.761 42.94020558479323 141.43865614670213 108.73899999999999 42.94034133470104 141.4388076499884 108.73899999999999 42.94034133470104 141.4388076499884 101.761</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94020558479323 141.43865614670213 101.761 42.940079147595576 141.43886598556287 101.761 42.940079147595576 141.43886598556287 108.73899999999999 42.94020558479323 141.43865614670213 108.73899999999999 42.94020558479323 141.43865614670213 101.761</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.940079147595576 141.43886598556287 101.761 42.94021498724567 141.43901748782716 101.761 42.94021498724567 141.43901748782716 108.73899999999999 42.940079147595576 141.43886598556287 108.73899999999999 42.940079147595576 141.43886598556287 101.761</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94021498724567 141.43901748782716 108.73899999999999 42.94034133470104 141.4388076499884 108.73899999999999 42.94020558479323 141.43865614670213 108.73899999999999 42.940079147595576 141.43886598556287 108.73899999999999 42.94021498724567 141.43901748782716 108.73899999999999</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636940</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.113</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.289</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_cf819613-bef1-4ee5-8b05-51a8b9ab2a28">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642202</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:measuredHeight uom="m">3.1</bldg:measuredHeight>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.941021124688966 141.44172442109877 100.942 42.94103713893644 141.4416974996301 100.942 42.941006018653084 141.4416632302054 100.942 42.94098991353254 141.44169003032613 100.942 42.941021124688966 141.44172442109877 100.942</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941021124688966 141.44172442109877 97.942 42.94098991353254 141.44169003032613 97.942 42.941006018653084 141.4416632302054 97.942 42.94103713893644 141.4416974996301 97.942 42.941021124688966 141.44172442109877 97.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941021124688966 141.44172442109877 97.942 42.94103713893644 141.4416974996301 97.942 42.94103713893644 141.4416974996301 100.942 42.941021124688966 141.44172442109877 100.942 42.941021124688966 141.44172442109877 97.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94103713893644 141.4416974996301 97.942 42.941006018653084 141.4416632302054 97.942 42.941006018653084 141.4416632302054 100.942 42.94103713893644 141.4416974996301 100.942 42.94103713893644 141.4416974996301 97.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941006018653084 141.4416632302054 97.942 42.94098991353254 141.44169003032613 97.942 42.94098991353254 141.44169003032613 100.942 42.941006018653084 141.4416632302054 100.942 42.941006018653084 141.4416632302054 97.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.94098991353254 141.44169003032613 97.942 42.941021124688966 141.44172442109877 97.942 42.941021124688966 141.44172442109877 100.942 42.94098991353254 141.44169003032613 100.942 42.94098991353254 141.44169003032613 97.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.941021124688966 141.44172442109877 100.942 42.94103713893644 141.4416974996301 100.942 42.941006018653084 141.4416632302054 100.942 42.94098991353254 141.44169003032613 100.942 42.941021124688966 141.44172442109877 100.942</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636889</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">2</uro:rankOrg>
					<uro:depth uom="m">2.898</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">1</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">3</uro:rankOrg>
					<uro:depth uom="m">3.311</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<bldg:Building gml:id="bldg_5dd40201-14ee-4f01-8ac5-2b2424902c6d">
			<gen:stringAttribute name="KeyCode">
				<gen:value>642252</gen:value>
			</gen:stringAttribute>
			<bldg:class codeSpace="../../codelists/Building_class.xml">3001</bldg:class>
			<bldg:lod0RoofEdge>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>42.93940780552608 141.4377062415385 107.932 42.93941911059372 141.43768820372068 107.932 42.93940068349793 141.4376667583771 107.932 42.93938937930307 141.4376849187107 107.932 42.93940780552608 141.4377062415385 107.932</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</bldg:lod0RoofEdge>
			<bldg:lod1Solid>
				<gml:Solid>
					<gml:exterior>
						<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93940780552608 141.4377062415385 104.932 42.93938937930307 141.4376849187107 104.932 42.93940068349793 141.4376667583771 104.932 42.93941911059372 141.43768820372068 104.932 42.93940780552608 141.4377062415385 104.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93940780552608 141.4377062415385 104.932 42.93941911059372 141.43768820372068 104.932 42.93941911059372 141.43768820372068 107.932 42.93940780552608 141.4377062415385 107.932 42.93940780552608 141.4377062415385 104.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93941911059372 141.43768820372068 104.932 42.93940068349793 141.4376667583771 104.932 42.93940068349793 141.4376667583771 107.932 42.93941911059372 141.43768820372068 107.932 42.93941911059372 141.43768820372068 104.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93940068349793 141.4376667583771 104.932 42.93938937930307 141.4376849187107 104.932 42.93938937930307 141.4376849187107 107.932 42.93940068349793 141.4376667583771 107.932 42.93940068349793 141.4376667583771 104.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93938937930307 141.4376849187107 104.932 42.93940780552608 141.4377062415385 104.932 42.93940780552608 141.4377062415385 107.932 42.93938937930307 141.4376849187107 107.932 42.93938937930307 141.4376849187107 104.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>42.93940780552608 141.4377062415385 107.932 42.93941911059372 141.43768820372068 107.932 42.93940068349793 141.4376667583771 107.932 42.93938937930307 141.4376849187107 107.932 42.93940780552608 141.4377062415385 107.932</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>
					</gml:exterior>
				</gml:Solid>
			</bldg:lod1Solid>
			<uro:buildingIDAttribute>
				<uro:BuildingIDAttribute>
					<uro:buildingID>01100-bldg-636939</uro:buildingID>
					<uro:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">01</uro:prefecture>
					<uro:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">01110</uro:city>
				</uro:BuildingIDAttribute>
			</uro:buildingIDAttribute>
			<uro:buildingDetailAttribute>
				<uro:BuildingDetailAttribute>
					<uro:buildingStructureType codeSpace="../../codelists/Building_buildingStructureType.xml">611</uro:buildingStructureType>
					<uro:surveyYear>2019</uro:surveyYear>
				</uro:BuildingDetailAttribute>
			</uro:buildingDetailAttribute>
			<uro:buildingDisasterRiskAttribute>
				<uro:BuildingRiverFloodingRiskAttribute>
					<uro:description codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_rankOrg.xml">1</uro:rankOrg>
					<uro:depth uom="m">0.071</uro:depth>
					<uro:adminType codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_adminType.xml">2</uro:adminType>
					<uro:scale codeSpace="../../codelists/BuildingRiverFloodingRiskAttribute_scale.xml">2</uro:scale>
				</uro:BuildingRiverFloodingRiskAttribute>
			</uro:buildingDisasterRiskAttribute>
			<uro:buildingDataQualityAttribute>
				<uro:BuildingDataQualityAttribute>
					<uro:lod1HeightType codeSpace="../../codelists/BuildingDataQualityAttribute_lod1HeightType.xml">2</uro:lod1HeightType>
				</uro:BuildingDataQualityAttribute>
			</uro:buildingDataQualityAttribute>
		</bldg:Building>
	</core:cityObjectMember>
</core:CityModel>
