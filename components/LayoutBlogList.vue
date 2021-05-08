<template>
  <div
    class="mb-2 md:w-2/12 border-2 border-blue-400 transition-10
          hover:text-blue-600 hover:bg-blue-100"
  >
    <a
      :href="'/blog/' + id + '/'"
      class="block px-4 py-3"
    >
      <time class="text-sm md:text-base mb-1 block md:w-1/6">
        {{ date | formatDate }}
      </time>
      <div class="md:w-10/12 md:text-lg">
        {{ title }}
      </div>
      <img :src="`${imageUrl}`" />
    </a>
  </div> 
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