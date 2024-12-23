import './assets/main.css'
import 'virtual:uno.css'
import App from './App.vue'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { router } from './router'

createApp(App)
	.use(createPinia())
	.use(PrimeVue, {
		theme: {
			preset: Aura,
			options: {
				cssLayer: {
					name: 'primevue',
					order: 'unocss, primevue',
				},
				darkModeSelector: '.dark-mode',
			},
		},
	})
	.use(router)
	.mount('#app')
