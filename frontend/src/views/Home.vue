<template>
  <div v-if="recipe" class="flex h-full justify-center w-full">
    <RouterLink :to="{name: Route.Recipe, params: {id: recipe.id}}">
      <Image :alt="`${recipe.name} Preview`" :src="src" width="250" />
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Image from 'primevue/image'
import { Route } from '@/router'
import { useRecipesStore } from '@/stores'

const store = useRecipesStore()

// Use the first recipe for now
const recipe = computed(() => store.recipes[0])
const src = computed(() => `${window.location.origin}/recipes/${recipe.value.image}`)
</script>
