<template>
  <all-wrapper>
    <layout-main-visual
      title="Blog"
      height="50"
    />

    <main class="w-6/12 m-auto pt-20">

      <h1 class="text-4xl text-gray-600 text-center md:mb-12">
        {{ item.title }}
      </h1>

      <time class="text-gray-600 text-lg">
        {{ item.date | formatDate }}
      </time>

      <div
        class="md:mt-10 md:mb-20 border"
        v-html="item.body"
      ></div>

    </main> 
  </all-wrapper>
</template>

<script>
import axios from 'axios'
export default {
  async asyncData({ $config, params, error }) {
    try {
      const { data } = await axios.get(`${$config.apiUrl}/blog/${params.id}`,
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
  h1,h2,h3,p {
    color: #444;
  }
  h2 {
    margin: 20px 0 10px;
    font-size: 2rem;
  }
  p {
    margin-bottom: 5px;
    font-size: 1.2rem;
  }
</style>
