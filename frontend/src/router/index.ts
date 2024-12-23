import { createRouter, createWebHistory } from 'vue-router'
import { routes } from './routes'

export { Route } from './routes'

export const router = createRouter({
	history: createWebHistory(),
	routes,
})
