<template>
  <!-- top -->
  <div
    class="relative mt-20 md:pt-16 pb-32 flex z-10"
    :style="`min-height: ${height}vh`"
  >
    <div :class="`absolute top-0 right-0 w-10/12 h-5/6 m-auto bg-center bg-cover ${visual}`">
      <span
        id="blackOverlay"
        class="w-full h-full absolute opacity-75"
      ></span>
    </div>

    <div class="absolute top-20 container relative mx-auto border-4 p-4">
      <!--<h1 class="text-gray-400 font-semibold text-5xl text-shadow">-->
      <h1 class="filter drop-shadow-lg text-blue-500 font-sm text-3xl md:text-4xl text-shadow">
        {{ title }}
      </h1>
      <p
        v-if="message" 
        class="mt-4 text-2xl text-gray-700 text-shadow-md"
      >
        {{ message }}
      </p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'LayoutMainVisual',
  props: {
    title: {
      type: String,
      default: "",
    },
    message: {
      type: String,
      default: ''
    },
    visual: {
      type: String,
      default: 'visual-home'
    },
    height: {
      type: String,
      default: "90"
    },
  }
}
</script>

<style scoped>
.visual-home {
  background-image: url('~@/assets/img/visual-home.jpg');
}
.visual-concept {
  background-image: url('~@/assets/img/visual-concept.jpg');
}
.visual-information {
  background-image: url('~@/assets/img/visual-information.jpg');
}
.visual-error {
  background-image: url('~@/assets/img/visual-error.jpg');
}
</style>