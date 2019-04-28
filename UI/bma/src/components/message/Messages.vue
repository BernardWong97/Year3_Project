<template>
    <div>
<!--        <div class="w3-card w3-dark-grey w3-center w3-round-xlarge w3-centre w3-margin-right w3-margin-left" >-->
<!--            {{conversation.user}}-->
<!--        </div>-->
<!--        <button v-on:click="getMessages">get</button>-->
<!--        <p>user: {{$username}}</p>-->
<!--        <p>{{data}}</p>-->

        <div v-bind:key="msg.id.timestamp" v-for="msg in conversation.data">
            <Message v-bind:data="msg" />
        </div>
    </div>
</template>

<script>
    // import Message from '@/models/message'
    import Message from '@/components/message/Message.vue'

    export default {
        name: "Messages",
        components: {Message},
        props: ["conversation"],
        data() {
            return {
                // allMessages: [],
                // messages: new Map(),
                // isOk: false,
            }
        },
        // methods: {
        //     getMessages(){
        //         this.$api.getMessages()
        //             .then(res => {
        //                 this.allMessages = res.data
        //                 // this.allMessages = res.data.map(m => {
        //                 //     let sm = new Message({m})
        //                 //     console.log(sm)
        //                 // })
        //
        //                 // console.log(res)
        //                 this.toMap()
        //             })
        //             .catch(err => console.log(err))
        //     },
        //     toMap(){
        //         this.allMessages.forEach(msg => {
        //             if( msg.sender === this.$username ){
        //                 // if user is a sender
        //                 if( this.messages.has(msg.receiver) ){
        //                     this.messages.get(msg.receiver).push(msg)
        //                 }
        //                 else{
        //                     this.messages.set(msg.receiver, [msg])
        //                 }
        //             }
        //             else{
        //                 // user is receiver
        //                 if( this.messages.has(msg.sender) ){
        //                     this.messages.get(msg.sender).push(msg)
        //                 }
        //                 else{
        //                     this.messages.set(msg.sender, [msg])
        //                 }
        //             }
        //
        //             console.log(this.messages)
        //         })
        //     }
        //
        // },
        async created () {
            await console.log("Messages got:")
            await console.log(this.conversation)

            // const res = await this.$api.getMessages()

            // this.allMessages = res.data

            // this.$api.getMessages()
            //     .then(res => {
            //         this.allMessages = res.data
            //     })
            //     .catch(err => console.log(err))
        },
    }
</script>

<style scoped>

</style>