<template>
  <section class="min-h-dvh p-2 space-y-2 flex flex-col">
    <main class="p-2 grow">
      <slot />
    </main>

    <footer>
      <fileprocess-stage />
    </footer>
  </section>
</template>
