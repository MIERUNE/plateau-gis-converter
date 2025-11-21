import type { Feature, FeatureCollection, Geometry } from 'geojson';

// メッシュ番号順で経緯度でのメッシュサイズを定義:(x, y)
type MESH_INFO = {
	parent: MeshLevelKey | null;
	ratio: number;
};

export type MeshLevelKey = '1' | '2' | '3';
export type MeshLevelEntry = {
	name: string;
	minZoom: number;
	maxZoom: number;
	parentLevel: MeshLevelKey | null;
	standardZoom: number;
};
const FIRST_MESH_SIZE: [number, number] = [
	1,
	2 / 3 + Number.EPSILON // temporary hack (https://github.com/MIERUNE/mesh-jp/pull/7)
];

const MESH_SIZES: Record<MeshLevelKey, MESH_INFO> = {
	'1': { parent: null, ratio: 1 },
	'2': { parent: '1', ratio: 8 },
	'3': { parent: '2', ratio: 10 }
};

export function codeToMeshLevel(code: string): MeshLevelKey | null {
	switch (code.length) {
		case 4:
			return '1';
		case 6:
			return '2';
		case 8:
			return '3';
		default:
			return null;
	}
}

function get_meshsize(meshnum: MeshLevelKey): [number, number] {
	const meshinfo = MESH_SIZES[meshnum];
	if (meshinfo.parent === null) {
		return FIRST_MESH_SIZE;
	}
	const parentMeshsize = get_meshsize(meshinfo.parent);
	const meshsize: [number, number] = [
		parentMeshsize[0] / meshinfo.ratio,
		parentMeshsize[1] / meshinfo.ratio
	];
	return meshsize;
}

export type MeshPatchProperties = { code: string; selected: boolean };
export type MeshPatchFeature = Feature<Geometry, MeshPatchProperties>;
export type MeshPatchFeatureCollection = FeatureCollection<Geometry, MeshPatchProperties>;

// メッシュコード生成範囲
const MINIMUM_LON = 122.0;
const MAXIMUM_LON = 154.0;
const MINIMUM_LAT = 20.0;
const MAXIMUM_LAT = 46.0;

/** 実際に利用可能な1次メッシュコードの一覧 */
// prettier-ignore
const AVAILABLE_PRIMARY_CODES = Object.freeze(new Set([
	'3036',
	'3622',
	'3623',
	'3624',
	'3631',
	'3641',
	'3653',
	'3724',
	'3725',
	'3741',
	'3823',
	'3824',
	'3831',
	'3841',
	'3926',
	'3927',
	'3928',
	'3942',
	'4027',
	'4028',
	'4040',
	'4042',
	'4128',
	'4129',
	'4142',
	'4229',
	'4230',
	'4328',
	'4329',
	'4429',
	'4440',
	'4529',
	'4530',
	'4531',
	'4540',
	'4629',
	'4630',
	'4631',
	'4728',
	'4729',
	'4730',
	'4731',
	'4739',
	'4740',
	'4828',
	'4829',
	'4830',
	'4831',
	'4839',
	'4928',
	'4929',
	'4930',
	'4931',
	'4932',
	'4933',
	'4934',
	'4939',
	'5029',
	'5030',
	'5031',
	'5032',
	'5033',
	'5034',
	'5035',
	'5036',
	'5038',
	'5039',
	'5129',
	'5130',
	'5131',
	'5132',
	'5133',
	'5134',
	'5135',
	'5136',
	'5137',
	'5138',
	'5139',
	'5229',
	'5231',
	'5232',
	'5233',
	'5234',
	'5235',
	'5236',
	'5237',
	'5238',
	'5239',
	'5240',
	'5332',
	'5333',
	'5334',
	'5335',
	'5336',
	'5337',
	'5338',
	'5339',
	'5340',
	'5432',
	'5433',
	'5435',
	'5436',
	'5437',
	'5438',
	'5439',
	'5440',
	'5531',
	'5536',
	'5537',
	'5538',
	'5539',
	'5540',
	'5541',
	'5636',
	'5637',
	'5638',
	'5639',
	'5640',
	'5641',
	'5738',
	'5739',
	'5740',
	'5741',
	'5839',
	'5840',
	'5841',
	'5939',
	'5940',
	'5941',
	'5942',
	'6039',
	'6040',
	'6041',
	'6139',
	'6140',
	'6141',
	'6239',
	'6240',
	'6241',
	'6243',
	'6339',
	'6340',
	'6341',
	'6342',
	'6343',
	'6439',
	'6440',
	'6441',
	'6442',
	'6443',
	'6444',
	'6445',
	'6540',
	'6541',
	'6542',
	'6543',
	'6544',
	'6545',
	'6546',
	'6641',
	'6642',
	'6643',
	'6644',
	'6645',
	'6646',
	'6647',
	'6740',
	'6741',
	'6742',
	'6747',
	'6748',
	'6840',
	'6841',
	'6842',
	'6847',
	'6848'
]));

