import tailwindcss from '@tailwindcss/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';
import basicSsl from '@vitejs/plugin-basic-ssl';

export default defineConfig({
	plugins: [
		sveltekit(),
		tailwindcss(),
		basicSsl(),
		SvelteKitPWA({
			registerType: 'autoUpdate',
			devOptions: {
				enabled: false
			},
			manifest: {
				name: 'Beer Counter - Tell dine øl',
				short_name: 'Beer Counter',
				description: 'En enkel app for å telle og registrere øl på arrangementer med venner.',
				theme_color: '#f8f8f7',
				background_color: '#f8f8f7',
				display: 'standalone',
				scope: '/',
				start_url: '/',
				lang: 'no',
				orientation: 'portrait-primary',
				categories: ['social', 'lifestyle', 'utilities'],
				icons: [
					{
						src: 'favicon-16x16.png',
						sizes: '16x16',
						type: 'image/png'
					},
					{
						src: 'favicon-32x32.png',
						sizes: '32x32',
					type: 'image/png'
					},
					{
						src: 'apple-touch-icon.png',
						sizes: '180x180',
						type: 'image/png',
						purpose: 'any'
					},
					{
						src: 'android-chrome-192x192.png',
						sizes: '192x192',
						type: 'image/png',
						purpose: 'any maskable'
					},
					{
						src: 'android-chrome-512x512.png',
						sizes: '512x512',
						type: 'image/png',
						purpose: 'any maskable'
					}
				]
			}
		})
	],

	test: {
		workspace: [
			{
				extends: './vite.config.ts',
				plugins: [svelteTesting()],

				test: {
					name: 'client',
					environment: 'jsdom',
					clearMocks: true,
					include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['src/lib/server/**'],
					setupFiles: ['./vitest-setup-client.ts']
				}
			},
			{
				extends: './vite.config.ts',

				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});
