<template>
  <all-wrapper>
    <layout-main-visual
      title="Blog"
      visual="visual-blog"
    />

    <main class="w-10/12 md:w-6/12 mx-auto pt-2">

      <h1 class="page-title text-4xl text-gray-600 text-center mb-4 md:mb-12">
        {{ item.title }}
      </h1>

      <time class="block text-gray-600 md:text-lg text-right">
        投稿日付　：　{{ item.date | formatDate }}
      </time>

      <time
        class="block text-gray-600 md:text-lg text-right"
        v-if="item.update"
      >
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

      let next = null

      const resNext = await axios.get(`${$config.apiUrl}/blog/` + 
        '?limit=1' + 
        '&fields=id,title,date' +
        '&orders=-date' +
        '&filters=date[less_than]current.date', {
        headers: { 'X-API-KEY': $config.apiKey }
      })

      console.log(resNext.data)

      if ( resNext.data.title && resNext.data.title.length > 0 ) {
        next = resNext.data.title
      }

      console.log(next)

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
  },
}
</script>

<style scope>
  .text-body li {
    margin-left: 20px;
    list-style: circle;
  }
  .page-title {
    font-size: 3rem;
    line-height: 1.5;
  }

  h1,h2,h3,p {
    color: #444;
    line-height: 1.5;
  }
  .text-body h1 {
    font-size: 2.8rem;
    line-height: 1.6;
    border-bottom: 8px solid rgba(96, 165, 250, .9);
  }
  h2 {
    margin: 45px 0 8px;
    padding-left: 1rem;
    font-size: 2rem;
    border-left: 8px solid rgba(96, 165, 250, .9);
  }
  h3 {
    margin: 35px 0 0;
    font-size: 1.6rem;
  }
  p {
    margin-bottom: 5px;
    font-size: 1.1rem;
    letter-spacing: 1px;
    line-height: 1.6;
    text-indent: .7rem;
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
      margin: 30px 0 0px;
      padding-left: 1rem;
      font-size: 1.6rem;
      border-left: 8px solid rgba(96, 165, 250, .9);
    }
    h3 {
      font-size: 1.2rem;
    }
    p {
      margin-bottom: 5px;
      font-size: 1rem;
      letter-spacing: 1px;
      line-height: 1.6;
    }
  }
</style>
