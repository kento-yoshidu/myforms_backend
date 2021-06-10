<template>
  <all-wrapper>
    <layout-main-visual
      title="過去のお知らせ"
      visual="visual-information"
    />

    <!-- main -->
    <main class="w-full md:max-w-4xl mx-auto px-6 md:px-0">

      <section class="mb-2 text-center">
        <p class="text-lg md:text-2xl text-center tracking-widest mb-10" style="color: orange">過去のお知らせ一覧です。</p>
      </section>

      <div
        class="mb-10 w-11/12 md:w-8/12 mx-auto transition-10 borde-2"
        :id="item.id"
        v-for="(item, index) in items" :key="index"
      >
        <div class="border-l-8 border-blue-400 pl-4 -ml-4 py-1 mb-4">
          <time class="text-sm md:text-base block md:w-1/6">
            {{ item.date | formatDate }}
          </time>
          <div class="md:w-10/12 text-lg md:text-2xl">
            {{ item.title }}
          </div>
        </div>

        <div
          v-html="item.body"
          class="mb-10"
        />

        <hr class="border-b-2 -ml-4">
      </div>

      <div class="mx-auto text-center md:mb-12">
        <a
          class="border-b-2 border-lightblue-500 pb-1 px-2"
          href="/information/"
        >
          最新のお知らせを見る
        </a>
      </div>

      <base-button
        name="HOME"
        link="/"
      />
    </main>
  </all-wrapper>
</template>

<script>
import axios from 'axios'

export default {
  async asyncData({ $config }) {
    const { data } = await axios.get(`${$config.apiUrl}/information?filters=isRecent[equals]false`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    return {
      items: data.contents,
    }
  },
}
</script>