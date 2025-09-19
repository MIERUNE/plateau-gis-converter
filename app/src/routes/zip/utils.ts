// Structure: { meshcode: { type: [filepath1, filepath2, ...] } }
export type MeshcodeData = Record<string, Record<string, string[]>>;

// Type labels in Japanese
export const TYPE_LABELS: Record<string, string> = {
	bldg: '建築物',
	tran: '交通',
	fld: '洪水浸水想定区域',
	luse: '土地利用',
	lsld: '土砂災害警戒区域',
	urf: '都市計画区域',
	frn: '都市設備',
	veg: '植生',
	dem: '地形',
	wtr: '水部',
	tnl: 'トンネル',
	cons: 'その他の構造物',
	gen: '汎用都市オブジェクト',
	brid: '橋梁',
	ubld: '地下建物',
	rwy: '鉄道',
	trk: '徒歩道',
	squr: '広場',
	wwy: '航路',
	tnm: '津波浸水想定',
	htd: '高潮浸水想定区域',
	ifld: '内水浸水想定区域'
};

export function getTypeLabel(type: string): string {
	if (type in TYPE_LABELS) {
		return TYPE_LABELS[type];
	}
	return type;
}
