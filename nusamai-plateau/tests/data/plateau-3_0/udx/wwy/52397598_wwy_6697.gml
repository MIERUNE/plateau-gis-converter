<?xml version="1.0" encoding="UTF-8"?>
<core:CityModel xmlns:grp="http://www.opengis.net/citygml/cityobjectgroup/2.0" xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:pbase="http://www.opengis.net/citygml/profiles/base/2.0" xmlns:smil20lang="http://www.w3.org/2001/SMIL20/Language" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:smil20="http://www.w3.org/2001/SMIL20/" xmlns:bldg="http://www.opengis.net/citygml/building/2.0" xmlns:uro="https://www.geospatial.jp/iur/uro/3.0" xmlns:xAL="urn:oasis:names:tc:ciq:xsdschema:xAL:2.0" xmlns:luse="http://www.opengis.net/citygml/landuse/2.0" xmlns:gen="http://www.opengis.net/citygml/generics/2.0" xmlns:dem="http://www.opengis.net/citygml/relief/2.0" xmlns:app="http://www.opengis.net/citygml/appearance/2.0" xmlns:tex="http://www.opengis.net/citygml/texturedsurface/2.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:tun="http://www.opengis.net/citygml/tunnel/2.0" xmlns:sch="http://www.ascc.net/xml/schematron" xmlns:veg="http://www.opengis.net/citygml/vegetation/2.0" xmlns:frn="http://www.opengis.net/citygml/cityfurniture/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:tran="http://www.opengis.net/citygml/transportation/2.0" xmlns:wtr="http://www.opengis.net/citygml/waterbody/2.0" xmlns:brid="http://www.opengis.net/citygml/bridge/2.0" xsi:schemaLocation="https://www.geospatial.jp/iur/uro/3.0 ../../schemas/iur/uro/3.0/urbanObject.xsd  http://www.opengis.net/citygml/2.0 http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd http://www.opengis.net/citygml/landuse/2.0 http://schemas.opengis.net/citygml/landuse/2.0/landUse.xsd  http://www.opengis.net/citygml/building/2.0 http://schemas.opengis.net/citygml/building/2.0/building.xsd  http://www.opengis.net/citygml/transportation/2.0 http://schemas.opengis.net/citygml/transportation/2.0/transportation.xsd  http://www.opengis.net/citygml/generics/2.0 http://schemas.opengis.net/citygml/generics/2.0/generics.xsd  http://www.opengis.net/citygml/relief/2.0 http://schemas.opengis.net/citygml/relief/2.0/relief.xsd  http://www.opengis.net/citygml/cityobjectgroup/2.0 http://schemas.opengis.net/citygml/cityobjectgroup/2.0/cityObjectGroup.xsd  http://www.opengis.net/gml http://schemas.opengis.net/gml/3.1.1/base/gml.xsd http://www.opengis.net/citygml/appearance/2.0 http://schemas.opengis.net/citygml/appearance/2.0/appearance.xsd ">
	<gml:boundedBy>
		<gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
			<gml:lowerCorner>35.31236110999866 139.72302778000028 0</gml:lowerCorner>
			<gml:upperCorner>35.4083333299986 139.77675000000028 0</gml:upperCorner>
		</gml:Envelope>
	</gml:boundedBy>
	<core:cityObjectMember>
		<uro:Waterway gml:id="wwy_272dfd56-06e9-4cf0-a14b-f3a3cdb9edde">
			<gml:name>中ノ瀬</gml:name>
			<tran:function codeSpace="../../codelists/Waterway_function.xml">01</tran:function>
			<tran:trafficArea>
				<tran:TrafficArea gml:id="traf_b8ceac2f-7a29-4122-bd1f-d1b390a8a0ee">
					<tran:lod2MultiSurface>
						<gml:MultiSurface>
							<gml:surfaceMember>
								<gml:Polygon gml:id="poly_b8ceac2f-7a29-4122-bd1f-d1b390a8a0ee">
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>35.4061666699986 139.77675000000028 0 35.4083333299986 139.76938889000027 0 35.34791666999864 139.7415277800003 0 35.32958332999865 139.72302778000028 0 35.31236110999866 139.7379166700003 0 35.345499999998644 139.7487777800003 0 35.4061666699986 139.77675000000028 0</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:MultiSurface>
					</tran:lod2MultiSurface>
				</tran:TrafficArea>
			</tran:trafficArea>
			<tran:lod0Network>
				<gml:CompositeCurve>
					<gml:curveMember>
						<gml:LineString>
							<gml:posList>35.32097245665506 139.73047301483513 0 35.335465550152016 139.73819184487812 0 35.336558148432935 139.73880427864935 0 35.33740267435338 139.73941459975754 0 35.33846362270621 139.740003003563 0 35.33933976612642 139.74063750167704 0 35.34036913582892 139.74120170904735 0 35.34127682245638 139.74186056428374 0 35.34227469601262 139.74240038298475 0 35.343213839717116 139.74308380411992 0 35.34418031331522 139.74359901054845 0 35.34515081341027 139.74430724153677 0 35.3478358800758 139.7457150936583 0 35.34849310311702 139.74593400967976 0 35.349851509575146 139.7466429451572 0 35.350483458304836 139.7468520445494 0 35.35186714741581 139.747570790706 0 35.35247378985581 139.7477701763458 0 35.353882793588326 139.74849863028584 0 35.35446409775823 139.74868840506758 0 35.355898448119326 139.74942646392196 0 35.3564543820184 139.74960673074628 0 35.35791411098134 139.7503542915956 0 35.35844464262457 139.75052515338055 0 35.35992978218295 139.75128211330994 0 35.360434879574086 139.75144367299094 0 35.36194546173279 139.75220992907919 0 35.36242509286418 139.7523622895761 0 35.36396114963036 139.75313773889553 0 35.36441528249216 139.7532810031566 0 35.36597684585726 139.75406554275125 0 35.366405448455325 139.75419981375302 0 35.36799255044009 139.75499334066046 0 35.36839559074193 139.755118721364 0 35.370008263360404 139.75592113261544 0 35.37038570936731 139.75603772601008 0 35.372023984626786 139.7568489186194 0 35.37237580430169 139.75695682768995 0 35.37403971423881 139.75777669867549 0 35.37436587556941 139.75787602643516 0 35.37605545219607 139.75870447277597 0 35.37635592314971 139.7587953222443 0 35.378071198489096 139.75963224093505 0 35.37834594703989 139.75971471513802 0 35.38008695313554 139.76056000315592 0 35.380335947237235 139.76063420512594 0 35.38210271613494 139.76148775941977 0 35.38232592374802 139.7615537922066 0 35.38411848746885 139.76241550974086 0 35.384315876560535 139.76247347641166 0 35.38613426715488 139.76334325411136 0 35.38630580567206 139.76339325775072 0 35.3881500551836 139.7642709925345 0 35.388295711070846 139.7643131362224 0 35.3901658515636 139.76519872502448 0 35.390285592772216 139.76523311184735 0 35.392181656285445 139.7661264515735 0 35.39227545075542 139.7661531846351 0 35.39419746935772 139.76705417217383 0 35.39426528502674 139.76707335460637 0 35.39621329078 139.76798188682858 0 35.39625509558347 139.7679936217597 0 35.398229120551875 139.76890959555203 0 35.39824488241387 139.76891398612673 0 35.40023516268795 139.7698327760132 0 35.40024444801558 139.76983894952505 0 35.40222621021009 139.7707491064601 0 35.4022590028103 139.7707708229058 0 35.40725477286644 139.77305346962828 0</gml:posList>
						</gml:LineString>
					</gml:curveMember>
				</gml:CompositeCurve>
			</tran:lod0Network>
			<tran:lod1MultiSurface>
				<gml:MultiSurface>
					<gml:surfaceMember>
						<gml:Polygon>
							<gml:exterior>
								<gml:LinearRing>
									<gml:posList>35.4061666699986 139.77675000000028 0 35.4083333299986 139.76938889000027 0 35.34791666999864 139.7415277800003 0 35.32958332999865 139.72302778000028 0 35.31236110999866 139.7379166700003 0 35.345499999998644 139.7487777800003 0 35.4061666699986 139.77675000000028 0</gml:posList>
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
							<gml:surfaceMember xlink:href="#poly_b8ceac2f-7a29-4122-bd1f-d1b390a8a0ee"/>
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>
			</tran:lod2MultiSurface>
			<uro:waterwayDetailAttribute>
				<uro:WaterwayDetailAttribute>
					<uro:routeId>001</uro:routeId>
					<uro:routeDirection codeSpace="../../codelists/WaterwayDetailAttribute_routeDirection.xml">03</uro:routeDirection>
				</uro:WaterwayDetailAttribute>
			</uro:waterwayDetailAttribute>
		</uro:Waterway>
	</core:cityObjectMember>
</core:CityModel>
