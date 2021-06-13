<template>
  <header class="pt-4 md:pt-6 pb-3 md:pb-6">
    <div class="flex justify-between items-center px-10 md:px-20">
      <h1 class="text-2xl">
        <a
          href="/"
          class="text-3xl md:text-5xl hover:text-blue-400"
        >
          Nuxt Dental Clinic
        </a>
      </h1>

      <div class="hidden md:text-3xl md:text-right md:block">
        <p>滋賀県草津市追分南99-99-99</p>
        <tel>090-0000-0000</tel>
      </div>

      <div class="md:hidden">
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
      class="w-full md:block absolute left-0 md:static z-20 bg-white text-lg md:text-2xl"
      :class="isOpen ? 'block' : 'hidden'"
    >
      <ul class="md:flex md:justify-center md:items-end text-center pt-5">
        <li class="w-full md:w-auto border-t border-b md:border-l md:border-r md:border-t-0 md:border-b-0 p-1 md:p-3">
          <a
            href="/"
            class="inline-block md:block py-0 md:py-2 md:px-5 md:px-0 w-full"
          >
            <div class="mb-1">ホーム</div> 
            <div class="text-blue-400 font-bold">HOME</div> 
          </a >
        </li>

        <li class="w-full md:w-auto border-b md:border-r md:border-t-0 md:border-b-0 p-1 md:p-3">
          <a
            href="/examine/"
            class="md:block inline-block py-0 md:py-2 md:px-5 md:px-0 w-full"
          >
            <div class="mb-1">診療内容</div> 
            <div class="text-blue-400 font-bold">EXAMINE</div> 
          </a>
        </li>

        <li class="w-full md:w-auto border-b md:border-r md:border-t-0 md:border-b-0 p-1 md:p-3">
          <a
            href="/information/"
            class="md:block inline-block py-0 md:py-2 md:px-5 md:px-0 w-full"
          >
            <div class="mb-1">お知らせ</div> 
            <div class="text-blue-400 font-bold">INFORMATION</div> 
          </a>
        </li>

        <li class="w-full md:w-auto border-b md:border-r md:border-t-0 md:border-b-0 p-1 md:p-3">
          <a
            href="/member/"
            class="md:block inline-block py-0 md:py-2 md:px-5 md:px-0 w-full"

          >
            <div class="mb-1">メンバー</div> 
            <div class="text-blue-400 font-bold">MEMBER</div> 
          </a>
        </li>

        <li class="w-full md:w-auto border-b md:border-r md:border-t-0 md:border-b-0 p-1 md:p-3">
          <a
            href="/blog/"
            class="md:block inline-block py-0 md:py-2 md:px-5 md:px-0 w-full"
          >
            <div class="mb-1">ブログ</div> 
            <div class="text-blue-400 font-bold">BLOG</div> 
          </a>
        </li>
      </ul>
    </nav>
    
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