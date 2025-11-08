/**
 * Beer bonus multiplier - because beer is awesome! 🍺
 * Beer gets a 10% bonus to celebrate the timeless tradition of beer drinking
 */
const BEER_BONUS_MULTIPLIER = 1.1;

/**
 * Calculate drink points based on volume (mL) and ABV percentage
 * Formula: (Volume (L) × ABV (decimal) × 0.789 × 1000) / 10
 * Beer gets a 10% bonus to honor the classic beverage!
 *
 * @param volumeML - Volume in milliliters (nullable)
 * @param abv - Alcohol by volume percentage (nullable)
 * @param drinkTypeId - Drink type ID (optional, used for beer bonus)
 * @returns Points scored for the drink
 */
export function calculateDrinkPoints(
	volumeML: number | null | undefined,
	abv: number | null | undefined,
	drinkTypeId?: string | null | undefined
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
	let score = (volumeL * abvDecimal * 0.789 * 1000) / 10;

	// Apply beer bonus if drink type is beer
	if (drinkTypeId === 'beer') {
		score *= BEER_BONUS_MULTIPLIER;
	}

	// Make sure result is not NaN
	if (isNaN(score)) {
		return 0.5;
	}

	return Math.round(score * 10) / 10;
}
