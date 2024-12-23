<template>
  <RouterView />
</template>

<script setup lang="ts">
import { useRecipesStore } from '@/stores'

useRecipesStore().loadRecipes()
</script>
