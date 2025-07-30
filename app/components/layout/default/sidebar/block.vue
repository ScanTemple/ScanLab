<template>
  <section class="bg-zinc-700 p-2">
    <header class="text-center font-mono px-2 text-lg mb-2">
      <slot name="header" />
    </header>

    <section>
      <slot name="body" />
    </section>
  </section>
</template>
