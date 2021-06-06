<template>
  <all-wrapper>
    <layout-main-visual
      title="お知らせ"
      visual="visual-information"
    />

    <!-- main -->
    <main class="w-full md:max-w-4xl mx-auto pt-8 px-6 md:px-0 border-2">

      <section class="mb-2 text-center border-2">
        <p class="text-2xl text-center tracking-widest mb-5" style="color: orange">鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。</p>
        <p class="text-2xl text-center tracking-widest mb-5" style="color: orange">哲学がハングアップする星座がホームページ上で食事する。</p>
      </section>

      <layout-information-list
        v-for="(item, index) in items"
        :id="item.id"
        :key="index"
        :date="item.date"
        :title="item.title"
        :body="item.body"
      />
    </main>
  </all-wrapper>
</template>

<script>
import axios from 'axios'

export default {
  async asyncData({ $config }) {
    const { data } = await axios.get(`${$config.apiUrl}/information`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    return {
      items: data.contents,
    }
  },
}
</script>