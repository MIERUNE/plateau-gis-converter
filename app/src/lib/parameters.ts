interface IntegerParameter {
	Integer: {
		value: number;
		min?: number;
		max?: number;
	};
}

interface StringParameter {
	String: {
		value: string;
	};
}

interface BooleanParameter {
	Boolean: {
		value: boolean;
	};
}

// TODO - In practice, use as IntegerParameter | StringParameter | BooleanParameter
type Parameter = IntegerParameter & StringParameter & BooleanParameter;

interface ParamsOptionItem {
	parameter: Parameter;
	description: string;
}

export interface ParamsOption {
	items: {
		[key: string]: ParamsOptionItem;
	};
}
