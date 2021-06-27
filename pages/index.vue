<template>
  <all-wrapper>
    <layout-main-visual
      title="Nuxt Dental Clinic"
      message="恐るべき結婚式がオウム貝のような戦艦を開発するかも知れない。"
    />

    <!-- main -->
    <page-title> Nuxt Dental Clinicへようこそ </page-title>

    <section
      class="
        relative
        mx-auto
        w-5/6
        mt-8
        mb-2
        md:mb-24 md:text-2xl
        text-center
        tracking-widest
      "
    >
      <p class="md:w-3/6 md:mx-auto text-blue-500 mb-3 md:mb-5">
        鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。
      </p>
      <p class="md:w-3/6 md:mx-auto text-blue-500 mb-8 md:mb-24">
        哲学がハングアップする星座がホームページ上で食事する。
      </p>

      <div class="md:mt-20 main-img relative bg-center bg-cover md:text-xl">
        <div
          class="
            md:absolute
            top-0
            md:left-40 md:w-2/6
            bg-white
            p-2
            md:py-16 md:px-10
          "
        >
          <p class="mx-auto mb-4 md:mb-10 font-semibold">
            嬉しいことに、布団が踊る硫黄が遠ざかる気がした子猫を搭載したヴァイオリンが酒に発明される。
          </p>
          <p class="mx-auto mb-4 md:mb-10 font-semibold">
            つまるところ、蛙が勢い良く蒸気を吹き出す絆創膏のサポートを打ち切るまでもない。
          </p>
          <p class="mx-auto mb-4 font-semibold">
            シャーロック・ホームズがドラゴンクエストテクノロジーとも呼ばれるはさみに恋文を送るなんてことはないが、マッシュルームが地獄から来た登山を持ち上げる。
          </p>
        </div>

        <div
          class="absolute -top-2 md:-top-10 left-4 md:left-28 z-1"
          style="
            background-color: rgba(30, 144, 255, 0.1);
            width: 20vw;
            height: 20vw;
          "
        ></div>
        <div
          class="absolute top-28 right-44 -z-1"
          style="
            background-color: rgba(0, 114, 225, 0.4);
            width: 9vw;
            height: 9vw;
          "
        ></div>
        <div
          class="absolute top-40 left-16 md:left-96 z-1"
          style="
            background-color: rgba(30, 144, 255, 0.2);
            width: 10vw;
            height: 10vw;
          "
        ></div>
        <div
          class="absolute top-48 md:top-96 right-16 md:right-2 z-1"
          style="
            background-color: rgba(30, 144, 255, 0.5);
            width: 10vw;
            height: 10vw;
          "
        ></div>
      </div>
    </section>

    <section class="w-full mx-auto md-16 md:mt-32 bg-gray-50">
      <base-heading title="Information" sub="クリニックからのお知らせ" />

      <div class="mb-14 w-10/12 mx-auto">
        <a
          v-for="(item, index) in infoItems"
          :key="index"
          class="block mb-10 w-11/12 md:w-6/12 mx-auto group"
          :href="`/information/#${item.id}`"
        >
          <div
            class="
              border-l-8 border-blue-400
              pl-4
              -ml-4
              mb-4
              group-hover:text-blue-500
              transition-50
            "
          >
            <time class="text-sm md:text-lg block mb-2">
              <fa :icon="faClock" class="mr-2" />{{ item.date | formatDate }}
            </time>
            <div class="md:w-10/12 md:text-2xl">
              {{ item.title }}
            </div>
          </div>

          <hr class="border-b-2 -ml-4" />
        </a>
      </div>

      <base-button name="お知らせ一覧" link="information/" />
    </section>

    <section class="md:w-4/6 mx-auto">
      <!-- 診療内容 -->
      <base-heading title="Examine" sub="診察内容" />

      <div class="flex flex-wrap my-10 md:my-14 w-11/12 mx-auto">
        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
          style="@media screen and (max-width: 768px) {height: 100px}"
        >
          <img src="~/assets/img/examine/image01.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">虫歯治療</p>
          </div>
        </a>

        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
        >
          <img src="~/assets/img/examine/image02.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">歯周病治療</p>
          </div>
        </a>

        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
        >
          <img src="~/assets/img/examine/image03.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">矯正歯科</p>
          </div>
        </a>

        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
        >
          <img src="~/assets/img/examine/image04.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">インプラント</p>
          </div>
        </a>
        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
        >
          <img src="~/assets/img/examine/image05.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">ホワイトニング</p>
          </div>
        </a>
        <a
          class="
            relative
            w-3/6
            md:w-2/6
            border
            z-2
            flex flex-col
            justify-center
            items-center
            text-white
          "
          href="examine"
        >
          <img src="~/assets/img/examine/image06.png" class="h-full" />
          <div
            class="
              absolute
              h-full
              w-full
              flex flex-col
              justify-center
              items-center
            "
            style="background-color: rgba(30, 144, 255, 0.3)"
          >
            <p class="text-xl md:text-4xl">小児歯科</p>
          </div>
        </a>
      </div>

      <base-button name="診察内容" link="examine/" />
    </section>

    <section class="w-11/12 mx-auto">
      <!-- 診療内容 -->
      <base-heading title="Member" sub="当院のメンバー" />

      <div class="relative flex flex-row-reverse justify-around md:mb-12">
        <div class="hidden md:block main-img2 md:w-6/12"></div>

        <div class="mb-4 w-10/12 md:w-4/12 p-2 md:text-2xl">
          <p class="mx-auto mb-6 font-semibold tracking-widest">
            嬉汚らわしい電信柱が雀の一部であるプロレスラーの本質であるに違いない。虫かごが焼きそばを売りに出す。
          </p>
          <p class="mx-auto mb-6 font-semibold tracking-widest">
            筋肉はその物体のようにも見えることがある。
          </p>
          <p class="mx-auto mb-6 font-semibold tracking-widest">
            雀は強力に誕生すると発表されたが、結局は、ノアの箱船のごとき食物連鎖が帆船であると信じられていた。
          </p>
        </div>

        <div
          class="absolute top-20 right-44"
          style="
            background-color: rgba(0, 114, 225, 0.6);
            width: 9vw;
            height: 9vw;
          "
        ></div>
        <div
          class="absolute top-32 left-16 md:left-96"
          style="
            background-color: rgba(30, 144, 255, 0.2);
            width: 10vw;
            height: 10vw;
          "
        ></div>
        <div
          class="absolute top-40 left-32 md:-bottom-8 left-16 md:right-2"
          style="
            background-color: rgba(30, 144, 255, 0.1);
            width: 12vw;
            height: 12vw;
          "
        ></div>
      </div>

      <base-button name="メンバー一覧" link="member" />
    </section>

    <section class="w-10/12 md:w-full mx-auto">
      <base-heading title="Blog" sub="ブログ" />

      <div class="mb-14 w-full mx-auto">
        <a
          v-for="(item, index) in blogItems"
          :key="index"
          class="block mb-10 w-full md:w-7/12 mx-auto group"
          :href="`/blog/${item.id}/`"
        >
          <div
            class="
              flex
              items-center
              justify-between
              border-l-8 border-blue-400
              pl-2
              -ml-4
              mb-4
              group-hover:text-blue-500
            "
          >
            <img :src="item.image.url" class="w-2/12 shadow md:w-1/12" />
            <div class="mx-2 md:mx-10">
              <time
                class="text-xs md:text-lg block md:w-3/6 mb-2 flex items-center"
              >
                <fa :icon="faClock" class="mr-2" />{{ item.date | formatDate }}
              </time>
              <time class="text-xs md:text-lg block md:w-2/6 flex items-center">
                <fa :icon="faSyncAlt" class="mr-2" />{{
                  item.update | formatDate
                }}
              </time>
            </div>
            <div class="md:w-10/12 text-sm md:text-2xl">
              {{ item.title }}
            </div>
          </div>

          <hr class="border-b-2 -ml-4" />
        </a>
      </div>

      <base-button name="ブログ記事一覧" link="blog" />
    </section>
  </all-wrapper>
</template>

<script>
import { faClock, faSyncAlt } from '@fortawesome/free-solid-svg-icons'

import axios from 'axios'

export default {
  async asyncData({ $config }) {
    const info = await axios.get(
      `${$config.apiUrl}/information?filters=isRecent[equals]true`,
      {
        headers: { 'X-API-KEY': $config.apiKey },
      }
    )
    const blog = await axios.get(`${$config.apiUrl}/blog?limit=3`, {
      headers: { 'X-API-KEY': $config.apiKey },
    })
    return {
      infoItems: info.data.contents,
      blogItems: blog.data.contents,
    }
  },
  computed: {
    faClock() {
      return faClock
    },
    faSyncAlt() {
      return faSyncAlt
    },
  },
}
</script>

<style scoped>
.main-img {
  background-image: url('~@/assets/img/main01.webp');
  height: 70vh;
  @media screen and (max-width: 768px) {
    background-image: none;
    height: 55vh;
  }
}

.main-img2 {
  background-image: url('~@/assets/img/main02.webp');
  height: 50vh;
  background-size: cover;
  background-position: center;
}

section {
  padding: 30px 0 20px;
}

@media screen and (min-width: 768px) {
  section {
    padding: 50px 0;
  }
}
</style>
