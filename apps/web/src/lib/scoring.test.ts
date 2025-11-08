import { describe, it, expect } from 'vitest';
import { calculateDrinkPoints } from './scoring';

describe('calculateDrinkPoints', () => {
it('calculates beer points correctly', () => {
// Beer (5% ABV)
expect(calculateDrinkPoints(330, 5)).toBe(1.3); // 0.33L beer
expect(calculateDrinkPoints(500, 5)).toBe(2.0); // 0.5L beer
expect(calculateDrinkPoints(568, 5)).toBe(2.2); // Pint
});

it('calculates wine points correctly', () => {
// Wine (12% ABV)
expect(calculateDrinkPoints(150, 12)).toBe(1.4); // Glass
expect(calculateDrinkPoints(200, 12)).toBe(1.9); // Large glass
});

it('calculates cocktail points with current ABV', () => {
// Current cocktail ABV (15%)
expect(calculateDrinkPoints(200, 15)).toBe(2.4); // Small cocktail
expect(calculateDrinkPoints(300, 15)).toBe(3.6); // Standard cocktail
});

it('calculates shot points correctly', () => {
// Shot (40% ABV)
expect(calculateDrinkPoints(40, 40)).toBe(1.3); // Standard shot
expect(calculateDrinkPoints(80, 40)).toBe(2.5); // Double shot
});

it('returns 0.5 for null or undefined values', () => {
expect(calculateDrinkPoints(null, 5)).toBe(0.5);
expect(calculateDrinkPoints(330, null)).toBe(0.5);
expect(calculateDrinkPoints(null, null)).toBe(0.5);
expect(calculateDrinkPoints(undefined, 5)).toBe(0.5);
expect(calculateDrinkPoints(330, undefined)).toBe(0.5);
});

it('returns 0.5 for zero or negative values', () => {
expect(calculateDrinkPoints(0, 5)).toBe(0.5);
expect(calculateDrinkPoints(330, 0)).toBe(0.5);
expect(calculateDrinkPoints(-100, 5)).toBe(0.5);
expect(calculateDrinkPoints(330, -5)).toBe(0.5);
});

it('returns 0.5 for NaN values', () => {
expect(calculateDrinkPoints(NaN, 5)).toBe(0.5);
expect(calculateDrinkPoints(330, NaN)).toBe(0.5);
});
});
