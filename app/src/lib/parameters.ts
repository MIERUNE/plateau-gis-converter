export type ParameterType = 'Integer' | 'String' | 'Boolean';

export type ParameterOption<T extends ParameterType> = T extends 'Integer'
	? { Integer: { value: number; min: number; max: number } }
	: T extends 'String'
		? { String: { value: string } }
		: T extends 'Boolean'
			? { Boolean: { value: boolean } }
			: never;

export type ParameterEntry<T extends ParameterType> = {
	description: string;
	required: string;
	parameter: ParameterOption<T>;
};

export type Parameters = {
	items: { [key: string]: ParameterEntry<ParameterType> };
};
