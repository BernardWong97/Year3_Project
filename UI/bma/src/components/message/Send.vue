<template>
    <div class="w3-container  w3-animate-bottom">
        <div class="w3-card-4 w3-white w3-round-xlarge w3-margin-top">
<!--            <div class="w3-container w3-dark-grey w3-center w3-round-xlarge">-->
<!--                <h2>Send Message</h2>-->
<!--            </div>-->

            <form @submit.prevent="sendMessage"
                  class="w3-container"
            action="">
<!--                <p>-->
<!--                    <label>User</label>-->
<!--                    <input class="w3-input"-->
<!--                           v-model="receiver"-->
<!--                           type="text"/>-->
<!--                </p>-->
                <p>
                    <label>Message</label>
                    <textarea class="w3-input"
                              v-model="text"
                              type="text"></textarea>
                </p>
                <p>
                    <button class="w3-btn w3-grey w3-round-large">Send</button>
                </p>
                <div class="w3-panel w3-display-container w3-round-xlarge"
                     v-if="msgShow"
                     v-bind:class="{ 'w3-green': isOk, 'w3-red': !isOk }"
                >
                    <span v-on:click="hideMessage"
                          class="w3-button w3-large w3-display-topright w3-border w3-round-large">&times;</span>
                    <h3 v-if="isOk">Success!</h3>
                    <h3 v-else>Fail!</h3>
                    <p>{{message}}</p>
                </div>
            </form>
        </div>
    </div>
</template>

<script>
    export default {
        name: "Send",
        data() {
            return {
                receiver: '',
                text: '',
                message: '',
                msgShow: false,
                isOk: false,
            }
        },

        // Add this section:
        methods : {
            sendMessage: function () {
                this.$api.postMessage({
                    message: {
                        receiver: this.receiver,
                        text: this.text,
                        value: 0,
                    }
                })
                    .then(res => {
                        console.log(res)
                        console.log("Message sent")
                        this.text = ''
                        // this.message = res.msg
                        this.message = "Message was send"
                        this.msgShow = true
                        this.isOk = true
                        console.log(res)
                    })
                    .catch(err => {
                        console.log(err)
                        this.message = err
                        // this.message = "Message send failed"
                        this.msgShow = true
                        this.isOk = false
                        console.log(err)
                    })

                // this.message = ''
            },
            hideMessage(){
                this.msgShow = false
            }

        },
        // async created () {
        //     const response = await this.$api.postMessage({
        //         message: {
        //             receiver: this.receiver,
        //             message: this.message,
        //             value: 0,
        //         }
        //     })
        //
        //     Console.log(response)
        // },

    }
</script>

<style scoped>

</style>