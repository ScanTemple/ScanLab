<template>
  <ui-modal-base>
    <template #trigger="params">
      <slot
        name="trigger"
        v-bind="params"
      />
    </template>

    <template #default="params">
      <section class="size-full">
        <header class="font-mono uppercase inline-flex items-center justify-between gap-2 w-full">
          <ui-button
            type="button"
            @click="params.hide()"
          >
            <icon name="ic:baseline-arrow-back" />
          </ui-button>

          <span class="grow">
            <slot
              name="header"
              v-bind="params"
            />
          </span>
        </header>

        <hr class="border-zinc-700/50 pt-2 mt-2">

        <div class="px-4 py-2">
          <slot
            name="body"
            v-bind="params"
          />
        </div>
      </section>
    </template>
  </ui-modal-base>
</template>
