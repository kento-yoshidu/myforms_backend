<template>
  <all-wrapper>
    <layout-main-visual
      title="Blog"
      height="50"
      visual="visual-blog"
    />

    <main class="w-10/12 md:w-6/12 mx-auto pt-4 md:pt-20">

      <h1 class="page-title text-4xl text-gray-600 text-center mb-4 md:mb-12">
        {{ item.title }}
      </h1>

      <time class="block text-gray-600 md:text-lg text-right">
        投稿日付　：　{{ item.date | formatDate }}
      </time>

      <time class="block text-gray-600 md:text-lg text-right">
        更新日付　：　{{ item.update | formatDate }}
      </time>

      <div
        class="text-body mt-6 md:mt-10 md:mb-20"
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
  .page-title {
    font-size: 3.5rem;
  }

  h1,h2,h3,p {
    color: #444;
  }
  .text-body h1 {
    font-size: 3rem;
    line-height: 1.6;
    border-bottom: 8px solid rgba(96, 165, 250, .9);
  }
  h2 {
    margin: 30px 0 10px;
    padding-left: 1rem;
    font-size: 2rem;
    border-left: 8px solid rgba(96, 165, 250, .9);
  }
  h3 {
    font-size: 1.8rem;
  }
  p {
    margin-bottom: 5px;
    font-size: 1.1rem;
    letter-spacing: 1px;
    line-height: 1.6;
  }

  @media screen and (max-width: 768px) {
    .page-title {
      font-size: 1.8rem;
    } 
    .text-body h1 {
      font-size: 2.1rem;
      line-height: 1.6;
      border-bottom: 7px solid rgba(96, 165, 250, .9);
    }
    h2 {
      margin: 15px 0 5px;
      padding-left: 1rem;
      font-size: 1.8rem;
      border-left: 8px solid rgba(96, 165, 250, .9);
    }
    h3 {
      font-size: 1.6rem;
    }
    p {
      margin-bottom: 5px;
      font-size: 1.1rem;
      letter-spacing: 1px;
      line-height: 1.6;
    }
  }
</style>
