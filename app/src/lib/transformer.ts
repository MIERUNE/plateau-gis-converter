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
}

export interface BooleanConfig {
	Boolean: boolean;
}

type TransformerParameterType = BooleanConfig | Selection<string>;

export type TransformerSettings = {
	configs: Array<TransformerConfig<TransformerParameterType>>;
};

export type TransformerOptions = {
	label: string;
	parameter_type: 'Boolean' | 'Selection';
	parameter_value: string;
}[];

// Type guard function to determine if the type is a BooleanConfig type.
export function isBooleanConfig(param: unknown): param is BooleanConfig {
	return (
		typeof param === 'object' &&
		param !== null &&
		'Boolean' in param &&
		typeof param.Boolean === 'boolean'
	);
}

// Type guard function to determine if the type is a Selection type.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
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
