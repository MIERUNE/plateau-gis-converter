export function abbreviatePath(path: string, maxLen: number): string {
	return path.length <= maxLen ? path : `…${path.slice(-maxLen)}`;
}

export function formatFilePath(path: string, maxLen: number): string {
	// Check if this is a ZIP file path
	if (path.includes('.zip/')) {
		const [zipPath, internalPath] = path.split('.zip/');
		const zipName = zipPath.split('/').pop() || '';
		const fileName = internalPath.split('/').pop() || '';
		const formattedPath = `📦 ${zipName}.zip → ${fileName}`;
		return formattedPath.length <= maxLen ? formattedPath : `…${formattedPath.slice(-maxLen)}`;
	}
	// Regular file
	return abbreviatePath(path, maxLen);
}
