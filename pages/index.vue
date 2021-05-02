<template>
  <all-wrapper>

    <layout-main-visual
      title="Nuxt Dental Clinic"
      message="hoeghoge"
    />

    <!-- main -->
    <main class="w-full md:max-w-4xl mx-auto pt-20 px-6 md:px-0">

      <section class="mb-10 text-center">
        <p class="text-2xl text-center tracking-widest mb-5" style="color: orange">鳩時計がすごい勇敢なロボットがマッシュルームの上で動作するえんどう豆を形づくる。</p>
        <p class="text-2xl text-center tracking-widest mb-20" style="color: orange">哲学がハングアップする星座がホームページ上で食事する。</p>

        <p class="w-10/12 mx-auto text-lg mb-3 tracking-wide">嬉しいことに、布団が踊る硫黄が遠ざかる気がした子猫を搭載したヴァイオリンが酒に発明される。</p>
        <p class="w-10/12 mx-auto text-lg mb-3 tracking-wide">つまるところ、蛙が勢い良く蒸気を吹き出す絆創膏のサポートを打ち切るまでもない。</p>
        <p class="w-10/12 mx-auto text-lg mb-3 tracking-wide">シャーロック・ホームズがドラゴンクエストテクノロジーとも呼ばれるはさみに恋文を送るなんてことはないが、マッシュルームが地獄から来た登山を持ち上げる。</p>
      </section>

      <section class="py-8">
        <base-heading>
          クリニックからのお知らせ
        </base-heading>
        <!-- お知らせ-->
        <div class="mb-20">
          <div
            class="bg-teal-100 border-t-4 border-teal-500 rounded-b text-teal-900 shadow-md mb-5"
          >
            <a href="/information/detail" class="block px-4 py-3">
              <time class="text-gray-700 text-base mb-1 block md:w-1/6">
                2020.08.15
              </time>
              <div class="md:w-10/12">お知らせタイトル</div>
            </a>
          </div>
          <div
            class="bg-teal-100 border-t-4 border-teal-500 rounded-b text-teal-900 shadow-md mb-5"
          >
            <a href="/information/detail" class="block px-4 py-3">
              <time class="text-gray-700 text-base mb-1 block md:w-1/6">
                2020.08.15
              </time>
              <div class="md:w-10/12">お知らせタイトル</div>
            </a>
          </div>
          <div
            class="bg-teal-100 border-t-4 border-teal-500 rounded-b text-teal-900 shadow-md mb-5"
          >
            <a href="/information/detail" class="block px-4 py-3">
              <time class="text-gray-700 text-base mb-1 block md:w-1/6">
                2020.08.15
              </time>
              <div class="md:w-10/12">お知らせタイトル</div>
            </a>
          </div>
        </div>

        <base-button
          name="お知らせを見る"
          link="information"
        />
      </section>

      <section class="border-2 py-8">
        <!-- 診療内容 -->
        <base-heading>
          診察内容
        </base-heading>

        <div class="flex flex-wrap justify-between mb-20 md:mb-0">
          <div class="md:w-56 mb-20 shadow-lg bg-gray-200">
            <div class="max-w">
              <img src="~/assets/img/visual-home.jpg" alt="商品名" class="w-full" />
              <div class="px-6 py-4">
                <div class="font-bold text-xl mb-2">アメリカンコーヒー</div>
                <p class="text-gray-700 text-base mb-4">480円（税込）</p>
              </div>
            </div>
          </div>
          <div class="md:w-56 mb-20 shadow-lg bg-gray-200">
            <div class="max-w">
              <img src="~/assets/img/visual-home.jpg" alt="商品名" class="w-full" />
              <div class="px-6 py-4">
                <div class="font-bold text-xl mb-2">アメリカンコーヒー</div>
                <p class="text-gray-700 text-base mb-4">480円（税込）</p>
              </div>
            </div>
          </div>
          <div class="md:w-56 mb-20 shadow-lg bg-gray-200">
            <div class="max-w">
              <img src="~/assets/img/visual-home.jpg" alt="商品名" class="w-full" />
              <div class="px-6 py-4">
                <div class="font-bold text-xl mb-2">アメリカンコーヒー</div>
                <p class="text-gray-700 text-base mb-4">480円（税込）</p>
              </div>
            </div>
          </div>
        </div>

        <base-button
          name="詳しく見る"
          link="information"
        />
      </section>
    </main>

  </all-wrapper>
</template>

<script></script>

<style>
.visual-home {
  background-image: url('~@/assets/img/visual-home.jpg');
}
</style>
