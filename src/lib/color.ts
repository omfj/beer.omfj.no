function hslToHex(h: number, s: number, l: number): string {
	l /= 100;
	s /= 100;

	const k = (n: number) => (n + h / 30) % 12;
	const a = s * Math.min(l, 1 - l);
	const f = (n: number) => l - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));

	const r = Math.round(255 * f(0));
	const g = Math.round(255 * f(8));
	const b = Math.round(255 * f(4));

	const toHex = (x: number) => x.toString(16).padStart(2, '0');

	return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

export function generateSoftColor() {
	const h = Math.floor(Math.random() * 360); // Any hue
	const s = Math.floor(Math.random() * 30) + 30; // 30–60% saturation
	const l = Math.floor(Math.random() * 20) + 75; // 75–95% lightness

	return hslToHex(h, s, l);
}
