/**
 * Calculate drink points based on volume (mL) and ABV percentage
 * Formula: (Volume (L) × ABV (decimal) × 0.789 × 1000) / 10
 *
 * @param volumeML - Volume in milliliters (nullable)
 * @param abv - Alcohol by volume percentage (nullable)
 * @returns Points scored for the drink
 */
export function calculateDrinkPoints(
	volumeML: number | null | undefined,
	abv: number | null | undefined
): number {
	// If either value is null, undefined, NaN, or 0, return default score
	if (
		volumeML == null ||
		abv == null ||
		isNaN(volumeML) ||
		isNaN(abv) ||
		volumeML <= 0 ||
		abv <= 0
	) {
		return 0.5;
	}

	const volumeL = volumeML / 1000;
	const abvDecimal = abv / 100;

	// Apply the formula: (Volume (L) × ABV (decimal) × 0.789 × 1000) / 10
	const score = (volumeL * abvDecimal * 0.789 * 1000) / 10;

	// Make sure result is not NaN
	if (isNaN(score)) {
		return 0.5;
	}

	return Math.round(score * 10) / 10;
}

/**
 * Convert a named weight class to a representative weight in kg (midpoint of range).
 * light: 40–60 kg → 50 kg, medium: 61–80 kg → 70 kg, heavy: 81+ kg → 90 kg
 */
export function weightToKg(weight: 'light' | 'medium' | 'heavy'): number {
	const map = { light: 50, medium: 70, heavy: 90 };
	return map[weight];
}

export function calculateBac(
	alcoholGram: number,
	weightKg: number,
	hours: number,
	gender: 'male' | 'female' | 'other'
): number {
	// r = Widmark-faktoren (norsk standard): mann 0.7, kvinne 0.6, annet bruker gjennomsnitt
	const r = gender === 'male' ? 0.7 : gender === 'female' ? 0.6 : 0.65;
	return alcoholGram / (weightKg * r) - 0.15 * hours;
}
