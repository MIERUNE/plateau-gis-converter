import { describe, expect, it } from 'vitest';
import {
	calculateFirstMeshBounds,
	calculateMeshBounds,
	meshcodeToCenter,
	meshcodeToPolygon,
	boundsToPolygon
} from '$lib/meshcode';

describe('meshcode', () => {
	describe('calculateFirstMeshBounds', () => {
		it('should calculate bounds for first mesh 5339', () => {
			const bounds = calculateFirstMeshBounds('5339');
			expect(bounds).toEqual({
				west: 139,
				east: 140,
				north: 36,
				south: 35.333333333333336
			});
		});

		it('should calculate bounds for first mesh 5438', () => {
			const bounds = calculateFirstMeshBounds('5438');
			expect(bounds).toEqual({
				west: 138,
				east: 139,
				north: 36.666666666666664,
				south: 36
			});
		});

		it('should throw error for invalid mesh code length', () => {
			expect(() => calculateFirstMeshBounds('533')).toThrow(
				'第1次地域区画は4桁以上のメッシュコードが必要です'
			);
		});

		it('should throw error for non-numeric mesh code', () => {
			expect(() => calculateFirstMeshBounds('53AB')).toThrow('無効なメッシュコード形式です');
		});
	});

	describe('calculateMeshBounds', () => {
		describe('6-digit mesh codes (第2次地域区画)', () => {
			it('should calculate bounds for mesh 533936', () => {
				const bounds = calculateMeshBounds('533936');
				expect(bounds.south).toBeCloseTo(35.583333333333336, 5);
				expect(bounds.west).toBeCloseTo(139.75, 5);
				expect(bounds.north).toBeCloseTo(35.66666666666667, 5);
				expect(bounds.east).toBeCloseTo(139.875, 5);
			});

			it('should calculate bounds for mesh 533945', () => {
				const bounds = calculateMeshBounds('533945');
				expect(bounds.south).toBeCloseTo(35.66666666666667, 5);
				expect(bounds.west).toBeCloseTo(139.625, 5);
				expect(bounds.north).toBeCloseTo(35.75, 5);
				expect(bounds.east).toBeCloseTo(139.75, 5);
			});

			it('should calculate bounds for mesh 544100', () => {
				const bounds = calculateMeshBounds('544100');
				expect(bounds.south).toBeCloseTo(36, 5);
				expect(bounds.west).toBeCloseTo(141, 5);
				expect(bounds.north).toBeCloseTo(36.083333333333336, 5);
				expect(bounds.east).toBeCloseTo(141.125, 5);
			});
		});

		describe('8-digit mesh codes (第3次地域区画)', () => {
			it('should calculate bounds for mesh 53393599', () => {
				const bounds = calculateMeshBounds('53393599');
				expect(bounds.south).toBeCloseTo(35.658333333333331, 5);
				expect(bounds.west).toBeCloseTo(139.7375, 5);
				expect(bounds.north).toBeCloseTo(35.666666666666664, 5);
				expect(bounds.east).toBeCloseTo(139.75, 5);
			});

			it('should calculate bounds for mesh 53394611', () => {
				const bounds = calculateMeshBounds('53394611');
				expect(bounds.south).toBeCloseTo(35.675, 5);
				expect(bounds.west).toBeCloseTo(139.7625, 5);
				expect(bounds.north).toBeCloseTo(35.68333333333333, 5);
				expect(bounds.east).toBeCloseTo(139.775, 5);
			});

			it('should calculate bounds for mesh 54401000', () => {
				const bounds = calculateMeshBounds('54401000');
				expect(bounds.south).toBeCloseTo(36.083333333333336, 5);
				expect(bounds.west).toBeCloseTo(140, 5);
				expect(bounds.north).toBeCloseTo(36.09166666666667, 5);
				expect(bounds.east).toBeCloseTo(140.0125, 5);
			});
		});

		describe('error handling', () => {
			it('should throw error for 5-digit mesh code', () => {
				expect(() => calculateMeshBounds('53393')).toThrow(
					'メッシュコードは6桁（第2次）または8桁（第3次）である必要があります'
				);
			});

			it('should throw error for 7-digit mesh code', () => {
				expect(() => calculateMeshBounds('5339359')).toThrow(
					'メッシュコードは6桁（第2次）または8桁（第3次）である必要があります'
				);
			});

			it('should throw error for non-numeric mesh code', () => {
				expect(() => calculateMeshBounds('5339AB')).toThrow('無効なメッシュコード形式です');
			});
		});
	});

	describe('boundsToPolygon', () => {
		it('should convert bounds to GeoJSON polygon coordinates', () => {
			const bounds = {
				north: 35.75,
				south: 35.666666666666664,
				east: 139.375,
				west: 139.25
			};
			const polygon = boundsToPolygon(bounds);

			expect(polygon.type).toBe('Polygon');
			expect(polygon.coordinates).toHaveLength(1);
			expect(polygon.coordinates[0]).toHaveLength(5);
			expect(polygon.coordinates[0][0]).toEqual([139.25, 35.666666666666664]);
			expect(polygon.coordinates[0][1]).toEqual([139.375, 35.666666666666664]);
			expect(polygon.coordinates[0][2]).toEqual([139.375, 35.75]);
			expect(polygon.coordinates[0][3]).toEqual([139.25, 35.75]);
			expect(polygon.coordinates[0][4]).toEqual([139.25, 35.666666666666664]);
		});
	});

	describe('meshcodeToPolygon', () => {
		it('should convert 6-digit mesh code to polygon', () => {
			const polygon = meshcodeToPolygon('533945');

			expect(polygon.type).toBe('Polygon');
			expect(polygon.coordinates).toHaveLength(1);
			expect(polygon.coordinates[0]).toHaveLength(5);

			const firstPoint = polygon.coordinates[0][0];
			const lastPoint = polygon.coordinates[0][4];
			expect(firstPoint).toEqual(lastPoint);

			expect(firstPoint[0]).toBeCloseTo(139.625, 5);
			expect(firstPoint[1]).toBeCloseTo(35.66666666666667, 5);
		});

		it('should convert 8-digit mesh code to polygon', () => {
			const polygon = meshcodeToPolygon('53394611');

			expect(polygon.type).toBe('Polygon');
			expect(polygon.coordinates).toHaveLength(1);
			expect(polygon.coordinates[0]).toHaveLength(5);

			const firstPoint = polygon.coordinates[0][0];
			expect(firstPoint[0]).toBeCloseTo(139.7625, 5);
			expect(firstPoint[1]).toBeCloseTo(35.675, 5);
		});
	});

	describe('meshcodeToCenter', () => {
		it('should calculate center for 6-digit mesh code', () => {
			const center = meshcodeToCenter('533945');
			expect(center[0]).toBeCloseTo(139.6875, 5);
			expect(center[1]).toBeCloseTo(35.708333333333336, 5);
		});

		it('should calculate center for 8-digit mesh code', () => {
			const center = meshcodeToCenter('53394611');
			expect(center[0]).toBeCloseTo(139.76875, 5);
			expect(center[1]).toBeCloseTo(35.679166666666667, 5);
		});

		it('should calculate center for boundary mesh codes', () => {
			const center1 = meshcodeToCenter('544100');
			expect(center1[0]).toBeCloseTo(141.0625, 5);
			expect(center1[1]).toBeCloseTo(36.04166666666667, 5);

			const center2 = meshcodeToCenter('54410000');
			expect(center2[0]).toBeCloseTo(141.00625, 5);
			expect(center2[1]).toBeCloseTo(36.00416666666667, 5);
		});
	});
});
