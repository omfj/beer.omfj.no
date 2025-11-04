export function formatDate(date: Date) {
	return new Intl.DateTimeFormat('nb-NO', {
		year: 'numeric',
		month: 'long',
		day: 'numeric'
	}).format(date);
}
