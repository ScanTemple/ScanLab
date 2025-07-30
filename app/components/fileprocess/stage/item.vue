<script setup lang="ts">
defineProps<{
  reference: DataStageLayer['reference']
}>()
</script>

<template>
  <li class="inline-block">
    <NuxtLink
      type="button"
      class="font-mono uppercase hover:bg-zinc-700 text-neutral-300/50 not-[&.router-link-active]:hover:text-neutral-300 [&.router-link-active]:border-b-zinc-300 not-[&.router-link-active]:hover:border-b-zinc-700 [&.router-link-active]:text-neutral-300 transition-colors py-2 px-4 border-y border-y-transparent items-center [&.router-link-active]:pointer-events-none inline-flex w-full gap-[1ch]"
      :to="reference.url"
    >
      <icon :name="reference.icon" />

      <span>
        {{ reference.label }}
      </span>
    </NuxtLink>
  </li>
</template>
