<?xml version="1.0" encoding="UTF-8"?>
<core:CityModel xmlns:brid="http://www.opengis.net/citygml/bridge/2.0" xmlns:tran="http://www.opengis.net/citygml/transportation/2.0" xmlns:frn="http://www.opengis.net/citygml/cityfurniture/2.0" xmlns:wtr="http://www.opengis.net/citygml/waterbody/2.0" xmlns:sch="http://www.ascc.net/xml/schematron" xmlns:veg="http://www.opengis.net/citygml/vegetation/2.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:tun="http://www.opengis.net/citygml/tunnel/2.0" xmlns:tex="http://www.opengis.net/citygml/texturedsurface/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:app="http://www.opengis.net/citygml/appearance/2.0" xmlns:gen="http://www.opengis.net/citygml/generics/2.0" xmlns:dem="http://www.opengis.net/citygml/relief/2.0" xmlns:luse="http://www.opengis.net/citygml/landuse/2.0" xmlns:uro="https://www.geospatial.jp/iur/uro/3.0" xmlns:xAL="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:smil20="http://www.w3.org/2001/SMIL20/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:smil20lang="http://www.w3.org/2001/SMIL20/Language" xmlns:pbase="http://www.opengis.net/citygml/profiles/base/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:grp="http://www.opengis.net/citygml/cityobjectgroup/2.0" xsi:schemaLocation="https://www.geospatial.jp/iur/uro/3.0 ../../schemas/iur/uro/3.0/urbanObject.xsd 
http://www.opengis.net/citygml/2.0 http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd
http://www.opengis.net/citygml/landuse/2.0 http://schemas.opengis.net/citygml/landuse/2.0/landUse.xsd 
http://www.opengis.net/citygml/building/2.0 http://schemas.opengis.net/citygml/building/2.0/building.xsd 
http://www.opengis.net/citygml/transportation/2.0 http://schemas.opengis.net/citygml/transportation/2.0/transportation.xsd 
http://www.opengis.net/citygml/generics/2.0 http://schemas.opengis.net/citygml/generics/2.0/generics.xsd 
http://www.opengis.net/citygml/relief/2.0 http://schemas.opengis.net/citygml/relief/2.0/relief.xsd 
http://www.opengis.net/citygml/cityobjectgroup/2.0 http://schemas.opengis.net/citygml/cityobjectgroup/2.0/cityObjectGroup.xsd 
http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/base/gml.xsd
http://www.opengis.net/citygml/appearance/2.0 http://schemas.opengis.net/citygml/appearance/2.0/appearance.xsd ">
	<gml:boundedBy>
		<gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
			<gml:lowerCorner>35.76581808130216 139.7255469249859 0</gml:lowerCorner>
			<gml:upperCorner>35.766448253443734 139.72610160416477 7.612128257751465</gml:upperCorner>
		</gml:Envelope>
	</gml:boundedBy>
	<core:cityObjectMember>
		<tran:Railway gml:id="rwy_58634d44-b0bd-4b22-bb6b-fafadda9203f">
			<gml:name>東北線</gml:name>
			<uro:tranDataQualityAttribute>
				<uro:TransportationDataQualityAttribute>
					<uro:lodType>3.1</uro:lodType>
				</uro:TransportationDataQualityAttribute>
			</uro:tranDataQualityAttribute>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_8dc05654-5ce4-45f1-b3f7-528f80d61b4f">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:lod2Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.766050514594376 139.72591466856227 0 35.766037909088 139.72592283740298 0 35.76586928350922 139.72603211281458 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76605051483487 139.7259146683725 0 35.766038428107684 139.72592420672905 0 35.765957060839554 139.7259895276275 0 35.76587779619816 139.72605445506602 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.765845941887484 139.7259708433709 0 35.76599576191852 139.72588491330293 0 35.76602563606087 139.7258672078964 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod2Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_56075ea9-0b42-4463-ac8c-bf9a687a3908">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:startPost>14kｍ</uro:startPost>
							<uro:endPost>14kｍ</uro:endPost>
							<uro:alignmentType codeSpace="../../codelists/RailwayTrackAttribute_alignmentType.xml">1</uro:alignmentType>
							<uro:lod3Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.765877795994186 139.72605445328642 7.612034320831299 35.765957064664924 139.72598952374264 7.612034320831299 35.76603842790418 139.7259242024445 7.612034320831299 35.76605051379383 139.72591467116425 7.612034320831299</gml:posList>
										</gml:LineString>
									</gml:curveMember>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76605049839714 139.7259146794539 7.612034320831299 35.76603791615442 139.72592284229947 7.612034320831299 35.765869290615896 139.72603210821214 7.612034320831299</gml:posList>
										</gml:LineString>
									</gml:curveMember>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76602563121837 139.72586720507465 7.612034320831299 35.765995754327044 139.72588491448096 7.612034320831299 35.765845944984484 139.72597084822718 7.612034320831299</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod3Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_1b25ac92-eb7a-4f1d-b11e-c9afceb761ce">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8112</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_638ef119-162a-4b13-8dfd-28ad9fcfbd83">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602017595003 139.72594694583407 7.612128257751465 35.76601986441119 139.72594615445882 7.612127780914307 35.766015545556264 139.72594961449033 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5590a8f4-9e37-47dc-8edc-7e68b7b89206">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595459162233 139.72598474592937 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465 35.765949811899546 139.72598776359618 7.612128257751465 35.76595459162233 139.72598474592937 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ee1af204-cfcc-4f54-8f8d-5bc0fdd3cf1c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766015545556264 139.72594961449033 7.612127780914307 35.765959516705664 139.72599450206403 7.612127780914307 35.76595987773704 139.7259952707594 7.612128257751465 35.766015545556264 139.72594961449033 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_884f1718-f1bd-4a9e-b443-ec805ee37822">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765959516705664 139.72599450206403 7.612127780914307 35.76587982691845 139.726059765459 7.612127780914307 35.76588013434369 139.7260605716892 7.612128257751465 35.765959516705664 139.72599450206403 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c24d9415-5c35-45f0-938b-eed7df9775e5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765875362512254 139.72604807479348 7.612127780914307 35.76594869920914 139.72598866488698 7.612128257751465 35.76594277831759 139.72599242052854 7.612128257751465 35.765875362512254 139.72604807479348 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c168ab1f-e13d-47c4-a911-3be3b780f847">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595459162233 139.72598474592937 7.612128257751465 35.76601867670379 139.72594313738276 7.612127780914307 35.766018350828624 139.7259423095759 7.612127780914307 35.76595459162233 139.72598474592937 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_17251242-e3e6-4583-9bac-e02390c1dbce">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765948348365505 139.7259879074846 7.612128257751465 35.76594869920914 139.72598866488698 7.612128257751465 35.765949811899546 139.72598776359618 7.612128257751465 35.765948348365505 139.7259879074846 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_dceb2ae5-3944-4522-9d69-920b44f8a903">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599799965771 139.72589220551572 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307 35.765926940255916 139.72593303943722 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_612f5e86-a8ce-44c3-b7ef-25461379ec0b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599769826153 139.7258913753138 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_97ed2bd9-5f0c-4d13-a934-3d1d7bcac234">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765926940255916 139.72593303943722 7.612127780914307 35.765926524475226 139.72593226018458 7.612127780914307 35.76584866137495 139.72597798825817 7.612127304077148 35.765926940255916 139.72593303943722 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2f3428f1-24d5-4a67-b0f1-43f1c9657075">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766016833694295 139.72587997370158 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_36598a4b-7381-40ad-8318-36616993670e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765993651996254 139.72587973672935 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_26ed37b0-9e3f-4906-9e10-04a0ee3175af">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599327367701 139.72587870767003 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_84255c00-1b4d-44c3-9040-febb887ed200">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765920799702116 139.72592152619183 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307 35.76584395317774 139.72596560452052 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_26f1867d-6001-43db-ad53-bf51d69c6d1a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601114007189 139.72586839673218 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4168177e-b4a0-42d0-93f5-5cd26e8906cd">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599769826153 139.7258913753138 7.612127780914307 35.765926524475226 139.72593226018458 7.612127780914307 35.765926940255916 139.72593303943722 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2e954dd3-d479-466c-8c65-32947c21ed48">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599799965771 139.72589220551572 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b82f7b5e-ee78-4e93-81fc-35765f529a78">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765926524475226 139.72593226018458 7.612127780914307 35.765848347057585 139.7259771655 7.612127780914307 35.76584866137495 139.72597798825817 7.612127304077148 35.765926524475226 139.72593226018458 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_25f98d1e-7eeb-4d0d-aeba-a98fb77e06ce">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601729279935 139.72588090696902 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_455ce7fa-73d5-4fb9-941a-6c0b3df0680d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599327367701 139.72587870767003 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_6b880df3-6aae-4cb6-85dd-45bfb5a39dc1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765993651996254 139.72587973672935 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c041fb14-92e9-48ec-b26f-dfa6737a3165">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765920394942334 139.72592076717493 7.612127780914307 35.7658436636727 139.72596484433674 7.612127780914307 35.76584395317774 139.72596560452052 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f90f490c-b374-44d7-9a77-2eed5ee23a56">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601151209596 139.7258691530936 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_16c63135-e0ec-4d73-95c6-65f42dbb8a7b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765948348365505 139.7259879074846 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465 35.76595392966078 139.72598336033664 7.612128257751465 35.765948348365505 139.7259879074846 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_3f5066cc-57d0-4635-8ee2-95e509d3e0f0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594869920914 139.72598866488698 7.612128257751465 35.765948348365505 139.7259879074846 7.612128257751465 35.76594277831759 139.72599242052854 7.612128257751465 35.76594869920914 139.72598866488698 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a8d032ae-efeb-4564-9d96-56870edc5347">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594277831759 139.72599242052854 7.612128257751465 35.765948348365505 139.7259879074846 7.612128257751465 35.76587111011812 139.72603692470028 7.612127780914307 35.76594277831759 139.72599242052854 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_3045136a-1ca0-476e-a003-f42695343e5d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76586707468197 139.72602630892868 7.612127780914307 35.765867369147564 139.7260270799065 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307 35.76586707468197 139.72602630892868 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e0a6811f-3485-4bd0-b599-2ee6e397d0ae">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047407895485 139.72590876185856 7.612127780914307 35.76604015440597 139.725913583972 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b56c6c65-37ba-4d1a-9f30-57d2247234a8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76601503004474 139.7259338741436 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_42cf4300-6e15-4e8c-afab-6cb6a02ade2d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7659542854752 139.72598413950166 7.612128257751465 35.76595459162233 139.72598474592937 7.612128257751465 35.766018350828624 139.7259423095759 7.612127780914307 35.7659542854752 139.72598413950166 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_af908fa8-711f-4323-886f-f6f9aa4acc03">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7659542854752 139.72598413950166 7.612128257751465 35.76601545572314 139.72593495595981 7.612128257751465 35.76595392966078 139.72598336033664 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_3e9c79ae-0e79-47bb-b95f-df15aaf61747">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765949811899546 139.72598776359618 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465 35.765948348365505 139.7259879074846 7.612128257751465 35.765949811899546 139.72598776359618 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_aea29471-f9da-4e02-b449-cacfec8d8a60">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76587111011812 139.72603692470028 7.612127780914307 35.76587140375753 139.72603769466676 7.612128257751465 35.76594277831759 139.72599242052854 7.612128257751465 35.76587111011812 139.72603692470028 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_49cba00b-8a97-43a8-9079-c0350988002c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765867369147564 139.7260270799065 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307 35.765867369147564 139.7260270799065 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b4736d45-de5a-4b3b-8690-55a356cab939">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604015440597 139.725913583972 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148 35.76604015440597 139.725913583972 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b60f4514-7dba-4a33-924d-fa86ae058101">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604015440597 139.725913583972 7.612127780914307 35.76603534808656 139.7259176192692 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307 35.76604015440597 139.725913583972 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_3bdab996-1abd-48c7-8088-26af7dd317dc">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76603534808656 139.7259176192692 7.612127780914307 35.76601503004474 139.7259338741436 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_48152d19-d62c-46ea-a56e-b0555c480a2d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601545572314 139.72593495595981 7.612128257751465 35.76601503004474 139.7259338741436 7.612127780914307 35.76595392966078 139.72598336033664 7.612128257751465 35.76601545572314 139.72593495595981 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0a6a851e-04fc-42df-8f78-a720743dedc1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765948872132554 139.72597252958772 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307 35.766013526387596 139.72593005523595 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_6664d42b-c19e-49ca-b183-a29bc97cad08">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766013526387596 139.72593005523595 7.612127780914307 35.76601390988379 139.72593102944376 7.612127780914307 35.76603462002231 139.72591613909347 7.612128257751465 35.766013526387596 139.72593005523595 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_26e0a2da-4e45-4376-a429-a22d87bcda3e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603462002231 139.72591613909347 7.612128257751465 35.766035036688905 139.725916986185 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148 35.76603462002231 139.72591613909347 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_90db05c7-b8e8-4f6c-9309-63248ee1c03d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594956287049 139.72597379600134 7.612127780914307 35.76601390988379 139.72593102944376 7.612127780914307 35.766013526387596 139.72593005523595 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5f5a8a2d-44fd-4a07-9680-cf8e24b55d74">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601390988379 139.72593102944376 7.612127780914307 35.766035036688905 139.725916986185 7.612127780914307 35.76603462002231 139.72591613909347 7.612128257751465 35.76601390988379 139.72593102944376 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_451fc3cc-c643-4d98-b831-57b5814e0ff0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047038942965 139.7259080564562 7.612127304077148 35.766035036688905 139.725916986185 7.612127780914307 35.76604015440597 139.725913583972 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fce23448-8e3d-4b51-a234-1bbb24aa346c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605345053108 139.72592028250926 7.612127304077148 35.766053068080346 139.72591955501815 7.612127780914307 35.76604141427912 139.72592995378818 7.612127780914307 35.76605345053108 139.72592028250926 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fa88bff4-d58d-4483-a26e-07a4a2865209">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604141427912 139.72592995378818 7.612127780914307 35.766041039224056 139.72592919118634 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465 35.76604141427912 139.72592995378818 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5222d023-a797-4808-ab9e-32b8c5f61a39">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604628963416 139.72592498535712 7.612127304077148 35.766053068080346 139.72591955501815 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.76604628963416 139.72592498535712 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8939a649-689b-438e-a6dd-0c2102301baf">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604075896585 139.7259286210055 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.76601867670379 139.72594313738276 7.612127780914307 35.76604075896585 139.7259286210055 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d5feaf9a-acb2-4abb-8f61-28456f6252ad">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766053068080346 139.72591955501815 7.612127780914307 35.76604628963416 139.72592498535712 7.612127304077148 35.76604141427912 139.72592995378818 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5f6483f1-49e7-4168-9db7-3289c818bbd6">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766041039224056 139.72592919118634 7.612127780914307 35.76601986441119 139.72594615445882 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465 35.766041039224056 139.72592919118634 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_61b690e3-c577-46b8-bb11-0d7b655de1e1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604033320552 139.72592775553184 7.612127780914307 35.76604075896585 139.7259286210055 7.612127780914307 35.76604628963416 139.72592498535712 7.612127304077148 35.76604033320552 139.72592775553184 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8ba8faab-4a62-4a09-9c87-0f30dc6c456d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604033320552 139.72592775553184 7.612127780914307 35.766018350828624 139.7259423095759 7.612127780914307 35.76601867670379 139.72594313738276 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b6477cb6-a382-4b65-88ba-00fe07aeccba">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604628963416 139.72592498535712 7.612127304077148 35.766041039224056 139.72592919118634 7.612127780914307 35.76604141427912 139.72592995378818 7.612127780914307 35.76604628963416 139.72592498535712 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5e6ef2e6-0d90-4f0d-9dac-61ebe7974075">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602017595003 139.72594694583407 7.612128257751465 35.766015545556264 139.72594961449033 7.612127780914307 35.76595987773704 139.7259952707594 7.612128257751465 35.76602017595003 139.72594694583407 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_defbf2cc-c1b8-4870-aeb3-154a6cfadf3c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595987773704 139.7259952707594 7.612128257751465 35.765959516705664 139.72599450206403 7.612127780914307 35.76588013434369 139.7260605716892 7.612128257751465 35.76595987773704 139.7259952707594 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f35b3eee-172a-4d13-bbf5-b32910885328">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76587506197984 139.72604728661076 7.612127780914307 35.765875362512254 139.72604807479348 7.612127780914307 35.76594277831759 139.72599242052854 7.612128257751465 35.76587506197984 139.72604728661076 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_06d202e2-561d-46de-b100-16c4d4f6c623">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c88cbbe3-af01-43c9-9a74-f62df2ccf303">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602306348567 139.72586230123608 0 35.765993736187454 139.7258796830282 0 35.76584394311207 139.7259655973716 0 35.76584835026486 139.72597716515497 0 35.76599768106859 139.7258913819518 0 35.76602863797468 139.72587293515647 0 35.76602306348567 139.72586230123608 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_67326868-06d5-47f5-a8e8-56565ca2194c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595440034623 139.72598404651484 0 35.765875364154795 139.72604807188196 0 35.76587982008738 139.7260597678985 0 35.765959666278 139.72599436454868 0 35.76604022978822 139.72592983048384 0 35.766053070371 139.725919544504 0 35.766047414624666 139.7259087547478 0 35.766036090930335 139.72591792057707 0 35.76595440034623 139.72598404651484 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a246997b-2ccc-46d0-b2ae-4589baf39024">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595440034623 139.72598404651484 0 35.76604005333885 139.72592805457043 0 35.766053070371 139.725919544504 0 35.766047414624666 139.7259087547478 0 35.76603491569308 139.72591706173057 0 35.76594989592051 139.72597357041477 0 35.7658673666031 139.72602708073612 0 35.76587111421501 139.72603691860883 0 35.76595440034623 139.72598404651484 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_20640fcd-23db-4ea8-b190-c24c404fdb21">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f9a96895-b672-403e-83e6-f81dba2259e3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604450207594 139.72593623201504 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465 35.76602298393437 139.72595407833157 7.612127780914307 35.76604450207594 139.72593623201504 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c8d84c43-0d0c-428a-b715-b26705e54240">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602298393437 139.72595407833157 7.612127780914307 35.766045227660086 139.72593770746937 7.452127933502197 35.76604450207594 139.72593623201504 7.612127780914307 35.76602298393437 139.72595407833157 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_350d2072-f12d-4341-ac6a-fd12b9d51770">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604141427912 139.72592995378818 7.612127780914307 35.76605687274914 139.7259267928303 7.612127780914307 35.76605345053108 139.72592028250926 7.612127304077148 35.76604141427912 139.72592995378818 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e95edc07-f970-43eb-a03d-610ac4359314">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595987773704 139.7259952707594 7.612128257751465 35.76602298393437 139.72595407833157 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465 35.76595987773704 139.7259952707594 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_26db8ef1-644b-4e06-8b7b-50d49282fb42">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76596330367154 139.72600255953296 7.612127780914307 35.76588013434369 139.7260605716892 7.612128257751465 35.765882638134144 139.7260671358864 7.612127780914307 35.76596330367154 139.72600255953296 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_feef33f0-0230-46eb-82fd-0b928e0845ea">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602298393437 139.72595407833157 7.612127780914307 35.76602351796133 139.72595543473398 7.452127933502197 35.766045227660086 139.72593770746937 7.452127933502197 35.76602298393437 139.72595407833157 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a29471a5-6940-40ac-a0c6-9652522e5556">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604450207594 139.72593623201504 7.612127780914307 35.766045227660086 139.72593770746937 7.452127933502197 35.76605687274914 139.7259267928303 7.612127780914307 35.76604450207594 139.72593623201504 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7afdafd7-843a-40a1-bc99-033428d1601c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76596330367154 139.72600255953296 7.612127780914307 35.765963997622435 139.7260040360414 7.452127933502197 35.76602298393437 139.72595407833157 7.612127780914307 35.76596330367154 139.72600255953296 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f8e08bb6-1d70-44ea-8cff-129b6d888317">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765882638134144 139.7260671358864 7.612127780914307 35.76588291467883 139.72606786098757 7.452127933502197 35.76596330367154 139.72600255953296 7.612127780914307 35.765882638134144 139.7260671358864 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_cdeb4233-1440-40fd-9bd2-85d0f872891f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765963997622435 139.7260040360414 7.452127933502197 35.76602351796133 139.72595543473398 7.452127933502197 35.76602298393437 139.72595407833157 7.612127780914307 35.765963997622435 139.7260040360414 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b04467b3-094d-4668-92c2-56b73726db56">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766045227660086 139.72593770746937 7.452127933502197 35.76605738993917 139.72592777647787 7.452127933502197 35.76605687274914 139.7259267928303 7.612127780914307 35.766045227660086 139.72593770746937 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_31540292-f7ba-417e-ae54-265c9c717518">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76588291467883 139.72606786098757 7.452127933502197 35.765963997622435 139.7260040360414 7.452127933502197 35.76596330367154 139.72600255953296 7.612127780914307 35.76588291467883 139.72606786098757 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ae165a7f-7d8a-4189-a1c7-d0e0307abcc3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604450207594 139.72593623201504 7.612127780914307 35.76604141427912 139.72592995378818 7.612127780914307 35.76602017595003 139.72594694583407 7.612128257751465 35.76604450207594 139.72593623201504 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4cc673be-3f6e-4115-812a-476688da1372">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604141427912 139.72592995378818 7.612127780914307 35.76604450207594 139.72593623201504 7.612127780914307 35.76605687274914 139.7259267928303 7.612127780914307 35.76604141427912 139.72592995378818 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_73461e3a-519e-4b61-9932-800c88b38bd1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595987773704 139.7259952707594 7.612128257751465 35.76596330367154 139.72600255953296 7.612127780914307 35.76602298393437 139.72595407833157 7.612127780914307 35.76595987773704 139.7259952707594 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1d287ff7-3b54-4a90-ae83-18aecb33fadb">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76596330367154 139.72600255953296 7.612127780914307 35.76595987773704 139.7259952707594 7.612128257751465 35.76588013434369 139.7260605716892 7.612128257751465 35.76596330367154 139.72600255953296 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b20bd2e9-21c6-46e0-a0b5-8a82cffeb24d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602268594188 139.72586158311353 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4ae7e4c2-2221-4ac8-8e6f-06778265c024">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601114007189 139.72586839673218 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e222350e-588d-443d-b144-518861961e1f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599327367701 139.72587870767003 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_09eb626b-95cb-4ad4-a659-6be94d2fbf30">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765920394942334 139.72592076717493 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307 35.7658436636727 139.72596484433674 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_6c9baa2b-e691-462b-897c-69131d285e0c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766018067851576 139.72585277563914 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307 35.76601114007189 139.72586839673218 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_346d3147-0cc8-4c79-8e82-1bf0ed6f9cd8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600660799114 139.72585918210825 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307 35.76599327367701 139.72587870767003 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0042c2f6-aa3b-4798-aac3-76ffee2be730">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76598959113399 139.7258686948798 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307 35.765920394942334 139.72592076717493 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5208536c-4e86-48c7-b129-a0a3fcc3883a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765914891147396 139.72591044835866 7.612127780914307 35.76583908205497 139.7259528217321 7.612127780914307 35.7658436636727 139.72596484433674 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9ffb9938-96a5-46bb-8c5a-be95fef77dfc">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765945139060165 139.7259656837022 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fc0ab26f-02bf-4289-ba55-e6ae8bdc5e9e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765948872132554 139.72597252958772 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307 35.76586707468197 139.72602630892868 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_036598cd-1b54-4bc4-97eb-fc581f7d9295">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603462002231 139.72591613909347 7.612128257751465 35.76602985646307 139.72590645310774 7.612127780914307 35.766013526387596 139.72593005523595 7.612127780914307 35.76603462002231 139.72591613909347 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d88742e9-980f-4cea-b088-666226ab92fa">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047038942965 139.7259080564562 7.612127304077148 35.7660418207068 139.72589808386138 7.612127780914307 35.76603462002231 139.72591613909347 7.612128257751465 35.766047038942965 139.7259080564562 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4da13acd-0459-4997-9a38-f470a619f6c0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76587140375753 139.72603769466676 7.612128257751465 35.76587506197984 139.72604728661076 7.612127780914307 35.76594277831759 139.72599242052854 7.612128257751465 35.76587140375753 139.72603769466676 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d31706b7-a11f-4156-a734-21ee0c5b0300">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76584866137495 139.72597798825817 7.612127304077148 35.765852281271016 139.7259874727521 7.612127780914307 35.765926940255916 139.72593303943722 7.612127780914307 35.76584866137495 139.72597798825817 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1e7233fc-b5c7-43ea-b1e5-e4eca3f5b9a1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765926940255916 139.72593303943722 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307 35.765926940255916 139.72593303943722 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0309f50e-eaba-4b55-8e8d-8b6033d1cc55">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599799965771 139.72589220551572 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_78b17bf0-1424-42b9-aab5-079a8c569f45">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765948872132554 139.72597252958772 7.612127780914307 35.766013526387596 139.72593005523595 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307 35.765948872132554 139.72597252958772 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_96777cb7-7689-49db-adc1-d999c4c7fd5a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765945139060165 139.7259656837022 7.612127780914307 35.765864015880894 139.72601830246643 7.612127780914307 35.76586707468197 139.72602630892868 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_baa452ee-953d-49d5-a8be-59601c143790">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602985646307 139.72590645310774 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307 35.766013526387596 139.72593005523595 7.612127780914307 35.76602985646307 139.72590645310774 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b929717a-4923-43cc-91ee-85a8552424d0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7660418207068 139.72589808386138 7.612127780914307 35.76602985646307 139.72590645310774 7.612127780914307 35.76603462002231 139.72591613909347 7.612128257751465 35.7660418207068 139.72589808386138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d0a5e108-59dc-413d-b847-f37acd67afbe">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765852281271016 139.7259874727521 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307 35.765926940255916 139.72593303943722 7.612127780914307 35.765852281271016 139.7259874727521 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fdb514b5-e0d5-4c94-bd45-3a978931243f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76593125016027 139.7259411203288 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307 35.76599799965771 139.72589220551572 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0205e42f-3935-4116-aa56-a86549fd5b2a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600090773012 139.7259002152268 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2b9feceb-db69-465c-856d-6b698628989a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601729279935 139.72588090696902 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307 35.76601729279935 139.72588090696902 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_283ecf4b-b005-4a96-b884-8e4b1dc3e268">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766020794785675 139.72588802756158 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_c03818f2-7468-4b1d-bc61-4dc4fa2c0202">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d921164d-c1c7-4691-aee6-6344421c619f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599769826153 139.7258913753138 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307 35.765926524475226 139.72593226018458 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b603ba65-5418-4dc7-b3a9-d74c3422f0cf">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599769826153 139.7258913753138 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9b38b960-3d6f-4dd2-a425-70b1706b661e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765926524475226 139.72593226018458 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307 35.765848347057585 139.7259771655 7.612127780914307 35.765926524475226 139.72593226018458 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_46bfb0d1-25b7-4d17-99a9-3052880bff91">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601151209596 139.7258691530936 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d9db6f58-03b0-4afc-b06c-e23a6be38c9a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765993651996254 139.72587973672935 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307 35.765926524475226 139.72593226018458 7.612127780914307 35.765993651996254 139.72587973672935 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d6171d4a-2773-436d-b0f2-10c8c4511c1b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76599769826153 139.7258913753138 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307 35.76601151209596 139.7258691530936 7.612127780914307 35.76599769826153 139.7258913753138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_cc6f81a4-6e86-457d-be41-e61b31bf5897">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765920799702116 139.72592152619183 7.612127780914307 35.76584395317774 139.72596560452052 7.612127780914307 35.765848347057585 139.7259771655 7.612127780914307 35.765920799702116 139.72592152619183 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b3f81ec6-0be0-4272-9c95-6619467145c6">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766016833694295 139.72587997370158 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307 35.766016833694295 139.72587997370158 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_624a9750-b34a-427f-8af8-0498a0543391">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465 35.76604033320552 139.72592775553184 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0d471971-975f-4c91-b758-594a62fe2fc1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465 35.76604033320552 139.72592775553184 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_dedde593-c1bd-454f-9040-42394d6519d4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601545572314 139.72593495595981 7.612128257751465 35.766018350828624 139.7259423095759 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d9127bbc-bfa3-4b83-942d-00542bde6039">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601545572314 139.72593495595981 7.612128257751465 35.766018350828624 139.7259423095759 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a148569d-0ff0-488e-adef-831a4584815e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ef32256c-598d-40bf-b9a7-e3d56bcbefcd">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603573030639 139.7259183962054 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307 35.76603573030639 139.7259183962054 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b0e7c20d-5d11-48b7-bca4-d3c322a3ea5f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604033320552 139.72592775553184 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_06072aee-6c29-4c4c-8a4d-23ba3ce753b3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604033320552 139.72592775553184 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307 35.76604033320552 139.72592775553184 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_dff39d93-c084-461d-84bc-2ef0aa08876a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595392966078 139.72598336033664 7.612128257751465 35.76594956287049 139.72597379600134 7.612127780914307 35.765948348365505 139.7259879074846 7.612128257751465 35.76595392966078 139.72598336033664 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_96b85c05-355a-48bf-ac22-ffd464c56ec9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604075896585 139.7259286210055 7.612127780914307 35.766041039224056 139.72592919118634 7.612127780914307 35.76604628963416 139.72592498535712 7.612127304077148 35.76604075896585 139.7259286210055 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_86996e71-f246-4dfb-ac3a-64ba2ba6a50a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765959516705664 139.72599450206403 7.612127780914307 35.765875362512254 139.72604807479348 7.612127780914307 35.76587982691845 139.726059765459 7.612127780914307 35.765959516705664 139.72599450206403 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7c892b99-3639-4386-8533-2b7ffcbe9627">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76604015440597 139.725913583972 7.612127780914307 35.766035036688905 139.725916986185 7.612127780914307 35.76603534808656 139.7259176192692 7.612127780914307 35.76604015440597 139.725913583972 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4c14bda4-a71a-4884-afb4-6d6f82f36f9e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601390988379 139.72593102944376 7.612127780914307 35.76603534808656 139.7259176192692 7.612127780914307 35.766035036688905 139.725916986185 7.612127780914307 35.76601390988379 139.72593102944376 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e0176d5b-130e-4043-8178-06e7b6461834">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595392966078 139.72598336033664 7.612128257751465 35.76601503004474 139.7259338741436 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307 35.76595392966078 139.72598336033664 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0cc32861-16bc-45dc-a8c5-9f0642f06790">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601867670379 139.72594313738276 7.612127780914307 35.766041039224056 139.72592919118634 7.612127780914307 35.76604075896585 139.7259286210055 7.612127780914307 35.76601867670379 139.72594313738276 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_22a08578-a0bc-4d3c-8013-cb0b7463e3d8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7659542854752 139.72598413950166 7.612128257751465 35.766018350828624 139.7259423095759 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_025113eb-14f3-4b1b-9d63-9f320a918262">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7659542854752 139.72598413950166 7.612128257751465 35.766018350828624 139.7259423095759 7.612127780914307 35.76601545572314 139.72593495595981 7.612128257751465 35.7659542854752 139.72598413950166 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7f49c319-17a4-4e19-bedc-900e9de3434e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594956287049 139.72597379600134 7.612127780914307 35.765867369147564 139.7260270799065 7.612127780914307 35.76587111011812 139.72603692470028 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b265c72e-12d1-4e4f-a38d-ae79eed7ea7f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594956287049 139.72597379600134 7.612127780914307 35.76587111011812 139.72603692470028 7.612127780914307 35.765948348365505 139.7259879074846 7.612128257751465 35.76594956287049 139.72597379600134 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2ec60d38-c874-4c22-b566-1b31e51a52b6">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595459162233 139.72598474592937 7.612128257751465 35.765949811899546 139.72598776359618 7.612128257751465 35.76594869920914 139.72598866488698 7.612128257751465 35.76595459162233 139.72598474592937 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2a4948ca-64a0-475a-915c-36a72e6a6c35">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601390988379 139.72593102944376 7.612127780914307 35.76601503004474 139.7259338741436 7.612127780914307 35.76603534808656 139.7259176192692 7.612127780914307 35.76601390988379 139.72593102944376 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_28d24b10-c837-48e7-8548-bf45dba38da8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601503004474 139.7259338741436 7.612127780914307 35.76601390988379 139.72593102944376 7.612127780914307 35.76594956287049 139.72597379600134 7.612127780914307 35.76601503004474 139.7259338741436 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8cdca25b-5f4e-4390-bb5d-e0d257233f67">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601986441119 139.72594615445882 7.612127780914307 35.76601867670379 139.72594313738276 7.612127780914307 35.76595459162233 139.72598474592937 7.612128257751465 35.76601986441119 139.72594615445882 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_25dfc33d-5102-40b2-944d-171a29b0c33a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595459162233 139.72598474592937 7.612128257751465 35.765959516705664 139.72599450206403 7.612127780914307 35.766015545556264 139.72594961449033 7.612127780914307 35.76595459162233 139.72598474592937 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1be15f30-c922-40d4-9046-928f4a968cc5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76601867670379 139.72594313738276 7.612127780914307 35.76601986441119 139.72594615445882 7.612127780914307 35.766041039224056 139.72592919118634 7.612127780914307 35.76601867670379 139.72594313738276 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_31379833-5a79-47bf-a116-684cd30aa8b2">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594869920914 139.72598866488698 7.612128257751465 35.765875362512254 139.72604807479348 7.612127780914307 35.765959516705664 139.72599450206403 7.612127780914307 35.76594869920914 139.72598866488698 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f7d70d2b-a747-48e6-b9a7-69b97b1af437">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76594869920914 139.72598866488698 7.612128257751465 35.765959516705664 139.72599450206403 7.612127780914307 35.76595459162233 139.72598474592937 7.612128257751465 35.76594869920914 139.72598866488698 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_6431cde2-fa51-43c1-82b6-6574ecab9e03">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76595459162233 139.72598474592937 7.612128257751465 35.766015545556264 139.72594961449033 7.612127780914307 35.76601986441119 139.72594615445882 7.612127780914307 35.76595459162233 139.72598474592937 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_23f33c9a-bb91-4d17-bad7-a085a411b7de">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_bcbe82da-a2d7-474a-8305-f67c855cce38">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76584394311207 139.7259655973716 0 35.765993736187454 139.7258796830282 0 35.76602306348567 139.72586230123608 0 35.76601807046018 139.72585277469332 0 35.76598977201877 139.7258685924021 0 35.76583907608801 139.72595282141071 0 35.76584394311207 139.7259655973716 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_069e3687-ffac-47fb-8f40-7a5109ad0aa8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76585227947714 139.72598747722907 0 35.766000208643135 139.72590064859455 0 35.76603271907039 139.72588071995614 0 35.76602863797468 139.72587293515647 0 35.76599768106958 139.72589138305776 0 35.76584835026486 139.72597716515497 0 35.76585227947714 139.72598747722907 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b9fa9c9d-e6cf-40f5-b083-175306fb4ed9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765875364154795 139.72604807188196 0 35.76595440034623 139.72598404651484 0 35.76587111421501 139.72603691860883 0 35.765875364154795 139.72604807188196 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_04cdd52d-a818-4a65-ba39-6e06582dfbe6">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7658673666031 139.72602708073612 0 35.76594989592051 139.72597357041477 0 35.766047414624666 139.7259087547478 0 35.766041818468715 139.72589807882554 0 35.76594571615039 139.7259653013469 0 35.76586402103683 139.7260182985012 0 35.7658673666031 139.72602708073612 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c311c76f-66fe-4a0a-ad0d-3a01b9f0a122">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76588290454582 139.72606786369582 0 35.76596343758938 139.72600449547502 0 35.76605738260426 139.72592776916463 0 35.766053071272374 139.72591954450277 0 35.76604022978822 139.72592983048384 0 35.765959666278 139.72599436454868 0 35.76587982008738 139.7260597678985 0 35.76588290454582 139.72606786369582 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:auxiliaryTrafficArea>
				<tran:AuxiliaryTrafficArea gml:id="traf_472fcd19-a6d5-42cb-ae20-b3821fcae9ef">
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a0178d2d-beae-412f-af21-a49bdae29a1b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765914891147396 139.72591044835866 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307 35.76589389037458 139.72585310570778 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d269b71e-9e76-4bb7-ad4c-4f949a852b7f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76598959113399 139.7258686948798 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307 35.765968590341465 139.7258113521864 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a787cbea-ef19-4b78-aa14-956624f9276f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76583908205497 139.7259528217321 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307 35.76581808130216 139.72589547912443 7.612127780914307 35.76583908205497 139.7259528217321 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ea667b87-73ec-4dc1-842c-46412f4476ca">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766045227660086 139.72593770746937 7.452127933502197 35.76602351796133 139.72595543473398 7.452127933502197 35.76605779600523 139.7259755995033 7.452127933502197 35.766045227660086 139.72593770746937 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c396e041-2d42-4c43-bb25-1d99c7c4ac0c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600660799114 139.72585918210825 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307 35.76598560719411 139.72580183940516 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f579d015-0ad4-4e44-84d8-3dd3d878a2a4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602351796133 139.72595543473398 7.452127933502197 35.765963997622435 139.7260040360414 7.452127933502197 35.766036086300915 139.72599332676043 7.452127933502197 35.76602351796133 139.72595543473398 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_539817bf-d90f-41b2-a774-5d3f708b8d36">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605738993917 139.72592777647787 7.452127933502197 35.766045227660086 139.72593770746937 7.452127933502197 35.76607076901669 139.7259666934518 7.452127933502197 35.76605738993917 139.72592777647787 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2da770cf-3748-4fb9-bd87-cd1193f31e88">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765963997622435 139.7260040360414 7.452127933502197 35.76588291467883 139.72606786098757 7.452127933502197 35.76598096131638 139.726035431192 7.452127933502197 35.765963997622435 139.7260040360414 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9a6fd492-9307-4ec1-92ee-6a788b8b969f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76598959113399 139.7258686948798 7.612127780914307 35.765968590341465 139.7258113521864 7.612127780914307 35.76589389037458 139.72585310570778 7.612127780914307 35.76598959113399 139.7258686948798 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_326ef9af-6a7b-4c01-b42a-93ebeee0d348">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600660799114 139.72585918210825 7.612127780914307 35.76598560719411 139.72580183940516 7.612127780914307 35.765968590341465 139.7258113521864 7.612127780914307 35.76600660799114 139.72585918210825 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0a3267b2-4273-47f4-9ec5-d14334ff27ca">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765914891147396 139.72591044835866 7.612127780914307 35.76589389037458 139.72585310570778 7.612127780914307 35.76581808130216 139.72589547912443 7.612127780914307 35.765914891147396 139.72591044835866 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_da400f99-9223-4e83-87f3-4fc1654df4ce">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602351796133 139.72595543473398 7.452127933502197 35.766036086300915 139.72599332676043 7.452127933502197 35.76605779600523 139.7259755995033 7.452127933502197 35.76602351796133 139.72595543473398 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2f0e9149-7bec-4116-9df9-81b09e902c34">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766018067851576 139.72585277563914 7.612127780914307 35.76599706705152 139.72579543292952 7.612127780914307 35.76598560719411 139.72580183940516 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d13860d5-76f6-48d1-85fd-af24221cbcb1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765963997622435 139.7260040360414 7.452127933502197 35.76598096131638 139.726035431192 7.452127933502197 35.766036086300915 139.72599332676043 7.452127933502197 35.765963997622435 139.7260040360414 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9f4e0118-cb97-43c5-beeb-0a67c8e6ecda">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766045227660086 139.72593770746937 7.452127933502197 35.76605779600523 139.7259755995033 7.452127933502197 35.76607076901669 139.7259666934518 7.452127933502197 35.766045227660086 139.72593770746937 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a4c62544-dced-4b35-9244-1e146a8be81b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76588291467883 139.72606786098757 7.452127933502197 35.765896605982824 139.72610160416477 7.452127933502197 35.76598096131638 139.726035431192 7.452127933502197 35.76588291467883 139.72606786098757 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7826a645-19e3-4298-8f7b-073902147a0a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600090773012 139.7259002152268 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ca6a29b5-a993-4ee1-b158-13676d706ec3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600090773012 139.7259002152268 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_96596190-2cae-4f2e-8270-9ab560329032">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765852281271016 139.7259874727521 7.612127780914307 35.765864015880894 139.72601830246643 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307 35.765852281271016 139.7259874727521 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c4373546-3f46-45ce-a162-1165269a4790">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602985646307 139.72590645310774 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307 35.76602985646307 139.72590645310774 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fe5baecf-29d7-452e-bd3d-7876e24cd48b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76600090773012 139.7259002152268 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307 35.76600090773012 139.7259002152268 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c1b7a995-2af4-4980-9625-57db02dd3c8a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766009766692385 139.7259205061119 7.612127780914307 35.76602985646307 139.72590645310774 7.612127780914307 35.766020794785675 139.72588802756158 7.612127780914307 35.766009766692385 139.7259205061119 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f490ad99-7e44-4824-bac3-77f77f1e0129">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.765864015880894 139.72601830246643 7.612127780914307 35.765945139060165 139.7259656837022 7.612127780914307 35.76593125016027 139.7259411203288 7.612127780914307 35.765864015880894 139.72601830246643 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a91ca246-fe7f-47ac-89e3-c895d9a74b17">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602985646307 139.72590645310774 7.612127780914307 35.7660418207068 139.72589808386138 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307 35.76602985646307 139.72590645310774 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:AuxiliaryTrafficArea>
			</tran:auxiliaryTrafficArea>
			<tran:lod0Network>
				<gml:CompositeCurve>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.76605051483487 139.7259146683725 0 35.766038428107684 139.72592420672905 0 35.765957060839554 139.7259895276275 0 35.76587779619816 139.72605445506602 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.765845941887484 139.7259708433709 0 35.76599576191852 139.72588491330293 0 35.76602563606087 139.7258672078964 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.766050514594376 139.72591466856227 0 35.766037909088 139.72592283740298 0 35.76586928350922 139.72603211281458 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
				</gml:CompositeCurve>
			</tran:lod0Network>
			<tran:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_ff427bd1-0b8b-4758-abbe-a6a68a810427">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76602306348567 139.72586230123608 0 35.765993736187454 139.7258796830282 0 35.76584394311207 139.7259655973716 0 35.76584835026486 139.72597716515497 0 35.76599768106859 139.7258913819518 0 35.76602863797468 139.72587293515647 0 35.76602306348567 139.72586230123608 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_d8515f0d-a003-4a42-8f35-dc4b8a27a7d0">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76595440034623 139.72598404651484 0 35.765875364154795 139.72604807188196 0 35.76587982008738 139.7260597678985 0 35.765959666278 139.72599436454868 0 35.76604022978822 139.72592983048384 0 35.766053070371 139.725919544504 0 35.766047414624666 139.7259087547478 0 35.766036090930335 139.72591792057707 0 35.76595440034623 139.72598404651484 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_7739abeb-a171-48b1-84f7-f969c5211d9e">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76595440034623 139.72598404651484 0 35.76604005333885 139.72592805457043 0 35.766053070371 139.725919544504 0 35.766047414624666 139.7259087547478 0 35.76603491569308 139.72591706173057 0 35.76594989592051 139.72597357041477 0 35.7658673666031 139.72602708073612 0 35.76587111421501 139.72603691860883 0 35.76595440034623 139.72598404651484 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod1MultiSurface>
			<tran:lod2MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_c88cbbe3-af01-43c9-9a74-f62df2ccf303"/>
							<gml:surfaceMember xlink:href="#poly_67326868-06d5-47f5-a8e8-56565ca2194c"/>
							<gml:surfaceMember xlink:href="#poly_a246997b-2ccc-46d0-b2ae-4589baf39024"/>
							<gml:surfaceMember xlink:href="#poly_bcbe82da-a2d7-474a-8305-f67c855cce38"/>
							<gml:surfaceMember xlink:href="#poly_069e3687-ffac-47fb-8f40-7a5109ad0aa8"/>
							<gml:surfaceMember xlink:href="#poly_b9fa9c9d-e6cf-40f5-b083-175306fb4ed9"/>
							<gml:surfaceMember xlink:href="#poly_04cdd52d-a818-4a65-ba39-6e06582dfbe6"/>
							<gml:surfaceMember xlink:href="#poly_c311c76f-66fe-4a0a-ad0d-3a01b9f0a122"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod2MultiSurface>
			<tran:lod3MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_638ef119-162a-4b13-8dfd-28ad9fcfbd83"/>
							<gml:surfaceMember xlink:href="#poly_5590a8f4-9e37-47dc-8edc-7e68b7b89206"/>
							<gml:surfaceMember xlink:href="#poly_ee1af204-cfcc-4f54-8f8d-5bc0fdd3cf1c"/>
							<gml:surfaceMember xlink:href="#poly_884f1718-f1bd-4a9e-b443-ec805ee37822"/>
							<gml:surfaceMember xlink:href="#poly_c24d9415-5c35-45f0-938b-eed7df9775e5"/>
							<gml:surfaceMember xlink:href="#poly_c168ab1f-e13d-47c4-a911-3be3b780f847"/>
							<gml:surfaceMember xlink:href="#poly_17251242-e3e6-4583-9bac-e02390c1dbce"/>
							<gml:surfaceMember xlink:href="#poly_f9a96895-b672-403e-83e6-f81dba2259e3"/>
							<gml:surfaceMember xlink:href="#poly_c8d84c43-0d0c-428a-b715-b26705e54240"/>
							<gml:surfaceMember xlink:href="#poly_350d2072-f12d-4341-ac6a-fd12b9d51770"/>
							<gml:surfaceMember xlink:href="#poly_e95edc07-f970-43eb-a03d-610ac4359314"/>
							<gml:surfaceMember xlink:href="#poly_26db8ef1-644b-4e06-8b7b-50d49282fb42"/>
							<gml:surfaceMember xlink:href="#poly_feef33f0-0230-46eb-82fd-0b928e0845ea"/>
							<gml:surfaceMember xlink:href="#poly_a29471a5-6940-40ac-a0c6-9652522e5556"/>
							<gml:surfaceMember xlink:href="#poly_7afdafd7-843a-40a1-bc99-033428d1601c"/>
							<gml:surfaceMember xlink:href="#poly_f8e08bb6-1d70-44ea-8cff-129b6d888317"/>
							<gml:surfaceMember xlink:href="#poly_cdeb4233-1440-40fd-9bd2-85d0f872891f"/>
							<gml:surfaceMember xlink:href="#poly_b04467b3-094d-4668-92c2-56b73726db56"/>
							<gml:surfaceMember xlink:href="#poly_31540292-f7ba-417e-ae54-265c9c717518"/>
							<gml:surfaceMember xlink:href="#poly_ae165a7f-7d8a-4189-a1c7-d0e0307abcc3"/>
							<gml:surfaceMember xlink:href="#poly_4cc673be-3f6e-4115-812a-476688da1372"/>
							<gml:surfaceMember xlink:href="#poly_73461e3a-519e-4b61-9932-800c88b38bd1"/>
							<gml:surfaceMember xlink:href="#poly_1d287ff7-3b54-4a90-ae83-18aecb33fadb"/>
							<gml:surfaceMember xlink:href="#poly_b20bd2e9-21c6-46e0-a0b5-8a82cffeb24d"/>
							<gml:surfaceMember xlink:href="#poly_4ae7e4c2-2221-4ac8-8e6f-06778265c024"/>
							<gml:surfaceMember xlink:href="#poly_e222350e-588d-443d-b144-518861961e1f"/>
							<gml:surfaceMember xlink:href="#poly_09eb626b-95cb-4ad4-a659-6be94d2fbf30"/>
							<gml:surfaceMember xlink:href="#poly_6c9baa2b-e691-462b-897c-69131d285e0c"/>
							<gml:surfaceMember xlink:href="#poly_346d3147-0cc8-4c79-8e82-1bf0ed6f9cd8"/>
							<gml:surfaceMember xlink:href="#poly_0042c2f6-aa3b-4798-aac3-76ffee2be730"/>
							<gml:surfaceMember xlink:href="#poly_5208536c-4e86-48c7-b129-a0a3fcc3883a"/>
							<gml:surfaceMember xlink:href="#poly_9ffb9938-96a5-46bb-8c5a-be95fef77dfc"/>
							<gml:surfaceMember xlink:href="#poly_fc0ab26f-02bf-4289-ba55-e6ae8bdc5e9e"/>
							<gml:surfaceMember xlink:href="#poly_036598cd-1b54-4bc4-97eb-fc581f7d9295"/>
							<gml:surfaceMember xlink:href="#poly_d88742e9-980f-4cea-b088-666226ab92fa"/>
							<gml:surfaceMember xlink:href="#poly_4da13acd-0459-4997-9a38-f470a619f6c0"/>
							<gml:surfaceMember xlink:href="#poly_d31706b7-a11f-4156-a734-21ee0c5b0300"/>
							<gml:surfaceMember xlink:href="#poly_1e7233fc-b5c7-43ea-b1e5-e4eca3f5b9a1"/>
							<gml:surfaceMember xlink:href="#poly_0309f50e-eaba-4b55-8e8d-8b6033d1cc55"/>
							<gml:surfaceMember xlink:href="#poly_78b17bf0-1424-42b9-aab5-079a8c569f45"/>
							<gml:surfaceMember xlink:href="#poly_96777cb7-7689-49db-adc1-d999c4c7fd5a"/>
							<gml:surfaceMember xlink:href="#poly_baa452ee-953d-49d5-a8be-59601c143790"/>
							<gml:surfaceMember xlink:href="#poly_b929717a-4923-43cc-91ee-85a8552424d0"/>
							<gml:surfaceMember xlink:href="#poly_d0a5e108-59dc-413d-b847-f37acd67afbe"/>
							<gml:surfaceMember xlink:href="#poly_fdb514b5-e0d5-4c94-bd45-3a978931243f"/>
							<gml:surfaceMember xlink:href="#poly_0205e42f-3935-4116-aa56-a86549fd5b2a"/>
							<gml:surfaceMember xlink:href="#poly_2b9feceb-db69-465c-856d-6b698628989a"/>
							<gml:surfaceMember xlink:href="#poly_283ecf4b-b005-4a96-b884-8e4b1dc3e268"/>
							<gml:surfaceMember xlink:href="#poly_d921164d-c1c7-4691-aee6-6344421c619f"/>
							<gml:surfaceMember xlink:href="#poly_b603ba65-5418-4dc7-b3a9-d74c3422f0cf"/>
							<gml:surfaceMember xlink:href="#poly_9b38b960-3d6f-4dd2-a425-70b1706b661e"/>
							<gml:surfaceMember xlink:href="#poly_46bfb0d1-25b7-4d17-99a9-3052880bff91"/>
							<gml:surfaceMember xlink:href="#poly_d9db6f58-03b0-4afc-b06c-e23a6be38c9a"/>
							<gml:surfaceMember xlink:href="#poly_d6171d4a-2773-436d-b0f2-10c8c4511c1b"/>
							<gml:surfaceMember xlink:href="#poly_cc6f81a4-6e86-457d-be41-e61b31bf5897"/>
							<gml:surfaceMember xlink:href="#poly_b3f81ec6-0be0-4272-9c95-6619467145c6"/>
							<gml:surfaceMember xlink:href="#poly_624a9750-b34a-427f-8af8-0498a0543391"/>
							<gml:surfaceMember xlink:href="#poly_0d471971-975f-4c91-b758-594a62fe2fc1"/>
							<gml:surfaceMember xlink:href="#poly_dedde593-c1bd-454f-9040-42394d6519d4"/>
							<gml:surfaceMember xlink:href="#poly_d9127bbc-bfa3-4b83-942d-00542bde6039"/>
							<gml:surfaceMember xlink:href="#poly_a148569d-0ff0-488e-adef-831a4584815e"/>
							<gml:surfaceMember xlink:href="#poly_ef32256c-598d-40bf-b9a7-e3d56bcbefcd"/>
							<gml:surfaceMember xlink:href="#poly_b0e7c20d-5d11-48b7-bca4-d3c322a3ea5f"/>
							<gml:surfaceMember xlink:href="#poly_06072aee-6c29-4c4c-8a4d-23ba3ce753b3"/>
							<gml:surfaceMember xlink:href="#poly_dff39d93-c084-461d-84bc-2ef0aa08876a"/>
							<gml:surfaceMember xlink:href="#poly_96b85c05-355a-48bf-ac22-ffd464c56ec9"/>
							<gml:surfaceMember xlink:href="#poly_86996e71-f246-4dfb-ac3a-64ba2ba6a50a"/>
							<gml:surfaceMember xlink:href="#poly_7c892b99-3639-4386-8533-2b7ffcbe9627"/>
							<gml:surfaceMember xlink:href="#poly_4c14bda4-a71a-4884-afb4-6d6f82f36f9e"/>
							<gml:surfaceMember xlink:href="#poly_e0176d5b-130e-4043-8178-06e7b6461834"/>
							<gml:surfaceMember xlink:href="#poly_0cc32861-16bc-45dc-a8c5-9f0642f06790"/>
							<gml:surfaceMember xlink:href="#poly_22a08578-a0bc-4d3c-8013-cb0b7463e3d8"/>
							<gml:surfaceMember xlink:href="#poly_025113eb-14f3-4b1b-9d63-9f320a918262"/>
							<gml:surfaceMember xlink:href="#poly_7f49c319-17a4-4e19-bedc-900e9de3434e"/>
							<gml:surfaceMember xlink:href="#poly_b265c72e-12d1-4e4f-a38d-ae79eed7ea7f"/>
							<gml:surfaceMember xlink:href="#poly_2ec60d38-c874-4c22-b566-1b31e51a52b6"/>
							<gml:surfaceMember xlink:href="#poly_2a4948ca-64a0-475a-915c-36a72e6a6c35"/>
							<gml:surfaceMember xlink:href="#poly_28d24b10-c837-48e7-8548-bf45dba38da8"/>
							<gml:surfaceMember xlink:href="#poly_8cdca25b-5f4e-4390-bb5d-e0d257233f67"/>
							<gml:surfaceMember xlink:href="#poly_25dfc33d-5102-40b2-944d-171a29b0c33a"/>
							<gml:surfaceMember xlink:href="#poly_1be15f30-c922-40d4-9046-928f4a968cc5"/>
							<gml:surfaceMember xlink:href="#poly_31379833-5a79-47bf-a116-684cd30aa8b2"/>
							<gml:surfaceMember xlink:href="#poly_f7d70d2b-a747-48e6-b9a7-69b97b1af437"/>
							<gml:surfaceMember xlink:href="#poly_dceb2ae5-3944-4522-9d69-920b44f8a903"/>
							<gml:surfaceMember xlink:href="#poly_612f5e86-a8ce-44c3-b7ef-25461379ec0b"/>
							<gml:surfaceMember xlink:href="#poly_97ed2bd9-5f0c-4d13-a934-3d1d7bcac234"/>
							<gml:surfaceMember xlink:href="#poly_2f3428f1-24d5-4a67-b0f1-43f1c9657075"/>
							<gml:surfaceMember xlink:href="#poly_36598a4b-7381-40ad-8318-36616993670e"/>
							<gml:surfaceMember xlink:href="#poly_26ed37b0-9e3f-4906-9e10-04a0ee3175af"/>
							<gml:surfaceMember xlink:href="#poly_84255c00-1b4d-44c3-9040-febb887ed200"/>
							<gml:surfaceMember xlink:href="#poly_26f1867d-6001-43db-ad53-bf51d69c6d1a"/>
							<gml:surfaceMember xlink:href="#poly_6431cde2-fa51-43c1-82b6-6574ecab9e03"/>
							<gml:surfaceMember xlink:href="#poly_4168177e-b4a0-42d0-93f5-5cd26e8906cd"/>
							<gml:surfaceMember xlink:href="#poly_2e954dd3-d479-466c-8c65-32947c21ed48"/>
							<gml:surfaceMember xlink:href="#poly_b82f7b5e-ee78-4e93-81fc-35765f529a78"/>
							<gml:surfaceMember xlink:href="#poly_25f98d1e-7eeb-4d0d-aeba-a98fb77e06ce"/>
							<gml:surfaceMember xlink:href="#poly_455ce7fa-73d5-4fb9-941a-6c0b3df0680d"/>
							<gml:surfaceMember xlink:href="#poly_6b880df3-6aae-4cb6-85dd-45bfb5a39dc1"/>
							<gml:surfaceMember xlink:href="#poly_c041fb14-92e9-48ec-b26f-dfa6737a3165"/>
							<gml:surfaceMember xlink:href="#poly_f90f490c-b374-44d7-9a77-2eed5ee23a56"/>
							<gml:surfaceMember xlink:href="#poly_16c63135-e0ec-4d73-95c6-65f42dbb8a7b"/>
							<gml:surfaceMember xlink:href="#poly_3f5066cc-57d0-4635-8ee2-95e509d3e0f0"/>
							<gml:surfaceMember xlink:href="#poly_a8d032ae-efeb-4564-9d96-56870edc5347"/>
							<gml:surfaceMember xlink:href="#poly_3045136a-1ca0-476e-a003-f42695343e5d"/>
							<gml:surfaceMember xlink:href="#poly_e0a6811f-3485-4bd0-b599-2ee6e397d0ae"/>
							<gml:surfaceMember xlink:href="#poly_b56c6c65-37ba-4d1a-9f30-57d2247234a8"/>
							<gml:surfaceMember xlink:href="#poly_42cf4300-6e15-4e8c-afab-6cb6a02ade2d"/>
							<gml:surfaceMember xlink:href="#poly_af908fa8-711f-4323-886f-f6f9aa4acc03"/>
							<gml:surfaceMember xlink:href="#poly_3e9c79ae-0e79-47bb-b95f-df15aaf61747"/>
							<gml:surfaceMember xlink:href="#poly_aea29471-f9da-4e02-b449-cacfec8d8a60"/>
							<gml:surfaceMember xlink:href="#poly_49cba00b-8a97-43a8-9079-c0350988002c"/>
							<gml:surfaceMember xlink:href="#poly_b4736d45-de5a-4b3b-8690-55a356cab939"/>
							<gml:surfaceMember xlink:href="#poly_b60f4514-7dba-4a33-924d-fa86ae058101"/>
							<gml:surfaceMember xlink:href="#poly_3bdab996-1abd-48c7-8088-26af7dd317dc"/>
							<gml:surfaceMember xlink:href="#poly_48152d19-d62c-46ea-a56e-b0555c480a2d"/>
							<gml:surfaceMember xlink:href="#poly_0a6a851e-04fc-42df-8f78-a720743dedc1"/>
							<gml:surfaceMember xlink:href="#poly_6664d42b-c19e-49ca-b183-a29bc97cad08"/>
							<gml:surfaceMember xlink:href="#poly_26e0a2da-4e45-4376-a429-a22d87bcda3e"/>
							<gml:surfaceMember xlink:href="#poly_90db05c7-b8e8-4f6c-9309-63248ee1c03d"/>
							<gml:surfaceMember xlink:href="#poly_5f5a8a2d-44fd-4a07-9680-cf8e24b55d74"/>
							<gml:surfaceMember xlink:href="#poly_451fc3cc-c643-4d98-b831-57b5814e0ff0"/>
							<gml:surfaceMember xlink:href="#poly_fce23448-8e3d-4b51-a234-1bbb24aa346c"/>
							<gml:surfaceMember xlink:href="#poly_fa88bff4-d58d-4483-a26e-07a4a2865209"/>
							<gml:surfaceMember xlink:href="#poly_5222d023-a797-4808-ab9e-32b8c5f61a39"/>
							<gml:surfaceMember xlink:href="#poly_8939a649-689b-438e-a6dd-0c2102301baf"/>
							<gml:surfaceMember xlink:href="#poly_d5feaf9a-acb2-4abb-8f61-28456f6252ad"/>
							<gml:surfaceMember xlink:href="#poly_5f6483f1-49e7-4168-9db7-3289c818bbd6"/>
							<gml:surfaceMember xlink:href="#poly_61b690e3-c577-46b8-bb11-0d7b655de1e1"/>
							<gml:surfaceMember xlink:href="#poly_8ba8faab-4a62-4a09-9c87-0f30dc6c456d"/>
							<gml:surfaceMember xlink:href="#poly_b6477cb6-a382-4b65-88ba-00fe07aeccba"/>
							<gml:surfaceMember xlink:href="#poly_5e6ef2e6-0d90-4f0d-9dac-61ebe7974075"/>
							<gml:surfaceMember xlink:href="#poly_defbf2cc-c1b8-4870-aeb3-154a6cfadf3c"/>
							<gml:surfaceMember xlink:href="#poly_f35b3eee-172a-4d13-bbf5-b32910885328"/>
							<gml:surfaceMember xlink:href="#poly_a0178d2d-beae-412f-af21-a49bdae29a1b"/>
							<gml:surfaceMember xlink:href="#poly_d269b71e-9e76-4bb7-ad4c-4f949a852b7f"/>
							<gml:surfaceMember xlink:href="#poly_a787cbea-ef19-4b78-aa14-956624f9276f"/>
							<gml:surfaceMember xlink:href="#poly_ea667b87-73ec-4dc1-842c-46412f4476ca"/>
							<gml:surfaceMember xlink:href="#poly_c396e041-2d42-4c43-bb25-1d99c7c4ac0c"/>
							<gml:surfaceMember xlink:href="#poly_f579d015-0ad4-4e44-84d8-3dd3d878a2a4"/>
							<gml:surfaceMember xlink:href="#poly_539817bf-d90f-41b2-a774-5d3f708b8d36"/>
							<gml:surfaceMember xlink:href="#poly_2da770cf-3748-4fb9-bd87-cd1193f31e88"/>
							<gml:surfaceMember xlink:href="#poly_9a6fd492-9307-4ec1-92ee-6a788b8b969f"/>
							<gml:surfaceMember xlink:href="#poly_326ef9af-6a7b-4c01-b42a-93ebeee0d348"/>
							<gml:surfaceMember xlink:href="#poly_0a3267b2-4273-47f4-9ec5-d14334ff27ca"/>
							<gml:surfaceMember xlink:href="#poly_da400f99-9223-4e83-87f3-4fc1654df4ce"/>
							<gml:surfaceMember xlink:href="#poly_2f0e9149-7bec-4116-9df9-81b09e902c34"/>
							<gml:surfaceMember xlink:href="#poly_d13860d5-76f6-48d1-85fd-af24221cbcb1"/>
							<gml:surfaceMember xlink:href="#poly_9f4e0118-cb97-43c5-beeb-0a67c8e6ecda"/>
							<gml:surfaceMember xlink:href="#poly_a4c62544-dced-4b35-9244-1e146a8be81b"/>
							<gml:surfaceMember xlink:href="#poly_7826a645-19e3-4298-8f7b-073902147a0a"/>
							<gml:surfaceMember xlink:href="#poly_ca6a29b5-a993-4ee1-b158-13676d706ec3"/>
							<gml:surfaceMember xlink:href="#poly_96596190-2cae-4f2e-8270-9ab560329032"/>
							<gml:surfaceMember xlink:href="#poly_c4373546-3f46-45ce-a162-1165269a4790"/>
							<gml:surfaceMember xlink:href="#poly_fe5baecf-29d7-452e-bd3d-7876e24cd48b"/>
							<gml:surfaceMember xlink:href="#poly_c1b7a995-2af4-4980-9625-57db02dd3c8a"/>
							<gml:surfaceMember xlink:href="#poly_f490ad99-7e44-4824-bac3-77f77f1e0129"/>
							<gml:surfaceMember xlink:href="#poly_a91ca246-fe7f-47ac-89e3-c895d9a74b17"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod3MultiSurface>
			<uro:railwayRouteAttribute>
				<uro:RailwayRouteAttribute>
					<uro:operatorType codeSpace="../../codelists/RailwayRouteAttribute_operatorType.xml">2</uro:operatorType>
					<uro:operator>東日本旅客鉄道</uro:operator>
					<uro:railwayType codeSpace="../../codelists/RailwayRouteAttribute_railwayType.xml">11</uro:railwayType>
					<uro:startStation>東京駅</uro:startStation>
					<uro:endStation>盛岡駅</uro:endStation>
				</uro:RailwayRouteAttribute>
			</uro:railwayRouteAttribute>			
		</tran:Railway>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<tran:Railway gml:id="rwy_c864250c-0fb8-4109-8ce6-668dedca11a6">
			<gml:name>東北線</gml:name>
			<uro:tranDataQualityAttribute>
				<uro:TransportationDataQualityAttribute>
					<uro:lodType>3.1</uro:lodType>
				</uro:TransportationDataQualityAttribute>
			</uro:tranDataQualityAttribute>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_9eba7546-88d3-4333-b7d6-41490933fef6">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:lod2Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76638212471012 139.7256510160119 0 35.76636648288356 139.72566349687506 0 35.766288027371765 139.72572609769074 0 35.766190714762295 139.7258040279997 0 35.76605051483487 139.7259146683725 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76602563606087 139.7258672078964 0 35.76617791574898 139.72577695657884 0 35.76628177453544 139.7257137842238 0 35.76636549142287 139.7256614200092 0 35.76638212471012 139.7256510160119 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod2Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_5313ca90-ed80-438b-ae7b-d25c7ccda248">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:startPost>14kｍ</uro:startPost>
							<uro:endPost>14kｍ</uro:endPost>
							<uro:alignmentType codeSpace="../../codelists/RailwayTrackAttribute_alignmentType.xml">1</uro:alignmentType>
							<uro:lod3Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76605051379383 139.72591467116425 7.612034320831299 35.76619070646397 139.72580402818517 7.612034320831299 35.76628802574931 139.72572610267395 7.612034320831299 35.766366486501575 139.72566349562476 7.612034320831299 35.766382125866926 139.72565101099235 7.612034320831299 35.76636549878557 139.7256614232971 7.612034320831299 35.76628178268198 139.72571378794623 7.612034320831299 35.76617791888233 139.7257769582918 7.612034320831299 35.76602563121837 139.72586720507465 7.612034320831299</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod3Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_e1f8d288-2458-469f-bf3d-4ec38c60b748">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_81f53f4b-4944-4f95-8b72-0129c89eaf66">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605345053108 139.72592028250926 7.612127304077148 35.76605687274914 139.7259267928303 7.612127780914307 35.7661933760676 139.72580989910225 7.612127780914307 35.76605345053108 139.72592028250926 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_325c109b-9e3b-4116-a379-67d64f75ee02">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605687274914 139.7259267928303 7.612127780914307 35.76605738993917 139.72592777647787 7.452127933502197 35.76619626628526 139.72581700753656 7.612127304077148 35.76605687274914 139.7259267928303 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_6cc0dca9-21c4-40e8-a0d5-dd6b7a521542">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629424747923 139.72573916602784 7.612127780914307 35.7661933760676 139.72580989910225 7.612127780914307 35.76619626628526 139.72581700753656 7.612127304077148 35.76629424747923 139.72573916602784 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ba35cfba-60bb-4186-b5b0-860bbdbebcdf">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605738993917 139.72592777647787 7.452127933502197 35.76619663927907 139.72581792522743 7.452127933502197 35.76619626628526 139.72581700753656 7.612127304077148 35.76605738993917 139.72592777647787 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fc38407f-ccb5-4c42-badc-027fa245ff2c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76619626628526 139.72581700753656 7.612127304077148 35.76619663927907 139.72581792522743 7.452127933502197 35.76629424747923 139.72573916602784 7.612127780914307 35.76619626628526 139.72581700753656 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c84cf172-91e2-4d5b-8e1b-a97a48a88396">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372085378826 139.72567891193026 7.612127304077148 35.7662908187702 139.72573186829518 7.612127780914307 35.76629424747923 139.72573916602784 7.612127780914307 35.766372085378826 139.72567891193026 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a2700918-f69c-45b4-bbb3-8c161df68d05">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76619663927907 139.72581792522743 7.452127933502197 35.76629487914906 139.7257405108266 7.452127933502197 35.76629424747923 139.72573916602784 7.612127780914307 35.76619663927907 139.72581792522743 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_60e8df33-afa8-4de9-bfc6-77668f1d0834">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629424747923 139.72573916602784 7.612127780914307 35.76629487914906 139.7257405108266 7.452127933502197 35.766372085378826 139.72567891193026 7.612127304077148 35.76629424747923 139.72573916602784 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_551ef64c-67ab-4640-a659-15655162953e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76639045993479 139.7256668912336 7.612127304077148 35.766385076121786 139.72565665237107 7.612127780914307 35.766372085378826 139.72567891193026 7.612127304077148 35.76639045993479 139.7256668912336 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9add2bc6-5189-436f-9fcf-895921a10e3f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629487914906 139.7257405108266 7.452127933502197 35.766372848647805 139.72568069109613 7.452127933502197 35.766372085378826 139.72567891193026 7.612127304077148 35.76629487914906 139.7257405108266 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a0964b26-7698-42be-89c9-68b687db2c52">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372085378826 139.72567891193026 7.612127304077148 35.766372848647805 139.72568069109613 7.452127933502197 35.76639045993479 139.7256668912336 7.612127304077148 35.766372085378826 139.72567891193026 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4a749672-1d5d-46ec-8155-a778b02e6cc0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372848647805 139.72568069109613 7.452127933502197 35.766391598198275 139.72566905600553 7.452127933502197 35.76639045993479 139.7256668912336 7.612127304077148 35.766372848647805 139.72568069109613 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5dac04e3-2c03-4580-8253-faf18b5cc77a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637868930691 139.725644474519 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307 35.766363031536336 139.72565637140406 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b5fb4f94-3706-4e8a-97d2-01316cbe8427">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766363031536336 139.72565637140406 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307 35.766363031536336 139.72565637140406 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_20c79060-7974-4df9-a3c3-b67e96cdae49">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766279093006844 139.72570793207203 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_54d0f61b-52d6-4d94-8a81-29b21a6e589e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617568370099 139.7257708736553 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8d2cc623-df6d-4852-b4e1-b621f6c6c7f2">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605687274914 139.7259267928303 7.612127780914307 35.76619626628526 139.72581700753656 7.612127304077148 35.7661933760676 139.72580989910225 7.612127780914307 35.76605687274914 139.7259267928303 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a1da37b9-0d7e-410e-9359-1a5cc16096c5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629424747923 139.72573916602784 7.612127780914307 35.7662908187702 139.72573186829518 7.612127780914307 35.7661933760676 139.72580989910225 7.612127780914307 35.76629424747923 139.72573916602784 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_591dad45-47d9-46aa-8c90-56b8a3183368">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372085378826 139.72567891193026 7.612127304077148 35.76636829549772 139.72567008002378 7.612127780914307 35.7662908187702 139.72573186829518 7.612127780914307 35.766372085378826 139.72567891193026 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_12cc733b-faf1-49fe-a8ed-2f17a9b9b431">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766385076121786 139.72565665237107 7.612127780914307 35.76636829549772 139.72567008002378 7.612127780914307 35.766372085378826 139.72567891193026 7.612127304077148 35.766385076121786 139.72565665237107 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b6b9f322-1207-4341-be3b-ce82f44a8645">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637868930691 139.725644474519 7.612127780914307 35.76637127142479 139.7256303494472 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ff7b8192-f9cd-4f0b-bae5-445fbc5cbf02">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76635628827614 139.72564209776417 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_290057c4-38bd-46e7-b92d-12ac488bea3a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627430137595 139.72569855610917 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_82b7cdad-7007-4576-9643-d31ccc63c90a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617269929112 139.72576363709797 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7d13aeb4-1ed6-4f07-8141-e1896734e5a0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7660418207068 139.72589808386138 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148 35.76617634478357 139.72580388365398 7.612127780914307 35.7660418207068 139.72589808386138 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_032afb5a-c065-438d-a632-9b58de28459a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602922318337 139.72587405030848 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307 35.7661647623805 139.72579290540247 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_144f5c17-5d2c-4014-af70-436ea177af17">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7661647623805 139.72579290540247 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307 35.76618070421888 139.72578336164008 7.612127780914307 35.7661647623805 139.72579290540247 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_64fb406a-33bf-4f66-bcf8-88126242a17f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76618070421888 139.72578336164008 7.612127780914307 35.76618636122224 139.72579581417514 7.612127304077148 35.76627290133284 139.72572754092067 7.612127304077148 35.76618070421888 139.72578336164008 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d394ac3d-d0c1-4913-bd7a-a8e7a69a72b9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603271981537 139.72588071932304 7.612127780914307 35.7660418207068 139.72589808386138 7.612127780914307 35.76617634478357 139.72580388365398 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b7353473-9b38-4d43-a70f-f2f76487d2b5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617634478357 139.72580388365398 7.612127780914307 35.76618636122224 139.72579581417514 7.612127304077148 35.76618070421888 139.72578336164008 7.612127780914307 35.76617634478357 139.72580388365398 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0ab375e1-a2fa-42d6-a1f4-9dfbe3bf1529">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603271981537 139.72588071932304 7.612127780914307 35.76617634478357 139.72580388365398 7.612127780914307 35.76618070421888 139.72578336164008 7.612127780914307 35.76603271981537 139.72588071932304 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_3230615d-ae6c-49ad-a343-1ac2b42e011d">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8112</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a0a2f0ed-2d35-4c13-b10a-a7ad36f0918d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629045002048 139.72573108323905 7.612127780914307 35.7661930452529 139.72580908547616 7.612127780914307 35.7661933760676 139.72580989910225 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8c4388f7-60e2-4ba9-ae6b-164a9d1485f2">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7661930452529 139.72580908547616 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307 35.76605345053108 139.72592028250926 7.612127304077148 35.7661930452529 139.72580908547616 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_75b53b74-78ad-47fb-bb62-114c59a30572">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047038942965 139.7259080564562 7.612127304077148 35.766047407895485 139.72590876185856 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9af04824-79d5-4308-bb18-d21ad2b18241">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627839266262 139.72572421631548 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465 35.76627902673178 139.72572270848286 7.612127304077148 35.76627839266262 139.72572421631548 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4dbdda7c-b4a2-4573-8a4a-ef01c0fd731e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627290133284 139.72572754092067 7.612127304077148 35.76618636122224 139.72579581417514 7.612127304077148 35.766186725034274 139.72579661493134 7.612127780914307 35.76627290133284 139.72572754092067 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e1a6f562-d046-42b2-8316-837d22e8c556">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637856911065 139.72566077082678 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.76636710031373 139.72566701519114 7.612127780914307 35.76637856911065 139.72566077082678 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_65a88b1a-5f18-43e2-bc78-5a6a85641b3e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766363385125224 139.7256571203675 7.612127780914307 35.766363031536336 139.72565637140406 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_36691171-f0b6-444b-bb4c-0242f9f58090">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766279093006844 139.72570793207203 7.612127780914307 35.76627949393959 139.7257087164096 7.612127780914307 35.76635612961561 139.7256618250723 7.612128257751465 35.766279093006844 139.72570793207203 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d3f8efbd-23da-4556-b9b5-4706f17211e9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76635612961561 139.7256618250723 7.612128257751465 35.766284171087946 139.72571864975563 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465 35.76635612961561 139.7256618250723 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ecf3c053-576d-4245-b3e6-253867d82b2c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766363385125224 139.7256571203675 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307 35.76635612961561 139.7256618250723 7.612128257751465 35.766363385125224 139.7256571203675 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_892504b1-2fd3-4667-81f0-3bba5cc3d473">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636740857964 139.72566783576775 7.612127304077148 35.76636710031373 139.72566701519114 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465 35.76636740857964 139.72566783576775 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4e11ae1b-b0c5-40c6-994e-1b22c2ff0f2a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636829549772 139.72567008002378 7.612127780914307 35.76636794018763 139.72566925191654 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307 35.76636829549772 139.72567008002378 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5d8caa67-be53-40cc-8df8-d4750390094e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76618070421888 139.72578336164008 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.7661647623805 139.72579290540247 7.612127780914307 35.76618070421888 139.72578336164008 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_3838da05-eaab-4b43-990c-2461b260bcbe">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602864041166 139.72587293917172 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c20a3e5d-45f4-4266-8e67-e64040de0fd9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627902673178 139.72572270848286 7.612127304077148 35.76618033516983 139.72578254907825 7.612127780914307 35.76618070421888 139.72578336164008 7.612127780914307 35.76627902673178 139.72572270848286 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_27aabcba-9d32-4e5c-815a-cf03dad6a55c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7662849534461 139.72572024410795 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465 35.76627839266262 139.72572421631548 7.612127780914307 35.7662849534461 139.72572024410795 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a1f3ff75-eb6a-42b2-8319-bcb783600247">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602268594188 139.72586158311353 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307 35.76602268594188 139.72586158311353 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4db4bf76-9df8-4b09-81f2-e2f2ec6114c5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627949393959 139.7257087164096 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.76627949393959 139.7257087164096 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ac05031b-c3d0-4361-8f26-5e47ff076458">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637909104791 139.72564523944934 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307 35.76637909104791 139.72564523944934 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0de8d4e3-fdb4-454b-992b-05dc2731b11b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766385076121786 139.72565665237107 7.612127780914307 35.76637856911065 139.72566077082678 7.612127780914307 35.76636829549772 139.72567008002378 7.612127780914307 35.766385076121786 139.72565665237107 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0dc4c999-9db0-4bd7-ad19-7c1ac5d694da">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7662908187702 139.72573186829518 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307 35.7661933760676 139.72580989910225 7.612127780914307 35.7662908187702 139.72573186829518 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d5f69e11-c205-41ed-9886-970721c286f7">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7661933760676 139.72580989910225 7.612127780914307 35.7661930452529 139.72580908547616 7.612127780914307 35.76605345053108 139.72592028250926 7.612127304077148 35.7661933760676 139.72580989910225 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1a2625b3-207b-4950-bf82-f947567822bf">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047038942965 139.7259080564562 7.612127304077148 35.766186725034274 139.72579661493134 7.612127780914307 35.76617634478357 139.72580388365398 7.612127780914307 35.766047038942965 139.7259080564562 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4cdb5e17-6784-4075-acf0-f6042979b23e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617634478357 139.72580388365398 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307 35.76618636122224 139.72579581417514 7.612127304077148 35.76617634478357 139.72580388365398 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7391ef9f-de55-4488-a111-2b0aa19e85f4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627839266262 139.72572421631548 7.612127780914307 35.76627902673178 139.72572270848286 7.612127304077148 35.76627290133284 139.72572754092067 7.612127304077148 35.76627839266262 139.72572421631548 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_33984a6c-4373-41e6-a150-3f6ccb075408">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627902673178 139.72572270848286 7.612127304077148 35.766284515307674 139.72571938050572 7.612128257751465 35.766284171087946 139.72571864975563 7.612127780914307 35.76627902673178 139.72572270848286 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e5316acb-7ec3-4137-925b-e5f10c294fa4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627290133284 139.72572754092067 7.612127304077148 35.766186725034274 139.72579661493134 7.612127780914307 35.76627839266262 139.72572421631548 7.612127780914307 35.76627290133284 139.72572754092067 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_24ecd071-7cd2-469e-ba45-e52a30c62851">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766385076121786 139.72565665237107 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.76637856911065 139.72566077082678 7.612127780914307 35.766385076121786 139.72565665237107 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_896db966-4fca-48f7-b24a-5bec354ed1b4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637856911065 139.72566077082678 7.612127780914307 35.76636710031373 139.72566701519114 7.612127780914307 35.76636740857964 139.72566783576775 7.612127304077148 35.76637856911065 139.72566077082678 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e91e557e-a24b-4c2b-927a-0533e9badf00">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76635612961561 139.7256618250723 7.612128257751465 35.766284515307674 139.72571938050572 7.612128257751465 35.766363385125224 139.7256571203675 7.612127780914307 35.76635612961561 139.7256618250723 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1fb8b827-22cb-49ca-8a7e-9d3f8b7299c3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7662849534461 139.72572024410795 7.612127780914307 35.76636740857964 139.72566783576775 7.612127304077148 35.766284515307674 139.72571938050572 7.612128257751465 35.7662849534461 139.72572024410795 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_35960fb9-48d1-4455-9986-a93a17ba37d8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7662908187702 139.72573186829518 7.612127780914307 35.76636829549772 139.72567008002378 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307 35.7662908187702 139.72573186829518 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b6154be9-e763-492e-b7a3-5d041784be28">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602922318337 139.72587405030848 7.612127780914307 35.7661647623805 139.72579290540247 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.76602922318337 139.72587405030848 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ba46035c-2e5b-4cde-8766-4a3088272e0b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76618070421888 139.72578336164008 7.612127780914307 35.76627290133284 139.72572754092067 7.612127304077148 35.76627902673178 139.72572270848286 7.612127304077148 35.76618070421888 139.72578336164008 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0ed41336-3041-431e-8f4f-499124e9fc03">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602306508788 139.72586230588342 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_49fb47c2-7773-4c39-aaa2-e1ff2262dbc1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766279093006844 139.72570793207203 7.612127780914307 35.76617568370099 139.7257708736553 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.766279093006844 139.72570793207203 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_09fb1318-196f-4d44-a853-130bd198f99d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637868930691 139.725644474519 7.612127780914307 35.766363031536336 139.72565637140406 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a62c9a63-30d3-44d1-aac1-f1b65bb1ac05">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637856911065 139.72566077082678 7.612127780914307 35.76636794018763 139.72566925191654 7.612127780914307 35.76636829549772 139.72567008002378 7.612127780914307 35.76637856911065 139.72566077082678 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_d9ebff81-18b7-4037-ac5b-67b946e1d161">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7661efb0-4496-41f9-b9b0-cb546c9f3ba5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76628515493665 139.7257189404669 0 35.76618635735079 139.72579690580073 0 35.766047414624666 139.7259087547478 0 35.766053070371 139.725919544504 0 35.76619328419243 139.72580889381163 0 35.766290606286574 139.7257309559099 0 35.76636819571302 139.7256690453182 0 35.76638468510641 139.7256558888159 0 35.766379088048694 139.72564523719853 0 35.76636405763676 139.72565703030972 0 35.76628515493665 139.7257189404669 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e992440c-354e-4175-8f7e-e3ad7d0eb702">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76628515493665 139.7257189404669 0 35.766366844718746 139.72566719024647 0 35.76638468510641 139.7256558888159 0 35.766379088048694 139.72564523719853 0 35.766363903294724 139.72565679605606 0 35.76627966903147 139.7257086016925 0 35.76617584124563 139.72577175519805 0 35.76602306348567 139.72586230123608 0 35.76602863797468 139.72587293515647 0 35.7661797865154 139.72578286909553 0 35.76628515493665 139.7257189404669 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_c17a507f-0920-4930-a4b2-e7773eee013d">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ff4651c8-06bf-4ddd-8e37-d559bed0cb04">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76603271907039 139.72588071995614 0 35.766041818468715 139.72589807882554 0 35.766047414624666 139.7259087547478 0 35.76618635735079 139.72579690580073 0 35.76628515493665 139.7257189404669 0 35.7661797865154 139.72578286909553 0 35.76602863797468 139.72587293515647 0 35.76603271907039 139.72588071995614 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e43e27cc-d9d1-4604-966a-bba79a64ef0e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602306348567 139.72586230123608 0 35.766175841246614 139.725771756304 0 35.76627966903147 139.7257086016925 0 35.76636390329572 139.725656797162 0 35.76637908804969 139.72564523830448 0 35.76637126986282 139.72563035831257 0 35.766326874862244 139.72566516160506 0 35.76627448523651 139.72569843725262 0 35.76617353959404 139.72576315180407 0 35.76601807046018 139.72585277469332 0 35.76602306348567 139.72586230123608 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8f566184-91d4-4c89-9876-1c2ea5ddcaec">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605738260426 139.72592776916463 0 35.76619617603737 139.72581828938027 0 35.76629455309776 139.72574076590377 0 35.76637142477569 139.72568157250234 0 35.766391604004646 139.72566905690547 0 35.766384685107404 139.72565588992185 0 35.76636819661536 139.72566904642295 0 35.766290606286574 139.7257309559099 0 35.76619328419343 139.7258088949176 0 35.766053071272374 139.72591954450277 0 35.76605738260426 139.72592776916463 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_29db250a-4889-4883-8d71-8767684ba868">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2d9d797b-c8e7-4996-b300-1f3ad0ac512c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766047407895485 139.72590876185856 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307 35.766047407895485 139.72590876185856 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a9d12e0d-8559-401f-910a-216dc9505d73">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602306508788 139.72586230588342 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.76602306508788 139.72586230588342 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_90cc6344-2d11-408a-80e4-6ba7a4dde918">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627949393959 139.7257087164096 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.76627902673178 139.72572270848286 7.612127304077148 35.76627949393959 139.7257087164096 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e12daad0-c873-4785-b21b-5b61341b207f">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636740857964 139.72566783576775 7.612127304077148 35.76636794018763 139.72566925191654 7.612127780914307 35.76637856911065 139.72566077082678 7.612127780914307 35.76636740857964 139.72566783576775 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a5a0166c-5d43-42f0-954b-c9d0051248d9">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636710031373 139.72566701519114 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.76637909104791 139.72564523944934 7.612127780914307 35.76636710031373 139.72566701519114 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7d4bb3b6-4ce1-4262-b00f-664374e4b863">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636740857964 139.72566783576775 7.612127304077148 35.7662849534461 139.72572024410795 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307 35.76636740857964 139.72566783576775 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d7d01e4d-a554-423b-b213-7eccfce1c072">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766186725034274 139.72579661493134 7.612127780914307 35.7661930452529 139.72580908547616 7.612127780914307 35.7662849534461 139.72572024410795 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b0c90ea4-8d2e-4f7e-8e28-d16ad5f03e04">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.7661930452529 139.72580908547616 7.612127780914307 35.76629045002048 139.72573108323905 7.612127780914307 35.7662849534461 139.72572024410795 7.612127780914307 35.7661930452529 139.72580908547616 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4ed7db42-5da4-4a5e-9941-bfee29f72b61">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627902673178 139.72572270848286 7.612127304077148 35.766284171087946 139.72571864975563 7.612127780914307 35.76627949393959 139.7257087164096 7.612127780914307 35.76627902673178 139.72572270848286 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_109823b3-4207-490f-b725-7bf0f16d01d5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627949393959 139.7257087164096 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.76627949393959 139.7257087164096 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d7aff346-b35b-49d2-9a31-0198756884c7">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76602864041166 139.72587293917172 7.612127780914307 35.76618033516983 139.72578254907825 7.612127780914307 35.76617600706214 139.7257716572528 7.612127780914307 35.76602864041166 139.72587293917172 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_e0ae67bb-8040-4e55-b849-8a7fdd322bc8">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766053068080346 139.72591955501815 7.612127780914307 35.7661930452529 139.72580908547616 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307 35.766053068080346 139.72591955501815 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7e398494-fa18-434b-a58c-5ab4368a30b0">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766363385125224 139.7256571203675 7.612127780914307 35.76636710031373 139.72566701519114 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.766363385125224 139.7256571203675 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8bb6b515-8b8b-4f84-9314-2ec1f4c7aec3">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766284515307674 139.72571938050572 7.612128257751465 35.76636710031373 139.72566701519114 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1a86138c-b73b-440e-a050-8f8321d7f76a">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766363385125224 139.7256571203675 7.612127780914307 35.76636710031373 139.72566701519114 7.612127780914307 35.76637909104791 139.72564523944934 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c83f43f9-079f-4e49-8bf4-dba899037335">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766284515307674 139.72571938050572 7.612128257751465 35.76636710031373 139.72566701519114 7.612127780914307 35.766363385125224 139.7256571203675 7.612127780914307 35.766284515307674 139.72571938050572 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5ed6ae1e-45b7-4a39-a0c1-9bc4a0e5632e">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627949393959 139.7257087164096 7.612127780914307 35.766284171087946 139.72571864975563 7.612127780914307 35.76635612961561 139.7256618250723 7.612128257751465 35.76627949393959 139.7257087164096 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_88bc3650-86a7-4183-91fe-9315241dc7dc">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76636794018763 139.72566925191654 7.612127780914307 35.76636740857964 139.72566783576775 7.612127304077148 35.76629045002048 139.72573108323905 7.612127780914307 35.76636794018763 139.72566925191654 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a88e75b8-f002-42c6-8bd5-f29f79d21e98">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627839266262 139.72572421631548 7.612127780914307 35.766186725034274 139.72579661493134 7.612127780914307 35.7662849534461 139.72572024410795 7.612127780914307 35.76627839266262 139.72572421631548 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:auxiliaryTrafficArea>
				<tran:AuxiliaryTrafficArea gml:id="traf_273df23e-bf0a-48c6-820c-57a51627ea22">
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0122c055-ae06-460f-8c9a-750f9f887d11">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766018067851576 139.72585277563914 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307 35.76599706705152 139.72579543292952 7.612127780914307 35.766018067851576 139.72585277563914 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_5986e068-8da7-4804-9bef-0baf5b4cbb84">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617269929112 139.72576363709797 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307 35.766151698448965 139.72570629430103 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a3ee3e20-e48e-4a88-a82c-f30a02f15284">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76619663927907 139.72581792522743 7.452127933502197 35.76605738993917 139.72592777647787 7.452127933502197 35.76621167537034 139.72585923768497 7.452127933502197 35.76619663927907 139.72581792522743 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_cbfb7467-61f6-466c-a207-b3a6e5cb7531">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627430137595 139.72569855610917 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307 35.766253300503024 139.72564121325655 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b5327f4d-3c3f-46ee-adc5-49e5dd26d8a4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629487914906 139.7257405108266 7.452127933502197 35.76619663927907 139.72581792522743 7.452127933502197 35.766309915266724 139.72578182332023 7.452127933502197 35.76629487914906 139.7257405108266 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_8b5a0a1c-173b-4998-a157-5d00815ed69d">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76635628827614 139.72564209776417 7.612127780914307 35.76637127142479 139.7256303494472 7.612127780914307 35.766335287376506 139.72558475486767 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_a67da159-dc70-4c0a-a6b1-e8b554753398">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372848647805 139.72568069109613 7.452127933502197 35.76629487914906 139.7257405108266 7.452127933502197 35.76638788478588 139.72572200361876 7.452127933502197 35.766372848647805 139.72568069109613 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_0e0ddec7-7218-44ec-bf8d-9f79468f2e0b">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766391598198275 139.72566905600553 7.452127933502197 35.766372848647805 139.72568069109613 7.452127933502197 35.76640490711436 139.72570816759878 7.452127933502197 35.766391598198275 139.72566905600553 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d17081c9-7fa9-44bf-9288-1c3d8b7d6025">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76617269929112 139.72576363709797 7.612127780914307 35.766151698448965 139.72570629430103 7.612127780914307 35.76599706705152 139.72579543292952 7.612127780914307 35.76617269929112 139.72576363709797 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_bce49164-ba1c-433f-a7b9-259b95649252">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76627430137595 139.72569855610917 7.612127780914307 35.766253300503024 139.72564121325655 7.612127780914307 35.766151698448965 139.72570629430103 7.612127780914307 35.76627430137595 139.72569855610917 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_287b8607-2ebb-4aff-b281-316e0bdf42bd">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76605738993917 139.72592777647787 7.452127933502197 35.76607076901669 139.7259666934518 7.452127933502197 35.76621167537034 139.72585923768497 7.452127933502197 35.76605738993917 139.72592777647787 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_9d8a5b5d-6434-47a0-b804-4bd3d24a9aa1">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76635628827614 139.72564209776417 7.612127780914307 35.766335287376506 139.72558475486767 7.612127780914307 35.766253300503024 139.72564121325655 7.612127780914307 35.76635628827614 139.72564209776417 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2a53fb97-4123-4ae9-b74d-3e71d48b2342">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76619663927907 139.72581792522743 7.452127933502197 35.76621167537034 139.72585923768497 7.452127933502197 35.766309915266724 139.72578182332023 7.452127933502197 35.76619663927907 139.72581792522743 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1f3d472a-ff7a-40d4-907e-db0d886b2741">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637127142479 139.7256303494472 7.612127780914307 35.766350270519595 139.72557300654304 7.612127780914307 35.766335287376506 139.72558475486767 7.612127780914307 35.76637127142479 139.7256303494472 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_4efaccbd-ce16-42b1-b6a2-34d7bb87cb17">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76629487914906 139.7257405108266 7.452127933502197 35.766309915266724 139.72578182332023 7.452127933502197 35.76638788478588 139.72572200361876 7.452127933502197 35.76629487914906 139.7257405108266 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2925a6cb-e43c-48e1-9d05-99060ce70b98">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766372848647805 139.72568069109613 7.452127933502197 35.76638788478588 139.72572200361876 7.452127933502197 35.76640490711436 139.72570816759878 7.452127933502197 35.766372848647805 139.72568069109613 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:AuxiliaryTrafficArea>
			</tran:auxiliaryTrafficArea>
			<tran:lod0Network>
				<gml:CompositeCurve>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.76638212471012 139.7256510160119 0 35.76636648288356 139.72566349687506 0 35.766288027371765 139.72572609769074 0 35.766190714762295 139.7258040279997 0 35.76605051483487 139.7259146683725 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.76602563606087 139.7258672078964 0 35.76617791574898 139.72577695657884 0 35.76628177453544 139.7257137842238 0 35.76636549142287 139.7256614200092 0 35.76638212471012 139.7256510160119 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
				</gml:CompositeCurve>
			</tran:lod0Network>
			<tran:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_d9e1f352-caff-414c-b7e7-2b1ee905f06c">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76628515493665 139.7257189404669 0 35.76618635735079 139.72579690580073 0 35.766047414624666 139.7259087547478 0 35.766053070371 139.725919544504 0 35.76619328419243 139.72580889381163 0 35.766290606286574 139.7257309559099 0 35.76636819571302 139.7256690453182 0 35.76638468510641 139.7256558888159 0 35.766379088048694 139.72564523719853 0 35.76636405763676 139.72565703030972 0 35.76628515493665 139.7257189404669 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_21dfab7e-3115-4382-8e7a-9f48c2eb6a87">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76628515493665 139.7257189404669 0 35.766366844718746 139.72566719024647 0 35.76638468510641 139.7256558888159 0 35.766379088048694 139.72564523719853 0 35.766363903294724 139.72565679605606 0 35.76627966903147 139.7257086016925 0 35.76617584124563 139.72577175519805 0 35.76602306348567 139.72586230123608 0 35.76602863797468 139.72587293515647 0 35.7661797865154 139.72578286909553 0 35.76628515493665 139.7257189404669 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod1MultiSurface>
			<tran:lod2MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_7661efb0-4496-41f9-b9b0-cb546c9f3ba5"/>
							<gml:surfaceMember xlink:href="#poly_e992440c-354e-4175-8f7e-e3ad7d0eb702"/>
							<gml:surfaceMember xlink:href="#poly_ff4651c8-06bf-4ddd-8e37-d559bed0cb04"/>
							<gml:surfaceMember xlink:href="#poly_e43e27cc-d9d1-4604-966a-bba79a64ef0e"/>
							<gml:surfaceMember xlink:href="#poly_8f566184-91d4-4c89-9876-1c2ea5ddcaec"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod2MultiSurface>
			<tran:lod3MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_a0a2f0ed-2d35-4c13-b10a-a7ad36f0918d"/>
							<gml:surfaceMember xlink:href="#poly_8c4388f7-60e2-4ba9-ae6b-164a9d1485f2"/>
							<gml:surfaceMember xlink:href="#poly_75b53b74-78ad-47fb-bb62-114c59a30572"/>
							<gml:surfaceMember xlink:href="#poly_9af04824-79d5-4308-bb18-d21ad2b18241"/>
							<gml:surfaceMember xlink:href="#poly_4dbdda7c-b4a2-4573-8a4a-ef01c0fd731e"/>
							<gml:surfaceMember xlink:href="#poly_e1a6f562-d046-42b2-8316-837d22e8c556"/>
							<gml:surfaceMember xlink:href="#poly_65a88b1a-5f18-43e2-bc78-5a6a85641b3e"/>
							<gml:surfaceMember xlink:href="#poly_36691171-f0b6-444b-bb4c-0242f9f58090"/>
							<gml:surfaceMember xlink:href="#poly_d3f8efbd-23da-4556-b9b5-4706f17211e9"/>
							<gml:surfaceMember xlink:href="#poly_ecf3c053-576d-4245-b3e6-253867d82b2c"/>
							<gml:surfaceMember xlink:href="#poly_892504b1-2fd3-4667-81f0-3bba5cc3d473"/>
							<gml:surfaceMember xlink:href="#poly_4e11ae1b-b0c5-40c6-994e-1b22c2ff0f2a"/>
							<gml:surfaceMember xlink:href="#poly_81f53f4b-4944-4f95-8b72-0129c89eaf66"/>
							<gml:surfaceMember xlink:href="#poly_325c109b-9e3b-4116-a379-67d64f75ee02"/>
							<gml:surfaceMember xlink:href="#poly_6cc0dca9-21c4-40e8-a0d5-dd6b7a521542"/>
							<gml:surfaceMember xlink:href="#poly_ba35cfba-60bb-4186-b5b0-860bbdbebcdf"/>
							<gml:surfaceMember xlink:href="#poly_fc38407f-ccb5-4c42-badc-027fa245ff2c"/>
							<gml:surfaceMember xlink:href="#poly_c84cf172-91e2-4d5b-8e1b-a97a48a88396"/>
							<gml:surfaceMember xlink:href="#poly_a2700918-f69c-45b4-bbb3-8c161df68d05"/>
							<gml:surfaceMember xlink:href="#poly_60e8df33-afa8-4de9-bfc6-77668f1d0834"/>
							<gml:surfaceMember xlink:href="#poly_551ef64c-67ab-4640-a659-15655162953e"/>
							<gml:surfaceMember xlink:href="#poly_9add2bc6-5189-436f-9fcf-895921a10e3f"/>
							<gml:surfaceMember xlink:href="#poly_a0964b26-7698-42be-89c9-68b687db2c52"/>
							<gml:surfaceMember xlink:href="#poly_4a749672-1d5d-46ec-8155-a778b02e6cc0"/>
							<gml:surfaceMember xlink:href="#poly_5dac04e3-2c03-4580-8253-faf18b5cc77a"/>
							<gml:surfaceMember xlink:href="#poly_b5fb4f94-3706-4e8a-97d2-01316cbe8427"/>
							<gml:surfaceMember xlink:href="#poly_20c79060-7974-4df9-a3c3-b67e96cdae49"/>
							<gml:surfaceMember xlink:href="#poly_54d0f61b-52d6-4d94-8a81-29b21a6e589e"/>
							<gml:surfaceMember xlink:href="#poly_8d2cc623-df6d-4852-b4e1-b621f6c6c7f2"/>
							<gml:surfaceMember xlink:href="#poly_a1da37b9-0d7e-410e-9359-1a5cc16096c5"/>
							<gml:surfaceMember xlink:href="#poly_591dad45-47d9-46aa-8c90-56b8a3183368"/>
							<gml:surfaceMember xlink:href="#poly_12cc733b-faf1-49fe-a8ed-2f17a9b9b431"/>
							<gml:surfaceMember xlink:href="#poly_b6b9f322-1207-4341-be3b-ce82f44a8645"/>
							<gml:surfaceMember xlink:href="#poly_ff7b8192-f9cd-4f0b-bae5-445fbc5cbf02"/>
							<gml:surfaceMember xlink:href="#poly_290057c4-38bd-46e7-b92d-12ac488bea3a"/>
							<gml:surfaceMember xlink:href="#poly_82b7cdad-7007-4576-9643-d31ccc63c90a"/>
							<gml:surfaceMember xlink:href="#poly_7d13aeb4-1ed6-4f07-8141-e1896734e5a0"/>
							<gml:surfaceMember xlink:href="#poly_032afb5a-c065-438d-a632-9b58de28459a"/>
							<gml:surfaceMember xlink:href="#poly_144f5c17-5d2c-4014-af70-436ea177af17"/>
							<gml:surfaceMember xlink:href="#poly_64fb406a-33bf-4f66-bcf8-88126242a17f"/>
							<gml:surfaceMember xlink:href="#poly_d394ac3d-d0c1-4913-bd7a-a8e7a69a72b9"/>
							<gml:surfaceMember xlink:href="#poly_b7353473-9b38-4d43-a70f-f2f76487d2b5"/>
							<gml:surfaceMember xlink:href="#poly_0ab375e1-a2fa-42d6-a1f4-9dfbe3bf1529"/>
							<gml:surfaceMember xlink:href="#poly_2d9d797b-c8e7-4996-b300-1f3ad0ac512c"/>
							<gml:surfaceMember xlink:href="#poly_a9d12e0d-8559-401f-910a-216dc9505d73"/>
							<gml:surfaceMember xlink:href="#poly_90cc6344-2d11-408a-80e4-6ba7a4dde918"/>
							<gml:surfaceMember xlink:href="#poly_e12daad0-c873-4785-b21b-5b61341b207f"/>
							<gml:surfaceMember xlink:href="#poly_a5a0166c-5d43-42f0-954b-c9d0051248d9"/>
							<gml:surfaceMember xlink:href="#poly_7d4bb3b6-4ce1-4262-b00f-664374e4b863"/>
							<gml:surfaceMember xlink:href="#poly_d7d01e4d-a554-423b-b213-7eccfce1c072"/>
							<gml:surfaceMember xlink:href="#poly_b0c90ea4-8d2e-4f7e-8e28-d16ad5f03e04"/>
							<gml:surfaceMember xlink:href="#poly_5d8caa67-be53-40cc-8df8-d4750390094e"/>
							<gml:surfaceMember xlink:href="#poly_4ed7db42-5da4-4a5e-9941-bfee29f72b61"/>
							<gml:surfaceMember xlink:href="#poly_109823b3-4207-490f-b725-7bf0f16d01d5"/>
							<gml:surfaceMember xlink:href="#poly_3838da05-eaab-4b43-990c-2461b260bcbe"/>
							<gml:surfaceMember xlink:href="#poly_c20a3e5d-45f4-4266-8e67-e64040de0fd9"/>
							<gml:surfaceMember xlink:href="#poly_27aabcba-9d32-4e5c-815a-cf03dad6a55c"/>
							<gml:surfaceMember xlink:href="#poly_a1f3ff75-eb6a-42b2-8319-bcb783600247"/>
							<gml:surfaceMember xlink:href="#poly_4db4bf76-9df8-4b09-81f2-e2f2ec6114c5"/>
							<gml:surfaceMember xlink:href="#poly_ac05031b-c3d0-4361-8f26-5e47ff076458"/>
							<gml:surfaceMember xlink:href="#poly_d7aff346-b35b-49d2-9a31-0198756884c7"/>
							<gml:surfaceMember xlink:href="#poly_e0ae67bb-8040-4e55-b849-8a7fdd322bc8"/>
							<gml:surfaceMember xlink:href="#poly_0de8d4e3-fdb4-454b-992b-05dc2731b11b"/>
							<gml:surfaceMember xlink:href="#poly_0dc4c999-9db0-4bd7-ad19-7c1ac5d694da"/>
							<gml:surfaceMember xlink:href="#poly_d5f69e11-c205-41ed-9886-970721c286f7"/>
							<gml:surfaceMember xlink:href="#poly_1a2625b3-207b-4950-bf82-f947567822bf"/>
							<gml:surfaceMember xlink:href="#poly_4cdb5e17-6784-4075-acf0-f6042979b23e"/>
							<gml:surfaceMember xlink:href="#poly_7391ef9f-de55-4488-a111-2b0aa19e85f4"/>
							<gml:surfaceMember xlink:href="#poly_33984a6c-4373-41e6-a150-3f6ccb075408"/>
							<gml:surfaceMember xlink:href="#poly_e5316acb-7ec3-4137-925b-e5f10c294fa4"/>
							<gml:surfaceMember xlink:href="#poly_24ecd071-7cd2-469e-ba45-e52a30c62851"/>
							<gml:surfaceMember xlink:href="#poly_896db966-4fca-48f7-b24a-5bec354ed1b4"/>
							<gml:surfaceMember xlink:href="#poly_e91e557e-a24b-4c2b-927a-0533e9badf00"/>
							<gml:surfaceMember xlink:href="#poly_1fb8b827-22cb-49ca-8a7e-9d3f8b7299c3"/>
							<gml:surfaceMember xlink:href="#poly_7e398494-fa18-434b-a58c-5ab4368a30b0"/>
							<gml:surfaceMember xlink:href="#poly_8bb6b515-8b8b-4f84-9314-2ec1f4c7aec3"/>
							<gml:surfaceMember xlink:href="#poly_1a86138c-b73b-440e-a050-8f8321d7f76a"/>
							<gml:surfaceMember xlink:href="#poly_c83f43f9-079f-4e49-8bf4-dba899037335"/>
							<gml:surfaceMember xlink:href="#poly_5ed6ae1e-45b7-4a39-a0c1-9bc4a0e5632e"/>
							<gml:surfaceMember xlink:href="#poly_88bc3650-86a7-4183-91fe-9315241dc7dc"/>
							<gml:surfaceMember xlink:href="#poly_a88e75b8-f002-42c6-8bd5-f29f79d21e98"/>
							<gml:surfaceMember xlink:href="#poly_35960fb9-48d1-4455-9986-a93a17ba37d8"/>
							<gml:surfaceMember xlink:href="#poly_b6154be9-e763-492e-b7a3-5d041784be28"/>
							<gml:surfaceMember xlink:href="#poly_ba46035c-2e5b-4cde-8766-4a3088272e0b"/>
							<gml:surfaceMember xlink:href="#poly_0ed41336-3041-431e-8f4f-499124e9fc03"/>
							<gml:surfaceMember xlink:href="#poly_49fb47c2-7773-4c39-aaa2-e1ff2262dbc1"/>
							<gml:surfaceMember xlink:href="#poly_09fb1318-196f-4d44-a853-130bd198f99d"/>
							<gml:surfaceMember xlink:href="#poly_a62c9a63-30d3-44d1-aac1-f1b65bb1ac05"/>
							<gml:surfaceMember xlink:href="#poly_0122c055-ae06-460f-8c9a-750f9f887d11"/>
							<gml:surfaceMember xlink:href="#poly_5986e068-8da7-4804-9bef-0baf5b4cbb84"/>
							<gml:surfaceMember xlink:href="#poly_a3ee3e20-e48e-4a88-a82c-f30a02f15284"/>
							<gml:surfaceMember xlink:href="#poly_cbfb7467-61f6-466c-a207-b3a6e5cb7531"/>
							<gml:surfaceMember xlink:href="#poly_b5327f4d-3c3f-46ee-adc5-49e5dd26d8a4"/>
							<gml:surfaceMember xlink:href="#poly_8b5a0a1c-173b-4998-a157-5d00815ed69d"/>
							<gml:surfaceMember xlink:href="#poly_a67da159-dc70-4c0a-a6b1-e8b554753398"/>
							<gml:surfaceMember xlink:href="#poly_0e0ddec7-7218-44ec-bf8d-9f79468f2e0b"/>
							<gml:surfaceMember xlink:href="#poly_d17081c9-7fa9-44bf-9288-1c3d8b7d6025"/>
							<gml:surfaceMember xlink:href="#poly_bce49164-ba1c-433f-a7b9-259b95649252"/>
							<gml:surfaceMember xlink:href="#poly_287b8607-2ebb-4aff-b281-316e0bdf42bd"/>
							<gml:surfaceMember xlink:href="#poly_9d8a5b5d-6434-47a0-b804-4bd3d24a9aa1"/>
							<gml:surfaceMember xlink:href="#poly_2a53fb97-4123-4ae9-b74d-3e71d48b2342"/>
							<gml:surfaceMember xlink:href="#poly_1f3d472a-ff7a-40d4-907e-db0d886b2741"/>
							<gml:surfaceMember xlink:href="#poly_4efaccbd-ce16-42b1-b6a2-34d7bb87cb17"/>
							<gml:surfaceMember xlink:href="#poly_2925a6cb-e43c-48e1-9d05-99060ce70b98"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod3MultiSurface>
			<uro:railwayRouteAttribute>
				<uro:RailwayRouteAttribute>
					<uro:operatorType codeSpace="../../codelists/RailwayRouteAttribute_operatorType.xml">2</uro:operatorType>
					<uro:operator>東日本旅客鉄道</uro:operator>
					<uro:railwayType codeSpace="../../codelists/RailwayRouteAttribute_railwayType.xml">11</uro:railwayType>
					<uro:startStation>東京駅</uro:startStation>
					<uro:endStation>盛岡駅</uro:endStation>
				</uro:RailwayRouteAttribute>
			</uro:railwayRouteAttribute>			
		</tran:Railway>
	</core:cityObjectMember>
	<core:cityObjectMember>
		<tran:Railway gml:id="rwy_4dbb7611-fbe8-435e-8d85-9c2ed0c5d80c">
			<gml:name>東北線</gml:name>
			<uro:tranDataQualityAttribute>
				<uro:TransportationDataQualityAttribute>
					<uro:lodType>3.1</uro:lodType>
				</uro:TransportationDataQualityAttribute>
			</uro:tranDataQualityAttribute>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_d6e4206e-f289-46fe-92fa-2e4717fdbf2e">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:lod2Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.76642510618788 139.7256170162689 0 35.76638212471012 139.7256510160119 0</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod2Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_6a605477-7c77-4ea4-970c-2ae4dc7e11f5">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8000</tran:function>
					<uro:railwayTrackAttribute>
						<uro:RailwayTrackAttribute>
							<uro:startPost>14kｍ</uro:startPost>
							<uro:endPost>14kｍ</uro:endPost>
							<uro:alignmentType codeSpace="../../codelists/RailwayTrackAttribute_alignmentType.xml">1</uro:alignmentType>
							<uro:lod3Network>
								<gml:CompositeCurve>
									<gml:curveMember>
										<gml:LineString>
											<gml:posList>35.766425103611276 139.725617018479 7.612034320831299 35.766382125866926 139.72565101099235 7.612034320831299</gml:posList>
										</gml:LineString>
									</gml:curveMember>
								</gml:CompositeCurve>
							</uro:lod3Network>
						</uro:RailwayTrackAttribute>
					</uro:railwayTrackAttribute>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_0474bdac-577e-4647-b574-6f955aaac8ec">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c1645425-dc3d-4598-a4f8-14d39b8a9877">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766425598090684 139.7256245842762 7.612127780914307 35.76639825624128 139.7256603468768 7.612127780914307 35.766428041756065 139.72562265042555 7.612127304077148 35.766425598090684 139.7256245842762 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d540f1b5-874c-4b1d-b62d-a84f30a16695">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76639045993479 139.7256668912336 7.612127304077148 35.766391598198275 139.72566905600553 7.452127933502197 35.76639825624128 139.7256603468768 7.612127780914307 35.76639045993479 139.7256668912336 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_93a6050a-9fab-4074-83f9-4ae06e1adf9c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76639825624128 139.7256603468768 7.612127780914307 35.76643321728934 139.7256328306766 7.452127933502197 35.76643256531721 139.72563154648674 7.612127780914307 35.76639825624128 139.7256603468768 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_77b2d5d4-ae00-4fbc-8ee3-99e414f2bc11">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76639825624128 139.7256603468768 7.612127780914307 35.766391598198275 139.72566905600553 7.452127933502197 35.76643321728934 139.7256328306766 7.452127933502197 35.76639825624128 139.7256603468768 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ad6af983-02f1-47f5-ae1b-132d9f3f09ce">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76641530046125 139.72559877984241 7.612127780914307 35.76637127142479 139.7256303494472 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307 35.76641530046125 139.72559877984241 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_867b44fa-8dbf-44a4-8a02-1dacace81cd7">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76639825624128 139.7256603468768 7.612127780914307 35.76643256531721 139.72563154648674 7.612127780914307 35.766428041756065 139.72562265042555 7.612127304077148 35.76639825624128 139.7256603468768 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_06b2f79b-9b52-47b2-a7bc-a65e35f8dff5">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766385076121786 139.72565665237107 7.612127780914307 35.76639045993479 139.7256668912336 7.612127304077148 35.76639825624128 139.7256603468768 7.612127780914307 35.766385076121786 139.72565665237107 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_c4359ef8-1fc5-4c9c-a708-cc012f1c4343">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76642166433483 139.72561175087534 7.612127780914307 35.76641530046125 139.72559877984241 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307 35.76642166433483 139.72561175087534 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_fdff90b2-5111-482e-834f-45f8a2e36bdf">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766385076121786 139.72565665237107 7.612127780914307 35.76639825624128 139.7256603468768 7.612127780914307 35.766425598090684 139.7256245842762 7.612127780914307 35.766385076121786 139.72565665237107 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_9e4bf456-0bb4-4fd1-b4b8-95d5ef04411b">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8112</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ab9b3870-35b4-4088-ae43-a68ef43a1a73">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76642204958967 139.72561253591 7.612127780914307 35.76642166433483 139.72561175087534 7.612127780914307 35.76637909104791 139.72564523944934 7.612127780914307 35.76642204958967 139.72561253591 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_bdadbb99-7030-4509-a296-8dcfa53e5ea2">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766428041756065 139.72562265042555 7.612127304077148 35.766427656250244 139.72562189222316 7.612127780914307 35.766425598090684 139.7256245842762 7.612127780914307 35.766428041756065 139.72562265042555 7.612127304077148</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ba71c43c-6440-41c3-9724-8ccd973fd965">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76642166433483 139.72561175087534 7.612127780914307 35.76637868930691 139.725644474519 7.612127780914307 35.76637909104791 139.72564523944934 7.612127780914307 35.76642166433483 139.72561175087534 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_915d92e8-e360-4795-90f2-a6d3b1b822ca">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766425598090684 139.7256245842762 7.612127780914307 35.766427656250244 139.72562189222316 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.766425598090684 139.7256245842762 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d56e4509-81fa-4866-a9ca-b2d21ea08198">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766425598090684 139.7256245842762 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.766385076121786 139.72565665237107 7.612127780914307 35.766425598090684 139.7256245842762 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_6c7de258-823f-4a4d-8dfe-aac7e2273ebf">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_09ea5b3d-3901-45ce-9976-32f1d80015d4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637909104791 139.72564523944934 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465 35.76642204958967 139.72561253591 7.612127780914307 35.76637909104791 139.72564523944934 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d1c4e15d-2f7b-4444-8c6c-1363c87837cb">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76638467851496 139.72565589637907 7.612128257751465 35.766427656250244 139.72562189222316 7.612127780914307 35.76642204958967 139.72561253591 7.612127780914307 35.76638467851496 139.72565589637907 7.612128257751465</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_bef8edee-d431-450e-be11-f532278e8591">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8120</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b5d4617d-863d-4169-8a2f-c63c63ebce60">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76642205356543 139.72561253268478 0 35.76641530629628 139.72559877931505 0 35.76637126986282 139.72563035831257 0 35.76637908804969 139.72564523830448 0 35.76642205356543 139.72561253268478 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_7bf11f27-fd20-476b-b85d-a113994b63ef">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766427666592044 139.72562188921404 0 35.766384685107404 139.72565588992185 0 35.766391604004646 139.72566905690547 0 35.76643321072819 139.72563282734924 0 35.766427666592044 139.72562188921404 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_53b332bd-d2d6-4f49-90f7-dd262576a623">
					<tran:function codeSpace="../../codelists/TrafficArea_function.xml">8111</tran:function>
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_2311d09c-e42a-4b15-8c70-c572eb0f356c">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76638468510641 139.7256558888159 0 35.766427666592044 139.72562188921404 0 35.76642205356543 139.72561253268478 0 35.766379088048694 139.72564523719853 0 35.76638468510641 139.7256558888159 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:auxiliaryTrafficArea>
				<tran:AuxiliaryTrafficArea gml:id="traf_11db18e2-236d-4559-a8cd-df03b69b066e">
					<tran:lod3MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_d6f001db-adde-4c16-b24f-0e35a177f397">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76637127142479 139.7256303494472 7.612127780914307 35.76641530046125 139.72559877984241 7.612127780914307 35.766350270519595 139.72557300654304 7.612127780914307 35.76637127142479 139.7256303494472 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_ddb3d4d4-f868-472f-91a0-f99bdadf65f6">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76643321728934 139.7256328306766 7.452127933502197 35.766391598198275 139.72566905600553 7.452127933502197 35.766448253443734 139.7256741432213 7.452127933502197 35.76643321728934 139.7256328306766 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_f90062a7-1561-4b9e-9209-90d91dacc0a4">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.76641530046125 139.72559877984241 7.612127780914307 35.76639317776114 139.7255469249859 7.612127780914307 35.766350270519595 139.72557300654304 7.612127780914307 35.76641530046125 139.72559877984241 7.612127780914307</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_1c81d627-9089-4bad-9a0a-8ce51966fe04">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.766391598198275 139.72566905600553 7.452127933502197 35.76640490711436 139.72570816759878 7.452127933502197 35.766448253443734 139.7256741432213 7.452127933502197 35.766391598198275 139.72566905600553 7.452127933502197</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod3MultiSurface>
				</tran:AuxiliaryTrafficArea>
			</tran:auxiliaryTrafficArea>
			<tran:lod0Network>
				<gml:CompositeCurve>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.76642510618788 139.7256170162689 0 35.76638212471012 139.7256510160119 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
				</gml:CompositeCurve>
			</tran:lod0Network>
			<tran:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon gml:id="poly_8e581995-9da4-490a-abbd-df7a490fe643">
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.76638468510641 139.7256558888159 0 35.766427666592044 139.72562188921404 0 35.76642205356543 139.72561253268478 0 35.766379088048694 139.72564523719853 0 35.76638468510641 139.7256558888159 0</gml:posList>
								</gml:LinearRing>
							</gml:exterior>
						</gml:Polygon>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod1MultiSurface>
			<tran:lod2MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_2311d09c-e42a-4b15-8c70-c572eb0f356c"/>
							<gml:surfaceMember xlink:href="#poly_b5d4617d-863d-4169-8a2f-c63c63ebce60"/>
							<gml:surfaceMember xlink:href="#poly_7bf11f27-fd20-476b-b85d-a113994b63ef"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod2MultiSurface>
			<tran:lod3MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:CompositeSurface>
							<gml:surfaceMember xlink:href="#poly_c1645425-dc3d-4598-a4f8-14d39b8a9877"/>
							<gml:surfaceMember xlink:href="#poly_d540f1b5-874c-4b1d-b62d-a84f30a16695"/>
							<gml:surfaceMember xlink:href="#poly_93a6050a-9fab-4074-83f9-4ae06e1adf9c"/>
							<gml:surfaceMember xlink:href="#poly_77b2d5d4-ae00-4fbc-8ee3-99e414f2bc11"/>
							<gml:surfaceMember xlink:href="#poly_ad6af983-02f1-47f5-ae1b-132d9f3f09ce"/>
							<gml:surfaceMember xlink:href="#poly_867b44fa-8dbf-44a4-8a02-1dacace81cd7"/>
							<gml:surfaceMember xlink:href="#poly_06b2f79b-9b52-47b2-a7bc-a65e35f8dff5"/>
							<gml:surfaceMember xlink:href="#poly_c4359ef8-1fc5-4c9c-a708-cc012f1c4343"/>
							<gml:surfaceMember xlink:href="#poly_ab9b3870-35b4-4088-ae43-a68ef43a1a73"/>
							<gml:surfaceMember xlink:href="#poly_bdadbb99-7030-4509-a296-8dcfa53e5ea2"/>
							<gml:surfaceMember xlink:href="#poly_ba71c43c-6440-41c3-9724-8ccd973fd965"/>
							<gml:surfaceMember xlink:href="#poly_915d92e8-e360-4795-90f2-a6d3b1b822ca"/>
							<gml:surfaceMember xlink:href="#poly_d56e4509-81fa-4866-a9ca-b2d21ea08198"/>
							<gml:surfaceMember xlink:href="#poly_09ea5b3d-3901-45ce-9976-32f1d80015d4"/>
							<gml:surfaceMember xlink:href="#poly_d1c4e15d-2f7b-4444-8c6c-1363c87837cb"/>
							<gml:surfaceMember xlink:href="#poly_fdff90b2-5111-482e-834f-45f8a2e36bdf"/>
							<gml:surfaceMember xlink:href="#poly_d6f001db-adde-4c16-b24f-0e35a177f397"/>
							<gml:surfaceMember xlink:href="#poly_ddb3d4d4-f868-472f-91a0-f99bdadf65f6"/>
							<gml:surfaceMember xlink:href="#poly_f90062a7-1561-4b9e-9209-90d91dacc0a4"/>
							<gml:surfaceMember xlink:href="#poly_1c81d627-9089-4bad-9a0a-8ce51966fe04"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod3MultiSurface>
			<uro:railwayRouteAttribute>
				<uro:RailwayRouteAttribute>
					<uro:operatorType codeSpace="../../codelists/RailwayRouteAttribute_operatorType.xml">2</uro:operatorType>
					<uro:operator>東日本旅客鉄道</uro:operator>
					<uro:railwayType codeSpace="../../codelists/RailwayRouteAttribute_railwayType.xml">11</uro:railwayType>
					<uro:startStation>東京駅</uro:startStation>
					<uro:endStation>盛岡駅</uro:endStation>
				</uro:RailwayRouteAttribute>
			</uro:railwayRouteAttribute>			
		</tran:Railway>
	</core:cityObjectMember>
</core:CityModel>