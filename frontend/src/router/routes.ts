import Home from '@/views/Home.vue'

export enum Route {
	Home = 'home',
	Recipe = 'recipe',
}

export const routes = [
	{
		component: Home,
		name: Route.Home,
		path: '/',
	},
	{
		component: () => import('@/views/Recipe.vue'),
		name: Route.Recipe,
		path: '/recipes/:id',
		props: true,
	},
]
