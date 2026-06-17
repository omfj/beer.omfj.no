/**
 * Calculate drink points based on volume (mL), ABV percentage, and drink type multiplier.
 * Formula: (Volume (L) × ABV (decimal) × 0.789 × 1000) / 10 × multiplier
 *
 * @param volumeML - Volume in milliliters (nullable)
 * @param abv - Alcohol by volume percentage, 0–100 (nullable). A 5.2% beer = 5.2.
 * @param multiplier - Drink type score multiplier (default 1.0)
 * @returns Points scored for the drink
 */
export function calculateDrinkPoints(
	volumeML: number | null | undefined,
	abv: number | null | undefined,
	multiplier = 1.0
): number {
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

	const score = ((volumeL * abvDecimal * 0.789 * 1000) / 10) * multiplier;

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

/**
 * Grams of pure alcohol in a drink.
 * grams = volume (mL) × ABV (decimal) × 0.789 (ethanol density g/mL)
 * Returns 0 when volume or ABV is missing/invalid.
 */
export function alcoholGrams(
	volumeML: number | null | undefined,
	abv: number | null | undefined
): number {
	if (
		volumeML == null ||
		abv == null ||
		isNaN(volumeML) ||
		isNaN(abv) ||
		volumeML <= 0 ||
		abv <= 0
	) {
		return 0;
	}
	return volumeML * (abv / 100) * 0.789;
}

/**
 * Widmark-faktoren (norwegian standard): male 0.7, woman 0.6, other use average.
 */
export function widmarkFactor(gender: 'male' | 'female' | 'other'): number {
	return gender === 'male' ? 0.7 : gender === 'female' ? 0.6 : 0.65;
}

/**
 * Estimate blood alcohol concentration (promille / ‰) using Widmark with
 * per-drink elimination. Each drink decays on its own clock and is floored at
 * zero, so alcohol consumed hours ago drops out instead of accumulating.
 *
 * BAC = Σ max(0, A_i / (weight × r) − 0.15 × hours_since_drink_i)
 *
 * @param drinks - Drinks with grams of pure alcohol and consumption time (ms epoch)
 * @param weightKg - Body weight in kg
 * @param gender - Used for the Widmark factor
 * @param nowMs - Reference time in ms (e.g. Date.now())
 */
export function calculateBac(
	drinks: { grams: number; consumedAtMs: number }[],
	weightKg: number,
	gender: 'male' | 'female' | 'other',
	nowMs: number
): number {
	const rm = weightKg * widmarkFactor(gender);
	const bac = drinks.reduce((sum, drink) => {
		const hours = (nowMs - drink.consumedAtMs) / 3_600_000;
		return sum + Math.max(0, drink.grams / rm - 0.15 * hours);
	}, 0);
	return Math.max(0, bac);
}
