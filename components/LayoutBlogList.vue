<template>
  <a
    :href="'/blog/' + id + '/'"
    class="flex flex-col px-4 py-8 md:p-10 md:w-4/12 border-2 border-gray-400"
  >
    <img
      :src="`${imageUrl}`"
      class="md:w-11/12 mx-auto mb-10"
    />

    <time class="text-sm md:text-lg mb-1 block fontsemi-bold">
      {{ date | formatDate }}
    </time>

    <div class="text-2xl">
      {{ title }}
    </div>

  </a>
</template>

<script>
export default {
  name: 'LayoutBlogList',
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
    image: {
      type: Object,
    },
    imageUrl: {
      type: String,
    }
  }
}
</script>