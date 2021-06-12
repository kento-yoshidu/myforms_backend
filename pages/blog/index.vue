<template>
  <all-wrapper>
    <layout-main-visual
      title="ブログ"
      visual="visual-blog"
    />

    <main class="border-2">
      <div class="flex flex-col border-2 w-10/12 md:w-6/12 mx-auto">
        <a
          v-for="(item, index) in items" :key="index"
          :href="`/blog/${item.id}/`"
          class="mb-10 text-center flex flex-start border-b-2"
        >
          <img
            :src="item.image.url"
            class="w-6/12 mx-auto md:w-2/12"
          />
          <div>
            <h3>{{ item.title }}</h3>
            <p>{{ item.date | formatDate }}</p>
            <p v-if="item.update">{{ item.update | formatDate }}</p>
          </div>
        </a>
      </div>

      <base-button
        name="HOME"
        link="/"
      />
    </main>

  </all-wrapper>
</template>

<script>
import axios from 'axios'
import BaseButton from '~/components/BaseButton.vue'

export default {
  components: { BaseButton },
  async asyncData({ $config }) {
    const { data } = await axios.get(`${$config.apiUrl}/blog`, {
      headers: { 'X-API-KEY': $config.apiKey }
    })
    return {
      items: data.contents,
    }
  }
}
</script>
