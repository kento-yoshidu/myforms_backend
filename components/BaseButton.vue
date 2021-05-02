<template>
  <div class="mb-10 mx-auto text-center">
    <a
      :href="link"
      class="text-purple-900 font-semibold md:text-lg xl:text-base px-10 md:px-10 xl:px-4 py-3 md:py-4 xl:py-3 leading-tight bg-white text-gray-800 border-2 border-purple-900
            hover:text-white"
      style="border: 2px solid orange;"
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