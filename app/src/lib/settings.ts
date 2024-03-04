type CrsOption = { value: number; label: string };

const filetypeOptions: Record<string, { label: string; extensions: string[]; crs: CrsOption[] }> = {
	geojson: {
		label: 'GeoJSON',
		extensions: [],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	gpkg: {
		label: 'GeoPackage',
		extensions: ['gpkg'],
		crs: [
			{ value: 4979, label: 'WGS84' },
			{ value: 10162, label: 'JGD 2011 / 平面直角座標系 I' },
			{ value: 10163, label: 'JGD 2011 / 平面直角座標系 II' },
			{ value: 10164, label: 'JGD 2011 / 平面直角座標系 III' },
			{ value: 10165, label: 'JGD 2011 / 平面直角座標系 IV' },
			{ value: 10166, label: 'JGD 2011 / 平面直角座標系 V' },
			{ value: 10167, label: 'JGD 2011 / 平面直角座標系 VI' },
			{ value: 10168, label: 'JGD 2011 / 平面直角座標系 VII' },
			{ value: 10169, label: 'JGD 2011 / 平面直角座標系 VIII' },
			{ value: 10170, label: 'JGD 2011 / 平面直角座標系 IX' },
			{ value: 10171, label: 'JGD 2011 / 平面直角座標系 X' },
			{ value: 10172, label: 'JGD 2011 / 平面直角座標系 XI' },
			{ value: 10173, label: 'JGD 2011 / 平面直角座標系 XII' },
			{ value: 10174, label: 'JGD 2011 / 平面直角座標系 XIII' }
		]
	},
	mvt: {
		label: 'Vector Tiles',
		extensions: [''],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	czml: {
		label: 'CZML',
		extensions: ['json'],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	cesiumtiles: {
		label: '3D Tiles',
		extensions: [''],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	kml: {
		label: 'KML',
		extensions: ['kml'],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	shapefile: {
		label: 'Shapefile',
		extensions: ['shp'],
		crs: [
			{ value: 4979, label: 'WGS84' }
			// TODO: more CRS options
		]
	},
	ply: {
		label: 'PLY',
		extensions: ['ply'],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	gltf: {
		label: 'glTF',
		extensions: [''],
		crs: [{ value: 4979, label: 'WGS84' }]
	},
	serde: {
		label: 'Serde',
		extensions: [''],
		crs: [{ value: 4979, label: 'WGS84' }]
	}
};

export { filetypeOptions };
