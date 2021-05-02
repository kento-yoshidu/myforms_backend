<template>
  <div>
    <header class="w-full absolute md:static bg-blue-300 px-2 py-2 z-50">
      <div class="md:max-w-3xl mx-auto md:flex md:items-center">
        <div
          class="w-full md:flex mx-auto px-6 md:px-0 flex justify-between items-center h-16"
        >
          <h1>
            <a
              class="text-white text-3xl font-semibold leading-relaxed inline-block mr-4 py-2 whitespace-no-wrap"
              href="/"
            >
              Nuxt Dental Clinic
            </a>
          </h1>
          <div class="text-white md:hidden">
            <button class="focus:outline-none">
              <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
                <path
                  d="M24 6h-24v-4h24v4zm0 4h-24v4h24v-4zm0 8h-24v4h24v-4z"
                />
                <path
                  d="M24 20.188l-8.315-8.209 8.2-8.282-3.697-3.697-8.212 8.318-8.31-8.203-3.666 3.666 8.321 8.24-8.206 8.313 3.666 3.666 8.237-8.318 8.285 8.203z"
                />
              </svg>
            </button>
          </div>
        </div>
        <nav
          class="w-full md:block absolute left-0 md:static bg-blue-300 md:bg-none z-20"
        >
          <ul class="md:flex md:justify-end md:items-end">
            <li class="w-full md:w-auto md:ml-5">
              <a
                href="/concept/"
                class="text-white md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
                >コンセプト</a
              >
            </li>
            <li class="w-full md:w-auto md:ml-5">
              <a
                href="/shop/"
                class="text-white md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
                >店舗情報</a
              >
            </li>
            <li class="w-full md:w-auto md:ml-5">
              <a
                href="/menu/"
                class="text-white md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
                >メニュー</a
              >
            </li>
            <li class="w-full md:w-auto md:ml-5">
              <a
                href="/information/"
                class="text-white md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
                >お知らせ</a
              >
            </li>
          </ul>
        </nav>
      </div>
    </header>

    <!-- top -->
    <div
      class="relative pt-16 pb-32 flex content-center items-center justify-center z-10"
      style="min-height: 75vh"
    >
      <div class="absolute top-0 w-full h-full bg-center bg-cover visual-home">
        <span
          id="blackOverlay"
          class="w-full h-full absolute opacity-75"
        ></span>
      </div>
      <div class="container relative mx-auto">
        <div class="items-center flex flex-wrap">
          <div class="w-full lg:w-6/12 px-4 ml-auto mr-auto text-center">
            <div>
              <h1 class="text-white font-semibold text-5xl text-shadow">
                Nuxt Dental Clinic
              </h1>
              <p class="mt-4 text-lg text-white text-shadow-md">
                お知らせやメニューをmicroCMSを導入したDEMOサイトになります。
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- main -->
    <main class="w-full md:max-w-3xl mx-auto pt-20 px-6 md:px-0">

      <section class="border-2 py-8">
        <!-- お知らせ-->
        <h2 class="font-sans text-lg text-gray-800 text-center text-3xl mb-10">
          クリニックからのお知らせ
        </h2>
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
        <div class="mb-10 mx-auto text-center">
          <a
            href="/information/"
            class="font-semibold md:text-lg xl:text-base px-4 md:px-5 xl:px-4 py-3 md:py-4 xl:py-3 leading-tight shadow-md rounded-lg bg-white hover:bg-gray-200 text-gray-800"
          >
            お知らせの一覧
          </a>
        </div>
      </section>

      <section class="border-2 py-8">
        <!-- 診療内容 -->
        <h2 class="font-sans text-lg text-gray-800 text-center text-3xl mb-10">
          診療内容
        </h2>
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
        <div class="mb-10 mx-auto text-center">
          <a
            href="/menu/"
            class="font-semibold md:text-lg xl:text-base px-4 md:px-5 xl:px-4 py-3 md:py-4 xl:py-3 leading-tight shadow-md rounded-lg bg-white hover:bg-gray-200 text-gray-800"
          >
            診察内容一覧
          </a>
        </div>
      </section>
    </main>

    <footer class="bg-blue-300 pb-10 pt-10">
      <p
        class="text-center text-white text-2xl font-semibold leading-relaxed whitespace-no-wrap"
      >
        Copyright © 2021 Nuxt Dental Clinic
      </p>
    </footer>
  </div>
</template>

<script></script>

<style>
.visual-home {
  background-image: url('~@/assets/img/visual-home.jpg');
}
</style>
