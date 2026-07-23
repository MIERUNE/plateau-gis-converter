export type InputFormat = 'citygml' | 'geojson' | 'mixed' | 'unknown';

type SupportedInputFormat = Exclude<InputFormat, 'mixed'>;

function detectPathInputFormat(path: string): SupportedInputFormat {
	const normalizedPath = path.toLowerCase();

	if (normalizedPath.endsWith('.gml')) {
		return 'citygml';
	}
	if (normalizedPath.endsWith('.geojson') || normalizedPath.endsWith('.json')) {
		return 'geojson';
	}

	return 'unknown';
}

export function detectInputFormat(inputPaths: readonly string[]): InputFormat {
	if (inputPaths.length === 0) {
		return 'unknown';
	}

	let detectedFormat: SupportedInputFormat | undefined;

	for (const path of inputPaths) {
		const pathFormat = detectPathInputFormat(path);
		if (pathFormat === 'unknown') {
			return 'unknown';
		}
		if (detectedFormat && detectedFormat !== pathFormat) {
			return 'mixed';
		}
		detectedFormat = pathFormat;
	}

	return detectedFormat ?? 'unknown';
}
