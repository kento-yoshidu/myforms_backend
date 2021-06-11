<template>
  <all-wrapper>

    <layout-main-visual
      title="Nuxt Dental Clinic"
      message="恐るべき結婚式がオウム貝のような戦艦を開発するかも知れない。"
    />

    <!-- main -->
      <page-title>
        Nuxt Dental Clinicへようこそ
      </page-title>

      <section class="relative mb-8 md:mb-24 text-center mx-auto w-5/6">
        <p class="md:w-3/6 md:mx-auto md:text-2xl text-center text-blue-500 tracking-widest mb-3 md:mb-5">鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。</p>
        <p class="md:w-3/6 md:mx-auto md:text-2xl text-center text-blue-500 tracking-widest mb-12 md:mb-24">哲学がハングアップする星座がホームページ上で食事する。</p>

        <div
          class="md:mt-28 main-img relative bg-center bg-cover"
        >
          <div class="absolute top-0 md:left-40 md:w-2/6 bg-white h-full p-2 md:p-10">
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">嬉しいことに、布団が踊る硫黄が遠ざかる気がした子猫を搭載したヴァイオリンが酒に発明される。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">つまるところ、蛙が勢い良く蒸気を吹き出す絆創膏のサポートを打ち切るまでもない。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">シャーロック・ホームズがドラゴンクエストテクノロジーとも呼ばれるはさみに恋文を送るなんてことはないが、マッシュルームが地獄から来た登山を持ち上げる。</p>
          </div>

          <div class="absolute -top-2 md:-top-10 left-4 md:left-28" style="background-color: rgba(30, 144, 255, 0.1); width: 20vw; height: 20vw;"></div>
          <div class="absolute top-28 right-44" style="background-color: rgba(0, 114, 225, 0.4); width: 9vw; height: 9vw;"></div>
          <div class="absolute top-56 left-16 md:left-96" style="background-color: rgba(30, 144, 255, 0.2); width: 10vw; height: 10vw;"></div>
          <div class="absolute bottom-4 md:-bottom-8 right-16 md:right-2" style="background-color: rgba(30, 144, 255, 0.5); width: 10vw; height: 10vw;"></div>
        </div>
      </section>

      <section class="py-6 w-full mx-auto md-16 md:mt-32 bg-gray-50">
        <base-heading
          title="Information"
          sub="クリニックからのお知らせ"
        />

        <div class="mb-14 w-9/12 mx-auto">
          <a
            class="block mb-10 w-11/12 md:w-8/12 mx-auto transition-10"
            v-for="(item, index) in infoItems" :key="index"
            :href="`/information/#${item.id}`"
          >
            <div class="border-l-8 border-blue-400 pl-4 -ml-4 mb-4">
              <time class="text-sm md:text-base block md:w-1/6">
                {{ item.date | formatDate }}
              </time>
              <div class="md:w-10/12 md:text-2xl">
                {{ item.title }}
              </div>
            </div>

            <hr class="border-b-2 -ml-4">
          </a>
        </div>

        <base-button
          name="お知らせを見る"
          link="information/"
        />
      </section>

      <section class="py-4 md:py-8 md:w-4/6 mx-auto">
        <!-- 診療内容 -->
        <base-heading
          title="Examine"
          sub="診察内容"
        />

        <div class="flex flex-wrap my-10 md:my-14 w-11/12 mx-auto">
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
            style="@media screen and (max-width: 768px) { height: 100px; }"
          >
            <img src="~/assets/img/examine/image01.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">虫歯治療</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image02.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">歯周病治療</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image03.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">矯正歯科</p>
            </div>
          </a>

          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image04.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">インプラント</p>
            </div>
          </a>
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image05.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">ホワイトニング</p>
            </div>
          </a>
          <a
            class="relative w-3/6 md:w-2/6 border z-2 flex flex-col justify-center items-center text-white"
            href="examine"
          >
            <img src="~/assets/img/examine/image06.png" class="h-full" />
            <div
              class="absolute h-full w-full flex flex-col justify-center items-center"
              style="background-color: rgba(30, 144, 255, .3);"
            >
              <p class="text-xl md:text-4xl">小児歯科</p>
            </div>
          </a>
        </div>

        <base-button
          name="詳しく見る"
          link="examine"
        />
      </section>

      <section class="w-11/12 mx-auto py-2 md:py-8">
        <!-- 診療内容 -->
        <base-heading
          title="Member"
          sub="当院のメンバー"
        />

        <div class="relative flex flex-row-reverse justify-around md:mb-12">
          <div
            class="hidden md:block main-img2 md:w-6/12"
          >
          </div>

          <div class="mb-8 w-10/12 md:w-4/12 h-full p-2">
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">嬉汚らわしい電信柱が雀の一部であるプロレスラーの本質であるに違いない。虫かごが焼きそばを売りに出す。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">筋肉はその物体のようにも見えることがある。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">雀は強力に誕生すると発表されたが、結局は、ノアの箱船のごとき食物連鎖が帆船であると信じられていた。</p>
          </div>

          <div class="absolute top-20 right-44" style="background-color: rgba(0, 114, 225, 0.6); width: 9vw; height: 9vw;"></div>
          <div class="absolute top-32 left-16 md:left-96" style="background-color: rgba(30, 144, 255, 0.2); width: 10vw; height: 10vw;"></div>
          <div class="absolute top-40 left-32 md:-bottom-8 left-16 md:right-2" style="background-color: rgba(30, 144, 255, 0.1); width: 12vw; height: 12vw;"></div>
        </div>

        <base-button
          name="詳しく見る"
          link="member"
        />
      </section>

      <section class="w-8/12 md:w-full mx-auto py-4 md:py-8">
        <!-- 診療内容 -->
        <base-heading
          title="Blog"
          sub="ブログ"
        />

        <div
          class="flex flex-wrap justify-center mb-20 items-between md:w-3/6 mx-auto border-2"
          v-for="(item, index) in blogItems" :key="index"
        >
          <a
            :href="'/blog/' + item.id + '/'"
            class="flex flex-col px-4 py-8 md:px-8 md:py-12 md:w-5/12 border-2 border-gray-400"
          >
            <img
              :src="`${item.image.url}`"
              class="w-3/6 md:w-6/12 mx-auto mb-10 md:mb-2"
            />

            <time class="text-sm md:text-lg mb-1 block fontsemi-bold">
              {{ item.date | formatDate }}
            </time>
            {{ item.title }}
          </a>
        </div>

        <base-button
          name="詳しく見る"
          link="blog"
        />
      </section>

  </all-wrapper>
</template>

<script>
import axios from 'axios'

export default {
  async asyncData({ $config }) {
    const info = await axios.get(`${$config.apiUrl}/information?filters=isRecent[equals]true`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    const blog = await axios.get(`${$config.apiUrl}/blog?limit=3`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    return {
      infoItems: info.data.contents,
      blogItems: blog.data.contents
    }
  },
}
</script>

<style>
.main-img {
  background-image: url('~@/assets/img/main01.webp');
  height: 70vh;
  @media screen and (max-width: 768px) {
    background-image: none ;
    height: 55vh;
  }
}

.main-img2 {
  background-image: url('~@/assets/img/main02.webp');
  height: 50vh;
  background-size: cover;
  background-position: center;
}
</style>