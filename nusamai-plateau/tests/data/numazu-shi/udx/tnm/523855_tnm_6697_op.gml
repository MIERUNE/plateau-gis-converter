<?xml version="1.0" encoding="UTF-8"?>
<core:CityModel xmlns:brid="http://www.opengis.net/citygml/bridge/2.0" xmlns:tran="http://www.opengis.net/citygml/transportation/2.0" xmlns:frn="http://www.opengis.net/citygml/cityfurniture/2.0" xmlns:wtr="http://www.opengis.net/citygml/waterbody/2.0" xmlns:sch="http://www.ascc.net/xml/schematron" xmlns:veg="http://www.opengis.net/citygml/vegetation/2.0" xmlns:tun="http://www.opengis.net/citygml/tunnel/2.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:tex="http://www.opengis.net/citygml/texturedsurface/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:gen="http://www.opengis.net/citygml/generics/2.0" xmlns:dem="http://www.opengis.net/citygml/relief/2.0" xmlns:app="http://www.opengis.net/citygml/appearance/2.0" xmlns:xAL="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" xmlns:uro="https://www.geospatial.jp/iur/uro/2.0" xmlns:luse="http://www.opengis.net/citygml/landuse/2.0" xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:smil20="http://www.w3.org/2001/SMIL20/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:smil20lang="http://www.w3.org/2001/SMIL20/Language" xmlns:pbase="http://www.opengis.net/citygml/profiles/base/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:grp="http://www.opengis.net/citygml/cityobjectgroup/2.0" xsi:schemaLocation="https://www.geospatial.jp/iur/uro/2.0 ../../../schemas/iur/uro/2.0/urbanObject.xsd https://www.geospatial.jp/iur/urf/2.0 ../../../schemas/iur/urf/2.0/urbanFunction.xsd http://www.opengis.net/citygml/2.0 http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd http://www.opengis.net/citygml/landuse/2.0 http://schemas.opengis.net/citygml/landuse/2.0/landUse.xsd http://www.opengis.net/citygml/building/2.0 http://schemas.opengis.net/citygml/building/2.0/building.xsd http://www.opengis.net/citygml/transportation/2.0 http://schemas.opengis.net/citygml/transportation/2.0/transportation.xsd http://www.opengis.net/citygml/generics/2.0 http://schemas.opengis.net/citygml/generics/2.0/generics.xsd http://www.opengis.net/citygml/waterbody/2.0 http://schemas.opengis.net/citygml/waterbody/2.0/waterBody.xsd http://www.opengis.net/citygml/relief/2.0 http://schemas.opengis.net/citygml/relief/2.0/relief.xsd http://www.opengis.net/citygml/cityobjectgroup/2.0 http://schemas.opengis.net/citygml/cityobjectgroup/2.0/cityObjectGroup.xsd http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/base/gml.xsd http://www.opengis.net/citygml/appearance/2.0 http://schemas.opengis.net/citygml/appearance/2.0/appearance.xsd">
	<gml:boundedBy>
		<gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
			<gml:lowerCorner>35.132065167120295 138.7499664401955 5.130000114440918</gml:lowerCorner>
			<gml:upperCorner>35.13233553667786 138.75002197641564 5.210000038146973</gml:upperCorner>
		</gml:Envelope>
	</gml:boundedBy>
	<core:cityObjectMember>
		<wtr:WaterBody gml:id="tnm_83c53be3-4311-43d4-bd11-b5c01292c6ed">
			 <gml:name codeSpace="../../../codelists/WaterBody_name.xml">10</gml:name> 
			<wtr:class codeSpace="../../../codelists/WaterBody_class.xml">1140</wtr:class>
			<wtr:function codeSpace="../../../codelists/WaterBody_function.xml">2</wtr:function>
			<wtr:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.132245473596086 138.749966990401 5.193333148956299 35.132245360458306 138.750021838764 5.185012340545654 35.132290392081565 138.75002197641564 5.210000038146973 35.132245473596086 138.749966990401 5.193333148956299</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13233553667786 138.74996726564387 5.210000038146973 35.132245473596086 138.749966990401 5.193333148956299 35.132290392081565 138.75002197641564 5.210000038146973 35.13233553667786 138.74996726564387 5.210000038146973</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</wtr:lod1MultiSurface>
			<uro:floodingRiskAttribute>
				<uro:WaterBodyTsunamiRiskAttribute>
					<uro:description codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_rankOrg.xml">1</uro:rankOrg>
				</uro:WaterBodyTsunamiRiskAttribute>
			</uro:floodingRiskAttribute>
		</wtr:WaterBody>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<wtr:WaterBody gml:id="tnm_4faf25b0-3db7-48e0-8cce-b5c93557aa55">
			 <gml:name codeSpace="../../../codelists/WaterBody_name.xml">10</gml:name> 
			<wtr:class codeSpace="../../../codelists/WaterBody_class.xml">1140</wtr:class>
			<wtr:function codeSpace="../../../codelists/WaterBody_function.xml">2</wtr:function>
			<wtr:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13220028360781 138.7500217009744 5.159999847412109 35.132245473596086 138.749966990401 5.193333148956299 35.132155320359715 138.74996671488404 5.150000095367432 35.13220028360781 138.7500217009744 5.159999847412109</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13220028360781 138.7500217009744 5.159999847412109 35.132245360458306 138.750021838764 5.185012340545654 35.132245473596086 138.749966990401 5.193333148956299 35.13220028360781 138.7500217009744 5.159999847412109</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13220028360781 138.7500217009744 5.159999847412109 35.132155320359715 138.74996671488404 5.150000095367432 35.132155207222304 138.75002156318655 5.144999980926514 35.13220028360781 138.7500217009744 5.159999847412109</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</wtr:lod1MultiSurface>
			<uro:floodingRiskAttribute>
				<uro:WaterBodyTsunamiRiskAttribute>
					<uro:description codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_rankOrg.xml">2</uro:rankOrg>
				</uro:WaterBodyTsunamiRiskAttribute>
			</uro:floodingRiskAttribute>
		</wtr:WaterBody>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<wtr:WaterBody gml:id="tnm_4bf9ecb4-e0d7-4df4-bff9-6129c82ccf1e">
			 <gml:name codeSpace="../../../codelists/WaterBody_name.xml">10</gml:name> 
			<wtr:class codeSpace="../../../codelists/WaterBody_class.xml">1140</wtr:class>
			<wtr:function codeSpace="../../../codelists/WaterBody_function.xml">2</wtr:function>
			<wtr:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13211013096191 138.75002142539938 5.130000114440918 35.132155207222304 138.75002156318655 5.144999980926514 35.132155320359715 138.74996671488404 5.150000095367432 35.13211013096191 138.75002142539938 5.130000114440918</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.13211013096191 138.75002142539938 5.130000114440918 35.132155320359715 138.74996671488404 5.150000095367432 35.132065167120295 138.7499664401955 5.130000114440918 35.13211013096191 138.75002142539938 5.130000114440918</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</wtr:lod1MultiSurface>
			<uro:floodingRiskAttribute>
				<uro:WaterBodyTsunamiRiskAttribute>
					<uro:description codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_description.xml">2</uro:description>
					<uro:rankOrg codeSpace="../../../codelists/WaterBodyTsunamiRiskAttribute_rankOrg.xml">3</uro:rankOrg>
				</uro:WaterBodyTsunamiRiskAttribute>
			</uro:floodingRiskAttribute>
		</wtr:WaterBody>
	</core:cityObjectMember>
</core:CityModel>