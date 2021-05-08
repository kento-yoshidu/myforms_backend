<template>
  <div>

  <h2>ブログリストA</h2>
    <p>{{title}}</p>
    <p>{{date}}</p>
  </div> 
</template>

<script>
export default {
  name: 'LayoutBlogList',
  props: {
    title: {
      type: String,
      required: true
    },
    date: {
      type: String,
      required: true
    },
    id: {
      type: String,
      required: true
    },
    picture: {
      type: String,
    }
  }
}
</script>