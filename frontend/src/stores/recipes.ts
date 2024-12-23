import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type Recipe } from '@/types'

const RECIPE_NAMES = ['pancakes'] as const

export const useRecipesStore = defineStore('🌮 Recipes', () => {
	// State
	const recipes = ref<Recipe[]>([])

	// Actions
	async function loadRecipes() {
		const requests = RECIPE_NAMES.map(name => fetch(`${window.location.origin}/recipes/${name}.json`))

		try {
			const responses = await Promise.all(requests)
			const json: Array<Promise<Recipe>> = responses.map(response => response.json())
			recipes.value = await Promise.all(json)
		} catch (e) {
			console.error(e)
		}
	}

	return { loadRecipes, recipes }
})
