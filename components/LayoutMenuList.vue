<template>
  <div :class="wrapClass">

    <div :class="itemClass">
      <div :class="blockClass">
        <img
          src="~/assets/img/menu04.jpg"
          alt="ケーキ"
          :class="imageClass"
        />
        <div :class="dataClass">
          <div class="font-bold text-xl mb-2">オリジナルパンケーキ</div>
          <p v-if="flagBody" class="text-base mb-2">オリジナルパンケーキ。自然素材にこだわりました。</p>
        </div>
      </div>
    </div>

    <div :class="itemClass">
      <div :class="blockClass">
        <img
          src="~/assets/img/menu04.jpg"
          alt="ケーキ"
          :class="imageClass"
        />
        <div :class="dataClass">
          <div class="font-bold text-xl mb-2">オリジナルパンケーキ</div>
          <p v-if="flagBody" class="text-base mb-2">オリジナルパンケーキ。自然素材にこだわりました。</p>
        </div>
      </div>
    </div>

    <div :class="itemClass">
      <div :class="blockClass">
        <img
          src="~/assets/img/menu04.jpg"
          alt="ケーキ"
          :class="imageClass"
        />
        <div :class="dataClass">
          <div class="font-bold text-xl mb-2">オリジナルパンケーキ</div>
          <p v-if="flagBody" class="text-base mb-2">オリジナルパンケーキ。自然素材にこだわりました。</p>
        </div>
      </div>
    </div>

  </div>
  
</template>

<script>
export default {
  name: 'LayoutMenuList',
  props: {
    wrapClass: {
      type: String,
      default: ''
    },
    itemClass: {
      type: String,
      default: "w-full mb-10 shadow-lg"
    },
    blockClass: {
      type: String,
      default: "w-full bg-gray-200 flex flex-wrap justify-between px-4 py-4"
    },
    imageClass: {
      type: String,
      default: "md:w-2/5"
    },
    dataClass: {
      type: String,
      default: "w-7/12 px-6 py-4 mb:px-0 mb:px-0"
    },
    flagBody: {
      type: Boolean,
      default: true
    }
  }
}
</script>
