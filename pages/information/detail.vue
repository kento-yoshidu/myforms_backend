<template>
  <layout-wrapper>
    <layout-visual
      title="詳細ページ"
      :height="40"
      visual="visual-information"
    />

    <div class="w-full md:max-w-3xl mx-auto pt-20">
      <h2 class="text-xl pb-5 border-b border-gray-500 border-solid font-bold">
        お知らせタイトル
      </h2>

      <div class="pt-4 mb-4">
        <h3>見出し</h3>
        <p>テキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキステキストトトトトトトトトトトトトトトトトトトトトテキスト</p>
        <ul>
          <li>リスト</li>
          <li>リスト</li>
          <li>リスト</li>
        </ul>  
      </div>
    </div>

    <base-button name="お知らせへ戻る" link="/information/" />

  </layout-wrapper>
</template>