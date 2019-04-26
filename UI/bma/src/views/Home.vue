<template>
  <div>
    <MessageSend />
    <HelloWorld msg="Welcome to Your Vue.js App"/>
  </div>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/components/HelloWorld.vue'
import MessageSend from '@/components/message/Send.vue'

export default {
  name: 'home',
  components: {
    HelloWorld,
    MessageSend,
  }
}
</script>

