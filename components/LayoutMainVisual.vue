<template>
  <!-- top -->
  <div
    class="relative mt-3 md:pt-11 md:pb-32 border-2"
    :style="`min-height: ${height}vh`"
  >
    <div :class="`filter hue-rotate absolute top-0 right-0 w-10/12 md:h-5/6 h-5/6 m-auto bg-center bg-cover ${visual}`">
      <span
        id="blackOverlay"
        class="w-full h-full absolute opacity-75"
      ></span>
    </div>

    <div class="absolute top-20 z-2 w-9/12 md:w-5/12 ml-5 md:ml-36 p-4 md:py-10 md:px-16"
      style="background-image: linear-gradient(to right, rgba(30,144,255, 0.5), rgba(0, 144, 0, 0.5));"
    >
      <h1 class="text-gray-200 font-sm text-2xl md:text-6xl text-shadow mb-3 md:mb-10">
        {{ title }}
      </h1>
      <p
        class="mt-4 md:ml-4 text-sm md:text-xl md:w-9/12 text-gray-100 tracking-wider"
      >
        腸は美しく新しい宗教を開くが、秋葉原がヴァイオリンに感心すると聞いたことがある。噂によると、馬は指紋のことを待ちわびるだろう。
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
      default: "60"
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