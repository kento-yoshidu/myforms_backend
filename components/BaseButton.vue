<template>
  <div class="my-8 mx-auto text-center">
    <a
      :href="link"
      class="
        text-blue-500
        text-sm
        md:text-base
        px-8
        md:px-12
        py-2
        leading-tight
        bg-white
        text-gray-800
        border-2 border-blue-500
        rounded-sm
        duration-200
        hover:text-white hover:bg-blue-500
      "
    >
      {{ name }}
      <fa
        :icon="faChevronRight"
        class="ml-2 text-sm md:text-lg leading-tight hover:ml-4"
      />
    </a>
  </div>
</template>

<script>
import { faChevronRight } from '@fortawesome/free-solid-svg-icons'

export default {
  name: 'BaseButton',
  props: {
    name: {
      type: String,
      requried: true,
    },
    link: {
      type: String,
      required: true,
    },
  },
  computed: {
    faChevronRight() {
      return faChevronRight
    },
  },
}
</script>
