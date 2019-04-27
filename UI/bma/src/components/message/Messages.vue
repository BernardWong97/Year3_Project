<template>
    <div>
        <h1>Messages</h1>
        <button v-on:click="getMessages">get</button>
<!--        <p>{{allMessages}}</p>-->

        <div v-bind:key="msg.id.timestamp" v-for="msg in allMessages">
            <Message v-bind:data="msg"/>
        </div>
    </div>
</template>

<script>
    // import Message from '@/models/message'
    import Message from '@/components/message/Message.vue'

    export default {
        name: "Messages",
        components: {Message},
        comments: {
            Message,
        },
        data() {
            return {
                allMessages: [],

                isOk: false,
            }
        },
        methods: {
            getMessages(){
                this.$api.getMessages()
                    .then(res => {
                        this.allMessages = res.data
                        // this.allMessages = res.data.map(m => {
                        //     let sm = new Message({m})
                        //     console.log(sm)
                        // })
                        console.log(res)
                    })
                    .catch(err => console.log(err))
            }

        },
        // async created () {
        //     const res = await this.$api.getMessages()
        //
        //     this.allMessages = res.data
        //
        //     // this.$api.getMessages()
        //     //     .then(res => {
        //     //         this.allMessages = res.data
        //     //     })
        //     //     .catch(err => console.log(err))
        // },
    }
</script>

<style scoped>

</style>