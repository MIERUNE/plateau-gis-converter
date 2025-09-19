export interface MeshBounds {
	north: number;
	south: number;
	east: number;
	west: number;
}

export interface PolygonCoordinates {
	type: 'Polygon';
	coordinates: number[][][];
}

const FIRST_MESH_LAT_FACTOR = 1.5;
const FIRST_MESH_LNG_OFFSET = 100;
const FIRST_MESH_LAT_SIZE = 2 / 3; // 40分 = 2/3度
const FIRST_MESH_LNG_SIZE = 1; // 1度

const SECOND_MESH_LAT_SIZE = 5 / 60; // 5分
const SECOND_MESH_LNG_SIZE = 7.5 / 60; // 7分30秒

const THIRD_MESH_LAT_SIZE = 30 / 3600; // 30秒
const THIRD_MESH_LNG_SIZE = 45 / 3600; // 45秒

export function calculateFirstMeshBounds(meshcode: string): MeshBounds {
	if (meshcode.length < 4) {
		throw new Error('第1次地域区画は4桁以上のメッシュコードが必要です');
	}

	const firstMeshLat = parseInt(meshcode.substring(0, 2));
	const firstMeshLng = parseInt(meshcode.substring(2, 4));

	if (isNaN(firstMeshLat) || isNaN(firstMeshLng)) {
		throw new Error('無効なメッシュコード形式です');
	}

	const south = firstMeshLat / FIRST_MESH_LAT_FACTOR;
	const west = firstMeshLng + FIRST_MESH_LNG_OFFSET;
	const north = south + FIRST_MESH_LAT_SIZE;
	const east = west + FIRST_MESH_LNG_SIZE;
	return { north, south, east, west };
}

function parseMeshCode(meshcode: string) {
	const ab = parseInt(meshcode.substring(0, 2));
	const cd = parseInt(meshcode.substring(2, 4));
	const e = meshcode.length > 4 ? parseInt(meshcode.substring(4, 5)) : 0;
	const f = meshcode.length > 5 ? parseInt(meshcode.substring(5, 6)) : 0;
	const g = meshcode.length > 6 ? parseInt(meshcode.substring(6, 7)) : 0;
	const h = meshcode.length > 7 ? parseInt(meshcode.substring(7, 8)) : 0;

	if (isNaN(ab) || isNaN(cd) || isNaN(e) || isNaN(f) || isNaN(g) || isNaN(h)) {
		throw new Error('無効なメッシュコード形式です');
	}

	return { ab, cd, e, f, g, h };
}

/**
 * 6桁メッシュコード（第2次地域区画）から境界座標を計算
 * JIS X 0410:2002 正確な計算式
 * @param meshcode 6桁のメッシュコード（例：533936）
 * @returns メッシュの境界座標
 */
function calculateMesh6Bounds(meshcode: string): MeshBounds {
	if (meshcode.length !== 6) {
		throw new Error('6桁メッシュコードが必要です');
	}

	const { ab, cd, e, f } = parseMeshCode(meshcode);

	const lat1 = ab / FIRST_MESH_LAT_FACTOR;
	const lon1 = cd + FIRST_MESH_LNG_OFFSET;

	const lat2 = e * SECOND_MESH_LAT_SIZE;
	const lon2 = f * SECOND_MESH_LNG_SIZE;

	const south = lat1 + lat2;
	const west = lon1 + lon2;
	const north = south + SECOND_MESH_LAT_SIZE;
	const east = west + SECOND_MESH_LNG_SIZE;

	return { north, south, east, west };
}

/**
 * 8桁メッシュコード（第3次地域区画・基準地域メッシュ）から境界座標を計算
 * JIS X 0410:2002 正確な計算式
 * @param meshcode 8桁のメッシュコード（例：53393946）
 * @returns メッシュの境界座標
 */
function calculateMesh8Bounds(meshcode: string): MeshBounds {
	if (meshcode.length !== 8) {
		throw new Error('8桁メッシュコードが必要です');
	}

	const { ab, cd, e, f, g, h } = parseMeshCode(meshcode);

	const lat1 = ab / FIRST_MESH_LAT_FACTOR;
	const lon1 = cd + FIRST_MESH_LNG_OFFSET;

	const lat2 = e * SECOND_MESH_LAT_SIZE;
	const lon2 = f * SECOND_MESH_LNG_SIZE;

	const lat3 = g * THIRD_MESH_LAT_SIZE;
	const lon3 = h * THIRD_MESH_LNG_SIZE;

	const south = lat1 + lat2 + lat3;
	const west = lon1 + lon2 + lon3;
	const north = south + THIRD_MESH_LAT_SIZE;
	const east = west + THIRD_MESH_LNG_SIZE;

	return { north, south, east, west };
}

/**
 * メッシュコードから境界座標を計算
 * @param meshcode 6桁（第2次）または8桁（第3次・基準地域メッシュ）のメッシュコード
 * @returns メッシュの境界座標
 */
export function calculateMeshBounds(meshcode: string): MeshBounds {
	if (meshcode.length === 6) {
		return calculateMesh6Bounds(meshcode); // 第2次地域区画（約10km×10km）
	} else if (meshcode.length === 8) {
		return calculateMesh8Bounds(meshcode); // 第3次地域区画・基準地域メッシュ（約1km×1km）
	} else {
		throw new Error('メッシュコードは6桁（第2次）または8桁（第3次）である必要があります');
	}
}

/**
 * メッシュ境界からGeoJSONポリゴン座標を生成
 * @param bounds メッシュの境界座標
 * @returns GeoJSONポリゴン座標
 */
export function boundsToPolygon(bounds: MeshBounds): PolygonCoordinates {
	const { north, south, east, west } = bounds;

	// 時計回りにポリゴンの頂点を定義
	const coordinates = [
		[
			[west, south], // 南西
			[east, south], // 南東
			[east, north], // 北東
			[west, north], // 北西
			[west, south] // 閉じる
		]
	];

	return {
		type: 'Polygon',
		coordinates
	};
}

/**
 * メッシュコードからGeoJSONポリゴンを生成
 * @param meshcode 6桁または8桁のメッシュコード
 * @returns GeoJSONポリゴン座標
 */
export function meshcodeToPolygon(meshcode: string): PolygonCoordinates {
	const bounds = calculateMeshBounds(meshcode);
	return boundsToPolygon(bounds);
}

/**
 * メッシュコードから中心座標を計算
 * @param meshcode 6桁または8桁のメッシュコード
 * @returns [経度, 緯度]
 */
export function meshcodeToCenter(meshcode: string): [number, number] {
	const bounds = calculateMeshBounds(meshcode);
	const centerLon = (bounds.west + bounds.east) / 2;
	const centerLat = (bounds.south + bounds.north) / 2;
	return [centerLon, centerLat];
}
