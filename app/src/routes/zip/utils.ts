// Structure: { meshcode: { type: [filepath1, filepath2, ...] } }
export type MeshcodeData = Record<string, Record<string, string[]>>;

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
  rfld: "ため池ハザードマップ",
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
	area: "区域",
	gen: '汎用都市オブジェクト',
	app: "アピアランス",
	ext: '拡張製品仕様書で追加した地物'
};

export function getTypeLabel(type: string): string {
	if (type in TYPE_LABELS) {
		return TYPE_LABELS[type];
	}
	return type;
}
