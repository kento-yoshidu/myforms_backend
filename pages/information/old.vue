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

      <layout-information-list
        v-for="(item, index) in items"
        :id="item.id"
        :key="index"
        :date="item.date"
        :title="item.title"
        :body="item.body"
      />

      <base-button
        name="最新のお知らせを見る"
        link="/information/"
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