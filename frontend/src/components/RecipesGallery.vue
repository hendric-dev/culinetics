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
        <Image :alt="`${recipe.item.name} Preview`" :src="getImageSrc(recipe.item.image)" :width="width" />
      </RouterLink>
    </template>
    <template #caption="recipe">
      <div class="flex justify-center">
        <p class="text-white">{{ recipe.item.name }}</p>
      </div>
    </template>
  </Galleria>
</template>

<script setup lang="ts">
import { Galleria, Image } from 'primevue'
import type { Recipe } from '@/types'
import { Route } from '@/router'

interface Props {
	recipes: Recipe[]
	width?: string
}

withDefaults(defineProps<Props>(), {
	recipes: () => [],
	width: '250',
})

const getImageSrc = (image: string): string => `${window.location.origin}/recipes/${image}`
</script>
