<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Personvern - Beer Counter</title>
</svelte:head>

<h1 class="mb-6 text-3xl font-medium">Personvernerklæring</h1>
<p class="text-foreground-muted mb-8">Sist oppdatert 23. september 2026.</p>

<div class="text-foreground-muted space-y-8">
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Hvem behandler opplysningene?</h2>
		<p>
			For spørsmål om personvern, innsyn, sletting eller innhold som bør fjernes, kontakt oss på
			<a class="text-primary hover:underline" href="mailto:me@omfj.no">me@omfj.no</a>.
		</p>
	</section>
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Hvilke opplysninger brukes?</h2>
		<p>
			Vi lagrer brukernavn og opplysninger som trengs for innlogging, inkludert passord i hashet
			form (bcrypt) og innloggingsøkter. Vi lagrer arrangementer, drikkeregistreringer, tidspunkt,
			bilder og hvilke brukere som har tilgang til passordbeskyttede arrangementer. Hvis du velger å
			oppgi vektklasse og kjønn i profilen, lagrer vi også disse opplysningene.
		</p>
	</section>
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Hvorfor brukes opplysningene?</h2>
		<p>
			Opplysningene brukes til å opprette og sikre kontoen din, vise arrangementer og
			drikkeregistreringer, kontrollere hvem som har tilgang og beregne poeng. Valgfri vektklasse og
			kjønn brukes sammen med registreringene til å vise et grovt promilleestimat. Dette er
			funksjoner i tjenesten du ber om når du oppretter konto og bruker den.
		</p>
	</section>
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Hvem kan se opplysningene?</h2>
		<p>
			Innloggede brukere som kan åpne et arrangement, kan se brukernavn, registreringer og bilder i
			arrangementet. Arrangementer uten passord kan åpnes av andre innloggede brukere. For
			passordbeskyttede arrangementer kreves tilgang. Vektklasse og kjønn brukes også i
			arrangementets poengoversikt og kan mottas av andre brukere med tilgang til arrangementet.
			Driften av tjenesten kan kreve at leverandører av lagring og hosting behandler opplysninger på
			våre vegne.
		</p>
	</section>
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Lagring og sletting</h2>
		<p>
			Kontoopplysninger og innhold lagres mens kontoen og registreringene finnes i tjenesten. Du kan
			slette egne drikkeregistreringer i arrangementet. Kontakt oss hvis du vil be om sletting av
			andre opplysninger eller hele kontoen. Vi vurderer slike forespørsler og opplyser deg hvis noe
			må oppbevares lenger. Kopier som andre har lagret eller tatt skjermbilde av, er utenfor vår
			kontroll.
		</p>
	</section>
	<section>
		<h2 class="text-foreground mb-2 text-xl font-medium">Dine rettigheter</h2>
		<p>
			Du kan be om innsyn, retting eller sletting av opplysninger om deg, og i aktuelle tilfeller be
			om begrensning eller protestere mot behandlingen.
		</p>
	</section>
	<p>
		Les også <a class="text-primary hover:underline" href={resolve('/vilkar')}>vilkårene for bruk</a
		>.
	</p>
</div>
