<template>
  <layout-wrapper>
    <layout-visual
      title="Information"
      :height="50"
      visual="visual-information"
    />

    <div class="w-full md:max-w-3xl mx-auto pt-20">

      <layout-information-list />

      <base-button name="トップへ戻る" link="/" />

    </div>

  </layout-wrapper>
</template>