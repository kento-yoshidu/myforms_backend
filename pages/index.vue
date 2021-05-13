<template>
  <all-wrapper>

    <layout-main-visual
      title="Nuxt Dental Clinic"
      message="恐るべき結婚式がオウム貝のような戦艦を開発するかも知れない。流れ星がブームを巻き起こした後、鉄格子は線路を改良した酢こんぶの胎内に宿ると思われる。"
    />

    <!-- main -->
      <page-title>
        Nuxt Dental Clinicへようこそ
      </page-title>

      <section class="relative mb-10 text-center mx-auto w-5/6">
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
          <div class="absolute top-72 left-16 md:left-96" style="background-color: rgba(30, 144, 255, 0.2); width: 10vw; height: 10vw;"></div>
          <div class="absolute bottom-4 md:-bottom-8 right-16 md:right-2" style="background-color: rgba(30, 144, 255, 0.5); width: 10vw; height: 10vw;"></div>
        </div>

      </section>

      <section class="py-8 md:w-4/6 mx-auto md-16 md:mt-32">
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

      <section class="py-4 md:py-8 md:w-4/6 mx-auto">
        <!-- 診療内容 -->
        <base-heading>
          診察内容
        </base-heading>

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
              style="background-color: rgba(30, 144, 255, .3);"
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
              style="background-color: rgba(30, 144, 255, .3);"
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
              style="background-color: rgba(30, 144, 255, .3);"
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
              style="background-color: rgba(30, 144, 255, .3);"
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
              style="background-color: rgba(30, 144, 255, .3);"
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

      <section class="py-4 md:py-8">
        <!-- 診療内容 -->
        <base-heading>
          当院のメンバー
        </base-heading>

        <div
          class="md:mt-28 mb-12 md:mb-20 md:mr-12 main-img2 relative bg-left bg-cover"
        >
          <div class="absolute top-0 left-0 md:w-3/6 bg-white h-full p-2 md:p-10">
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">嬉汚らわしい電信柱が雀の一部であるプロレスラーの本質であるに違いない。虫かごが焼きそばを売りに出す。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">筋肉はその物体のようにも見えることがある。</p>
            <p class="md:text-lg mx-auto mb-3 font-semibold text-gray-800 tracking-widest">雀は強力に誕生すると発表されたが、結局は、ノアの箱船のごとき食物連鎖が帆船であると信じられていた。</p>
          </div>

          <div
            class="absolute -top-2 md:-top-14 left-4 md:left-52"
            style="background-color: rgba(30, 144, 255, 0.1); width: 6vw; height: 6vw;"
          ></div>
          <div class="absolute top-28 right-44" style="background-color: rgba(0, 114, 225, 0.6); width: 9vw; height: 9vw;"></div>
          <div class="absolute top-72 left-16 md:left-96" style="background-color: rgba(30, 144, 255, 0.2); width: 10vw; height: 10vw;"></div>
          <div class="absolute bottom-4 md:-bottom-8 left-16 md:right-2" style="background-color: rgba(30, 144, 255, 0.1); width: 22vw; height: 22vw;"></div>
        </div>

        <base-button
          name="詳しく見る"
          link="examine"
        />
      </section>

      <section class="py-4 md:py-8">
        <!-- 診療内容 -->
        <base-heading>
          ブログ
        </base-heading>

        <div class="flex flex-wrap justify-around mb-20 items-between md:w-4/6 mx-auto">
          <layout-blog-list
            v-for="(item, index) in blogItems"
              :id="item.id"
              :key="index"
              :date="item.date"
              :title="item.title"
              :image="item.image"
              :imageUrl="item.image.url"
          />
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
    const info = await axios.get(`${$config.apiUrl}/information?limit=3`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    const blog = await axios.get(`${$config.apiUrl}/blog?limit=10`, {
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
  background-image: url('~@/assets/img/main01.jpg');
  height: 70vh;
  @media screen and (max-width: 768px) {
    background-image: none ;
    height: 55vh;
  }
}

.main-img2 {
  background-image: url('~@/assets/img/main02.jpg');
  height: 70vh;
  @media screen and (max-width: 768px) {
    background-image: none ;
    height: 55vh;
  }
}
</style>