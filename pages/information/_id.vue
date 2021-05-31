<template>
  <all-wrapper>
    <layout-main-visual
      title="お知らせ"
    />

    <main class="w-6/12 m-auto pt-20">

      <base-heading>
        {{ item.title }}
      </base-heading>

      <time class="text-gray-700 text-base">
        {{ item.date | formatDate }}
      </time>

      <div class="mb-20" v-html="item.body"></div>

    </main> 
  </all-wrapper>
</template>

<script>
import axios from 'axios'
export default {
  async asyncData({ $config, params, error }) {
    try {
      const { data } = await axios.get(`${$config.apiUrl}/information/${params.id}`,
        {
          headers: { 'X-API-KEY': $config.apiKey }
        }
      )
      return {
        item: data
      }
    } catch (err) {
      error ({
        errorCode: 404,
      })
    }
  },
  head() {
    return {
      title: this.item.title,
    }
  }
}
</script>

<style scope>
  h2 {
    color: red;
  }
</style>