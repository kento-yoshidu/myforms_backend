<template>
  <div
    class="mb-10 md:w-8/12 mx-auto transition-10"
  >
    <time class="text-sm md:text-base mb-1 block md:w-1/6">
      {{ date | formatDate }}
    </time>
    <div class="md:w-10/12 text-2xl mb-4">
      {{ title }}
    </div>

    <div
      v-html="body"
    />
  </div>
</template>

<script>
export default {
  name: 'LayoutInformationList',
  props: {
    title: {
      type: String,
      required: true
    },
    date: {
      type: String,
      required: true
    },
    id: {
      type: String,
      required: true
    },
    body: {
      type: String,
    }
  }
} 
</script>
