type epsgOption = { value: number; label: string };

const filetypeOptions: Record<string, { label: string; extensions: string[]; epsg: epsgOption[] }> =
	{
		gpkg: {
			label: 'GeoPackage',
			extensions: ['gpkg'],
			epsg: [
				{ value: 4979, label: 'WGS84' },
				{ value: 6669, label: 'JGD 2011 / 平面直角座標系 I' },
				{ value: 6670, label: 'JGD 2011 / 平面直角座標系 II' },
				{ value: 6671, label: 'JGD 2011 / 平面直角座標系 III' },
				{ value: 6672, label: 'JGD 2011 / 平面直角座標系 IV' },
				{ value: 6673, label: 'JGD 2011 / 平面直角座標系 V' },
				{ value: 6674, label: 'JGD 2011 / 平面直角座標系 VI' },
				{ value: 6675, label: 'JGD 2011 / 平面直角座標系 VII' },
				{ value: 6676, label: 'JGD 2011 / 平面直角座標系 VIII' },
				{ value: 6677, label: 'JGD 2011 / 平面直角座標系 IX' },
				{ value: 6678, label: 'JGD 2011 / 平面直角座標系 X' },
				{ value: 6679, label: 'JGD 2011 / 平面直角座標系 XI' },
				{ value: 6680, label: 'JGD 2011 / 平面直角座標系 XII' },
				{ value: 6681, label: 'JGD 2011 / 平面直角座標系 XIII' },
				{ value: 6682, label: 'JGD 2011 / 平面直角座標系 XIV' },
				{ value: 6683, label: 'JGD 2011 / 平面直角座標系 XV' },
				{ value: 6684, label: 'JGD 2011 / 平面直角座標系 XVI' },
				{ value: 6685, label: 'JGD 2011 / 平面直角座標系 XVII' },
				{ value: 6686, label: 'JGD 2011 / 平面直角座標系 XVIII' },
				{ value: 6687, label: 'JGD 2011 / 平面直角座標系 XIX' }
			]
		},
		geojson: {
			label: 'GeoJSON',
			extensions: [],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},

		cesiumtiles: {
			label: '3D Tiles',
			extensions: [''],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},
		mvt: {
			label: 'Vector Tiles (MVT)',
			extensions: [''],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},

		czml: {
			label: 'CZML',
			extensions: ['json'],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},

		kml: {
			label: 'KML',
			extensions: ['kml'],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},
		gltf: {
			label: 'glTF',
			extensions: [''],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},
		ply: {
			label: 'PLY',
			extensions: ['ply'],
			epsg: [{ value: 4979, label: 'WGS84' }]
		},

		shapefile: {
			label: 'Shapefile',
			extensions: [''],
			epsg: [
				{ value: 4979, label: 'WGS84' },
				{ value: 6669, label: 'JGD 2011 / 平面直角座標系 I' },
				{ value: 6670, label: 'JGD 2011 / 平面直角座標系 II' },
				{ value: 6671, label: 'JGD 2011 / 平面直角座標系 III' },
				{ value: 6672, label: 'JGD 2011 / 平面直角座標系 IV' },
				{ value: 6673, label: 'JGD 2011 / 平面直角座標系 V' },
				{ value: 6674, label: 'JGD 2011 / 平面直角座標系 VI' },
				{ value: 6675, label: 'JGD 2011 / 平面直角座標系 VII' },
				{ value: 6676, label: 'JGD 2011 / 平面直角座標系 VIII' },
				{ value: 6677, label: 'JGD 2011 / 平面直角座標系 IX' },
				{ value: 6678, label: 'JGD 2011 / 平面直角座標系 X' },
				{ value: 6679, label: 'JGD 2011 / 平面直角座標系 XI' },
				{ value: 6680, label: 'JGD 2011 / 平面直角座標系 XII' },
				{ value: 6681, label: 'JGD 2011 / 平面直角座標系 XIII' },
				{ value: 6682, label: 'JGD 2011 / 平面直角座標系 XIV' },
				{ value: 6683, label: 'JGD 2011 / 平面直角座標系 XV' },
				{ value: 6684, label: 'JGD 2011 / 平面直角座標系 XVI' },
				{ value: 6685, label: 'JGD 2011 / 平面直角座標系 XVII' },
				{ value: 6686, label: 'JGD 2011 / 平面直角座標系 XVIII' },
				{ value: 6687, label: 'JGD 2011 / 平面直角座標系 XIX' }
			]
		}
	};

export { filetypeOptions };
