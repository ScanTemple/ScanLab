<template>
  <section class="min-h-dvh p-2 space-y-2 flex flex-col">
    <main class="p-2 grow">
      <slot />
    </main>

    <footer class="p-2 flex justify-center bg-zinc-800/30 shadow sticky bottom-2 leading-none backdrop-blur-md">
      <fileprocess-stage />
    </footer>
  </section>
</template>
