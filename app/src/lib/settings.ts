const filetypeOptions: Record<string, { label: string; extensions: string[] }> = {
	geojson: {
		label: 'GeoJSON',
		extensions: ['json', 'geojson']
	},
	gpkg: {
		label: 'GeoPackage',
		extensions: ['gpkg']
	},
	mvt: {
		label: 'Vector Tiles',
		extensions: ['']
	},
	shapefile: {
		label: 'Shapefile',
		extensions: ['shp']
	},
	serde: {
		label: 'Serde',
		extensions: ['']
	}
};

const crsOptions = [
	{ value: 'EPSG:6678', label: 'JGD2011 / Japan Plane Rectangular CS X' },
	{ value: 'EPSG:4326', label: 'WGS 84' },
	{ value: 'EPSG:3857', label: 'Web Mercator' }
];

export { filetypeOptions, crsOptions };
