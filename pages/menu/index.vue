<template>
  <layout-wrapper>
    <layout-visual title="MENU" :height="50" visual="visual-menu" />
    <div class="container w-7/12 mx-auto pt-20 pb-20 px-6 md:px-0">
      <div class="mb-20">
        <layout-menu-list />
      </div>
      <base-button name="トップへ戻る" link="/" />
    </div>
  </layout-wrapper>
</template>