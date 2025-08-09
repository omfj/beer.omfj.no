<script lang="ts">
	interface Props {
		title?: string;
		description?: string;
		url?: string;
		image?: string;
		type?: string;
	}

	let {
		title = 'Beer Counter - Tell dine øl',
		description = 'En enkel app for å telle og registrere øl på arrangementer med venner. Opprett arrangementer, ta bilder av øl, og konkurrer med venner om hvem som drikker mest.',
		url,
		image,
		type = 'website'
	}: Props = $props();

	const fullTitle = title.includes('Beer Counter') ? title : `${title} - Beer Counter`;
	const canonicalUrl = url || (typeof window !== 'undefined' ? window.location.href : '');
	const ogImage = image || '/android-chrome-512x512.png';
</script>

<svelte:head>
	<title>{fullTitle}</title>
	<meta name="description" content={description} />

	<!-- Open Graph -->
	<meta property="og:title" content={fullTitle} />
	<meta property="og:description" content={description} />
	<meta property="og:type" content={type} />
	{#if canonicalUrl}
		<meta property="og:url" content={canonicalUrl} />
		<link rel="canonical" href={canonicalUrl} />
	{/if}
	<meta property="og:image" content={ogImage} />
	<meta property="og:image:alt" content={title} />

	<!-- Twitter Card -->
	<meta name="twitter:title" content={fullTitle} />
	<meta name="twitter:description" content={description} />
	<meta name="twitter:image" content={ogImage} />
</svelte:head>
