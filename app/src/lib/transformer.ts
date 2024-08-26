export interface Selection<T extends string> {
	label: string;
	value: T;
}

// 選択肢と現在の選択状態を持つ型
export interface SelectionConfig<T extends string> {
	options: Selection<T>[];
	selected_value: T;
}

// Booleanインターフェースを定義
export interface BooleanConfig {
	Boolean: boolean;
}

// TransformerConfigのジェネリック型
export interface TransformerConfig<T> {
	key: string;
	label: string;
	parameter: T;
	requirements: string[];
}

// TransformerRegistryの定義
export type TransformerRegistry = {
	configs: Array<
		| TransformerConfig<boolean> // boolean の場合
		| TransformerConfig<SelectionConfig<string>> // Selection の場合
	>;
};
// BooleanConfig型かどうかを判定する型ガード関数
export function isBooleanConfig(param: any): param is BooleanConfig {
	return typeof param === 'object' && 'Boolean' in param && typeof param.Boolean === 'boolean';
}

// Selection型かどうかを判定する型ガード関数
export function isSelectionArray(param: any): param is Selection<string>[] {
	return (
		Array.isArray(param) &&
		param.every((item) => {
			return typeof item.label === 'string' && typeof item.value === 'string';
		})
	);
}

export function hasSelection(param: any): param is { Selection: Selection<string>[] } {
	return (
		param &&
		Array.isArray(param.Selection) &&
		param.Selection.options.every((item) => {
			return typeof item.label === 'string' && typeof item.value === 'string';
		})
	);
}

// 型判定のための関数
export function isBoolean(param) {
	return typeof param === 'boolean';
}

export function isSelectionConfig(param) {
	return typeof param.selectedValue === 'string' && Array.isArray(param.options);
}
