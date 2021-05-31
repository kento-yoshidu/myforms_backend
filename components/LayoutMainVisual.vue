<template>
  <!-- top -->
  <div
    class="relative mt-3 md:pt-11 md:pb-32 h-48 md:h-96"
  >
    <div :class="`absolute top-0 right-0 w-10/12 md:h-full h-5/6 m-auto bg-center bg-cover ${visual}`">
      <span
        id="blackOverlay"
        class="w-full h-full absolute opacity-75"
      ></span>
    </div>

    <div class="absolute top-10 z-2 w-9/12 md:w-5/12 ml-5 md:ml-36 p-4 md:py-10 md:px-16"
      style="background-image: linear-gradient(to right, rgba(30,144,255, 0.4), rgba(0, 114, 165, 0.8));"
    >
      <h1 class="text-gray-200 font-sm text-2xl md:text-6xl text-shadow mb-3 md:mb-10">
        {{ title }}
      </h1>
      <p
        class="mt-4 md:ml-4 text-sm md:text-xl md:w-10/12 text-gray-100 tracking-widest"
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
  }
}
</script>

<style scoped>
.visual-home {
  background-image: url('~@/assets/img/visual-home.webp');
}
.visual-concept {
  background-image: url('~@/assets/img/visual-concept.jpg');
}
.visual-information {
  background-image: url('~@/assets/img/visual-information.jpg');
}
.visual-blog {
  background-image: url('~@/assets/img/visual-blog.webp');
}
.visual-error {
  background-image: url('~@/assets/img/visual-error.jpg');
}
</style>