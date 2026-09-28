<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<footer class="border-background-darkest mt-16 border-t py-8">
	<p class="text-center text-sm text-gray-500">
		Laget med ❤️ av
		<a href="https://omfj.no" target="_blank" rel="noopener noreferrer" class="underline"> omfj </a>
		se koden på
		<a
			href="https://omfj.no/github/beer.omfj.no"
			target="_blank"
			rel="noopener noreferrer"
			class="underline"
		>
			GitHub.
		</a>
	</p>
	<nav class="mt-4 flex justify-center gap-4 text-sm" aria-label="Juridisk informasjon">
		<a class="underline" href={resolve('/vilkar')}>Vilkår</a>
		<a class="underline" href={resolve('/personvern')}>Personvern</a>
	</nav>
</footer>
