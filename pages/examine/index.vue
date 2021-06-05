<template>
  <all-wrapper>
    <layout-main-visual
      title="診療内容"
      visual="visual-information"
      message="当院で行っている診療をご案内します。"
    />

    <!-- main -->
    <main class="w-full md:max-w-4xl mx-auto pt-4 px-6 md:px-0">

      <section class="mb-10 text-center">
        <p class="text-lg md:text-2xl text-center tracking-widest mb-5" style="color: orange">虫歯治療から予防歯科、審美歯科まで幅広く対応しています。</p>

        <p class="w-10/12 mx-auto md:text-lg mb-3 tracking-wide mt-8">ただいまページを作成中です。</p>
        <p class="w-10/12 mx-auto md:text-lg mb-3 tracking-wide">申し訳ありませんが、もうしばらくお待ちください。</p>
      </section>
    </main>

    <base-button
      name="HOMEに戻る"
      link="/"
    />
  </all-wrapper>

</template>

<style scoped>
main {
  min-height: 50vh;
}
</style>