<template>
  <div class="wrapper">
    <slot />
  </div>
</template>

<script>
export default {
  name: 'LayoutWrapper',
}
</script>

<style scoped>
.wrapper {
  padding-top: 8000px;
  border: 1px solid red;
  @media (min-width: 768px) {
    padding-top: 0;
  }
}
</style>