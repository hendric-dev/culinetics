import { createRouter, createWebHashHistory } from 'vue-router'
import { routes } from './routes'

export { Route } from './routes'

export const router = createRouter({
	history: createWebHashHistory(),
	routes,
})
