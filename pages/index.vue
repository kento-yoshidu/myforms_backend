<template>
  <all-wrapper>

    <layout-main-visual
      title="Nuxt Dental Clinic"
      message="恐るべき結婚式がオウム貝のような戦艦を開発するかも知れない。流れ星がブームを巻き起こした後、鉄格子は線路を改良した酢こんぶの胎内に宿ると思われる。"
      height="60"
    />

    <!-- main -->
    <main class="w-full mx-auto pt-10 px-6 md:px-0 border-2">

      <page-title>
        Nuxt Dental Clinicへようこそ
      </page-title>

      <section class="mb-10 text-center">
        <p class="md:text-2xl text-center text-blue-500 tracking-widest mb-3 md:mb-5">鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。</p>
        <p class="md:text-2xl text-center text-blue-500 tracking-widest mb-5 md:mb-12">哲学がハングアップする星座がホームページ上で食事する。</p>

        <p class="md:w-11/12 mx-auto mb-3 tracking-wide">嬉しいことに、布団が踊る硫黄が遠ざかる気がした子猫を搭載したヴァイオリンが酒に発明される。</p>
        <p class="w-10/12 mx-auto mb-3 tracking-wide">つまるところ、蛙が勢い良く蒸気を吹き出す絆創膏のサポートを打ち切るまでもない。</p>
        <p class="w-10/12 mx-auto mb-3 tracking-wide">シャーロック・ホームズがドラゴンクエストテクノロジーとも呼ばれるはさみに恋文を送るなんてことはないが、マッシュルームが地獄から来た登山を持ち上げる。</p>
      </section>

      <section>
        <div
          class="w-screen m-auto border-2 h-5/6 bg-cover"
          style="background-image: url('/_nuxt/assets/img/sec2.jpg'); height: 10vh"
        >
        </div>
      </section>
      <section class="py-8">
        <base-heading>
          クリニックからのお知らせ
        </base-heading>

        <div class="mb-14">
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

        <div class="flex flex-wrap my-10 md:my-14">
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image01.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));
                    hover: background-image: linear-gradient(to right bottom, rgba(0,144,0, 0.3), rgba(30, 144, 255, 0.3));"
            >
              <p class="text-2xl md:text-4xl">虫歯治療</p>
              <p class="text-lg">jgapirjpa</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image02.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));"
            >
              <p class="text-2xl md:text-4xl">歯周病治療</p>
              <p class="text-2xl">jgapirjpa</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image03.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));"
            >
              <p class="text-2xl md:text-4xl">矯正歯科</p>
              <p class="text-2xl">jgapirjpa</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image04.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));"
            >
              <p class="text-2xl md:text-4xl">インプラント</p>
              <p class="text-2xl">jgapirjpa</p>
            </div>
          </a>
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image05.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));"
            >
              <p class="text-2xl md:text-4xl">ホワイトニング</p>
              <p class="text-2xl">jgapirjpa</p>
            </div>
          </a>
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image06.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-image: linear-gradient(to right bottom, rgba(30,144,255, 0.3), rgba(0, 144, 0, 0.3));"
            >
              <p class="text-2xl md:text-4xl">小児歯科</p>
              <p class="text-2xl">jgapirjpa</p>
            </div>
          </a>
        </div>

        <base-button
          name="詳しく見る"
          link="examine"
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
