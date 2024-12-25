<template>
  <Card>
    <template #header>
      <Image :alt="`${recipe.name} image`" :src="`../images/${recipe.image}`" width="100%" />
    </template>
    <template #title>{{ recipe.name }}</template>
    <template #content>
      <li v-for="ingredient in recipe.ingredients">
        {{ ingredient }}
      </li>
      <Divider />
      <li v-for="instruction in recipe.instructions">
        {{ instruction }}
      </li>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { Card, Divider, Image } from 'primevue'
import { type Recipe } from '@/types'

interface Props {
	recipe: Recipe
}

defineProps<Props>()
</script>
