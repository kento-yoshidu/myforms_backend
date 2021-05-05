<template>
  <all-wrapper>

    <layout-main-visual
      title="Nuxt Dental Clinic"
      height="60"
    />

    <!-- main -->
    <main class="w-full md:max-w-4xl mx-auto pt-10 px-6 md:px-0">

      <page-title>
        Nuxt Dental Clinicへようこそ
      </page-title>

      <section class="mb-10 text-center">
        <p class="text-2xl text-center text-blue-500 tracking-widest mb-5">鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。</p>
        <p class="text-2xl text-center text-blue-500 tracking-widest mb-20">哲学がハングアップする星座がホームページ上で食事する。</p>

        <p class="w-10/12 mx-auto text-sm mb-3 tracking-wide">嬉しいことに、布団が踊る硫黄が遠ざかる気がした子猫を搭載したヴァイオリンが酒に発明される。</p>
        <p class="w-10/12 mx-auto text-sm mb-3 tracking-wide">つまるところ、蛙が勢い良く蒸気を吹き出す絆創膏のサポートを打ち切るまでもない。</p>
        <p class="w-10/12 mx-auto text-sm mb-3 tracking-wide">シャーロック・ホームズがドラゴンクエストテクノロジーとも呼ばれるはさみに恋文を送るなんてことはないが、マッシュルームが地獄から来た登山を持ち上げる。</p>
      </section>

      <section class="py-8">
        <base-heading>
          クリニックからのお知らせ
        </base-heading>

        <div class="mb-20">
          <layout-information-list
            v-for="(item, index) in infoItems"
            :id="item.id"
            :key="index"
            :date="item.date"
            :title="item.title"
          />
        </div>

        <base-button
          name="お知らせを見る"
          link="information"
        />
      </section>

      <section class="py-8">
        <!-- 診療内容 -->
        <base-heading>
          診察内容
        </base-heading>

        <base-button
          name="詳しく見る"
          link="information"
        />
      </section>

      <section class="py-8">
        <!-- 診療内容 -->
        <base-heading>
          当院のメンバー
        </base-heading>
      </section>
    </main>

  </all-wrapper>
</template>

<script>
import axios from 'axios'

export default {
  async asyncData({ $config }) {
    const info = await axios.get(`${$config.apiUrl}/information?limit=3`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    return {
      infoItems: info.data.contents,
    }
  },
}
</script>

<style>
.visual-home {
  background-image: url('~@/assets/img/visual-home.jpg');
}
</style>
