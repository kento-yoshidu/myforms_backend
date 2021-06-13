<template>
  <div>

    <layout-header />

    <Nuxt />

    <layout-footer />

  </div>
</template>

<style>
html {
  color: #444;
  font-size: 62.5%;
  line-height: 1.5;
  font-family: 'Source Sans Pro', -apple-system, BlinkMacSystemFont, 'Segoe UI',
    Roboto, 'Helvetica Neue', Arial, sans-serif;
}

main {
  min-height: 60vh;
}

@media screen and (min-width: 768px) {
  main {
    min-height: 50vh;
    padding-bottom: 8vh;
  } 
}
</style>