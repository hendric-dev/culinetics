import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type Recipe } from '@/types'

const RECIPE_NAMES = ['chili-con-carne', 'gyudon', 'pancakes'] as const

export const useRecipesStore = defineStore('🌮 Recipes', () => {
	// State
	const recipes = ref<Recipe[]>([])

	// Actions
	async function loadRecipes() {
		const imports = RECIPE_NAMES.map(name => import(`@/assets/recipes/${name}.json`))

		try {
			recipes.value = await Promise.all(imports)
		} catch (e) {
			console.error(e)
		}
	}

	return { loadRecipes, recipes }
})
