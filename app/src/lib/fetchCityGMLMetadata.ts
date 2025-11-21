import { invoke } from '@tauri-apps/api/core';

export interface CityGMLFeatureTypeInfo {
	label: string;
	fileCount: number;
	files: CityGMLRemoteFile[];
}

export interface CityGMLRemoteFile {
	meshcode: string;
	featureType: string;
	url: string;
	maxLod?: number;
	fileSize?: number;
	features?: number;
}

export interface FetchCityGmlMetadata {
	featureTypes: { [key: string]: CityGMLFeatureTypeInfo };
	meshes: { [meshcode: string]: { [featureType: string]: CityGMLRemoteFile } };
}

export type FetchCityGmlMetadataResult =
	| ({
			success: true;
	  } & FetchCityGmlMetadata)
	| { success: false; message: string; type: string };

export async function fetchCityGMLMetadataByMeshcodes(
	meshcodes: string[],
	strict: boolean = false
): Promise<FetchCityGmlMetadataResult> {
	try {
		const response = await invoke<FetchCityGmlMetadata>('fetch_citygml_metadata', {
			meshcodes,
			strict
		});
		return { success: true, ...response };
	} catch (e) {
		console.error('Error fetching CityGML metadata by meshcodes:', e);
		return {
			success: false,
			type: e.type,
			message: e.message
		};
	}
}
