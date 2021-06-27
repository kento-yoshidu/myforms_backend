<template>
  <!-- top -->
  <div class="relative mt-3 md:pt-11 md:pb-32 h-48 mb-10 visual-image">
    <!-- 背景画像 -->
    <div
      :class="`absolute top-0 right-0 w-10/12 h-full md:h-full m-auto bg-center bg-cover ${visual}`"
    >
      <span id="blackOverlay" class="w-full h-full absolute opacity-75"></span>
    </div>

    <!-- 青い背景の説明文 -->
    <div
      class="
        absolute
        top-4
        z-2
        w-10/12
        md:w-6/12
        ml-5
        md:ml-36
        p-4
        md:py-10 md:px-16
      "
      style="background-image: linear-gradient(to right,rgba(30, 144, 255, 0.4), rgba(0, 114, 165, 0.8))"
    >
      <h1
        class="
          text-gray-200
          font-sm
          text-3xl
          md:text-6xl
          text-shadow
          mb-3
          md:mb-10
        "
      >
        {{ title }}
      </h1>
      <p
        class="
          mt-4
          md:ml-4
          text-base
          md:text-2xl md:w-11/12
          text-gray-100
          tracking-widest
        "
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
      default: '',
    },
    message: {
      type: String,
      default: '',
    },
    visual: {
      type: String,
      default: 'visual-home',
    },
  },
}
</script>

<style scoped>
.visual-image {
  height: 30vh;
}

@media screen and (min-width: 768px) {
  .visual-image {
    height: 50vh;
  }
}

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
