import './assets/main.css'
import 'virtual:uno.css'
import App from './App.vue'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

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
			},
		},
	})
	.mount('#app')