function get_start_offset(meshnum: MeshLevelKey, lonlat: [number, number]): [number, number] {
	const meshsize = get_meshsize(meshnum);

	let x_offset = 0;
	while (lonlat[0] >= MINIMUM_LON + meshsize[0] * (x_offset + 1)) {
		x_offset += 1;
	}

	let y_offset = 0;
	while (lonlat[1] >= MINIMUM_LAT + meshsize[1] * (y_offset + 1)) {
		y_offset += 1;
	}

	return [x_offset, y_offset];
}

function get_end_offset(meshnum: MeshLevelKey, lonlat: [number, number]): [number, number] {
	const meshsize = get_meshsize(meshnum);

	let x_offset = 0;
	while (lonlat[0] <= MAXIMUM_LON - meshsize[0] * (x_offset + 1)) {
		x_offset += 1;
	}
	let y_offset = 0;
	while (lonlat[1] <= MAXIMUM_LAT - meshsize[1] * (y_offset + 1)) {
		y_offset += 1;
	}

	return [x_offset, y_offset];
}

function getMeshCode(meshnum: MeshLevelKey, x: number, y: number): string {
	const ratio = MESH_SIZES[meshnum].ratio;
	let meshcode: string;
	if (meshnum === '1') {
		return '';
	} else {
		meshcode = String(y % ratio) + String(x % ratio);
	}

	const parent = MESH_SIZES[meshnum].parent;
	if (parent === null) {
		return meshcode;
	} else {
		return getMeshCode(parent, Math.floor(x / ratio), Math.floor(y / ratio)) + meshcode;
	}
}

export function makeMeshPatch(
	meshnum: MeshLevelKey,
	x: number,
	y: number,
	selectedMeshcodes: string[] = []
): MeshPatchFeature | null {
	const meshsize = get_meshsize(meshnum);
	const left_lon = MINIMUM_LON + x * meshsize[0];
	const bottom_lat = MINIMUM_LAT + y * meshsize[1];
	const right_lon = MINIMUM_LON + (x + 1) * meshsize[0];
	const top_lat = MINIMUM_LAT + (y + 1) * meshsize[1];

	const base_meshcode = String(Math.floor(bottom_lat * 1.5)) + String(Math.floor(left_lon) - 100);
	if (!AVAILABLE_PRIMARY_CODES.has(base_meshcode)) {
		return null;
	}

	const code = base_meshcode + getMeshCode(meshnum, x, y);
	return {
		type: 'Feature',
		id: code,
		geometry: {
			type: 'Polygon',
			coordinates: [
				[
					[left_lon, bottom_lat],
					[left_lon, top_lat],
					[right_lon, top_lat],
					[right_lon, bottom_lat],
					[left_lon, bottom_lat]
				]
			]
		},
		properties: {
			code,
			selected: selectedMeshcodes.includes(code)
		}
	};
}

export function makeMeshPatches(
	meshnum: MeshLevelKey,
	extent: [number, number, number, number] | null = null,
	selectedMeshcodes: string[] = []
): MeshPatchFeatureCollection {
	const [x_size, y_size] = get_meshsize(meshnum);
	const x_mesh_count = Math.ceil((MAXIMUM_LON - MINIMUM_LON) / x_size);
	const y_mesh_count = Math.ceil((MAXIMUM_LAT - MINIMUM_LAT) / y_size);

	let start_offset = [0, 0];
	let end_offset = [0, 0];
	if (extent) {
		let min_lon = 0;
		let max_lon = 0;
		if (extent[0] < extent[2]) {
			min_lon = extent[0];
			max_lon = extent[2];
		} else {
			min_lon = extent[2];
			max_lon = extent[0];
		}

		let min_lat = 0;
		let max_lat = 0;
		if (extent[1] < extent[3]) {
			min_lat = extent[1];
			max_lat = extent[3];
		} else {
			min_lat = extent[3];
			max_lat = extent[1];
		}

		const start_lonlat: [number, number] = [min_lon, min_lat];
		const end_lonlat: [number, number] = [max_lon, max_lat];

		start_offset = get_start_offset(meshnum, start_lonlat);
		end_offset = get_end_offset(meshnum, end_lonlat);
	}

	const geojson: MeshPatchFeatureCollection = {
		type: 'FeatureCollection',
		features: []
	};

	for (let y = start_offset[1]; y < y_mesh_count - end_offset[1]; y++) {
		for (let x = start_offset[0]; x < x_mesh_count - end_offset[0]; x++) {
			const f = makeMeshPatch(meshnum, x, y, selectedMeshcodes);
			if (f) geojson.features.push(f);
		}
	}
	return geojson;
}
