<template>
    <div class="w3-card-4 w3-margin-top w3-margin-left w3-margin-right w3-round-xlarge w3-animate-bottom w3-center"
         style="width:70%"
         v-bind:class="{'w3-right': isUserMessage, 'w3-left': !isUserMessage}"
    >

<!--        <header class="w3-container w3-dark-grey w3-round-xlarge w3-center"-->
<!--        >-->
<!--            <h3>{{data.sender}}</h3>-->
<!--        </header>-->
        <div class="w3-container"
             v-bind:class="{'w3-right': isUserMessage, 'w3-left': !isUserMessage}"
        >
            <p class="w3-small">{{data.load}}</p>

        </div>
    </div>
</template>

<script>
    export default {
        name: "Message",
        props: ["data"],
        data() {
            return {
                isUserMessage: true,
            }
        },
        methods: { },
        created() {
            this.$api.getInfo()
                .then(res => res.data.username)
                .catch(err => console.log(err))
                .then(user => {
                    this.isUserMessage = user === this.data.sender;
                })
                .catch(err => console.log(err))
        }
    }
</script>

<style scoped>

</style>