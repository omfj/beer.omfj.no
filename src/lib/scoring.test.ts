import { describe, it, expect } from 'vitest';
import { alcoholGrams, calculateBac } from './scoring';

const HOUR = 3_600_000;

describe('alcoholGrams', () => {
	it('computes grams for a 0.5L 5% beer', () => {
		// 500 mL × 0.05 × 0.789 ≈ 19.7 g
		expect(alcoholGrams(500, 5)).toBeCloseTo(19.7, 1);
	});

	it('returns 0 for missing volume or abv', () => {
		expect(alcoholGrams(null, 5)).toBe(0);
		expect(alcoholGrams(500, null)).toBe(0);
		expect(alcoholGrams(0, 5)).toBe(0);
	});
});

describe('calculateBac', () => {
	const beer = alcoholGrams(500, 5); // ~19.7 g

	it('gives ~0.40‰ for one fresh beer (70 kg male)', () => {
		const bac = calculateBac([{ grams: beer, consumedAtMs: 0 }], 70, 'male', 0);
		expect(bac).toBeCloseTo(0.4, 1);
	});

	it('decays a single beer to 0 after a few hours', () => {
		// 0.40‰ − 0.15 × 3h = -0.05 → floored at 0
		const bac = calculateBac([{ grams: beer, consumedAtMs: 0 }], 70, 'male', 3 * HOUR);
		expect(bac).toBe(0);
	});

	it('keeps recent drinks high while old ones drop off (10 beers, 1/h)', () => {
		// One beer per hour for 10 hours, evaluated at hour 10.
		const drinks = Array.from({ length: 10 }, (_, i) => ({
			grams: beer,
			consumedAtMs: i * HOUR
		}));
		const bac = calculateBac(drinks, 70, 'male', 10 * HOUR);
		// Old lumped formula gave ~2.5‰; per-drink decay leaves only the recent
		// drinks (~0.35‰), since anything older than ~2.7h has fully eliminated.
		expect(bac).toBeGreaterThan(0.2);
		expect(bac).toBeLessThan(0.6);
	});

	it('uses a higher factor for women (less body water → higher BAC)', () => {
		const drinks = [{ grams: beer, consumedAtMs: 0 }];
		const male = calculateBac(drinks, 70, 'male', 0);
		const female = calculateBac(drinks, 70, 'female', 0);
		expect(female).toBeGreaterThan(male);
	});
});
