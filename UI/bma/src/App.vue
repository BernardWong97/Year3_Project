<template>
  <div id="app">
    <Header style="height:30%"/>
    <router-view/>
  </div>
</template>

<script>
  import Header from '@/components/Header.vue'

  export default {
    components: {
      Header,
    }
  }
</script>

<style>
    @import './assets/w3.css';
</style>
