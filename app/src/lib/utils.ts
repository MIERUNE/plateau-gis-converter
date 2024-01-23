export function abbreviatePath(path: string, maxLen: number): string {
	return path.length <= maxLen ? path : `…${path.slice(-maxLen)}`;
}
