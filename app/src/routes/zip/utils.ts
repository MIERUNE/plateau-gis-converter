// Structure: { meshcode: { type: [filepath1, filepath2, ...] } }
export type MeshcodeData = Record<string, Record<string, string[]>>;

export type RemoteFileInfo = {
	meshcode: string;
	featureType: string;
	url: string;
	maxLod?: number;
	fileSize?: number;
	features?: number;
};

export type RemoteMeshcodeData = Record<string, Record<string, RemoteFileInfo[]>>;

export type FeatureTypeLookup = Record<string, string>;

// Type labels in Japanese
export const TYPE_LABELS: Record<string, string> = {
	bldg: '建築物',
	tran: '交通（道路）',
	rwy: '交通（鉄道）',
	trk: '交通（徒歩道）',
	squr: '交通（広場）',
	wwy: '交通（航路）',
	luse: '土地利用',
	fld: '洪水浸水想定区域',
	tnm: '津波浸水想定',
	htd: '高潮浸水想定区域',
	ifld: '内水浸水想定区域',
	rfld: 'ため池ハザードマップ',
	lsld: '土砂災害警戒区域',
	urf: '都市計画決定情報',
	brid: '橋梁',
	tnl: 'トンネル',
	cons: 'その他の構造物',
	frn: '都市設備',
	unf: '地下埋設物',
	ubld: '地下街',
	veg: '植生',
	dem: '地形',
	wtr: '水部',
	area: '区域',
	gen: '汎用都市オブジェクト',
	app: 'アピアランス',
	ext: '拡張製品仕様書で追加した地物'
};

export function getTypeLabel(type: string): string {
	if (type in TYPE_LABELS) {
		return TYPE_LABELS[type];
	}
	return type;
}

export function mergeRemoteMeshcodeData(
	target: RemoteMeshcodeData,
	source: RemoteMeshcodeData
): RemoteMeshcodeData {
	const merged: RemoteMeshcodeData = { ...target };

	for (const [meshcode, types] of Object.entries(source)) {
		const existingTypes = merged[meshcode] ? { ...merged[meshcode] } : {};
		for (const [featureType, files] of Object.entries(types)) {
			const targetList = existingTypes[featureType] ? [...existingTypes[featureType]] : [];
			const existingUrls = new Set(targetList.map((f) => f.url));
			for (const file of files) {
				if (!existingUrls.has(file.url)) {
					targetList.push(file);
				}
			}
			existingTypes[featureType] = targetList;
		}
		merged[meshcode] = existingTypes;
	}

	return merged;
}

const JAPAN_DEFAULT_BOUNDS = {
	west: 122,
	south: 20,
	east: 154,
	north: 46
} as const;

function padNumber(value: number, length: number): string {
	return value.toString().padStart(length, '0');
}

function calculateSecondMeshBounds(meshcode: string) {
	const latIndex = parseInt(meshcode.substring(0, 2), 10);
	const lonIndex = parseInt(meshcode.substring(2, 4), 10);
	const latSubIndex = parseInt(meshcode.substring(4, 5), 10);
	const lonSubIndex = parseInt(meshcode.substring(5, 6), 10);

	const baseLat = latIndex / 1.5;
	const baseLon = lonIndex + 100;

	const lat = baseLat + latSubIndex * (5 / 60);
	const lon = baseLon + lonSubIndex * (7.5 / 60);

	return {
		south: lat,
		north: lat + 5 / 60,
		west: lon,
		east: lon + 7.5 / 60
	};
}

function boundsIntersect(
	bounds: { south: number; north: number; west: number; east: number },
	target: { south: number; north: number; west: number; east: number }
): boolean {
	return !(
		bounds.east < target.west ||
		bounds.west > target.east ||
		bounds.north < target.south ||
		bounds.south > target.north
	);
}

export function generateSecondMeshCodes(bounds = JAPAN_DEFAULT_BOUNDS): string[] {
	const meshcodes: string[] = [];

	const latIndexMin = Math.floor(bounds.south * 1.5);
	const latIndexMax = Math.ceil(bounds.north * 1.5);
	const lonIndexMin = Math.floor(bounds.west) - 100;
	const lonIndexMax = Math.ceil(bounds.east) - 100;

	for (let latIndex = latIndexMin; latIndex <= latIndexMax; latIndex++) {
		for (let lonIndex = lonIndexMin; lonIndex <= lonIndexMax; lonIndex++) {
			for (let latSubIndex = 0; latSubIndex < 8; latSubIndex++) {
				for (let lonSubIndex = 0; lonSubIndex < 8; lonSubIndex++) {
					const meshcode =
						padNumber(latIndex, 2) +
						padNumber(lonIndex, 2) +
						latSubIndex.toString() +
						lonSubIndex.toString();

					try {
						const meshBounds = calculateSecondMeshBounds(meshcode);
						if (!boundsIntersect(meshBounds, bounds)) {
							continue;
						}
						meshcodes.push(meshcode);
					} catch {
						// Ignore invalid mesh codes at boundary edges
					}
				}
			}
		}
	}

	return meshcodes;
}
