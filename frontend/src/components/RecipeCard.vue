<template>
  <Card>
    <template #header>
      <img :alt="`${recipe.name} image`" class="w-full" :src="image" />
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
import { Card, Divider } from 'primevue'
import { computed } from 'vue'
import { type Recipe } from '@/types'

const props = defineProps<{ recipe: Recipe }>()

const image = computed(() => `${window.location.origin}/recipes/${props.recipe.image}`)
</script>
