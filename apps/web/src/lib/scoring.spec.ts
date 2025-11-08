import { describe, it, expect } from 'vitest';
import { calculateDrinkPoints } from './scoring';

describe('calculateDrinkPoints', () => {
	it('returns 0.5 for null volume', () => {
		expect(calculateDrinkPoints(null, 5)).toBe(0.5);
	});

	it('returns 0.5 for null ABV', () => {
		expect(calculateDrinkPoints(500, null)).toBe(0.5);
	});

	it('returns 0.5 for zero volume', () => {
		expect(calculateDrinkPoints(0, 5)).toBe(0.5);
	});

	it('returns 0.5 for zero ABV', () => {
		expect(calculateDrinkPoints(500, 0)).toBe(0.5);
	});

	it('calculates points correctly for standard beer (500ml, 5% ABV)', () => {
		const points = calculateDrinkPoints(500, 5);
		// Formula: (0.5L × 0.05 × 0.789 × 1000) / 10 = 1.97 rounded to 2.0
		expect(points).toBe(2.0);
	});

	it('calculates points correctly for wine (150ml, 12% ABV)', () => {
		const points = calculateDrinkPoints(150, 12);
		// Formula: (0.15L × 0.12 × 0.789 × 1000) / 10 = 1.42 rounded to 1.4
		expect(points).toBe(1.4);
	});

	it('applies beer bonus when drink type is beer', () => {
		const basePoints = calculateDrinkPoints(500, 5);
		const beerPoints = calculateDrinkPoints(500, 5, 'beer');
		// Beer should get 10% bonus
		expect(beerPoints).toBeGreaterThan(basePoints);
		expect(beerPoints).toBe(2.2); // 2.0 × 1.1 = 2.2
	});

	it('does not apply beer bonus to wine', () => {
		const winePoints = calculateDrinkPoints(150, 12, 'wine');
		const wineWithoutType = calculateDrinkPoints(150, 12);
		expect(winePoints).toBe(wineWithoutType);
		expect(winePoints).toBe(1.4);
	});

	it('does not apply beer bonus to cocktails', () => {
		const cocktailPoints = calculateDrinkPoints(300, 15, 'cocktail');
		const cocktailWithoutType = calculateDrinkPoints(300, 15);
		expect(cocktailPoints).toBe(cocktailWithoutType);
	});

	it('handles large volumes correctly', () => {
		const points = calculateDrinkPoints(1000, 5);
		// Formula: (1L × 0.05 × 0.789 × 1000) / 10 = 3.95 rounded to 3.9
		expect(points).toBe(3.9);
	});

	it('handles large volumes with beer bonus', () => {
		const points = calculateDrinkPoints(1000, 5, 'beer');
		// Formula: (1L × 0.05 × 0.789 × 1000) / 10 × 1.1 = 4.29 rounded to 4.3
		expect(points).toBe(4.3);
	});

	it('handles high ABV correctly', () => {
		const points = calculateDrinkPoints(40, 40);
		// Formula: (0.04L × 0.40 × 0.789 × 1000) / 10 = 1.26 rounded to 1.3
		expect(points).toBe(1.3);
	});

	it('rounds to one decimal place', () => {
		const points = calculateDrinkPoints(333, 5);
		// Should return a number with at most 1 decimal place
		expect(points.toString()).toMatch(/^\d+\.\d$/);
	});
});
