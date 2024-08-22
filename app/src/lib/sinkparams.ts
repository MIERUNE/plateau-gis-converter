export interface IntegerParameter {
	Integer: {
		value: number;
		min?: number;
		max?: number;
	};
}

export interface StringParameter {
	String: {
		value: string;
	};
}

export interface BooleanParameter {
	Boolean: {
		value: boolean;
	};
}

export interface SelectionParameter {
	Selection: {
		value: { [key: string]: string }[];
	};
}

type Parameter = IntegerParameter | StringParameter | BooleanParameter | SelectionParameter;

interface ParamsOptionItem {
	parameter: Parameter;
	description: string;
	label?: string;
}

export interface SinkParameters {
	items: {
		[key: string]: ParamsOptionItem;
	};
}

// Check the parameter type
export function isIntegerParameter(
	parameter: Parameter
): parameter is { Integer: { value: number; min: number; max: number } } {
	return (parameter as { Integer?: unknown }).Integer !== undefined;
}
export function isStringParameter(
	parameter: Parameter
): parameter is { String: { value: string } } {
	return (parameter as { String?: unknown }).String !== undefined;
}
export function isBooleanParameter(
	parameter: Parameter
): parameter is { Boolean: { value: boolean } } {
	return (parameter as { Boolean?: unknown }).Boolean !== undefined;
}

export function isSelectionParameter(
	parameter: Parameter
): parameter is { Selection: { value: { [key: string]: string }[] } } {
	return (parameter as { Selection?: unknown }).Selection !== undefined;
}

export function createRangeArray(min: number, max: number): number[] {
	if (min > max) throw new Error('min should be less than or equal to max');
	return Array.from({ length: max - min + 1 }, (_, i) => min + i);
}
