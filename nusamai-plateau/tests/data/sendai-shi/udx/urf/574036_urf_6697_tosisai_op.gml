<?xml version='1.0' encoding='utf-8'?>
<core:CityModel xmlns:core="http://www.opengis.net/citygml/2.0" xmlns:gml="http://www.opengis.net/gml" xmlns:urf="https://www.geospatial.jp/iur/urf/3.1" xmlns:uro="https://www.geospatial.jp/iur/uro/3.1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.opengis.net/citygml/2.0 http://schemas.opengis.net/citygml/2.0/cityGMLBase.xsd http://www.opengis.net/citygml/profiles/base/2.0 http://schemas.opengis.net/citygml/profiles/base/2.0/CityGML.xsd https://www.geospatial.jp/iur/uro/3.1 ../../schemas/iur/uro/3.1/urbanObject.xsd https://www.geospatial.jp/iur/urf/3.1 ../../schemas/iur/urf/3.1/urbanFunction.xsd">
  <gml:boundedBy>
    <gml:Envelope srsName="http://www.opengis.net/def/crs/EPSG/0/6697" srsDimension="3">
      <gml:lowerCorner>38.259986275843026 140.87405188736534 0</gml:lowerCorner>
      <gml:upperCorner>38.26097058609435 140.87507409548869 0</gml:upperCorner>
    </gml:Envelope>
  </gml:boundedBy>
  <core:cityObjectMember>
    <urf:SpecialUrbanRenaissanceDistrict gml:id="urf_ac645479-546f-4b17-8868-a7e714af17aa">
      <gml:name>一番町三丁目南地区</gml:name>
      <core:creationDate>2023-03-22</core:creationDate>
      <urf:function codeSpace="../../codelists/Common_districtsAndZonesType.xml">21</urf:function>
      <urf:validFrom>2020-09-16</urf:validFrom>
      <urf:validFromType codeSpace="../../codelists/Common_validType.xml">3</urf:validFromType>
      <urf:custodian>仙台市</urf:custodian>
      <urf:notificationNumber>市告示第434号</urf:notificationNumber>
      <urf:finalNotificationDate>2020-09-16</urf:finalNotificationDate>
      <urf:finalNotificationNumber>市告示第434号</urf:finalNotificationNumber>
      <urf:prefecture codeSpace="../../codelists/Common_localPublicAuthorities.xml">04</urf:prefecture>
      <urf:city codeSpace="../../codelists/Common_localPublicAuthorities.xml">04100</urf:city>
      <urf:lod1MultiSurface>
        <gml:MultiSurface>
          <gml:surfaceMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>38.260874472799564 140.87408962337489 0 38.26052697156933 140.87426754637244 0 38.26050418625536 140.87420937020417 0 38.260501797631605 140.87419680031897 0 38.26047579634581 140.87407737260412 0 38.26047031552352 140.87405188736534 0 38.259986275843026 140.87418709464825 0 38.26013926086013 140.87507409548869 0 38.26097058609435 140.87458608472465 0 38.260874472799564 140.87408962337489 0</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:surfaceMember>
        </gml:MultiSurface>
      </urf:lod1MultiSurface>
      <urf:dataQualityAttribute>
        <uro:DataQualityAttribute>
          <uro:geometrySrcDescLod1 codeSpace="../../codelists/DataQualityAttribute_geometrySrcDesc.xml">202</uro:geometrySrcDescLod1>
          <uro:thematicSrcDesc codeSpace="../../codelists/DataQualityAttribute_thematicSrcDesc.xml">202</uro:thematicSrcDesc>
        </uro:DataQualityAttribute>
      </urf:dataQualityAttribute>
    <urf:useToBeInduced>-</urf:useToBeInduced>
      <urf:maximumFloorAreaRate>1050</urf:maximumFloorAreaRate>
      <urf:minimumFloorAreaRate>600</urf:minimumFloorAreaRate>
      <urf:maximumBuildingCoverageRate>70</urf:maximumBuildingCoverageRate>
      <urf:minimumBuildingArea uom="m2">1000</urf:minimumBuildingArea>
      <urf:maximumBuildingHeight>高層部100ｍ、低層部30ｍ</urf:maximumBuildingHeight>
      <urf:setbackSize>2ｍ、3ｍ、5.5ｍ</urf:setbackSize>
      </urf:SpecialUrbanRenaissanceDistrict>
  </core:cityObjectMember>
</core:CityModel>