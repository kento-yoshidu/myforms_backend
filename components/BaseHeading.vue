<template>
  <h2 class="font-sans text-3xl md:text-6xl text-gray-600 text-center mb-8 md:mb-14">
    {{ title }} <span class="text-xs md:text-3xl">{{ sub }}</span>
  </h2>
</template>

<style scoped>
h2::first-letter {
  color: rgb(0, 114, 225);
}
</style>

<script>
export default {
  name: 'BaseHeading',
  props: {
    title: {
      type: String,
      default: "",
      require: true
    },
    sub: {
      type: String,
      default: "",
      require: true,
    }
  }
}
</script>