<template>
  <div class="flex flex-col h-full items-center w-full">
    <h1>Choose your Meal</h1>
    <RecipesGallery :recipes="store.recipes" width="400" />
  </div>
</template>

<script setup lang="ts">
import RecipesGallery from '@/components/RecipesGallery.vue'
import { useRecipesStore } from '@/stores'

const store = useRecipesStore()
</script>
