<template>
  <div class="my-14 mx-auto text-center">
    <a
      :href="link"
      class="text-blue-500 font-semibold text-sm md:text-3xl px-8 md:px-10 xl:px-8 py-3 md:py-4 xl:py-3 leading-tight bg-white text-gray-800
            border-2 border-blue-500 rounded-sm duration-100
            hover:text-white hover:bg-blue-500"
    >
      {{ name }}
    </a>
  </div>
</template>

<script>
export default {
  name: 'BaseButton',
  props: {
    name: {
      type: String,
      requried: true
    },
    link: {
      type: String,
      required: true
    }
  }
}
</script>