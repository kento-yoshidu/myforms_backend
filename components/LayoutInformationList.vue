<template>
  <div
    class="mb-10 w-11/12 md:w-8/12 mx-auto transition-10"
    :id="id"
  >
    <div class="border-l-8 border-blue-400 pl-4 -ml-4 py-1 mb-4">
      <time class="text-sm md:text-base block md:w-1/6">
        {{ date | formatDate }}
      </time>
      <div class="md:w-10/12 text-lg md:text-2xl">
        {{ title }}
      </div>
    </div>

    <div
      v-html="body"
      class="mb-10"
    />

    <hr class="border-b-2 -ml-4">
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
