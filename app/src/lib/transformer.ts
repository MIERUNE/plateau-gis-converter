export interface SelectionOptions<T extends string> {
	label: string;
	value: T;
}

export interface SelectionConfig<T extends string> {
	options: SelectionOptions<T>[];
	selected_value: T;
}

export interface Selection<T extends string> {
	Selection: SelectionConfig<T>;
}

export interface TransformerConfig<T> {
	key: string;
	label: string;
	parameter: T;
	requirements: string[];
}

export interface BooleanConfig {
	Boolean: boolean;
}

// TransformerRegistryの定義
export type TransformerRegistry = {
	configs: Array<TransformerConfig<boolean> | TransformerConfig<SelectionConfig<string>>>;
};

export type TransformerOptions = {
	label: string;
	parameter_type: 'Boolean' | 'Selection';
	parameter_value: string;
}[];

// BooleanConfig型かどうかを判定する型ガード関数
export function isBooleanConfig(param: any): param is BooleanConfig {
	return typeof param === 'object' && 'Boolean' in param && typeof param.Boolean === 'boolean';
}

// SelectionConfigのジェネリック型かどうかを判定する型ガード関数
export function isSelectionConfig<T extends string>(param: any): param is Selection<T> {
	return (
		typeof param === 'object' &&
		'Selection' in param &&
		typeof param.Selection === 'object' &&
		Array.isArray(param.Selection.options) &&
		'selected_value' in param.Selection &&
		typeof param.Selection.selected_value === 'string'
	);
}
