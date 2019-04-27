<template>
  <div>
    <Messages />
    <MessageSend />
    <HelloWorld msg="Welcome to Your Vue.js App"/>
  </div>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/components/HelloWorld.vue'
import MessageSend from '@/components/message/Send.vue'
import Messages from '@/components/message/Messages.vue'

export default {
  name: 'home',
  components: {
    HelloWorld,
    MessageSend,
    Messages,
  }
}
</script>

