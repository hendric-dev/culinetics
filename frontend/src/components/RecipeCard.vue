<template>
  <Card>
    <template #header>
      <img :alt="`${recipe.name} image`" class="w-full" :src="image" />
    </template>
    <template #title>{{ recipe.name }}</template>
    <template #content>
      <p class="m-0">
        Lorem ipsum dolor sit amet, consectetur adipisicing elit. Inventore sed consequuntur error repudiandae numquam deserunt quisquam repellat libero asperiores earum nam nobis, culpa ratione quam perferendis esse, cupiditate neque
        quas!
      </p>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { Card } from 'primevue'
import { computed } from 'vue'
import { type Recipe } from '@/types'

const props = defineProps<{ recipe: Recipe }>()

const image = computed(() => `${window.location.origin}/recipes/${props.recipe.image}`)
</script>
