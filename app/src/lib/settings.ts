type epsgOption = { value: number; label: string };

const filetypeOptions: Record<string, { label: string; extensions: string[]; epsg: epsgOption[] }> =
	{
		gpkg: {
			label: 'GeoPackage',
			extensions: ['gpkg'],
			epsg: [
				{ value: 4979, label: 'WGS 84 (EPSG:4979)' },
				{ value: 3857, label: 'Web Mercator (EPSG:3857)' },
				{ value: 6669, label: 'JGD2011 / 平面直角座標系 I (EPSG:6669)' },
				{ value: 6670, label: 'JGD2011 / 平面直角座標系 II (EPSG:6670)' },
				{ value: 6671, label: 'JGD2011 / 平面直角座標系 III (EPSG:6671)' },
				{ value: 6672, label: 'JGD2011 / 平面直角座標系 IV (EPSG:6672)' },
				{ value: 6673, label: 'JGD2011 / 平面直角座標系 V (EPSG:6673)' },
				{ value: 6674, label: 'JGD2011 / 平面直角座標系 VI (EPSG:6674)' },
				{ value: 6675, label: 'JGD2011 / 平面直角座標系 VII (EPSG:6675)' },
				{ value: 6676, label: 'JGD2011 / 平面直角座標系 VIII (EPSG:6676)' },
				{ value: 6677, label: 'JGD2011 / 平面直角座標系 IX (EPSG:6677)' },
				{ value: 6678, label: 'JGD2011 / 平面直角座標系 X (EPSG:6678)' },
				{ value: 6679, label: 'JGD2011 / 平面直角座標系 XI (EPSG:6679)' },
				{ value: 6680, label: 'JGD2011 / 平面直角座標系 XII (EPSG:6680)' },
				{ value: 6681, label: 'JGD2011 / 平面直角座標系 XIII (EPSG:6681)' },
				{ value: 6682, label: 'JGD2011 / 平面直角座標系 XIV (EPSG:6682)' },
				{ value: 6683, label: 'JGD2011 / 平面直角座標系 XV (EPSG:6683)' },
				{ value: 6684, label: 'JGD2011 / 平面直角座標系 XVI (EPSG:6684)' },
				{ value: 6685, label: 'JGD2011 / 平面直角座標系 XVII (EPSG:6685)' },
				{ value: 6686, label: 'JGD2011 / 平面直角座標系 XVIII (EPSG:6686)' },
				{ value: 6687, label: 'JGD2011 / 平面直角座標系 XIX (EPSG:6687)' },
				{ value: 10162, label: 'JGD2011 / 平面直角座標系 I + 標高 (EPSG:10162)' },
				{ value: 10163, label: 'JGD2011 / 平面直角座標系 II + 標高 (EPSG:10163)' },
				{ value: 10164, label: 'JGD2011 / 平面直角座標系 III + 標高 (EPSG:10164)' },
				{ value: 10165, label: 'JGD2011 / 平面直角座標系 IV + 標高 (EPSG:10165)' },
				{ value: 10166, label: 'JGD2011 / 平面直角座標系 V + 標高 (EPSG:10166)' },
				{ value: 10167, label: 'JGD2011 / 平面直角座標系 VI + 標高 (EPSG:10167)' },
				{ value: 10168, label: 'JGD2011 / 平面直角座標系 VII + 標高 (EPSG:10168)' },
				{ value: 10169, label: 'JGD2011 / 平面直角座標系 VIII + 標高 (EPSG:10169)' },
				{ value: 10170, label: 'JGD2011 / 平面直角座標系 IX + 標高 (EPSG:10170)' },
				{ value: 10171, label: 'JGD2011 / 平面直角座標系 X + 標高 (EPSG:10171)' },
				{ value: 10172, label: 'JGD2011 / 平面直角座標系 XI + 標高 (EPSG:10172)' },
				{ value: 10173, label: 'JGD2011 / 平面直角座標系 XII + 標高 (EPSG:10173)' },
				{ value: 10174, label: 'JGD2011 / 平面直角座標系 XIII + 標高 (EPSG:10174)' }
			]
		},
		geojson: {
			label: 'GeoJSON',
			extensions: [],
			epsg: [{ value: 4979, label: 'WGS 84 (EPSG:4979)' }]
		},

		cesiumtiles: {
			label: '3D Tiles',
			extensions: [''],
			epsg: [
				{ value: 4979, label: 'WGS 84 (EPSG:4979) (楕円体高)' },
				{ value: 6697, label: '特殊: JGD2011 (EPSG:6697) (標高)' }
			]
		},
		mvt: {
			label: 'Vector Tiles (MVT)',
			extensions: [''],
			epsg: [{ value: 4979, label: 'WGS 84' }]
		},

		czml: {
			label: 'CZML',
			extensions: ['json'],
			epsg: [{ value: 4979, label: 'WGS 84 (EPSG:4979)' }]
		},

		kml: {
			label: 'KML',
			extensions: ['kml'],
			epsg: [{ value: 6697, label: 'JGD2011 (EPSG:6697)' }]
		},
		gltf: {
			label: 'glTF',
			extensions: [''],
			epsg: [{ value: 4979, label: 'WGS 84 (EPSG:4979)' }]
		},
		ply: {
			label: 'PLY',
			extensions: ['ply'],
			epsg: [{ value: 4979, label: 'WGS 84 (EPSG:4979)' }]
		},

		shapefile: {
			label: 'Shapefile',
			extensions: [''],
			epsg: [
				{ value: 4979, label: 'WGS 84 (EPSG:4979)' },
				{ value: 3857, label: 'Web Mercator (EPSG:3857)' },
				{ value: 6669, label: 'JGD2011 / 平面直角座標系 I (EPSG:6669)' },
				{ value: 6670, label: 'JGD2011 / 平面直角座標系 II (EPSG:6670)' },
				{ value: 6671, label: 'JGD2011 / 平面直角座標系 III (EPSG:6671)' },
				{ value: 6672, label: 'JGD2011 / 平面直角座標系 IV (EPSG:6672)' },
				{ value: 6673, label: 'JGD2011 / 平面直角座標系 V (EPSG:6673)' },
				{ value: 6674, label: 'JGD2011 / 平面直角座標系 VI (EPSG:6674)' },
				{ value: 6675, label: 'JGD2011 / 平面直角座標系 VII (EPSG:6675)' },
				{ value: 6676, label: 'JGD2011 / 平面直角座標系 VIII (EPSG:6676)' },
				{ value: 6677, label: 'JGD2011 / 平面直角座標系 IX (EPSG:6677)' },
				{ value: 6678, label: 'JGD2011 / 平面直角座標系 X (EPSG:6678)' },
				{ value: 6679, label: 'JGD2011 / 平面直角座標系 XI (EPSG:6679)' },
				{ value: 6680, label: 'JGD2011 / 平面直角座標系 XII (EPSG:6680)' },
				{ value: 6681, label: 'JGD2011 / 平面直角座標系 XIII (EPSG:6681)' },
				{ value: 6682, label: 'JGD2011 / 平面直角座標系 XIV (EPSG:6682)' },
				{ value: 6683, label: 'JGD2011 / 平面直角座標系 XV (EPSG:6683)' },
				{ value: 6684, label: 'JGD2011 / 平面直角座標系 XVI (EPSG:6684)' },
				{ value: 6685, label: 'JGD2011 / 平面直角座標系 XVII (EPSG:6685)' },
				{ value: 6686, label: 'JGD2011 / 平面直角座標系 XVIII (EPSG:6686)' },
				{ value: 6687, label: 'JGD2011 / 平面直角座標系 XIX (EPSG:6687)' },
				{ value: 10162, label: 'JGD2011 / 平面直角座標系 I + 標高 (EPSG:10162)' },
				{ value: 10163, label: 'JGD2011 / 平面直角座標系 II + 標高 (EPSG:10163)' },
				{ value: 10164, label: 'JGD2011 / 平面直角座標系 III + 標高 (EPSG:10164)' },
				{ value: 10165, label: 'JGD2011 / 平面直角座標系 IV + 標高 (EPSG:10165)' },
				{ value: 10166, label: 'JGD2011 / 平面直角座標系 V + 標高 (EPSG:10166)' },
				{ value: 10167, label: 'JGD2011 / 平面直角座標系 VI + 標高 (EPSG:10167)' },
				{ value: 10168, label: 'JGD2011 / 平面直角座標系 VII + 標高 (EPSG:10168)' },
				{ value: 10169, label: 'JGD2011 / 平面直角座標系 VIII + 標高 (EPSG:10169)' },
				{ value: 10170, label: 'JGD2011 / 平面直角座標系 IX + 標高 (EPSG:10170)' },
				{ value: 10171, label: 'JGD2011 / 平面直角座標系 X + 標高 (EPSG:10171)' },
				{ value: 10172, label: 'JGD2011 / 平面直角座標系 XI + 標高 (EPSG:10172)' },
				{ value: 10173, label: 'JGD2011 / 平面直角座標系 XII + 標高 (EPSG:10173)' },
				{ value: 10174, label: 'JGD2011 / 平面直角座標系 XIII + 標高 (EPSG:10174)' }
			]
		}
	};

export { filetypeOptions };
