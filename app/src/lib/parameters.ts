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

type Parameter = IntegerParameter | StringParameter | BooleanParameter;

interface ParamsOptionItem {
	parameter: Parameter;
	description: string;
	gui_label?: string;
}

export interface ParamsOption {
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
