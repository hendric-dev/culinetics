<template>
  <div class="flex h-full justify-center w-full">
    <RecipeCard
      v-if="recipe"
      class="mt-8 w-96"
      :recipe="recipe"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import RecipeCard from '@/components/RecipeCard.vue'
import { useRecipesStore } from '@/stores'

const props = defineProps<{ id: string }>()

const store = useRecipesStore()
const recipe = computed(() => store.recipes.find(({ id }) => id === props.id))
</script>
