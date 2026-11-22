// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://mfcollins3.github.io',
	base: '/time',
	integrations: [
		starlight({
			title: 'Naked Time',
			description:
				'Plan your day and track the time you spend on what matters, using the Pomodoro Technique.',
			favicon: '/favicon.svg',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/mfcollins3/time',
				},
			],
			editLink: {
				baseUrl: 'https://github.com/mfcollins3/time/edit/main/website/',
			},
			sidebar: [
				{
					label: 'Getting started',
					items: [{ autogenerate: { directory: 'getting-started' } }],
				},
				{
					label: 'Guides',
					items: [{ autogenerate: { directory: 'guides' } }],
				},
			],
		}),
	],
});
