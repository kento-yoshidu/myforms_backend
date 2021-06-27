<template>
  <h1
    class="
      md:mt-16
      mb-5
      md:mb-4
      w-5/6
      mx-auto
      font-sans
      text-3xl
      md:text-6xl
      text-gray-600 text-center
    "
  >
    <slot />
  </h1>
</template>

<script>
export default {
  name: 'PageTitle',
}
</script>
