<template>
  <div class="flex h-full justify-center w-full">
    <RecipesGallery :recipes="store.recipes" />
  </div>
</template>

<script setup lang="ts">
import RecipesGallery from '@/components/RecipesGallery.vue'
import { useRecipesStore } from '@/stores'

const store = useRecipesStore()
</script>
