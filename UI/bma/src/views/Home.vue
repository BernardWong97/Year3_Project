<template>
    <div >
        <div class="w3-card w3-dark-grey w3-center w3-round-xlarge w3-centre w3-margin-right w3-margin-left"
             v-on:click="showContacts"
        >
            <router-link :to="{ name: 'addressBook', params: { addressBook}}" >{{conversation.user}}</router-link>

        </div>
<!--        <button v-on:click="getMessages">get</button>-->
<!--        <Messages v-bind:data="messages.get(buddy)"/>-->
        <Messages v-bind:conversation="conversation"/>
        <MessageSend class="w3-bottom"/>
<!--        <HelloWorld msg="Welcome to Your Vue.js App"/>-->
    </div>
</template>

<script>
    // @ is an alias to /src
    import axios from 'axios'
    // import HelloWorld from '@/components/HelloWorld.vue'
    import MessageSend from '@/components/message/Send.vue'
    import Messages from '@/components/message/Messages.vue'

    export default {
        name: 'home',
        props: ["user"],
        components: {
            // HelloWorld,
            MessageSend,
            Messages,
        },
        data() {
            return {
                allMessages: [],
                messages: new Map(),
                conversation: {},
                appUser: String,
                addressBook: [],
            }
        },
        methods: {
            toMap({data}){
                const map = new Map()

                data.forEach(msg => {
                    // console.log(`${msg.sender} === ${this.appUser} => ${msg.sender === this.appUser}`)
                    if( msg.sender === this.appUser ){
                        // if user is a sender
                        if( map.has(msg.receiver) ){
                            map.get(msg.receiver).push(msg)
                        }
                        else{
                            map.set(msg.receiver, [msg])
                        }
                    }
                    else{
                        // user is receiver
                        if( map.has(msg.sender) ){
                            map.get(msg.sender).push(msg)
                        }
                        else{
                            map.set(msg.sender, [msg])
                        }
                    }
                })

                // console.log(`To map: ${map}`)
                // console.log(map)

                return map
            },
            conversationWith({user}){
                this.conversation = {
                    data: this.messages.get(user),
                    user: user,
                }
            },
            showContacts(){

            }
        },

        async created() {
            const resMessages = await axios.get('http://192.168.1.7:8001/app/message')
            const resAppInfo = await axios.get('http://192.168.1.7:8001/app/info')

            this.appUser = resAppInfo.data.username

            this.messages = this.toMap({data: resMessages.data})
            this.allMessages = resMessages.data
            if( !this.user ){ this.user = "Bernard" }
            this.conversationWith({user: this.user})
            this.addressBook = this.messages.keys()

            console.log(this.addressBook)
            // console.log(this.conversation)

        }
    }
</script>

