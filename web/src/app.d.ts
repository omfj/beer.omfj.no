// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {}

	interface ImportMetaEnv {
		readonly VITE_API_BASE_URL?: string;
	}
}

export {};
