<template>
  <div>
    <header class="bg-yellow-200 mt-20 h-96">
      <h2 class="text-3xl">Title</h2>
    </header>

    <section class="mt-56">
      <Title
        title="Message"
        message="当院の院長からのメッセージです。"
      />
    </section>

    <section>
      <Title
        title="Message"
        message="こんいちは"
      />
    </section>

  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({})
</script>

<style>
.container {
  margin: 0 auto;
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-content: center;
  text-align: center;
}

.title {
  font-family: 'Quicksand', 'Source Sans Pro', -apple- system, BlinkMaxSystemFont;
  display: block;
  font-weight: 300;
  font-size: 100px;
  color: #35495e;
  letter-spacing: 1px;
}

section {
  width: 80%;
  margin: 50px auto;
  border: 1px solid red;
}

@media screen and (max-width: 768px) {
  section {
    width: 90%;
  }
}
</style>