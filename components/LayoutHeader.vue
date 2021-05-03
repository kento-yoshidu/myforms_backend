<template>
  <header class="w-full absolute md:static bg-white px-2 py-2 z-50">
    <div class="md:w-full mx-auto md:flex md:items-center">
      <div
        class="w-full md:flex mx-auto px-6 md:px-0 flex justify-around items-center h-16"
      >
        <h1>
          <a
            class="text-gray-500 text-3xl font-semibold leading-relaxed inline-block mr-4 py-2 whitespace-no-wrap"
            href="/"
          >
            Nuxt Dental Clinic
          </a>
        </h1>
        <div class="text-gray-900 md:hidden">
          <button
            class="focus:outline-none"
            @click="isOpen = !isOpen"
          >
            <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
              <path
                v-show="!isOpen"
                d="M24 6h-24v-4h24v4zm0 4h-24v4h24v-4zm0 8h-24v4h24v-4z"
              />
              <path
                v-show="isOpen"
                d="M24 20.188l-8.315-8.209 8.2-8.282-3.697-3.697-8.212 8.318-8.31-8.203-3.666 3.666 8.321 8.24-8.206 8.313 3.666 3.666 8.237-8.318 8.285 8.203z"
              />
            </svg>
          </button>
        </div>
      </div>

      <nav
        class="w-full md:block absolute left-0 md:static bg-white md:bg-none z-20"
        :class="isOpen ? 'block' : 'hidden'"
      >
        <ul class="md:flex md:justify-start md:items-end">
          <li class="w-full md:w-auto md:ml-5">
            <a
              href="/concept/"
              class="text-lg text-gray-500 md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
              >当院のポリシー</a
            >
          </li>
          <li class="w-full md:w-auto md:ml-5">
            <a
              href="/examine/"
              class="text-lg text-gray-500 md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
              >診察内容</a
            >
          </li>
          <li class="w-full md:w-auto md:ml-5">
            <a
              href="/information/"
              class="text-lg text-gray-500 md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
              >お知らせ</a
            >
          </li>
          <li class="w-full md:w-auto md:ml-5">
            <a
              href="/member/"
              class="text-lg text-gray-500 md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
              >メンバー</a
            >
          </li>
          <li class="w-full md:w-auto md:ml-5">
            <a
              href="/site/"
              class="text-lg text-gray-500 md:block inline-block md:py-0 py-5 px-5 md:px-0 w-full"
              >サイト構成</a
            >
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<script>
export default {
  name: 'LayoutHeader',
  data() {
    return {
      isOpen: false
    }
  }
}
</script>