import { svelteTesting } from '@testing-library/svelte/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';
import tailwindcss from '@tailwindcss/vite';
import devtoolsJson from 'vite-plugin-devtools-json';
import mkcert from 'vite-plugin-mkcert';
import { loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, process.cwd(), '');

	return {
		plugins: [
			sveltekit(),
			tailwindcss(),
			devtoolsJson(),
			mkcert(),
			SvelteKitPWA({
				registerType: 'autoUpdate',
				kit: {
					spa: true,
					adapterFallback: 'index.html'
				},
				workbox: {
					// SPA builds have only client assets; the adapter creates the fallback later.
					globPatterns: ['client/**/*.{js,css,ico,png,svg,webp,webmanifest}'],
					modifyURLPrefix: { 'client/': '' }
				},
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
		// Keep local browser requests same-origin while the API runs on a separate port.
		server: {
			proxy: {
				'/api': {
					target: env.API_PROXY_TARGET || 'http://localhost:3000',
					changeOrigin: true,
					rewrite: (path) => path.replace(/^\/api/, '')
				}
			}
		},

		test: {
			projects: [
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
	};
});
