<template>
    <div>
        <h1>Address Book</h1>
        <div v-bind:key="user.id" v-for="user in addressBook">
            <router-link :to="{ name: 'home', params: { user}}" >{{user}}</router-link>
        </div>
    </div>
</template>

<script>
    export default {
        name: "addressBook",
        props: ["addressBook"],
        async created(){
            await console.log(this.addressBook)
        }
    }
</script>

<style scoped>

</style>