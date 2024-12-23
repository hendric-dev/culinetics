<template>
  <Galleria
    :circular="true"
    :showIndicators="true"
    :showItemNavigators="true"
    :showItemNavigatorsOnHover="true"
    :showThumbnails="false"
    :value="recipes"
  >
    <template #item="recipe">
      <RouterLink :to="{name: Route.Recipe, params: {id: recipe.item.id}}">
        <Image :alt="`${recipe.item.name} Preview`" :src="getImageSrc(recipe.item.image)" width="250" />
      </RouterLink>
    </template>
    <template #thumbnail="recipe">
      <Image :alt="`${recipe.item.name} Preview`" :src="getImageSrc(recipe.item.image)" />
    </template>
  </Galleria>
</template>

<script setup lang="ts">
import { Galleria, Image } from 'primevue'
import type { Recipe } from '@/types'
import { Route } from '@/router'

const props = defineProps<{ recipes: Recipe[] }>()

const getImageSrc = (image: string): string => `${window.location.origin}/recipes/${image}`
</script>
