import { browser } from '$app/environment';
import { getContext, setContext } from 'svelte';

type Theme = 'light' | 'dark';

class ThemeState {
	#theme = $state<Theme>('light');

	constructor() {
		$effect(() => {
			if (!browser) return;
			const stored = localStorage.getItem('theme') as Theme | null;
			if (stored === 'dark' || stored === 'light') {
				this.#theme = stored;
			}
		});

		// Save to localStorage and update document class when theme changes
		$effect(() => {
			if (!browser) return;
			localStorage.setItem('theme', this.#theme);
			document.documentElement.classList.toggle('dark', this.#theme === 'dark');
		});
	}

	get current(): Theme {
		return this.#theme;
	}

	get isDark(): boolean {
		return this.current === 'dark';
	}

	toggle() {
		this.#disableTransitions();
		this.#theme = this.#theme === 'light' ? 'dark' : 'light';
	}

	set(theme: Theme) {
		this.#disableTransitions();
		this.#theme = theme;
	}

	// Disable transitions during theme change
	#disableTransitions() {
		if (!browser) return;
		document.documentElement.classList.add('disable-transitions');

		setTimeout(() => {
			document.documentElement.classList.remove('disable-transitions');
		}, 0);
	}
}

const THEME_KEY = Symbol('theme');

export function createThemeContext() {
	return setContext(THEME_KEY, new ThemeState());
}

export function getThemeContext() {
	return getContext<ReturnType<typeof createThemeContext>>(THEME_KEY);
}
