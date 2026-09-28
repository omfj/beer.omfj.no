<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Vilkår - Beer Counter</title>
</svelte:head>

<h1 class="mb-6 text-3xl font-medium">Vilkår for bruk</h1>
<p class="text-foreground-muted mb-8">Sist oppdatert 23. september 2026.</p>

<div class="space-y-8">
	<section>
		<h2 class="mb-2 text-xl font-medium">Om tjenesten</h2>
		<p class="text-foreground-muted">
			Beer Counter lar deg registrere drikke og dele registreringer og bilder i arrangementer. Du må
			ha en konto for å bruke tjenesten.
		</p>
	</section>
	<section>
		<h2 class="mb-2 text-xl font-medium">Hvem kan se innholdet?</h2>
		<div class="text-foreground-muted space-y-3">
			<p>
				Andre innloggede brukere med tilgang til et arrangement kan se brukernavn, registreringer og
				bilder som er knyttet til arrangementet. Arrangementer uten passord kan åpnes av andre
				innloggede brukere. For arrangementer med passord må brukeren ha fått tilgang.
			</p>
			<p>
				Ikke last opp bilder eller registrer opplysninger du ikke vil dele med disse brukerne. Andre
				kan lagre eller ta skjermbilder av innhold de får se.
			</p>
		</div>
	</section>
	<section>
		<h2 class="mb-2 text-xl font-medium">Ansvar for innhold</h2>
		<div class="text-foreground-muted space-y-3">
			<p>
				Du er ansvarlig for det du laster opp. Last bare opp bilder du har rett til å dele, og
				innhent tillatelse fra personer som kan gjenkjennes på bildene. Innholdet må følge norsk lov
				og være passende for deling i arrangementet.
			</p>
			<p>
				Du kan slette dine egne drikkeregistreringer i arrangementet. Hvis du vil melde fra om
				andres innhold eller be om at et bilde fjernes, bruk kontaktinformasjonen i
				<a class="text-primary hover:underline" href={resolve('/personvern')}
					>personvernerklæringen</a
				>.
			</p>
		</div>
	</section>
	<section>
		<h2 class="mb-2 text-xl font-medium">Promilleberegning</h2>
		<p class="text-foreground-muted">
			Promillen som vises i tjenesten, er et grovt estimat basert på registreringer og eventuelle
			profilopplysninger. Den kan avvike fra faktisk promille. Ikke bruk estimatet til å avgjøre om
			det er trygt eller lovlig å kjøre eller utføre andre risikofylte aktiviteter.
		</p>
	</section>
	<section>
		<h2 class="mb-2 text-xl font-medium">Personopplysninger</h2>
		<p class="text-foreground-muted">
			Les <a class="text-primary hover:underline" href={resolve('/personvern')}
				>personvernerklæringen</a
			> for informasjon om hvordan opplysningene dine behandles og hvilke rettigheter du har.
		</p>
	</section>
</div>
